import 'package:flutter/material.dart';

import '../services/app_settings_controller.dart';
import '../services/settings_service.dart';

/// Settings tab: GUI theme (system/light/dark), contrast (normal/high) and
/// a 50%-200% UI scale — all persisted via [AppSettingsController].
class SettingsTab extends StatelessWidget {
  const SettingsTab({super.key, required this.controller});

  final AppSettingsController controller;

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: controller,
      builder: (context, _) {
        if (!controller.loaded) {
          return const Center(child: CircularProgressIndicator());
        }

        final theme = Theme.of(context);
        final scalePercent = (controller.uiScale * 100).round();

        return ListView(
          padding: const EdgeInsets.all(24),
          children: [
            Text('GUI theme', style: theme.textTheme.titleLarge),
            const SizedBox(height: 12),
            SegmentedButton<ThemeMode>(
              key: const Key('settings_theme_mode'),
              segments: const [
                ButtonSegment(
                  value: ThemeMode.system,
                  label: Text('System'),
                  icon: Icon(Icons.brightness_auto),
                ),
                ButtonSegment(
                  value: ThemeMode.light,
                  label: Text('Light'),
                  icon: Icon(Icons.light_mode),
                ),
                ButtonSegment(
                  value: ThemeMode.dark,
                  label: Text('Dark'),
                  icon: Icon(Icons.dark_mode),
                ),
              ],
              selected: {controller.themeMode},
              onSelectionChanged: (selection) =>
                  controller.setThemeMode(selection.first),
            ),
            const SizedBox(height: 32),
            Text('Contrast', style: theme.textTheme.titleLarge),
            const SizedBox(height: 12),
            SegmentedButton<bool>(
              key: const Key('settings_contrast'),
              segments: const [
                ButtonSegment(value: false, label: Text('Normal')),
                ButtonSegment(value: true, label: Text('High contrast')),
              ],
              selected: {controller.highContrast},
              onSelectionChanged: (selection) =>
                  controller.setHighContrast(selection.first),
            ),
            const SizedBox(height: 32),
            Text('UI scale', style: theme.textTheme.titleLarge),
            Text(
              '$scalePercent%',
              key: const Key('settings_scale_label'),
              style: theme.textTheme.bodyLarge,
            ),
            Slider(
              key: const Key('settings_scale_slider'),
              value: controller.uiScale,
              min: SettingsService.minUiScale,
              max: SettingsService.maxUiScale,
              divisions: 15,
              label: '$scalePercent%',
              onChanged: controller.setUiScale,
            ),
          ],
        );
      },
    );
  }
}
