#include "wallclock.h"

#include <QTimeZone>

namespace clockapp::wallclock {

QDateTime time_in_zone(const QString& tz, const QDateTime& utc_now) {
    QTimeZone zone(tz.toUtf8());
    if (!zone.isValid()) zone = QTimeZone::UTC;
    return utc_now.toTimeZone(zone);
}

QString format_clock(const QDateTime& local_time, bool colon_visible) {
    const QChar sep = colon_visible ? QLatin1Char(':') : QLatin1Char(' ');
    const QTime t = local_time.time();
    return QStringLiteral("%1%2%3%4%5")
        .arg(t.hour(), 2, 10, QLatin1Char('0'))
        .arg(sep)
        .arg(t.minute(), 2, 10, QLatin1Char('0'))
        .arg(sep)
        .arg(t.second(), 2, 10, QLatin1Char('0'));
}

QString format_date(const QDateTime& local_time) {
    return local_time.date().toString(QStringLiteral("yyyy-MM-dd (dddd)"));
}

}  // namespace clockapp::wallclock
