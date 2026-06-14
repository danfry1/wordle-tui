//! All rendering. Kept free of game logic so the look can evolve independently.

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;
use crate::game::{MAX_GUESSES, Status, Tile, WORD_LEN};

// Palette tuned to the official Wordle colours.
const GREEN: Color = Color::Rgb(83, 141, 78);
const YELLOW: Color = Color::Rgb(181, 159, 59);
const GRAY: Color = Color::Rgb(58, 58, 60);
const KEY_IDLE: Color = Color::Rgb(129, 131, 132);
const BORDER_EMPTY: Color = Color::Rgb(58, 58, 60);
const BORDER_ACTIVE: Color = Color::Rgb(120, 124, 126);
const TEXT: Color = Color::Rgb(248, 248, 248);
const DIM: Color = Color::Rgb(140, 140, 140);

// Odd height so the inner area has a true middle row to center the glyph on.
const TILE_W: u16 = 9;
const TILE_H: u16 = 5;
const GAP: u16 = 1;

pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let grid_w = WORD_LEN as u16 * TILE_W + (WORD_LEN as u16 - 1) * GAP;
    let grid_h = MAX_GUESSES as u16 * TILE_H + (MAX_GUESSES as u16 - 1) * GAP;
    let content_w = grid_w.max(41);
    // header(2) + gap(1) + grid + msg(1) + gap(1) + keyboard(3) + footer(1)
    let content_h = 2 + 1 + grid_h + 1 + 1 + 3 + 1;

    let content = center(area, content_w, content_h);

    let rows = Layout::vertical([
        Constraint::Length(2),      // header
        Constraint::Length(1),      // spacer
        Constraint::Length(grid_h), // grid
        Constraint::Length(1),      // message
        Constraint::Length(1),      // spacer
        Constraint::Length(3),      // keyboard
        Constraint::Length(1),      // footer
    ])
    .split(content);

    render_header(frame, rows[0], app);
    render_grid(frame, rows[2], app);
    render_message(frame, rows[3], app);
    render_keyboard(frame, rows[5], app);
    render_footer(frame, rows[6], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = Line::from(vec![
        Span::styled("W", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(
            "O",
            Style::default().fg(YELLOW).add_modifier(Modifier::BOLD),
        ),
        Span::styled("R", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(
            "D",
            Style::default().fg(YELLOW).add_modifier(Modifier::BOLD),
        ),
        Span::styled("L", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled("E", Style::default().fg(GRAY).add_modifier(Modifier::BOLD)),
    ]);

    let mut sub = String::new();
    if let Some(n) = app.daily.number {
        sub.push_str(&format!("No. {n}  ·  "));
    }
    sub.push_str(&app.daily.date);
    sub.push_str(if app.daily.official {
        "  ·  official"
    } else {
        "  ·  practice"
    });

    let subtitle = Line::from(Span::styled(sub, Style::default().fg(DIM)));

    frame.render_widget(
        Paragraph::new(vec![title, subtitle]).alignment(Alignment::Center),
        area,
    );
}

fn render_grid(frame: &mut Frame, area: Rect, app: &App) {
    let grid_w = WORD_LEN as u16 * TILE_W + (WORD_LEN as u16 - 1) * GAP;
    let origin_x = area.x + area.width.saturating_sub(grid_w) / 2;

    // Horizontal shake offset for an invalid submission.
    let shake = app
        .shake_until
        .map(|t| {
            let ms = t.elapsed().as_millis();
            if ms < 400 {
                let phase = (ms / 60) % 4;
                match phase {
                    0 | 2 => 1i16,
                    1 => -1i16,
                    _ => 0,
                }
            } else {
                0
            }
        })
        .unwrap_or(0);

    for row in 0..MAX_GUESSES {
        let y = area.y + row as u16 * (TILE_H + GAP);
        for col in 0..WORD_LEN {
            let mut x = origin_x as i16 + col as i16 * (TILE_W + GAP) as i16;
            // Only shake the active (in-progress) row.
            if row == app.game.guesses.len() {
                x += shake;
            }
            let rect = Rect {
                x: x.max(0) as u16,
                y,
                width: TILE_W,
                height: TILE_H,
            };
            let (ch, tile, active) = cell_state(app, row, col);
            render_tile(frame, rect, ch, tile, active);
        }
    }
}

/// Milliseconds before each successive tile in a row flips to its colour.
const REVEAL_STEP: u128 = 130;

/// Resolves what to show in a given grid cell.
fn cell_state(app: &App, row: usize, col: usize) -> (Option<char>, Tile, bool) {
    let g = &app.game;
    if row < g.guesses.len() {
        let guess = &g.guesses[row];
        // Stagger the colour reveal of the most recently submitted row.
        if let Some((reveal_row, started)) = app.reveal_row
            && reveal_row == row
            && started.elapsed().as_millis() < (col as u128 + 1) * REVEAL_STEP
        {
            return (Some(guess.letters[col]), Tile::Empty, true);
        }
        (Some(guess.letters[col]), guess.tiles[col], false)
    } else if row == g.guesses.len() && g.status == Status::Playing {
        let ch = g.current.chars().nth(col);
        (ch, Tile::Empty, ch.is_some())
    } else {
        (None, Tile::Empty, false)
    }
}

fn render_tile(frame: &mut Frame, area: Rect, ch: Option<char>, tile: Tile, active: bool) {
    let (bg, border) = match tile {
        Tile::Correct => (GREEN, GREEN),
        Tile::Present => (YELLOW, YELLOW),
        Tile::Absent => (GRAY, GRAY),
        Tile::Empty => {
            if active {
                (Color::Reset, BORDER_ACTIVE)
            } else {
                (Color::Reset, BORDER_EMPTY)
            }
        }
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(border))
        .style(Style::default().bg(bg));

    // Paint the bordered, colour-filled tile first.
    let inner = block.inner(area);
    frame.render_widget(block, area);
    if inner.height == 0 || inner.width == 0 {
        return;
    }

    // Place the letter on the exact middle row of the inner area so it sits
    // dead-centre rather than hugging the top border.
    let mid = Rect {
        x: inner.x,
        y: inner.y + inner.height / 2,
        width: inner.width,
        height: 1,
    };
    let glyph = ch
        .map(|c| c.to_ascii_uppercase().to_string())
        .unwrap_or_default();
    let text_style = Style::default().fg(TEXT).add_modifier(Modifier::BOLD);
    frame.render_widget(
        Paragraph::new(Line::from(Span::styled(glyph, text_style))).alignment(Alignment::Center),
        mid,
    );
}

fn render_message(frame: &mut Frame, area: Rect, app: &App) {
    let line = if let Some(t) = &app.toast {
        let color = if t.is_error { YELLOW } else { GREEN };
        Line::from(Span::styled(
            t.text.clone(),
            Style::default().fg(color).add_modifier(Modifier::BOLD),
        ))
    } else {
        Line::default()
    };
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

const KEYBOARD: [&str; 3] = ["qwertyuiop", "asdfghjkl", "zxcvbnm"];

fn render_keyboard(frame: &mut Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();
    for (i, row) in KEYBOARD.iter().enumerate() {
        let mut spans: Vec<Span> = Vec::new();
        // Indent lower rows slightly for the staggered keyboard look.
        if i > 0 {
            spans.push(Span::raw("  ".repeat(i)));
        }
        if i == 2 {
            spans.push(key_cap("⏎", KEY_IDLE));
            spans.push(Span::raw(" "));
        }
        for c in row.chars() {
            let idx = (c as u8 - b'a') as usize;
            let color = match app.game.keyboard[idx] {
                Tile::Correct => GREEN,
                Tile::Present => YELLOW,
                Tile::Absent => GRAY,
                Tile::Empty => KEY_IDLE,
            };
            spans.push(key_cap(&c.to_uppercase().to_string(), color));
            spans.push(Span::raw(" "));
        }
        if i == 2 {
            spans.push(key_cap("⌫", KEY_IDLE));
        }
        lines.push(Line::from(spans));
    }

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

fn key_cap(label: &str, bg: Color) -> Span<'static> {
    Span::styled(
        format!(" {label} "),
        Style::default()
            .bg(bg)
            .fg(TEXT)
            .add_modifier(Modifier::BOLD),
    )
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let line = if app.game.status == Status::Playing {
        Line::from(Span::styled(
            "type a guess · Enter submit · Esc/Ctrl-C quit",
            Style::default().fg(DIM),
        ))
    } else {
        Line::from(vec![
            Span::styled("S", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" share result · ", Style::default().fg(DIM)),
            Span::styled(
                "Esc",
                Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" / ", Style::default().fg(DIM)),
            Span::styled("Q", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(" quit", Style::default().fg(DIM)),
        ])
    };
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

/// Returns a rect of the given size centred within `area` (clamped).
fn center(area: Rect, width: u16, height: u16) -> Rect {
    let w = width.min(area.width);
    let h = height.min(area.height);
    let [_, mid, _] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(h),
        Constraint::Fill(1),
    ])
    .areas(area);
    let [_, center, _] = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(w),
        Constraint::Fill(1),
    ])
    .areas(mid);
    center
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::App;
    use crate::daily::Daily;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn app_with(solution: &str) -> App {
        App::new(Daily {
            solution: solution.to_string(),
            date: "2026-06-14".to_string(),
            number: Some(1821),
            official: true,
        })
    }

    /// Renders the app to an in-memory backend and flattens the buffer to text.
    fn screen(width: u16, height: u16, app: &App) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|f| render(f, app)).unwrap();
        let buf = terminal.backend().buffer();
        let mut out = String::new();
        for y in 0..height {
            for x in 0..width {
                if let Some(cell) = buf.cell((x, y)) {
                    out.push_str(cell.symbol());
                }
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn renders_title_and_metadata() {
        let app = app_with("crane");
        let s = screen(80, 44, &app);
        assert!(s.contains("WORDLE"), "title missing:\n{s}");
        assert!(s.contains("No. 1821"), "puzzle number missing");
        assert!(s.contains("2026-06-14"), "date missing");
        assert!(s.contains("official"), "official badge missing");
    }

    #[test]
    fn renders_playing_footer() {
        let app = app_with("crane");
        let s = screen(80, 44, &app);
        assert!(s.contains("type a guess"), "footer missing:\n{s}");
    }

    #[test]
    fn renders_win_message_and_share_footer() {
        let mut app = app_with("crane");
        for c in "crane".chars() {
            app.on_char(c);
        }
        app.on_enter();
        assert_eq!(app.game.status, Status::Won);
        let s = screen(80, 44, &app);
        assert!(s.contains("Genius!"), "win toast missing:\n{s}");
        assert!(s.contains("share result"), "share footer missing");
    }

    #[test]
    fn practice_badge_shown_when_not_official() {
        let mut app = app_with("crane");
        app.daily.official = false;
        let s = screen(80, 44, &app);
        assert!(s.contains("practice"), "practice badge missing:\n{s}");
    }

    #[test]
    fn does_not_panic_at_extreme_sizes() {
        let app = app_with("crane");
        // Tiny and degenerate sizes must clamp gracefully, not panic.
        for (w, h) in [(1, 1), (3, 2), (10, 6), (40, 20), (80, 44), (200, 120)] {
            let _ = screen(w, h, &app);
        }
    }
}
