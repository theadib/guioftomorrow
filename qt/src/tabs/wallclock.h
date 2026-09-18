#pragma once

#include <QDateTime>
#include <QString>

namespace clockapp::wallclock {

// Converts a UTC instant into local wall-clock time in the given IANA zone.
// Uses Qt's own timezone database (QTimeZone/QDateTime::toTimeZone), so
// unlike the imgui/C contester's setenv("TZ", ...)+tzset() approach, this
// touches no process-global state and needs no follow-up localtime_r call.
// Falls back to UTC if `tz` isn't a zone Qt recognizes.
QDateTime time_in_zone(const QString& tz, const QDateTime& utc_now);

// Formats hh:mm:ss, replacing the ':' separators with a space when
// `colon_visible` is false -- this is toggled once a second by the caller
// to produce the blinking-separator animation.
QString format_clock(const QDateTime& local_time, bool colon_visible);

// Formats the date portion, e.g. "2026-09-18 (Friday)".
QString format_date(const QDateTime& local_time);

}  // namespace clockapp::wallclock
