#include <QtTest>

#include "tabs/synctime.h"

using namespace clockapp::synctime;

class TstSynctime : public QObject {
    Q_OBJECT
private slots:
    void fractionOfPeriodWrapsCorrectly();
    void angleForFractionMapsTurnsToRadians();
    void pointOnCirclePlacesCardinalPoints();
};

void TstSynctime::fractionOfPeriodWrapsCorrectly() {
    QCOMPARE(fraction_of_period(0.0, 1.0), 0.0);
    QCOMPARE(fraction_of_period(0.25, 1.0), 0.25);
    QCOMPARE(fraction_of_period(1.25, 1.0), 0.25);
    QCOMPARE(fraction_of_period(90.0, 60.0), 0.5);
}

void TstSynctime::angleForFractionMapsTurnsToRadians() {
    constexpr double kPi = 3.14159265358979323846;
    QCOMPARE(angle_for_fraction(0.0), 0.0);
    QVERIFY(qFuzzyCompare(angle_for_fraction(0.25), kPi / 2.0));
    QVERIFY(qFuzzyCompare(angle_for_fraction(1.0), 2.0 * kPi));
}

void TstSynctime::pointOnCirclePlacesCardinalPoints() {
    constexpr double kPi = 3.14159265358979323846;
    const Point center{0.0, 0.0};

    // qFuzzyCompare() breaks down comparing against exactly zero, so nudge
    // both sides by +1.0 for the coordinates expected to land at zero.
    const auto p0 = point_on_circle(center, 10.0, 0.0);  // top
    QVERIFY(qFuzzyCompare(p0.x + 1.0, 1.0));
    QVERIFY(qFuzzyCompare(p0.y, -10.0));

    const auto p90 = point_on_circle(center, 10.0, kPi / 2.0);  // right
    QVERIFY(qFuzzyCompare(p90.x, 10.0));
    QVERIFY(qFuzzyCompare(p90.y + 1.0, 1.0));

    const auto p180 = point_on_circle(center, 10.0, kPi);  // bottom
    QVERIFY(qFuzzyCompare(p180.x + 1.0, 1.0));
    QVERIFY(qFuzzyCompare(p180.y, 10.0));
}

QTEST_MAIN(TstSynctime)
#include "tst_synctime.moc"
