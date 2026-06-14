//! wordle-tui — a beautiful terminal word-guessing game built with ratatui.
//!
//! By default it plays a self-contained puzzle drawn from a bundled word list
//! (deterministic per day). With `--daily` it instead fetches and plays the
//! official New York Times daily puzzle via the system `curl`, falling back to
//! the offline word if the network is unavailable.

mod app;
mod daily;
mod game;
mod ui;
mod words;

use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use app::App;
use daily::Daily;
use game::Status;

const HELP: &str = "\
wordle-tui — a terminal word-guessing game

USAGE:
    wordle-tui [OPTIONS]

OPTIONS:
    --daily        Play the official New York Times daily puzzle (requires network).
                   Can also be enabled with WORDLE_TUI_DAILY=1.
    -h, --help     Print this help and exit.
    -V, --version  Print version and exit.

CONTROLS:
    a-z            Type a guess          Enter   Submit
    Backspace      Delete a letter       Esc     Quit
    S              Share result (after the game ends)

Note: \"Wordle\" is a trademark of The New York Times Company. wordle-tui is an
independent project, not affiliated with or endorsed by the NYT.";

/// Parsed command-line options.
struct Options {
    daily: bool,
    show_help: bool,
    show_version: bool,
}

impl Options {
    fn from_env() -> Self {
        let mut opts = Options {
            daily: std::env::var("WORDLE_TUI_DAILY")
                .map(|v| v == "1")
                .unwrap_or(false),
            show_help: false,
            show_version: false,
        };
        for arg in std::env::args().skip(1) {
            match arg.as_str() {
                "--daily" | "--online" => opts.daily = true,
                "-h" | "--help" => opts.show_help = true,
                "-V" | "--version" => opts.show_version = true,
                _ => {}
            }
        }
        opts
    }
}

fn main() -> std::io::Result<()> {
    let opts = Options::from_env();
    if opts.show_help {
        println!("{HELP}");
        return Ok(());
    }
    if opts.show_version {
        println!("wordle-tui {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    let daily = if opts.daily {
        daily::fetch_today().unwrap_or_else(offline_daily)
    } else {
        offline_daily()
    };

    let mut terminal = ratatui::init();
    let mut app = App::new(daily);

    let result = run(&mut terminal, &mut app);

    ratatui::restore();

    // Print a parting summary to the normal screen after the TUI tears down.
    if app.game.status != Status::Playing {
        print_summary(&app);
    }
    result
}

fn run(terminal: &mut ratatui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(frame, app))?;
        app.tick();

        // Poll with a timeout so animations and toast expiry keep ticking
        // even when the player isn't pressing keys.
        if event::poll(Duration::from_millis(90))?
            && let Event::Key(key) = event::read()?
        {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            handle_key(app, key.code, key.modifiers);
        }

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, code: KeyCode, mods: KeyModifiers) {
    // Global quit chords.
    if matches!(code, KeyCode::Char('c') | KeyCode::Char('d'))
        && mods.contains(KeyModifiers::CONTROL)
    {
        app.should_quit = true;
        return;
    }

    match app.game.status {
        Status::Playing => match code {
            KeyCode::Esc => app.should_quit = true,
            KeyCode::Enter => app.on_enter(),
            KeyCode::Backspace => app.on_backspace(),
            KeyCode::Char(c) if c.is_ascii_alphabetic() => app.on_char(c),
            _ => {}
        },
        _ => match code {
            KeyCode::Esc | KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Char('s') => share(app),
            _ => {}
        },
    }
}

/// Builds the shareable result text and copies it to the clipboard via pbcopy.
fn share(app: &mut App) {
    let header = share_header(app);
    let text = format!("{header}\n{}", app.game.share_grid());

    if copy_to_clipboard(&text) {
        app.toast("Result copied to clipboard!", false);
    } else {
        app.toast("Press Esc to quit (clipboard unavailable)", true);
    }
}

fn share_header(app: &App) -> String {
    let score = match app.game.status {
        Status::Won => app.game.guesses.len().to_string(),
        _ => "X".to_string(),
    };
    match app.daily.number {
        Some(n) => format!("Wordle {} {score}/6", with_thousands(n)),
        None => format!("Wordle {} {score}/6", app.daily.date),
    }
}

/// Formats an integer with comma thousands separators, e.g. 1821 -> "1,821".
fn with_thousands(n: u32) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    let len = digits.len();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (len - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

fn copy_to_clipboard(text: &str) -> bool {
    let Ok(mut child) = Command::new("pbcopy").stdin(Stdio::piped()).spawn() else {
        return false;
    };
    if let Some(stdin) = child.stdin.as_mut()
        && stdin.write_all(text.as_bytes()).is_err()
    {
        return false;
    }
    child.wait().map(|s| s.success()).unwrap_or(false)
}

fn print_summary(app: &App) {
    let header = share_header(app);
    println!();
    match app.game.status {
        Status::Won => println!("  You solved it! {header}"),
        _ => println!(
            "  The word was {}.  {header}",
            app.game.solution_string().to_uppercase()
        ),
    }
    println!();
    print!("{}", app.game.share_grid());
    println!();
}

/// Picks a deterministic offline word so the same day yields the same puzzle.
fn offline_daily() -> Daily {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let day = secs / 86_400;
    let idx = (day as usize) % words::ANSWERS.len();
    Daily {
        solution: words::ANSWERS[idx].to_string(),
        date: daily::today_date_string(),
        number: None,
        official: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn daily(number: Option<u32>, solution: &str) -> Daily {
        Daily {
            solution: solution.to_string(),
            date: "2026-06-14".to_string(),
            number,
            official: true,
        }
    }

    fn play(app: &mut App, word: &str) {
        for c in word.chars() {
            app.on_char(c);
        }
        app.on_enter();
    }

    #[test]
    fn thousands_separator() {
        assert_eq!(with_thousands(0), "0");
        assert_eq!(with_thousands(7), "7");
        assert_eq!(with_thousands(42), "42");
        assert_eq!(with_thousands(999), "999");
        assert_eq!(with_thousands(1000), "1,000");
        assert_eq!(with_thousands(1821), "1,821");
        assert_eq!(with_thousands(12345), "12,345");
        assert_eq!(with_thousands(1_000_000), "1,000,000");
    }

    #[test]
    fn offline_daily_is_valid() {
        let d = offline_daily();
        assert_eq!(d.solution.len(), 5);
        assert!(words::ANSWERS.contains(&d.solution.as_str()));
        assert!(!d.official);
        assert!(d.number.is_none());
    }

    #[test]
    fn share_header_win_uses_guess_count_and_comma() {
        let mut app = App::new(daily(Some(1821), "crane"));
        play(&mut app, "crane");
        assert_eq!(app.game.status, Status::Won);
        assert_eq!(share_header(&app), "Wordle 1,821 1/6");
    }

    #[test]
    fn share_header_loss_uses_x() {
        let mut app = App::new(daily(Some(1821), "crane"));
        for w in ["about", "blind", "ghost", "lucky", "vivid", "wharf"] {
            play(&mut app, w);
        }
        assert_eq!(app.game.status, Status::Lost);
        assert_eq!(share_header(&app), "Wordle 1,821 X/6");
    }

    #[test]
    fn share_header_without_number_uses_date() {
        let mut app = App::new(daily(None, "crane"));
        play(&mut app, "crane");
        assert_eq!(share_header(&app), "Wordle 2026-06-14 1/6");
    }
}
