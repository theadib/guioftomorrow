import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:timezone/data/latest.dart' as tz_data;

import 'package:flutter_clock/main.dart';
import 'package:flutter_clock/services/app_settings_controller.dart';
import 'package:flutter_clock/services/settings_service.dart';

void main() {
  late AppSettingsController controller;

  setUpAll(() {
    tz_data.initializeTimeZones();
  });

  setUp(() {
    SharedPreferences.setMockInitialValues({});
    controller = AppSettingsController();
  });

  Future<void> pumpApp(WidgetTester tester) async {
    await tester.pumpWidget(ClockApp(settingsController: controller));
    await tester.pump();
    await tester.tap(find.text('Settings'));
    await tester.pumpAndSettle();
  }

  testWidgets('defaults to system theme, normal contrast, 100% scale', (
    WidgetTester tester,
  ) async {
    await pumpApp(tester);

    final app = tester.widget<MaterialApp>(find.byType(MaterialApp));
    expect(app.themeMode, ThemeMode.system);
    expect(find.text('100%'), findsOneWidget);

    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('picking Dark updates the MaterialApp themeMode and persists', (
    WidgetTester tester,
  ) async {
    await pumpApp(tester);

    await tester.tap(find.text('Dark'));
    await tester.pumpAndSettle();

    final app = tester.widget<MaterialApp>(find.byType(MaterialApp));
    expect(app.themeMode, ThemeMode.dark);
    expect(await SettingsService().loadThemeMode(), ThemeMode.dark);

    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('toggling high contrast persists', (WidgetTester tester) async {
    await pumpApp(tester);

    await tester.tap(find.text('High contrast'));
    await tester.pumpAndSettle();

    expect(controller.highContrast, isTrue);
    expect(await SettingsService().loadHighContrast(), isTrue);

    await tester.pumpWidget(const SizedBox.shrink());
  });

  testWidgets('changing the UI scale rescales text app-wide and persists', (
    WidgetTester tester,
  ) async {
    await pumpApp(tester);

    final slider = tester.widget<Slider>(
      find.byKey(const Key('settings_scale_slider')),
    );
    slider.onChanged!(2.0);
    await tester.pumpAndSettle();

    expect(find.text('200%'), findsOneWidget);
    final context = tester.element(find.text('Settings').first);
    expect(MediaQuery.of(context).textScaler.scale(10), 20);
    expect(await SettingsService().loadUiScale(), 2.0);

    await tester.pumpWidget(const SizedBox.shrink());
  });
}
