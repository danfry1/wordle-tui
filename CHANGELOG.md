# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-14

### Added

- Terminal Wordle-style game built with ratatui: 6×5 grid, color-coded tile
  feedback, and standard duplicate-letter scoring rules.
- Live on-screen keyboard that tracks per-letter state.
- Staggered tile reveal animation and shake feedback on invalid guesses.
- Shareable emoji result grid copied to the clipboard.
- Offline play by default using a bundled, deterministic-per-day word list.
- Optional `--daily` mode (also `WORDLE_TUI_DAILY=1`) that plays the official
  New York Times daily puzzle, fetched via the system `curl`.
- `--help` and `--version` flags.
- Extensive test suite covering scoring, game state, word-list invariants,
  date/JSON parsing, formatting, and rendering.

[Unreleased]: https://github.com/danfry1/wordle-tui/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/danfry1/wordle-tui/releases/tag/v0.1.0
