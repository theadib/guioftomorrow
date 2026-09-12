import 'package:flutter/material.dart';
import 'package:timezone/data/latest.dart' as tz_data;

import 'screens/stopwatch_tab.dart';
import 'screens/synctime_tab.dart';
import 'screens/wallclock_tab.dart';

void main() {
  tz_data.initializeTimeZones();
  runApp(const ClockApp());
}

class ClockApp extends StatelessWidget {
  const ClockApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'GUI of Tomorrow — Flutter',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.teal),
        useMaterial3: true,
      ),
      home: const ClockHomePage(),
    );
  }
}

class ClockHomePage extends StatefulWidget {
  const ClockHomePage({super.key});

  @override
  State<ClockHomePage> createState() => _ClockHomePageState();
}

class _ClockHomePageState extends State<ClockHomePage>
    with SingleTickerProviderStateMixin {
  late final TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 3, vsync: this);
  }

  @override
  void dispose() {
    _tabController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('GUI of Tomorrow — Flutter'),
        bottom: TabBar(
          controller: _tabController,
          tabs: const [
            Tab(icon: Icon(Icons.public), text: 'Wallclock'),
            Tab(icon: Icon(Icons.timer_outlined), text: 'Stopwatch'),
            Tab(icon: Icon(Icons.sync), text: 'Synctime'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: const [WallclockTab(), StopwatchTab(), SynctimeTab()],
      ),
    );
  }
}
