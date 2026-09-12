import 'dart:async';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';

/// A single recorded lap: its own duration and the cumulative elapsed time
/// at the moment it was recorded.
class LapRecord {
  const LapRecord({
    required this.lapNumber,
    required this.lapDuration,
    required this.totalDuration,
  });

  final int lapNumber;
  final Duration lapDuration;
  final Duration totalDuration;
}

String formatStopwatchDuration(Duration duration) {
  String two(int n) => n.toString().padLeft(2, '0');
  String three(int n) => n.toString().padLeft(3, '0');

  final hours = duration.inHours;
  final minutes = duration.inMinutes.remainder(60);
  final seconds = duration.inSeconds.remainder(60);
  final millis = duration.inMilliseconds.remainder(1000);
  return '${two(hours)}:${two(minutes)}:${two(seconds)}.${three(millis)}';
}

/// Builds the plain-text report exported by the stopwatch tab. Kept as a
/// pure function (no file I/O) so it can be unit tested directly.
String buildExportText(List<LapRecord> laps) {
  final buffer = StringBuffer()
    ..writeln('Stopwatch recording exported at ${DateTime.now().toIso8601String()}')
    ..writeln('Lap\tLap time\tTotal time');
  for (final lap in laps) {
    buffer.writeln(
      '${lap.lapNumber}\t'
      '${formatStopwatchDuration(lap.lapDuration)}\t'
      '${formatStopwatchDuration(lap.totalDuration)}',
    );
  }
  return buffer.toString();
}

/// Stopwatch tab: start/stop/reset/lap plus exporting the recorded laps as a
/// text file.
class StopwatchTab extends StatefulWidget {
  const StopwatchTab({super.key});

  @override
  State<StopwatchTab> createState() => _StopwatchTabState();
}

class _StopwatchTabState extends State<StopwatchTab> {
  final Stopwatch _stopwatch = Stopwatch();
  Timer? _ticker;
  final List<LapRecord> _laps = [];
  Duration _lastLapTotal = Duration.zero;
  String? _statusMessage;

  bool get _isRunning => _stopwatch.isRunning;

  void _toggleRunning() {
    setState(() {
      if (_isRunning) {
        _stopwatch.stop();
        _ticker?.cancel();
      } else {
        _stopwatch.start();
        _ticker = Timer.periodic(
          const Duration(milliseconds: 30),
          (_) => setState(() {}),
        );
      }
    });
  }

  void _recordLap() {
    if (!_isRunning) return;
    setState(() {
      final total = _stopwatch.elapsed;
      final lapDuration = total - _lastLapTotal;
      _lastLapTotal = total;
      _laps.add(
        LapRecord(
          lapNumber: _laps.length + 1,
          lapDuration: lapDuration,
          totalDuration: total,
        ),
      );
    });
  }

  void _reset() {
    setState(() {
      _stopwatch.stop();
      _stopwatch.reset();
      _ticker?.cancel();
      _laps.clear();
      _lastLapTotal = Duration.zero;
      _statusMessage = null;
    });
  }

  Future<void> _export() async {
    final text = buildExportText(_laps);
    try {
      // Application-support (not "Documents") is used because it doesn't
      // depend on xdg-user-dirs being configured, which isn't guaranteed on
      // minimal Linux setups.
      final dir = await getApplicationSupportDirectory();
      final fileName =
          'stopwatch_export_${DateTime.now().millisecondsSinceEpoch}.txt';
      final file = File('${dir.path}/$fileName');
      await file.writeAsString(text);
      if (!mounted) return;
      setState(() => _statusMessage = 'Exported to ${file.path}');
    } catch (error) {
      if (!mounted) return;
      setState(() => _statusMessage = 'Export failed: $error');
    }
  }

  @override
  void dispose() {
    _ticker?.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        children: [
          const SizedBox(height: 16),
          Text(
            formatStopwatchDuration(_stopwatch.elapsed),
            key: const Key('stopwatch_time'),
            style: Theme.of(context).textTheme.displayMedium,
          ),
          const SizedBox(height: 16),
          Wrap(
            alignment: WrapAlignment.center,
            spacing: 12,
            children: [
              ElevatedButton(
                key: const Key('stopwatch_start_stop'),
                onPressed: _toggleRunning,
                child: Text(_isRunning ? 'Stop' : 'Start'),
              ),
              ElevatedButton(
                key: const Key('stopwatch_lap'),
                onPressed: _isRunning ? _recordLap : null,
                child: const Text('Lap'),
              ),
              ElevatedButton(
                key: const Key('stopwatch_reset'),
                onPressed: !_isRunning &&
                        (_stopwatch.elapsed > Duration.zero || _laps.isNotEmpty)
                    ? _reset
                    : null,
                child: const Text('Reset'),
              ),
              ElevatedButton(
                key: const Key('stopwatch_export'),
                onPressed: _laps.isNotEmpty ? _export : null,
                child: const Text('Export'),
              ),
            ],
          ),
          if (_statusMessage != null) ...[
            const SizedBox(height: 8),
            Text(_statusMessage!, key: const Key('stopwatch_status')),
          ],
          const SizedBox(height: 16),
          Expanded(
            child: ListView.builder(
              key: const Key('stopwatch_lap_list'),
              itemCount: _laps.length,
              itemBuilder: (context, index) {
                final lap = _laps[_laps.length - 1 - index];
                return ListTile(
                  dense: true,
                  leading: Text('#${lap.lapNumber}'),
                  title: Text('Lap ${formatStopwatchDuration(lap.lapDuration)}'),
                  trailing: Text(
                    'Total ${formatStopwatchDuration(lap.totalDuration)}',
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}
