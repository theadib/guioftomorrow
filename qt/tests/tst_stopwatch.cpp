#include <QFile>
#include <QTemporaryDir>
#include <QtTest>

#include "tabs/stopwatch.h"

using namespace clockapp::stopwatch;

class TstStopwatch : public QObject {
    Q_OBJECT
private slots:
    void formatElapsedSubHour();
    void formatElapsedPastOneHour();
    void buildExportTextReportsLapsAndTotal();
    void buildExportTextHandlesEmptyLaps();
    void defaultExportFileNameIncludesTimestamp();
    void writeExportFileRoundTrips();
};

void TstStopwatch::formatElapsedSubHour() {
    QCOMPARE(format_elapsed(0), QStringLiteral("00:00.000"));
    QCOMPARE(format_elapsed(65'432), QStringLiteral("01:05.432"));
}

void TstStopwatch::formatElapsedPastOneHour() {
    QCOMPARE(format_elapsed(3'661'000), QStringLiteral("1:01:01.000"));
}

void TstStopwatch::buildExportTextReportsLapsAndTotal() {
    const QVector<Lap> laps = {Lap{1, 1000, 1000}, Lap{2, 500, 1500}};
    const QString report = build_export_text(laps, 1500);
    QVERIFY(report.contains(QStringLiteral("Lap 1")));
    QVERIFY(report.contains(QStringLiteral("Lap 2")));
    QVERIFY(report.contains(format_elapsed(1500)));
}

void TstStopwatch::buildExportTextHandlesEmptyLaps() {
    const QString report = build_export_text({}, 0);
    QVERIFY(report.contains(QStringLiteral("no laps recorded")));
}

void TstStopwatch::defaultExportFileNameIncludesTimestamp() {
    const QDateTime t(QDate(2024, 1, 2), QTime(3, 24, 5));
    QCOMPARE(default_export_file_name(t), QStringLiteral("stopwatch_export_20240102_032405.txt"));
}

void TstStopwatch::writeExportFileRoundTrips() {
    QTemporaryDir dir;
    QVERIFY(dir.isValid());
    const QString path = dir.filePath(QStringLiteral("export.txt"));
    QVERIFY(write_export_file(path, QStringLiteral("hello")));

    QFile file(path);
    QVERIFY(file.open(QIODevice::ReadOnly | QIODevice::Text));
    QCOMPARE(QString::fromUtf8(file.readAll()), QStringLiteral("hello"));
}

QTEST_MAIN(TstStopwatch)
#include "tst_stopwatch.moc"
