# GUI of Tomorrow
Since more than 20 years I am developing less complex GUI applications using wxWidgets later Qt.

Time to change. I want to use a modern approach that works on Windows, Linux and possibly Android.
Nowadays important to me possibility for gui tests.

This repo hosts several projects demonstrating different GUI application frameworks.

## Demonstrator
The demonstrator is a clock application with several tabs.
- wallclock displays date and time,  
  the user can change timezone
- stopwatch start, stop, reset, laptime  
  the user can export its recordings as text to i.e. file
- synctime displays two moving arcs for 1 second per rotation and 1 minuteper rotation, in the centre daytime hh:mm:ss as text  
  the arcs are updating permanently, so user can compare two computer times visually
- settings:  
  gui theme, system, light or dark, with contraste options normal or high contrast.  
  scaling of fonts borders..., 50% ... 200%


The above will represent some key features in gui apps like:
- user interaction
- timed events
- persistent settings
- export to
- graphical scene elements
- standard gui elements, styling

## Requirements
for each project a README.md is required that:
- explains installing development environment
- explains building, running, debugging
- explain creating the deliverable application package
each project has an IMPLEMENTATION.md
- completenes checklist
- side notes

## Comparation
each framwork is reported in a section in the OVERVIEW.md in the repo root
- briefly stating pro and cons
- when to use this framework and when not to choose that framework
- complexity level of implementation
- ability of the framework support mvc or similar pattern
- ability for gui tests
- ability for theming (dark, light, system)
- ability to create dockable (sub)windows
- abilty to use the framework using Python
- Lines of code for the implementation (no comment, empty lines, ...)

## Contester
I handselected the contesters

- Flutter Dart
- Tauri Rust
- Slint Rust
- Dioxus Rust
- Iced Rust
- egui Rust
- imgui C++

not in the list
- qt
- slint, uses qt on windows
- c#
- 







