# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/).

## 2026-06-22
### Added
- Completed conversion of the application from React/TypeScript to Yew/Rust/WebAssembly.
- Implemented pure Rust Tailwind CSS compiler pipeline (no node_modules or npm dependencies).
- Added unit tests for game logic, local storage, stats persistence, and word lists.
- Dynamically sized the virtual keyboard to occupy exactly 2/3 width and 2/3 of the bottom 2/3 of screen height (`h-[44vh]`).
- Ensured uniform key box sizing across standard and special (`ENTER` / `DELETE`) keys.
- Updated repository workflows and LICENSE file to align with GPL-3.0 copyleft licensing.
