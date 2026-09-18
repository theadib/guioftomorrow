cmake_minimum_required(VERSION 3.20)
project(qt_clock LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_AUTOMOC ON)
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE RelWithDebInfo)
endif()

option(QT_CLOCK_BUILD_TESTS "Build the unit test executables" ON)

# QStyleHints::colorScheme()/colorSchemeChanged (used by theming.cpp for
# live, push-based OS theme following) need Qt 6.5+.
find_package(Qt6 6.5 REQUIRED COMPONENTS Widgets)
if(QT_CLOCK_BUILD_TESTS)
  find_package(Qt6 REQUIRED COMPONENTS Test)
endif()

set(QT_CLOCK_WARNING_FLAGS -Wall -Wextra)

# --- Pure/testable core logic, no QWidgets dependency -----------------------
add_library(qt_clock_core STATIC
  src/settings.cpp
  src/tabs/wallclock.cpp
  src/tabs/stopwatch.cpp
  src/tabs/synctime.cpp
)
target_include_directories(qt_clock_core PUBLIC src)
target_link_libraries(qt_clock_core PUBLIC Qt6::Core)
target_compile_options(qt_clock_core PRIVATE ${QT_CLOCK_WARNING_FLAGS})

# --- Application --------------------------------------------------------------
add_executable(qt_clock
  src/main.cpp
  src/mainwindow.cpp
  src/theming.cpp
  src/tabs/wallclock_tab.cpp
  src/tabs/stopwatch_tab.cpp
  src/tabs/synctime_tab.cpp
  src/tabs/settings_tab.cpp
)
target_link_libraries(qt_clock PRIVATE qt_clock_core Qt6::Widgets)
target_compile_options(qt_clock PRIVATE ${QT_CLOCK_WARNING_FLAGS})

if(WIN32)
  set_target_properties(qt_clock PROPERTIES WIN32_EXECUTABLE TRUE)
elseif(APPLE)
  set_target_properties(qt_clock PROPERTIES MACOSX_BUNDLE TRUE)
endif()

# --- Unit tests (QtTest) ------------------------------------------------------
if(QT_CLOCK_BUILD_TESTS)
  enable_testing()

  function(qt_clock_add_test name)
    add_executable(${name} tests/${name}.cpp)
    target_link_libraries(${name} PRIVATE qt_clock_core Qt6::Test)
    target_compile_options(${name} PRIVATE ${QT_CLOCK_WARNING_FLAGS})
    add_test(NAME ${name} COMMAND ${name})
  endfunction()

  qt_clock_add_test(tst_settings)
  qt_clock_add_test(tst_wallclock)
  qt_clock_add_test(tst_stopwatch)
  qt_clock_add_test(tst_synctime)
endif()
