#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    FillX,
    FillO,
}

impl Square {
    pub fn is_empty(&self) -> bool {
        match self {
            Square::Empty => true,
            _ => false,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Square::Empty => "",
            Square::FillX => "X",
            Square::FillO => "O",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    pub squares: [Square; 9],
}

impl Board {
    pub fn new_empty() -> Self {
        Self {
            squares: [Square::Empty; 9],
        }
    }

    pub fn calculate_winner(&self) -> Square {
        static LINES: &[[usize; 3]] = &[
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for &[a, b, c] in LINES {
            let a = &self.squares[a];
            if !a.is_empty() && a == &self.squares[b] && a == &self.squares[c] {
                return *a;
            }
        }

        Square::Empty
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    // make the fields private so that we can guard the game logic
    history: Vec<Board>,
    step_number: usize,
}

impl Game {
    pub fn new() -> Self {
        let mut history = Vec::with_capacity(9);
        history.push(Board::new_empty());
        Game {
            history,
            step_number: 0,
        }
    }

    pub fn current(&self) -> &Board {
        &self.history[self.step_number]
    }

    pub fn full_history(&self) -> &Vec<Board> {
        &self.history
    }

    pub fn step_number(&self) -> usize {
        self.step_number
    }

    #[inline]
    pub fn x_is_next(&self) -> bool {
        self.step_number % 2 == 0
    }

    #[inline]
    pub fn next_player(&self) -> Square {
        if self.x_is_next() {
            Square::FillX
        } else {
            Square::FillO
        }
    }

    /// returns whether state changed
    pub fn click(&mut self, i: usize) -> bool {
        let current = self.current();

        // i valid and the game is not over and the square is not filled
        if i <= 8 && current.squares[i].is_empty() && current.calculate_winner().is_empty() {
            // copy current board as the next
            let mut next = *current;
            next.squares[i] = self.next_player();

            let keep = self.step_number + 1;
            if keep < self.history.len() {
                self.history.truncate(keep);
            }
            self.history.push(next);

            self.step_number += 1;

            true
        } else {
            false
        }
    }

    /// returns whether state changed
    pub fn jump_to(&mut self, i: usize) -> bool {
        if &i == &self.step_number || &i >= &self.history.len() {
            return false;
        }
        self.step_number = i;
        true
    }
}
