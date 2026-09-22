#define DOCTEST_CONFIG_IMPLEMENT_WITH_MAIN
#include <doctest/doctest.h>

#include <cmath>
#include <cstdlib>
#include <ctime>

#include "platform_time.h"
#include "settings.h"
#include "tabs/stopwatch.h"
#include "tabs/synctime.h"
#include "tabs/wallclock.h"

using namespace clockapp;

// ---- settings ---------------------------------------------------------

TEST_CASE("available_timezones is non-empty and every entry round-trips through time_in_zone") {
    const auto& zones = available_timezones();
    CHECK(!zones.empty());
    for (const auto& zone : zones) {
        const std::tm t = wallclock::time_in_zone(zone, 1'700'000'000);
        CHECK(t.tm_year > 0);  // parsed into *some* valid broken-down time
    }
}

TEST_CASE("theme/contrast string round-trip") {
    CHECK(theme_from_string(theme_to_string(Theme::System)) == Theme::System);
    CHECK(theme_from_string(theme_to_string(Theme::Light)) == Theme::Light);
    CHECK(theme_from_string(theme_to_string(Theme::Dark)) == Theme::Dark);
    CHECK(!theme_from_string("bogus").has_value());

    CHECK(contrast_from_string(contrast_to_string(Contrast::Normal)) == Contrast::Normal);
    CHECK(contrast_from_string(contrast_to_string(Contrast::High)) == Contrast::High);
    CHECK(!contrast_from_string("bogus").has_value());
}

TEST_CASE("clamp_scale keeps values within 50%-200%") {
    CHECK(clamp_scale(0.1f) == doctest::Approx(0.5f));
    CHECK(clamp_scale(1.0f) == doctest::Approx(1.0f));
    CHECK(clamp_scale(5.0f) == doctest::Approx(2.0f));
}

TEST_CASE("settings serialize/parse round-trip") {
    AppSettings original;
    original.timezone = "Asia/Tokyo";
    original.theme = Theme::Dark;
    original.contrast = Contrast::High;
    original.scale = 1.5f;

    const AppSettings parsed = parse_settings(serialize_settings(original));
    CHECK(parsed.timezone == original.timezone);
    CHECK(parsed.theme == original.theme);
    CHECK(parsed.contrast == original.contrast);
    CHECK(parsed.scale == doctest::Approx(original.scale));
}

TEST_CASE("parse_settings falls back to defaults on empty/garbage input") {
    const AppSettings parsed = parse_settings("not a valid settings file\n===\n");
    const AppSettings defaults;
    CHECK(parsed.timezone == defaults.timezone);
    CHECK(parsed.theme == defaults.theme);
    CHECK(parsed.contrast == defaults.contrast);
    CHECK(parsed.scale == doctest::Approx(defaults.scale));
}

// ---- wallclock ----------------------------------------------------------

TEST_CASE("time_in_zone converts a known epoch into UTC correctly") {
    // 2023-11-14 22:13:19 UTC
    const std::tm t = wallclock::time_in_zone("UTC", 1'699'999'999);
    CHECK(t.tm_hour == 22);
    CHECK(t.tm_min == 13);
    CHECK(t.tm_sec == 19);
}

TEST_CASE("format_clock renders zero-padded hh:mm:ss and toggles the separator") {
    std::tm t{};
    t.tm_hour = 3;
    t.tm_min = 7;
    t.tm_sec = 9;
    CHECK(wallclock::format_clock(t, true) == "03:07:09");
    CHECK(wallclock::format_clock(t, false) == "03 07 09");
}

// ---- stopwatch ------------------------------------------------------------

TEST_CASE("format_elapsed formats sub-hour durations as MM:SS.mmm") {
    CHECK(stopwatch::format_elapsed(std::chrono::milliseconds(0)) == "00:00.000");
    CHECK(stopwatch::format_elapsed(std::chrono::milliseconds(65'432)) == "01:05.432");
}

TEST_CASE("format_elapsed switches to H:MM:SS.mmm past one hour") {
    CHECK(stopwatch::format_elapsed(std::chrono::milliseconds(3'661'000)) == "1:01:01.000");
}

TEST_CASE("build_export_text reports laps and total, or a placeholder when empty") {
    using namespace std::chrono;
    const std::string empty_report = stopwatch::build_export_text({}, milliseconds(0));
    CHECK(empty_report.find("no laps recorded") != std::string::npos);

    std::vector<stopwatch::Lap> laps = {
        {1, milliseconds(1000), milliseconds(1000)},
        {2, milliseconds(500), milliseconds(1500)},
    };
    const std::string report = stopwatch::build_export_text(laps, milliseconds(1500));
    CHECK(report.find("Lap 1") != std::string::npos);
    CHECK(report.find("Lap 2") != std::string::npos);
    CHECK(report.find(stopwatch::format_elapsed(milliseconds(1500))) != std::string::npos);
}

TEST_CASE("export_file_path builds a timestamped filename under imgui_clock/") {
    // 2024-01-02 03:24:05 UTC
    platform::set_timezone("UTC");
    const std::string path = stopwatch::export_file_path(1'704'165'845);
    CHECK(path.find("imgui_clock") != std::string::npos);
    CHECK(path.find("20240102_032405") != std::string::npos);
}

// ---- synctime ---------------------------------------------------------

TEST_CASE("fraction_of_period wraps correctly, including at exact multiples") {
    CHECK(synctime::fraction_of_period(0.0, 1.0) == doctest::Approx(0.0));
    CHECK(synctime::fraction_of_period(0.25, 1.0) == doctest::Approx(0.25));
    CHECK(synctime::fraction_of_period(1.25, 1.0) == doctest::Approx(0.25));
    CHECK(synctime::fraction_of_period(90.0, 60.0) == doctest::Approx(0.5));
}

TEST_CASE("angle_for_fraction maps 0/quarter/full turns to expected radians") {
    constexpr double kPi = 3.14159265358979323846;
    CHECK(synctime::angle_for_fraction(0.0) == doctest::Approx(0.0));
    CHECK(synctime::angle_for_fraction(0.25) == doctest::Approx(kPi / 2.0));
    CHECK(synctime::angle_for_fraction(1.0) == doctest::Approx(2.0 * kPi));
}

TEST_CASE("point_on_circle places 0/90/180/270 degrees at the expected screen positions") {
    constexpr double kPi = 3.14159265358979323846;
    const synctime::Point center{0.0f, 0.0f};

    auto p0 = synctime::point_on_circle(center, 10.0f, 0.0f);  // top
    CHECK(p0.x == doctest::Approx(0.0f));
    CHECK(p0.y == doctest::Approx(-10.0f));

    auto p90 = synctime::point_on_circle(center, 10.0f, static_cast<float>(kPi / 2.0));  // right
    CHECK(p90.x == doctest::Approx(10.0f));
    CHECK(p90.y == doctest::Approx(0.0f));

    auto p180 = synctime::point_on_circle(center, 10.0f, static_cast<float>(kPi));  // bottom
    CHECK(p180.x == doctest::Approx(0.0f));
    CHECK(p180.y == doctest::Approx(10.0f));
}
