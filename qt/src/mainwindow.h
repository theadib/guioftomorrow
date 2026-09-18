#pragma once

#include <QFont>
#include <QMainWindow>

#include "settings.h"

class MainWindow : public QMainWindow {
    Q_OBJECT
public:
    explicit MainWindow(QWidget* parent = nullptr);

private slots:
    void onSettingsChanged();
    void onSystemColorSchemeChanged();

private:
    clockapp::AppSettings m_settings;
    QFont m_baseFont;
};
