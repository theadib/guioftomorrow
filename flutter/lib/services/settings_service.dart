import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Persists user-configurable settings (timezone, theme, contrast, UI scale)
/// across app restarts using the platform's key-value store.
class SettingsService {
  static const String _timezoneKey = 'selected_timezone';
  static const String _themeModeKey = 'theme_mode';
  static const String _highContrastKey = 'high_contrast';
  static const String _uiScaleKey = 'ui_scale';

  static const String defaultTimezone = 'UTC';
  static const ThemeMode defaultThemeMode = ThemeMode.system;
  static const bool defaultHighContrast = false;
  static const double defaultUiScale = 1;
  static const double minUiScale = 0.5;
  static const double maxUiScale = 2;

  Future<String> loadTimezone() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString(_timezoneKey) ?? defaultTimezone;
  }

  Future<void> saveTimezone(String timezoneName) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_timezoneKey, timezoneName);
  }

  Future<ThemeMode> loadThemeMode() async {
    final prefs = await SharedPreferences.getInstance();
    return switch (prefs.getString(_themeModeKey)) {
      'light' => ThemeMode.light,
      'dark' => ThemeMode.dark,
      _ => defaultThemeMode,
    };
  }

  Future<void> saveThemeMode(ThemeMode mode) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_themeModeKey, mode.name);
  }

  Future<bool> loadHighContrast() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getBool(_highContrastKey) ?? defaultHighContrast;
  }

  Future<void> saveHighContrast(bool value) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setBool(_highContrastKey, value);
  }

  /// A UI scale factor applied to text (and icon) sizes app-wide, between
  /// [minUiScale] (50%) and [maxUiScale] (200%).
  Future<double> loadUiScale() async {
    final prefs = await SharedPreferences.getInstance();
    final value = prefs.getDouble(_uiScaleKey) ?? defaultUiScale;
    return value.clamp(minUiScale, maxUiScale);
  }

  Future<void> saveUiScale(double value) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setDouble(_uiScaleKey, value.clamp(minUiScale, maxUiScale));
  }
}
