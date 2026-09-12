import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:flutter_clock/screens/synctime_tab.dart';

void main() {
  testWidgets('renders the two-arc dial with the current time centred', (
    WidgetTester tester,
  ) async {
    await tester.pumpWidget(
      const MaterialApp(home: Scaffold(body: SynctimeTab())),
    );
    await tester.pump();

    expect(find.byType(CustomPaint), findsWidgets);
    expect(find.byKey(const Key('synctime_text')), findsOneWidget);

    final text = tester.widget<Text>(find.byKey(const Key('synctime_text')));
    expect(text.data, matches(RegExp(r'^\d{2}:\d{2}:\d{2}$')));

    // Unmount to cancel the periodic redraw timer before the test ends.
    await tester.pumpWidget(const SizedBox.shrink());
  });
}
