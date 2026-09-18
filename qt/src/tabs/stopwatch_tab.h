#pragma once

#include <QElapsedTimer>
#include <QVector>
#include <QWidget>

#include "stopwatch.h"

QT_BEGIN_NAMESPACE
class QLabel;
class QPushButton;
class QListWidget;
QT_END_NAMESPACE

namespace clockapp::ui {

class StopwatchTab : public QWidget {
    Q_OBJECT
public:
    explicit StopwatchTab(QWidget* parent = nullptr);

private slots:
    void onStartStop();
    void onLap();
    void onReset();
    void onExport();
    void refresh();

private:
    qint64 elapsedMs() const;

    bool m_running = false;
    QElapsedTimer m_elapsedTimer;
    qint64 m_accumulatedMs = 0;
    qint64 m_lastLapTotalMs = 0;
    QVector<clockapp::stopwatch::Lap> m_laps;

    QLabel* m_elapsedLabel;
    QPushButton* m_startStopButton;
    QPushButton* m_lapButton;
    QPushButton* m_resetButton;
    QPushButton* m_exportButton;
    QListWidget* m_lapsList;
    QLabel* m_statusLabel;
};

}  // namespace clockapp::ui
