#include "mainwindow.h"

#include <QApplication>
#include <QGuiApplication>
#include <QStyleHints>
#include <QTabWidget>

#include "tabs/settings_tab.h"
#include "tabs/stopwatch_tab.h"
#include "tabs/synctime_tab.h"
#include "tabs/wallclock_tab.h"
#include "theming.h"

MainWindow::MainWindow(QWidget* parent)
    : QMainWindow(parent), m_settings(clockapp::load_settings()), m_baseFont(QApplication::font()) {
    setWindowTitle(tr("GUI of Tomorrow - Qt Widgets clock demonstrator"));
    resize(900, 640);

    auto* tabs = new QTabWidget(this);
    auto* wallclockTab = new clockapp::ui::WallclockTab(m_settings, this);
    auto* stopwatchTab = new clockapp::ui::StopwatchTab(this);
    auto* synctimeTab = new clockapp::ui::SynctimeTab(this);
    auto* settingsTab = new clockapp::ui::SettingsTab(m_settings, this);

    tabs->addTab(wallclockTab, tr("Wallclock"));
    tabs->addTab(stopwatchTab, tr("Stopwatch"));
    tabs->addTab(synctimeTab, tr("Synctime"));
    tabs->addTab(settingsTab, tr("Settings"));
    setCentralWidget(tabs);

    connect(wallclockTab, &clockapp::ui::WallclockTab::settingsChanged, this,
            &MainWindow::onSettingsChanged);
    connect(settingsTab, &clockapp::ui::SettingsTab::settingsChanged, this,
            &MainWindow::onSettingsChanged);
    // Real, push-based OS theme following (Qt 6.5+) -- unlike the imgui/C
    // contester, which has no such API and has to poll `gsettings` every
    // 2 seconds instead. See ../OVERVIEW.md and this project's README.
    connect(QGuiApplication::styleHints(), &QStyleHints::colorSchemeChanged, this,
            &MainWindow::onSystemColorSchemeChanged);

    clockapp::ui::apply_style(m_settings, m_baseFont);
}

void MainWindow::onSettingsChanged() {
    clockapp::save_settings(m_settings);
    clockapp::ui::apply_style(m_settings, m_baseFont);
}

void MainWindow::onSystemColorSchemeChanged() {
    if (m_settings.theme == clockapp::Theme::System) {
        clockapp::ui::apply_style(m_settings, m_baseFont);
    }
}
