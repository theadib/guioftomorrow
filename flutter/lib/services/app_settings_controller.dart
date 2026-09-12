import 'package:flutter/material.dart';

import 'settings_service.dart';

/// Holds the app-wide, persisted appearance settings (theme mode, contrast,
/// UI scale) shown on the Settings tab and consumed by [ClockApp][] to
/// rebuild `MaterialApp`.
///
/// [ClockApp]: ../main.dart
class AppSettingsController extends ChangeNotifier {
  AppSettingsController({SettingsService? settingsService})
    : _settings = settingsService ?? SettingsService();

  final SettingsService _settings;

  ThemeMode _themeMode = SettingsService.defaultThemeMode;
  bool _highContrast = SettingsService.defaultHighContrast;
  double _uiScale = SettingsService.defaultUiScale;
  bool _loaded = false;

  ThemeMode get themeMode => _themeMode;
  bool get highContrast => _highContrast;
  double get uiScale => _uiScale;

  /// False until the persisted values have been loaded once at startup.
  bool get loaded => _loaded;

  Future<void> load() async {
    _themeMode = await _settings.loadThemeMode();
    _highContrast = await _settings.loadHighContrast();
    _uiScale = await _settings.loadUiScale();
    _loaded = true;
    notifyListeners();
  }

  Future<void> setThemeMode(ThemeMode mode) async {
    if (mode == _themeMode) return;
    _themeMode = mode;
    notifyListeners();
    await _settings.saveThemeMode(mode);
  }

  Future<void> setHighContrast(bool value) async {
    if (value == _highContrast) return;
    _highContrast = value;
    notifyListeners();
    await _settings.saveHighContrast(value);
  }

  Future<void> setUiScale(double value) async {
    final clamped = value.clamp(
      SettingsService.minUiScale,
      SettingsService.maxUiScale,
    );
    if (clamped == _uiScale) return;
    _uiScale = clamped;
    notifyListeners();
    await _settings.saveUiScale(clamped);
  }
}
