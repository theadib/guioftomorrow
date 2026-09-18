#pragma once

#include <QDateTime>
#include <QString>
#include <QVector>

#include <QtGlobal>

namespace clockapp::stopwatch {

struct Lap {
    int index;
    qint64 split_ms;        // time since the previous lap
    qint64 total_at_lap_ms;  // time since start when recorded
};

// Formats a duration as "MM:SS.mmm", or "H:MM:SS.mmm" once it reaches an
// hour -- pure formatting, no wall-clock/system-time dependency.
QString format_elapsed(qint64 elapsed_ms);

// Builds the plain-text lap report written out by the "Export" button.
QString build_export_text(const QVector<Lap>& laps, qint64 total_ms);

// Default filename offered by the export file dialog, e.g.
// "stopwatch_export_20260918_153000.txt". `now` is injected so the naming
// scheme is independently testable.
QString default_export_file_name(const QDateTime& now);

// Writes `text` to `path`, creating the parent directory if needed.
// Returns false if the file could not be written.
bool write_export_file(const QString& path, const QString& text);

}  // namespace clockapp::stopwatch
