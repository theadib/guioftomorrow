#include "stopwatch_tab.h"

#include <QDateTime>
#include <QDir>
#include <QFileDialog>
#include <QFont>
#include <QHBoxLayout>
#include <QLabel>
#include <QListWidget>
#include <QPushButton>
#include <QStandardPaths>
#include <QTimer>
#include <QVBoxLayout>

namespace clockapp::ui {

StopwatchTab::StopwatchTab(QWidget* parent) : QWidget(parent) {
    m_elapsedLabel = new QLabel(clockapp::stopwatch::format_elapsed(0), this);
    QFont font = m_elapsedLabel->font();
    font.setPointSizeF(font.pointSizeF() * 2.0);
    m_elapsedLabel->setFont(font);

    m_startStopButton = new QPushButton(tr("Start"), this);
    m_lapButton = new QPushButton(tr("Lap"), this);
    m_lapButton->setEnabled(false);
    m_resetButton = new QPushButton(tr("Reset"), this);
    m_exportButton = new QPushButton(tr("Export laps to file..."), this);
    m_statusLabel = new QLabel(this);
    m_statusLabel->setWordWrap(true);
    m_lapsList = new QListWidget(this);

    auto* buttonRow = new QHBoxLayout();
    buttonRow->addWidget(m_startStopButton);
    buttonRow->addWidget(m_lapButton);
    buttonRow->addWidget(m_resetButton);
    buttonRow->addWidget(m_exportButton);
    buttonRow->addStretch();

    auto* layout = new QVBoxLayout(this);
    layout->addWidget(m_elapsedLabel);
    layout->addLayout(buttonRow);
    layout->addWidget(m_statusLabel);
    layout->addWidget(new QLabel(tr("Laps"), this));
    layout->addWidget(m_lapsList, 1);

    connect(m_startStopButton, &QPushButton::clicked, this, &StopwatchTab::onStartStop);
    connect(m_lapButton, &QPushButton::clicked, this, &StopwatchTab::onLap);
    connect(m_resetButton, &QPushButton::clicked, this, &StopwatchTab::onReset);
    connect(m_exportButton, &QPushButton::clicked, this, &StopwatchTab::onExport);

    auto* timer = new QTimer(this);
    connect(timer, &QTimer::timeout, this, &StopwatchTab::refresh);
    timer->start(50);
}

qint64 StopwatchTab::elapsedMs() const {
    qint64 elapsed = m_accumulatedMs;
    if (m_running) elapsed += m_elapsedTimer.elapsed();
    return elapsed;
}

void StopwatchTab::onStartStop() {
    if (!m_running) {
        m_running = true;
        m_elapsedTimer.start();
        m_startStopButton->setText(tr("Stop"));
        m_lapButton->setEnabled(true);
    } else {
        m_accumulatedMs = elapsedMs();
        m_running = false;
        m_startStopButton->setText(tr("Start"));
        m_lapButton->setEnabled(false);
    }
    refresh();
}

void StopwatchTab::onLap() {
    const qint64 total = elapsedMs();
    const int index = m_laps.size() + 1;
    const qint64 split = total - m_lastLapTotalMs;
    m_laps.push_back(clockapp::stopwatch::Lap{index, split, total});
    m_lastLapTotalMs = total;

    m_lapsList->insertItem(0, tr("Lap %1   split %2   total %3")
                                   .arg(index)
                                   .arg(clockapp::stopwatch::format_elapsed(split))
                                   .arg(clockapp::stopwatch::format_elapsed(total)));
}

void StopwatchTab::onReset() {
    m_running = false;
    m_accumulatedMs = 0;
    m_lastLapTotalMs = 0;
    m_laps.clear();
    m_lapsList->clear();
    m_statusLabel->clear();
    m_startStopButton->setText(tr("Start"));
    m_lapButton->setEnabled(false);
    refresh();
}

void StopwatchTab::onExport() {
    const qint64 total = elapsedMs();
    const QString text = clockapp::stopwatch::build_export_text(m_laps, total);

    const QString dir = QStandardPaths::writableLocation(QStandardPaths::DocumentsLocation);
    const QString suggested = QDir(dir).filePath(
        clockapp::stopwatch::default_export_file_name(QDateTime::currentDateTime()));

    const QString path =
        QFileDialog::getSaveFileName(this, tr("Export laps"), suggested, tr("Text files (*.txt)"));
    if (path.isEmpty()) return;  // user cancelled

    const bool ok = clockapp::stopwatch::write_export_file(path, text);
    m_statusLabel->setText(ok ? tr("Exported to %1").arg(path)
                              : tr("Failed to write %1").arg(path));
}

void StopwatchTab::refresh() {
    m_elapsedLabel->setText(clockapp::stopwatch::format_elapsed(elapsedMs()));
}

}  // namespace clockapp::ui
