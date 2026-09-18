cmake_minimum_required(VERSION 3.20)
project(imgui_clock LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
if(NOT CMAKE_BUILD_TYPE AND NOT CMAKE_CONFIGURATION_TYPES)
  set(CMAKE_BUILD_TYPE RelWithDebInfo)
endif()

option(IMGUI_CLOCK_BUILD_TESTS "Build the unit test executable" ON)

# Applied only to this project's own targets below (via target_compile_options),
# never to the fetched glfw/imgui/doctest targets, to keep third-party builds quiet.
set(IMGUI_CLOCK_WARNING_FLAGS -Wall -Wextra)

include(FetchContent)

# --- GLFW: windowing/input backend -----------------------------------------
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_TESTS OFF CACHE BOOL "" FORCE)
set(GLFW_BUILD_DOCS OFF CACHE BOOL "" FORCE)
set(GLFW_INSTALL OFF CACHE BOOL "" FORCE)

FetchContent_Declare(
  glfw
  GIT_REPOSITORY https://github.com/glfw/glfw.git
  GIT_TAG 3.5.1
  GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(glfw)

# --- Dear ImGui: fetched as sources, no CMake support upstream -------------
FetchContent_Declare(
  imgui
  GIT_REPOSITORY https://github.com/ocornut/imgui.git
  GIT_TAG v1.92.9b
  GIT_SHALLOW TRUE
)
FetchContent_MakeAvailable(imgui)

add_library(imgui STATIC
  ${imgui_SOURCE_DIR}/imgui.cpp
  ${imgui_SOURCE_DIR}/imgui_draw.cpp
  ${imgui_SOURCE_DIR}/imgui_tables.cpp
  ${imgui_SOURCE_DIR}/imgui_widgets.cpp
  ${imgui_SOURCE_DIR}/imgui_demo.cpp
  ${imgui_SOURCE_DIR}/backends/imgui_impl_glfw.cpp
  ${imgui_SOURCE_DIR}/backends/imgui_impl_opengl3.cpp
)
target_include_directories(imgui PUBLIC
  ${imgui_SOURCE_DIR}
  ${imgui_SOURCE_DIR}/backends
)
target_link_libraries(imgui PUBLIC glfw)
if(UNIX AND NOT APPLE)
  find_package(OpenGL REQUIRED)
  target_link_libraries(imgui PUBLIC OpenGL::GL)
elseif(APPLE)
  target_link_libraries(imgui PUBLIC "-framework OpenGL")
elseif(WIN32)
  target_link_libraries(imgui PUBLIC opengl32)
endif()

# --- App sources, split so the pure/testable logic has no ImGui/GLFW deps --
add_library(imgui_clock_core STATIC
  src/settings.cpp
  src/tabs/wallclock.cpp
  src/tabs/stopwatch.cpp
  src/tabs/synctime.cpp
)
target_include_directories(imgui_clock_core PUBLIC src)
target_compile_options(imgui_clock_core PRIVATE ${IMGUI_CLOCK_WARNING_FLAGS})

add_executable(imgui_clock
  src/main.cpp
  src/tabs/wallclock_ui.cpp
  src/tabs/stopwatch_ui.cpp
  src/tabs/synctime_ui.cpp
  src/tabs/settings_tab.cpp
)
target_link_libraries(imgui_clock PRIVATE imgui_clock_core imgui)
target_compile_options(imgui_clock PRIVATE ${IMGUI_CLOCK_WARNING_FLAGS})

# --- Unit tests --------------------------------------------------------------
if(IMGUI_CLOCK_BUILD_TESTS)
  FetchContent_Declare(
    doctest
    GIT_REPOSITORY https://github.com/doctest/doctest.git
    GIT_TAG v2.5.3
    GIT_SHALLOW TRUE
  )
  FetchContent_MakeAvailable(doctest)

  enable_testing()
  add_executable(imgui_clock_tests tests/test_main.cpp)
  target_link_libraries(imgui_clock_tests PRIVATE imgui_clock_core doctest::doctest)
  target_compile_options(imgui_clock_tests PRIVATE ${IMGUI_CLOCK_WARNING_FLAGS})

  include(CTest)
  add_test(NAME imgui_clock_tests COMMAND imgui_clock_tests)
endif()
