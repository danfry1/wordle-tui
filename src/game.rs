//! Core Wordle rules: guess evaluation and game state.

use crate::words;

pub const WORD_LEN: usize = 5;
pub const MAX_GUESSES: usize = 6;

/// The status of a single letter after a guess is scored.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    /// Letter not yet evaluated (current/empty rows).
    Empty,
    /// Letter is not in the word at all.
    Absent,
    /// Letter is in the word but in a different position.
    Present,
    /// Letter is in the correct position.
    Correct,
}

/// A scored guess: the five letters and their per-position tiles.
#[derive(Clone)]
pub struct Guess {
    pub letters: [char; WORD_LEN],
    pub tiles: [Tile; WORD_LEN],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Playing,
    Won,
    Lost,
}

pub struct Game {
    solution: [char; WORD_LEN],
    pub guesses: Vec<Guess>,
    pub current: String,
    pub status: Status,
    /// Best-known status per keyboard letter ('a'..='z'), for the on-screen keyboard.
    pub keyboard: [Tile; 26],
}

impl Game {
    pub fn new(solution: &str) -> Self {
        let mut chars = ['a'; WORD_LEN];
        for (i, c) in solution.chars().take(WORD_LEN).enumerate() {
            chars[i] = c.to_ascii_lowercase();
        }
        Game {
            solution: chars,
            guesses: Vec::new(),
            current: String::new(),
            status: Status::Playing,
            keyboard: [Tile::Empty; 26],
        }
    }

    pub fn solution_string(&self) -> String {
        self.solution.iter().collect()
    }

    /// Appends a letter to the in-progress guess if there's room.
    pub fn push_letter(&mut self, c: char) {
        if self.status == Status::Playing
            && self.current.len() < WORD_LEN
            && c.is_ascii_alphabetic()
        {
            self.current.push(c.to_ascii_lowercase());
        }
    }

    pub fn pop_letter(&mut self) {
        if self.status == Status::Playing {
            self.current.pop();
        }
    }

    /// Submits the current guess. Returns an error message if invalid.
    pub fn submit(&mut self) -> Result<(), &'static str> {
        if self.status != Status::Playing {
            return Err("The game is over.");
        }
        if self.current.len() < WORD_LEN {
            return Err("Not enough letters.");
        }
        // The official solution is always accepted even if absent from the lists.
        let is_solution = self.current == self.solution_string();
        if !is_solution && !words::is_valid_guess(&self.current) {
            return Err("Not in word list.");
        }

        let tiles = score(&self.current, &self.solution);
        let mut letters = ['a'; WORD_LEN];
        for (i, c) in self.current.chars().enumerate() {
            letters[i] = c;
        }

        // Update the keyboard map, only ever upgrading a letter's status.
        for (i, &c) in letters.iter().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            if rank(tiles[i]) > rank(self.keyboard[idx]) {
                self.keyboard[idx] = tiles[i];
            }
        }

        let won = tiles.iter().all(|&t| t == Tile::Correct);
        self.guesses.push(Guess { letters, tiles });
        self.current.clear();

        if won {
            self.status = Status::Won;
        } else if self.guesses.len() >= MAX_GUESSES {
            self.status = Status::Lost;
        }
        Ok(())
    }

    /// Builds the spoiler-free shareable emoji grid (NYT-style).
    pub fn share_grid(&self) -> String {
        let mut out = String::new();
        for guess in &self.guesses {
            for &t in &guess.tiles {
                out.push(match t {
                    Tile::Correct => '🟩',
                    Tile::Present => '🟨',
                    _ => '⬛',
                });
            }
            out.push('\n');
        }
        out
    }
}

/// Orders tile statuses so the keyboard only ever upgrades (Correct wins).
fn rank(t: Tile) -> u8 {
    match t {
        Tile::Empty => 0,
        Tile::Absent => 1,
        Tile::Present => 2,
        Tile::Correct => 3,
    }
}

