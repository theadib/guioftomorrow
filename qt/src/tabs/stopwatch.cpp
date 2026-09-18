#include "stopwatch.h"

#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QTextStream>

namespace clockapp::stopwatch {

QString format_elapsed(qint64 elapsed_ms) {
    const qint64 hours_part = elapsed_ms / 3'600'000;
    const qint64 minutes_part = (elapsed_ms / 60'000) % 60;
    const qint64 seconds_part = (elapsed_ms / 1000) % 60;
    const qint64 millis_part = elapsed_ms % 1000;

    if (hours_part > 0) {
        return QStringLiteral("%1:%2:%3.%4")
            .arg(hours_part)
            .arg(minutes_part, 2, 10, QLatin1Char('0'))
            .arg(seconds_part, 2, 10, QLatin1Char('0'))
            .arg(millis_part, 3, 10, QLatin1Char('0'));
    }
    return QStringLiteral("%1:%2.%3")
        .arg(minutes_part, 2, 10, QLatin1Char('0'))
        .arg(seconds_part, 2, 10, QLatin1Char('0'))
        .arg(millis_part, 3, 10, QLatin1Char('0'));
}

QString build_export_text(const QVector<Lap>& laps, qint64 total_ms) {
    QString out;
    QTextStream stream(&out);
    stream << "Stopwatch export (Qt Widgets contester)\n";
    stream << "========================================\n";
    if (laps.isEmpty()) {
        stream << "(no laps recorded)\n";
    } else {
        for (const auto& lap : laps) {
            stream << "Lap " << lap.index << ": split " << format_elapsed(lap.split_ms)
                   << ", total " << format_elapsed(lap.total_at_lap_ms) << "\n";
        }
    }
    stream << "----------------------------------------\n";
    stream << "Total elapsed: " << format_elapsed(total_ms) << "\n";
    return out;
}

QString default_export_file_name(const QDateTime& now) {
    return QStringLiteral("stopwatch_export_%1.txt")
        .arg(now.toString(QStringLiteral("yyyyMMdd_HHmmss")));
}

bool write_export_file(const QString& path, const QString& text) {
    QDir().mkpath(QFileInfo(path).absolutePath());
    QFile file(path);
    if (!file.open(QIODevice::WriteOnly | QIODevice::Truncate | QIODevice::Text)) return false;
    QTextStream stream(&file);
    stream << text;
    return true;
}

}  // namespace clockapp::stopwatch
