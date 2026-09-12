import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:timezone/data/latest.dart' as tz_data;

import 'package:flutter_clock/main.dart';

void main() {
  setUpAll(() {
    tz_data.initializeTimeZones();
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('shows all three tabs and switches between them', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(const ClockApp());
    await tester.pump();

    expect(find.text('Wallclock'), findsOneWidget);
    expect(find.text('Stopwatch'), findsOneWidget);
    expect(find.text('Synctime'), findsOneWidget);

    // Wallclock is the initial tab.
    expect(find.byKey(const Key('wallclock_time')), findsOneWidget);

    await tester.tap(find.text('Stopwatch'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('stopwatch_time')), findsOneWidget);

    await tester.tap(find.text('Synctime'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('synctime_text')), findsOneWidget);

    // Unmount so the periodic timers started by each tab are cancelled
    // before the test ends.
    await tester.pumpWidget(const SizedBox.shrink());
  });
}
