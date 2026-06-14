# wordle-tui

A beautiful terminal word-guessing game built with [ratatui](https://ratatui.rs).

Six guesses, five letters, color-coded feedback, an on-screen keyboard, and a
shareable emoji grid — all in your terminal.

![wordle-tui demo](https://raw.githubusercontent.com/danfry1/wordle-tui/main/demo/wordle-tui.gif)

## Features

- 🎨 Polished TUI: chunky color tiles, staggered reveal, shake-on-invalid, toasts
- ⌨️  Live on-screen keyboard that tracks letter states
- 📋 Shareable result grid (🟩🟨⬛) copied to the clipboard
- 📦 Self-contained by default — bundled word list, no network needed
- 🌐 Optional `--daily` mode plays the official New York Times puzzle
- 🪶 One direct dependency (`ratatui`); no `rand`, no HTTP crate

## Install

### From source (cargo)

```sh
cargo install wordle-tui
```

### Homebrew (tap)

```sh
brew install danfry1/tap/wordle-tui
```

### Nix

```sh
# Run without installing
nix run github:danfry1/wordle-tui

# Or add to a flake / install into a profile
nix profile install github:danfry1/wordle-tui
```

### Prebuilt binaries

Download a binary for your platform from the
[Releases](https://github.com/danfry1/wordle-tui/releases) page.

## Usage

```sh
wordle-tui            # play today's offline puzzle
wordle-tui --daily    # play the official NYT daily puzzle (needs network)
wordle-tui --help
```

`--daily` can also be enabled with `WORDLE_TUI_DAILY=1`.

### Controls

| Key | Action |
| --- | --- |
| `a`–`z` | Type a letter |
| `Enter` | Submit guess |
| `Backspace` | Delete a letter |
| `S` | Share result (after the game ends) |
| `Esc` / `Ctrl-C` | Quit |

## How it works

- **Offline (default):** the daily puzzle is chosen deterministically from a
  bundled list, so everyone playing on the same date gets the same word.
- **`--daily`:** fetches the official solution from the public NYT endpoint
  using the system `curl`, falling back to the offline word if the network is
  unavailable. No HTTP/TLS Rust dependency is pulled in.
- Clipboard sharing uses the platform clipboard tool (`pbcopy` on macOS).

## Development

```sh
cargo run            # play
cargo test           # run the test suite
cargo clippy         # lint
cargo fmt            # format
```

## License

[MIT](LICENSE) © Daniel Fry

## Disclaimer

"Wordle" is a trademark of The New York Times Company. **wordle-tui** is an
independent, unofficial project and is not affiliated with, endorsed by, or
sponsored by The New York Times.