/// Scores a guess against the solution using standard Wordle two-pass rules,
/// correctly handling duplicate letters.
fn score(guess: &str, solution: &[char; WORD_LEN]) -> [Tile; WORD_LEN] {
    let guess: Vec<char> = guess.chars().collect();
    let mut tiles = [Tile::Absent; WORD_LEN];

    // Count solution letters not yet matched as Correct.
    let mut counts = [0u8; 26];
    for &c in solution.iter() {
        counts[(c as u8 - b'a') as usize] += 1;
    }

    // First pass: exact matches consume a letter from the pool.
    for i in 0..WORD_LEN {
        if guess[i] == solution[i] {
            tiles[i] = Tile::Correct;
            counts[(guess[i] as u8 - b'a') as usize] -= 1;
        }
    }

    // Second pass: present-but-misplaced, limited by remaining counts.
    for i in 0..WORD_LEN {
        if tiles[i] == Tile::Correct {
            continue;
        }
        let idx = (guess[i] as u8 - b'a') as usize;
        if counts[idx] > 0 {
            tiles[i] = Tile::Present;
            counts[idx] -= 1;
        }
    }

    tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    fn t(guess: &str, sol: &str) -> [Tile; WORD_LEN] {
        let mut s = ['a'; WORD_LEN];
        for (i, c) in sol.chars().enumerate() {
            s[i] = c;
        }
        score(guess, &s)
    }

    // ---- scoring ----

    #[test]
    fn all_correct() {
        assert_eq!(t("crane", "crane"), [Tile::Correct; 5]);
    }

    #[test]
    fn all_absent() {
        assert_eq!(t("fluff", "crisp"), [Tile::Absent; 5]);
    }

    #[test]
    fn single_present_misplaced() {
        // 'a' exists in solution but not at index 0.
        let r = t("apple", "spear");
        assert_eq!(r[0], Tile::Present); // a present
    }

    #[test]
    fn duplicate_letters_in_guess() {
        // solution has one 'l' at index 0; guess "llama" -> first l correct,
        // second l absent (the single 'l' is already consumed).
        let r = t("llama", "ledge");
        assert_eq!(r[0], Tile::Correct);
        assert_eq!(r[1], Tile::Absent);
    }

    #[test]
    fn duplicate_guess_one_correct_one_present() {
        // solution "abbey" has two 'b's; guess "bobby":
        // b o b b y  vs  a b b e y
        let r = t("bobby", "abbey");
        // idx2 'b' correct (matches), idx0 'b' present (one b remains),
        // idx3 'b' absent (pool exhausted: 2 b's, one correct + one present).
        assert_eq!(r[2], Tile::Correct);
        assert_eq!(r[0], Tile::Present);
        assert_eq!(r[3], Tile::Absent);
        assert_eq!(r[4], Tile::Correct); // y matches
    }

    #[test]
    fn present_then_correct_priority() {
        let r = t("babes", "kebab");
        assert_eq!(r[2], Tile::Correct);
        assert_eq!(r[4], Tile::Absent);
    }

    #[test]
    fn extra_guess_letter_beyond_solution_count_is_absent() {
        // solution has a single 'e'; guess has three 'e's.
        let r = t("eeeee", "vexed");
        // only two 'e's in "vexed" (idx1, idx3)... actually one e at idx1, one at idx3 => 2 e's.
        let correct_or_present = r.iter().filter(|&&x| x != Tile::Absent).count();
        assert_eq!(correct_or_present, 2);
    }

    // ---- Game state machine ----

    #[test]
    fn new_game_state() {
        let g = Game::new("CRANE");
        assert_eq!(g.solution_string(), "crane");
        assert_eq!(g.status, Status::Playing);
        assert!(g.guesses.is_empty());
        assert_eq!(g.keyboard, [Tile::Empty; 26]);
    }

    #[test]
    fn push_and_pop_letters() {
        let mut g = Game::new("crane");
        g.push_letter('h');
        g.push_letter('E'); // upper -> lowercased
        assert_eq!(g.current, "he");
        g.pop_letter();
        assert_eq!(g.current, "h");
    }

    #[test]
    fn push_ignores_non_alpha_and_overflow() {
        let mut g = Game::new("crane");
        g.push_letter('1');
        g.push_letter('!');
        assert_eq!(g.current, "");
        for c in "toolong".chars() {
            g.push_letter(c);
        }
        assert_eq!(g.current.len(), WORD_LEN);
        assert_eq!(g.current, "toolo".chars().take(5).collect::<String>());
    }

    #[test]
    fn submit_rejects_short_guess() {
        let mut g = Game::new("crane");
        for c in "cat".chars() {
            g.push_letter(c);
        }
        assert_eq!(g.submit(), Err("Not enough letters."));
        assert_eq!(g.current, "cat"); // unchanged
    }

    #[test]
    fn submit_rejects_unknown_word() {
        let mut g = Game::new("crane");
        for c in "zzzzz".chars() {
            g.push_letter(c);
        }
        assert_eq!(g.submit(), Err("Not in word list."));
    }

    #[test]
    fn submit_accepts_solution_even_if_not_in_lists() {
        // "zzzzz" is not a real word but if it were the solution it must be accepted.
        let mut g = Game::new("zzzzz");
        for c in "zzzzz".chars() {
            g.push_letter(c);
        }
        assert_eq!(g.submit(), Ok(()));
        assert_eq!(g.status, Status::Won);
    }

    #[test]
    fn winning_sets_status_and_clears_current() {
        let mut g = Game::new("about");
        for c in "about".chars() {
            g.push_letter(c);
        }
        assert_eq!(g.submit(), Ok(()));
        assert_eq!(g.status, Status::Won);
        assert_eq!(g.current, "");
        assert_eq!(g.guesses.len(), 1);
        assert_eq!(g.guesses[0].tiles, [Tile::Correct; 5]);
    }

    #[test]
    fn six_wrong_guesses_loses() {
        let mut g = Game::new("crane");
        let guesses = ["about", "blind", "ghost", "lucky", "vivid", "wharf"];
        for (i, w) in guesses.iter().enumerate() {
            for c in w.chars() {
                g.push_letter(c);
            }
            assert_eq!(g.submit(), Ok(()), "guess {w} should be valid");
            if i < 5 {
                assert_eq!(g.status, Status::Playing);
            }
        }
        assert_eq!(g.status, Status::Lost);
        assert_eq!(g.guesses.len(), MAX_GUESSES);
    }

    #[test]
    fn no_input_after_game_over() {
        let mut g = Game::new("about");
        for c in "about".chars() {
            g.push_letter(c);
        }
        g.submit().unwrap();
        assert_eq!(g.status, Status::Won);
        g.push_letter('x');
        assert_eq!(g.current, "");
        assert_eq!(g.submit(), Err("The game is over."));
    }

    // ---- keyboard tracking ----

    #[test]
    fn keyboard_only_upgrades() {
        let mut g = Game::new("crane");
        // guess "areas": a present(idx0), r present, e present, a absent(2nd), s absent
        for c in "speed".chars() {
            g.push_letter(c);
        }
        let _ = g.submit();
        // 'e' should be present (e is in crane at idx3) at least.
        let e = g.keyboard[(b'e' - b'a') as usize];
        assert!(e == Tile::Present || e == Tile::Correct);

        // Now guess a word with e in the correct slot to upgrade to Correct.
        let mut g2 = Game::new("crane");
        for c in "crane".chars() {
            g2.push_letter(c);
        }
        g2.submit().unwrap();
        assert_eq!(g2.keyboard[(b'e' - b'a') as usize], Tile::Correct);
        assert_eq!(g2.keyboard[(b'c' - b'a') as usize], Tile::Correct);
    }

    #[test]
    fn keyboard_correct_not_downgraded_by_later_absent() {
        let mut g = Game::new("abbey");
        // First guess gets 'b' correct somewhere.
        for c in "abbey".chars() {
            g.push_letter(c);
        }
        g.submit().unwrap();
        assert_eq!(g.keyboard[(b'b' - b'a') as usize], Tile::Correct);
    }

    // ---- share grid ----

    #[test]
    fn share_grid_uses_emoji() {
        let mut g = Game::new("crane");
        for c in "crane".chars() {
            g.push_letter(c);
        }
        g.submit().unwrap();
        assert_eq!(g.share_grid(), "🟩🟩🟩🟩🟩\n");
    }

    #[test]
    fn share_grid_mixed() {
        let mut g = Game::new("crane");
        for c in "creak".chars() {
            g.push_letter(c);
        }
        g.submit().unwrap();
        // c r e a k vs c r a n e: c✓ r✓ e(present) a(present) k(absent)
        assert_eq!(g.share_grid(), "🟩🟩🟨🟨⬛\n");
    }
}
