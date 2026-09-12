import 'package:flutter/material.dart';
import 'package:timezone/data/latest.dart' as tz_data;

import 'screens/settings_tab.dart';
import 'screens/stopwatch_tab.dart';
import 'screens/synctime_tab.dart';
import 'screens/wallclock_tab.dart';
import 'services/app_settings_controller.dart';

void main() {
  tz_data.initializeTimeZones();
  runApp(const ClockApp());
}

class ClockApp extends StatefulWidget {
  const ClockApp({super.key, this.settingsController});

  /// Overridable for tests; defaults to a real controller otherwise.
  final AppSettingsController? settingsController;

  @override
  State<ClockApp> createState() => _ClockAppState();
}

class _ClockAppState extends State<ClockApp> {
  late final AppSettingsController _settings;

  @override
  void initState() {
    super.initState();
    _settings = widget.settingsController ?? AppSettingsController();
    _settings.addListener(_onSettingsChanged);
    _settings.load();
  }

  void _onSettingsChanged() => setState(() {});

  @override
  void dispose() {
    _settings.removeListener(_onSettingsChanged);
    super.dispose();
  }

  ThemeData _buildTheme(Brightness brightness) {
    return ThemeData(
      colorScheme: ColorScheme.fromSeed(
        seedColor: Colors.teal,
        brightness: brightness,
        contrastLevel: _settings.highContrast ? 1 : 0,
      ),
      useMaterial3: true,
    );
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'GUI of Tomorrow — Flutter',
      themeMode: _settings.themeMode,
      theme: _buildTheme(Brightness.light),
      darkTheme: _buildTheme(Brightness.dark),
      builder: (context, child) {
        final mediaQuery = MediaQuery.of(context);
        return MediaQuery(
          data: mediaQuery.copyWith(
            textScaler: TextScaler.linear(_settings.uiScale),
          ),
          child: child!,
        );
      },
      home: ClockHomePage(settingsController: _settings),
    );
  }
}

class ClockHomePage extends StatefulWidget {
  const ClockHomePage({super.key, required this.settingsController});

  final AppSettingsController settingsController;

  @override
  State<ClockHomePage> createState() => _ClockHomePageState();
}

class _ClockHomePageState extends State<ClockHomePage>
    with SingleTickerProviderStateMixin {
  late final TabController _tabController;

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 4, vsync: this);
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
          isScrollable: true,
          tabs: const [
            Tab(icon: Icon(Icons.public), text: 'Wallclock'),
            Tab(icon: Icon(Icons.timer_outlined), text: 'Stopwatch'),
            Tab(icon: Icon(Icons.sync), text: 'Synctime'),
            Tab(icon: Icon(Icons.settings), text: 'Settings'),
          ],
        ),
      ),
      body: TabBarView(
        controller: _tabController,
        children: [
          const WallclockTab(),
          const StopwatchTab(),
          const SynctimeTab(),
          SettingsTab(controller: widget.settingsController),
        ],
      ),
    );
  }
}
