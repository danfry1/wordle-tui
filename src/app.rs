//! Application state and input handling, decoupled from rendering.

use std::time::Instant;

use crate::daily::Daily;
use crate::game::{Game, Status};

/// A transient toast message shown under the grid.
pub struct Toast {
    pub text: String,
    pub created: Instant,
    pub is_error: bool,
}

pub struct App {
    pub game: Game,
    pub daily: Daily,
    pub toast: Option<Toast>,
    /// Set when the player presses a key on a finished row, for the shake animation.
    pub shake_until: Option<Instant>,
    /// Reveal animation start for the most recently submitted row.
    pub reveal_row: Option<(usize, Instant)>,
    pub should_quit: bool,
}

impl App {
    pub fn new(daily: Daily) -> Self {
        let game = Game::new(&daily.solution);
        App {
            game,
            daily,
            toast: None,
            shake_until: None,
            reveal_row: None,
            should_quit: false,
        }
    }

    pub fn toast<S: Into<String>>(&mut self, text: S, is_error: bool) {
        self.toast = Some(Toast {
            text: text.into(),
            created: Instant::now(),
            is_error,
        });
    }

    pub fn on_char(&mut self, c: char) {
        if self.game.status != Status::Playing {
            return;
        }
        self.game.push_letter(c);
    }

    pub fn on_backspace(&mut self) {
        self.game.pop_letter();
    }

    pub fn on_enter(&mut self) {
        if self.game.status != Status::Playing {
            return;
        }
        match self.game.submit() {
            Ok(()) => {
                let row = self.game.guesses.len() - 1;
                self.reveal_row = Some((row, Instant::now()));
                match self.game.status {
                    Status::Won => {
                        let n = self.game.guesses.len();
                        self.toast(praise(n), false);
                    }
                    Status::Lost => {
                        self.toast(
                            format!(
                                "The word was {}",
                                self.game.solution_string().to_uppercase()
                            ),
                            true,
                        );
                    }
                    Status::Playing => {}
                }
            }
            Err(msg) => {
                self.shake_until = Some(Instant::now());
                self.toast(msg, true);
            }
        }
    }

    /// Clears the toast once it has been visible long enough.
    pub fn tick(&mut self) {
        if let Some(t) = &self.toast {
            let keep = if self.game.status == Status::Playing {
                2200
            } else {
                60_000
            };
            if t.created.elapsed().as_millis() as u64 > keep {
                self.toast = None;
            }
        }
    }
}

fn praise(guesses: usize) -> &'static str {
    match guesses {
        1 => "Genius!",
        2 => "Magnificent!",
        3 => "Impressive!",
        4 => "Splendid!",
        5 => "Great!",
        _ => "Phew!",
    }
}
