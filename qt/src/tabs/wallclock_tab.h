#pragma once

#include <QWidget>

#include "../settings.h"

QT_BEGIN_NAMESPACE
class QLabel;
class QComboBox;
QT_END_NAMESPACE

namespace clockapp::ui {

class WallclockTab : public QWidget {
    Q_OBJECT
public:
    explicit WallclockTab(clockapp::AppSettings& settings, QWidget* parent = nullptr);

signals:
    void settingsChanged();

private slots:
    void tick();
    void onTimezoneChanged(const QString& zone);

private:
    clockapp::AppSettings& m_settings;
    QLabel* m_clockLabel;
    QLabel* m_dateLabel;
    QComboBox* m_timezoneCombo;
};

}  // namespace clockapp::ui
