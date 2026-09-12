import 'package:shared_preferences/shared_preferences.dart';

/// Persists user-configurable settings (currently: the wallclock timezone)
/// across app restarts using the platform's key-value store.
class SettingsService {
  static const String _timezoneKey = 'selected_timezone';
  static const String defaultTimezone = 'UTC';

  Future<String> loadTimezone() async {
    final prefs = await SharedPreferences.getInstance();
    return prefs.getString(_timezoneKey) ?? defaultTimezone;
  }

  Future<void> saveTimezone(String timezoneName) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setString(_timezoneKey, timezoneName);
  }
}
