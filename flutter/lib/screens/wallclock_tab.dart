import 'dart:async';

import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'package:timezone/timezone.dart' as tz;

import '../services/settings_service.dart';

/// A curated set of IANA timezone names offered in the dropdown. The full
/// `timezone` database has several hundred entries; a short, well-known list
/// keeps the demo UI usable.
const List<String> kAvailableTimezones = [
  'UTC',
  'Europe/London',
  'Europe/Berlin',
  'Europe/Moscow',
  'America/New_York',
  'America/Los_Angeles',
  'America/Sao_Paulo',
  'Asia/Dubai',
  'Asia/Kolkata',
  'Asia/Shanghai',
  'Asia/Tokyo',
  'Australia/Sydney',
  'Pacific/Auckland',
];

/// Wallclock tab: shows the current date and time in a user-selectable,
/// persisted timezone.
class WallclockTab extends StatefulWidget {
  const WallclockTab({super.key, this.settingsService});

  /// Overridable for tests; defaults to a real [SettingsService] otherwise.
  final SettingsService? settingsService;

  @override
  State<WallclockTab> createState() => _WallclockTabState();
}

class _WallclockTabState extends State<WallclockTab> {
  late final SettingsService _settings;
  late final Timer _timer;
  String _timezoneName = SettingsService.defaultTimezone;
  DateTime _now = DateTime.now();
  bool _settingsLoaded = false;

  @override
  void initState() {
    super.initState();
    _settings = widget.settingsService ?? SettingsService();
    _loadTimezone();
    _timer = Timer.periodic(const Duration(seconds: 1), (_) {
      setState(() => _now = DateTime.now());
    });
  }

  Future<void> _loadTimezone() async {
    final saved = await _settings.loadTimezone();
    if (!mounted) return;
    setState(() {
      _timezoneName = saved;
      _settingsLoaded = true;
    });
  }

  Future<void> _onTimezoneChanged(String? value) async {
    if (value == null) return;
    setState(() => _timezoneName = value);
    await _settings.saveTimezone(value);
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  static String _twoDigits(int n) => n.toString().padLeft(2, '0');

  /// A small clock-face touch: the `:` separators blink on/off once a
  /// second, like a classic digital clock, instead of the time just sitting
  /// there static.
  Widget _buildAnimatedTime(BuildContext context, tz.TZDateTime zoned) {
    final style = Theme.of(context).textTheme.displayMedium;
    final blinkOn = zoned.second.isEven;
    final colon = AnimatedOpacity(
      duration: const Duration(milliseconds: 300),
      opacity: blinkOn ? 1 : 0.2,
      child: Text(':', style: style),
    );

    return Row(
      key: const Key('wallclock_time'),
      mainAxisSize: MainAxisSize.min,
      children: [
        Text(_twoDigits(zoned.hour), style: style),
        colon,
        Text(_twoDigits(zoned.minute), style: style),
        colon,
        Text(_twoDigits(zoned.second), style: style),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    // The bundled tz database registers this zone as "Etc/UTC"; the "UTC"
    // alias isn't present, so look it up via the dedicated constant instead.
    final location = _timezoneName == 'UTC'
        ? tz.UTC
        : tz.getLocation(_timezoneName);
    final zoned = tz.TZDateTime.from(_now, location);
    final dateText = DateFormat('EEEE, d MMMM y').format(zoned);

    return Center(
      child: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            _buildAnimatedTime(context, zoned),
            const SizedBox(height: 8),
            Text(
              dateText,
              key: const Key('wallclock_date'),
              style: Theme.of(context).textTheme.titleMedium,
            ),
            const SizedBox(height: 32),
            if (_settingsLoaded)
              DropdownButton<String>(
                key: const Key('wallclock_timezone_dropdown'),
                value: _timezoneName,
                items: kAvailableTimezones
                    .map(
                      (name) =>
                          DropdownMenuItem(value: name, child: Text(name)),
                    )
                    .toList(),
                onChanged: _onTimezoneChanged,
              ),
          ],
        ),
      ),
    );
  }
}
