#include <QTimeZone>
#include <QtTest>

#include "tabs/wallclock.h"

using namespace clockapp::wallclock;

class TstWallclock : public QObject {
    Q_OBJECT
private slots:
    void timeInZoneConvertsUtcCorrectly();
    void unknownZoneFallsBackToUtc();
    void formatClockZeroPadsAndTogglesSeparator();
    void formatDateProducesExpectedString();
};

void TstWallclock::timeInZoneConvertsUtcCorrectly() {
    // 2023-11-14 22:13:19 UTC
    const QDateTime utc = QDateTime::fromSecsSinceEpoch(1'699'999'999, QTimeZone::UTC);
    const QDateTime local = time_in_zone(QStringLiteral("UTC"), utc);
    QCOMPARE(local.time().hour(), 22);
    QCOMPARE(local.time().minute(), 13);
    QCOMPARE(local.time().second(), 19);
}

void TstWallclock::unknownZoneFallsBackToUtc() {
    const QDateTime utc = QDateTime::fromSecsSinceEpoch(1'699'999'999, QTimeZone::UTC);
    const QDateTime local = time_in_zone(QStringLiteral("Not/AZone"), utc);
    QCOMPARE(local.time(), utc.time());
}

void TstWallclock::formatClockZeroPadsAndTogglesSeparator() {
    const QDateTime t(QDate(2026, 1, 1), QTime(3, 7, 9));
    QCOMPARE(format_clock(t, true), QStringLiteral("03:07:09"));
    QCOMPARE(format_clock(t, false), QStringLiteral("03 07 09"));
}

void TstWallclock::formatDateProducesExpectedString() {
    const QDateTime t(QDate(2026, 9, 18), QTime(0, 0, 0));
    QVERIFY(format_date(t).startsWith(QStringLiteral("2026-09-18")));
}

QTEST_MAIN(TstWallclock)
#include "tst_wallclock.moc"
