import 'dart:async';
import 'dart:math' as math;

import 'package:flutter/material.dart';

/// Synctime tab: two arcs sweep continuously around a shared centre — the
/// outer one completing a full rotation every second, the inner one every
/// minute — with the current `hh:mm:ss` printed in the middle. Comparing the
/// arcs of two computers side by side makes clock drift visible at a glance.
class SynctimeTab extends StatefulWidget {
  const SynctimeTab({super.key});

  @override
  State<SynctimeTab> createState() => _SynctimeTabState();
}

class _SynctimeTabState extends State<SynctimeTab> {
  late final Timer _timer;
  DateTime _now = DateTime.now();

  @override
  void initState() {
    super.initState();
    // ~30 fps is smooth enough for the sweep while staying light-weight.
    _timer = Timer.periodic(const Duration(milliseconds: 33), (_) {
      setState(() => _now = DateTime.now());
    });
  }

  @override
  void dispose() {
    _timer.cancel();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final timeText =
        '${_now.hour.toString().padLeft(2, '0')}:'
        '${_now.minute.toString().padLeft(2, '0')}:'
        '${_now.second.toString().padLeft(2, '0')}';

    return Center(
      child: AspectRatio(
        aspectRatio: 1,
        child: Padding(
          padding: const EdgeInsets.all(24),
          child: CustomPaint(
            painter: _SynctimePainter(
              now: _now,
              color: Theme.of(context).colorScheme.primary,
            ),
            child: Center(
              child: Text(
                timeText,
                key: const Key('synctime_text'),
                style: Theme.of(context).textTheme.headlineMedium,
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _SynctimePainter extends CustomPainter {
  _SynctimePainter({required this.now, required this.color});

  final DateTime now;
  final Color color;

  static const double _trailSweep = math.pi / 3; // 60 degree comet trail

  @override
  void paint(Canvas canvas, Size size) {
    final center = size.center(Offset.zero);
    final radius = size.shortestSide / 2;

    final millis = now.millisecondsSinceEpoch;
    final secondFraction = (millis % 1000) / 1000;
    final minuteFraction = (millis % 60000) / 60000;
    const startOffset = -math.pi / 2; // 12 o'clock

    final secondAngle = startOffset + secondFraction * 2 * math.pi;
    final minuteAngle = startOffset + minuteFraction * 2 * math.pi;

    final secondRadius = radius * 0.85;
    final minuteRadius = radius * 0.6;

    final trackPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2
      ..color = color.withValues(alpha: 0.15);
    canvas.drawCircle(center, secondRadius, trackPaint);
    canvas.drawCircle(center, minuteRadius, trackPaint);

    final secondPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 8
      ..strokeCap = StrokeCap.round
      ..color = color;
    canvas.drawArc(
      Rect.fromCircle(center: center, radius: secondRadius),
      secondAngle - _trailSweep,
      _trailSweep,
      false,
      secondPaint,
    );

    final minutePaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 8
      ..strokeCap = StrokeCap.round
      ..color = color.withValues(alpha: 0.55);
    canvas.drawArc(
      Rect.fromCircle(center: center, radius: minuteRadius),
      minuteAngle - _trailSweep,
      _trailSweep,
      false,
      minutePaint,
    );
  }

  @override
  bool shouldRepaint(covariant _SynctimePainter oldDelegate) =>
      oldDelegate.now != now || oldDelegate.color != color;
}
