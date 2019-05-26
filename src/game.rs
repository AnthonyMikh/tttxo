use crate::battlefield::{Battlefield, Figure};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Mark {
    X,
    O,
}

impl std::str::FromStr for Mark {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, ()> {
        match line {
            "x" | "X" => Ok(Mark::X),
            "o" | "O" => Ok(Mark::O),
             _ => Err(()),
        }
    }
}

impl std::default::Default for Mark {
    fn default() -> Self {
        Mark::X
    }
}

impl Mark {
    pub(crate) fn opposite(self) -> Self {
        use Mark::*;

        match self {
            X => O,
            O => X,
        }
    }
}

#[derive(Default)]
pub(crate) struct Board(pub(crate) [Option<Mark>; 9]);

impl Board {
    fn all_lines(&self) -> [[Option<Mark>; 3]; 8] {
        match self.0 {
            [
                l1, c1, r1,
                l2, c2, r2,
                l3, c3, r3,
            ] => [
                //rows
                [l1, c1, r1],
                [l1, c2, r2],
                [l3, c3, r3],

                //cols
                [l1, l2, l3],
                [c1, c2, c3],
                [r1, r2, r3],

                //diagonals
                [l1, c2, r3],
                [r1, c2, l3]
            ]
        }
    }
}

fn complete_line(line: &[Option<Mark>; 3]) -> Option<Mark> {
    const X_: Option<Mark> = Some(Mark::X);
    const O_: Option<Mark> = Some(Mark::O);

    match line {
        [X_, X_, X_] => X_,
        [O_, O_, O_] => O_,
        _ => None,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Player {
    First,
    Second,
}

impl Player {
    fn next(self) -> Self {
        use Player::*;

        match self {
            First => Second,
            Second => First,
        }
    }
}

pub(crate) enum TurnResult {
    WinOf(Player),
    Draw,
}

pub(crate) struct Game {
    pub(crate) board: Board,
    pub(crate) board_repr: Battlefield,
    pub(crate) current_player: Player,
    first_player_mark: Mark,
    free_cells: usize,
}

impl Game {
    pub(crate) fn new(first_player_mark: Mark) -> Self {
        use std::default::Default;

        Self {
            board: Board::default(),
            board_repr: Battlefield::default(),
            current_player: Player::First,
            first_player_mark,
            free_cells: 9,
        }
    }

    fn current_mark(&self) -> Mark {
        match self.current_player {
            Player::First => self.first_player_mark,
            Player::Second => self.first_player_mark.opposite(),
        }
    }

    fn owner_of(&self, mark: Mark) -> Player {
        if self.first_player_mark == mark {
            Player::First
        } else {
            Player::Second
        }
    }

    /// Make turn into specified location.
    /// If this turn completes the line, return the winner, otherwise return None.
    /// Panics if specified cell is not empty or index is out of bounds
    pub(crate) fn make_turn(&mut self, turn: usize) -> Option<TurnResult> {
        assert!(turn < 9, "invalid index: {}", turn);
        assert!(self.board.0[turn].is_none());

        self.board_repr.0[turn] = Some(Figure::Own);
        self.board.0[turn] = Some(self.current_mark());
        self.board_repr.0.iter_mut().for_each(|fig| {
            if let Some(fig) = fig {
                fig.reverse()
            }
        });
        self.current_player = self.current_player.next();

        self.free_cells -= 1;
        if self.free_cells == 0 {
            return Some(TurnResult::Draw)
        }

        self.board.all_lines().iter()
            .find_map(complete_line)
            .map(|mark| TurnResult::WinOf(self.owner_of(mark)))
    }
}
