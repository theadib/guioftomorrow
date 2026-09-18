#pragma once

#include <QWidget>

#include "../settings.h"

QT_BEGIN_NAMESPACE
class QComboBox;
class QSlider;
class QLabel;
QT_END_NAMESPACE

namespace clockapp::ui {

class SettingsTab : public QWidget {
    Q_OBJECT
public:
    explicit SettingsTab(clockapp::AppSettings& settings, QWidget* parent = nullptr);

signals:
    void settingsChanged();

private slots:
    void onThemeChanged(int index);
    void onContrastChanged(int index);
    void onScaleChanged(int value);

private:
    clockapp::AppSettings& m_settings;
    QComboBox* m_themeCombo;
    QComboBox* m_contrastCombo;
    QSlider* m_scaleSlider;
    QLabel* m_scaleLabel;
};

}  // namespace clockapp::ui
