import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:timezone/data/latest.dart' as tz_data;

import 'package:flutter_clock/screens/wallclock_tab.dart';
import 'package:flutter_clock/services/settings_service.dart';

void main() {
  setUpAll(() {
    tz_data.initializeTimeZones();
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  Future<void> pumpWallclock(WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(home: Scaffold(body: WallclockTab())),
    );
    await tester.pump();
  }

  testWidgets('shows the current time and defaults to UTC', (
    WidgetTester tester,
  ) async {
    await pumpWallclock(tester);

    expect(find.byKey(const Key('wallclock_time')), findsOneWidget);
    final dropdown = tester.widget<DropdownButton<String>>(
      find.byKey(const Key('wallclock_timezone_dropdown')),
    );
    expect(dropdown.value, SettingsService.defaultTimezone);

    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('persists the selected timezone across rebuilds', (
    WidgetTester tester,
  ) async {
    await pumpWallclock(tester);

    // Drive the dropdown's onChanged callback directly rather than the
    // popup menu overlay: it exercises the same persistence logic without
    // depending on exact popup-item hit-test geometry.
    final dropdown = tester.widget<DropdownButton<String>>(
      find.byKey(const Key('wallclock_timezone_dropdown')),
    );
    dropdown.onChanged!('Asia/Tokyo');
    await tester.pump();

    final updatedDropdown = tester.widget<DropdownButton<String>>(
      find.byKey(const Key('wallclock_timezone_dropdown')),
    );
    expect(updatedDropdown.value, 'Asia/Tokyo');

    final settings = SettingsService();
    expect(await settings.loadTimezone(), 'Asia/Tokyo');

    await tester.pumpWidget(const SizedBox.shrink());
  });
}
