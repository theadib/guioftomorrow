import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:flutter_clock/screens/stopwatch_tab.dart';

void main() {
  Future<void> pumpStopwatch(WidgetTester tester) async {
    await tester.pumpWidget(
      const MaterialApp(home: Scaffold(body: StopwatchTab())),
    );
  }

  testWidgets('start, lap and reset drive the button states', (
    WidgetTester tester,
  ) async {
    await pumpStopwatch(tester);

    final startStop = find.byKey(const Key('stopwatch_start_stop'));
    final lap = find.byKey(const Key('stopwatch_lap'));
    final reset = find.byKey(const Key('stopwatch_reset'));
    final export = find.byKey(const Key('stopwatch_export'));

    expect(tester.widget<ElevatedButton>(lap).onPressed, isNull);
    expect(tester.widget<ElevatedButton>(export).onPressed, isNull);
    expect(find.text('Start'), findsOneWidget);

    await tester.tap(startStop);
    await tester.pump();
    expect(find.text('Stop'), findsOneWidget);
    expect(tester.widget<ElevatedButton>(lap).onPressed, isNotNull);
    expect(tester.widget<ElevatedButton>(reset).onPressed, isNull);

    await tester.pump(const Duration(milliseconds: 100));
    await tester.tap(lap);
    await tester.pump();
    expect(find.textContaining('Lap 00:00:00'), findsOneWidget);

    await tester.tap(startStop);
    await tester.pump();
    expect(find.text('Start'), findsOneWidget);
    expect(tester.widget<ElevatedButton>(reset).onPressed, isNotNull);
    expect(tester.widget<ElevatedButton>(export).onPressed, isNotNull);

    await tester.tap(reset);
    await tester.pump();
    expect(find.byKey(const Key('stopwatch_status')), findsNothing);
    expect(tester.widget<ElevatedButton>(export).onPressed, isNull);

    await tester.pumpWidget(const SizedBox.shrink());
  });

  test('formatStopwatchDuration pads to hh:mm:ss.mmm', () {
    expect(
      formatStopwatchDuration(
        const Duration(hours: 1, minutes: 2, seconds: 3, milliseconds: 45),
      ),
      '01:02:03.045',
    );
    expect(formatStopwatchDuration(Duration.zero), '00:00:00.000');
  });

  test('buildExportText reports every lap in order', () {
    final laps = [
      const LapRecord(
        lapNumber: 1,
        lapDuration: Duration(seconds: 1),
        totalDuration: Duration(seconds: 1),
      ),
      const LapRecord(
        lapNumber: 2,
        lapDuration: Duration(seconds: 2),
        totalDuration: Duration(seconds: 3),
      ),
    ];

    final text = buildExportText(laps);

    expect(text, contains('Lap\tLap time\tTotal time'));
    expect(text, contains('1\t00:00:01.000\t00:00:01.000'));
    expect(text, contains('2\t00:00:02.000\t00:00:03.000'));
  });
}
