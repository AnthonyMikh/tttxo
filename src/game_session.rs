use std::io::Result;

use crate::ask::{ask_value_validated, ask_value_or_default, Answer};
use crate::game::{Game, Player, Mark, Board, TurnResult};
use crate::strategy::{Strategy, make_default_strategy};

enum UserReaction {
    Turn(usize),
    Quit,
}

fn numpad_to_idx(key: usize) -> usize {
     (0..)
        .zip(&NUMPAD)
        .find_map(|(idx, &numpad_key)| if numpad_key == key { Some(idx) } else { None })
        .unwrap()
}


impl std::str::FromStr for UserReaction {
    type Err = ();

    fn from_str(line: &str) -> std::result::Result<Self, ()> {
        match line.parse() {
            Ok(turn) => Ok(UserReaction::Turn(turn)),
            Err(_) if line == "q" || line == "Q" => Ok(UserReaction::Quit),
            _ => Err(()),
        }
    }
}

fn ask_user_turn(buf: &mut String, field: &Board) -> Result<UserReaction> {
    let mut input = ask_value_validated(
        buf,
        "Your turn (`q` to quit): ",
        "Invalid input",
        |input| match input {
            UserReaction::Quit => true,
            UserReaction::Turn(turn) => *turn > 0 && *turn <= 9 && field.0[numpad_to_idx(*turn)].is_none(),
        },
        "Invalid turn",
    )?;
    if let UserReaction::Turn(ref mut val) = input {
        *val = numpad_to_idx(*val);
    }
    Ok(input)
}

fn ask_user_mark(buf: &mut String) -> Result<Mark> {
    ask_value_or_default(
        buf,
        "Choose your figure ([X]/O): ",
        "Invalid figure",
    )
}

fn ask_if_makes_first_turn(buf: &mut String) -> Result<Answer> {
    ask_value_or_default(
        buf,
        "Would you like to make first turn? ([y]/n): ",
        "Invalid input",
    )
}

const NUMPAD: [usize; 9] = [
    7, 8, 9,
    4, 5, 6,
    1, 2, 3,
];

pub(crate) fn print_game_board(board: &Board) {
    for (figs, nums) in board.0.chunks(3).zip(NUMPAD.chunks(3)) {
        for (fig, num) in figs.iter().zip(nums) {
            match fig {
                // REVIEW: rendering: would look better with some spaces between characters IMO
                // REVIEW: clippy: why not just println!("O")?
                Some(Mark::X) => print!("{}", "X"),
                Some(Mark::O) => print!("{}", "O"),
                None => print!("{}", num),
            }
        }
        println!();
    }
}

pub(crate) enum GameResult {
    Win,
    Lose,
    Draw,
}

impl std::fmt::Display for GameResult {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GameResult::Win => fmt.write_str("You win!"),
            GameResult::Lose => fmt.write_str("You lose"),
            GameResult::Draw => fmt.write_str("Draw"),
        }
    }
}

// REVIEW: style: generally you don't need pub(crate) in a binary, just pub would do
pub(crate) enum SessionResult {
    Finished(GameResult),
    Aborted,
}

pub(crate) struct GameSession {
    user_player: Player,
    pub(crate) game: Game,
    input_buf: String,
    ai: Box<dyn Strategy>,
}

impl GameSession {
    // REVIEW: style: some vertical line breaks?
    pub(crate) fn new_session(mut buf: String) -> Result<Self> {
        let user_player = if ask_if_makes_first_turn(&mut buf)?.into() {
            Player::First
        } else {
            Player::Second
        };
        let user_mark = ask_user_mark(&mut buf)?;
        let first_player_mark = if let Player::First = user_player {
            user_mark
        } else {
            user_mark.opposite()
        };
        let game = Game::new(first_player_mark);
        let ai = Box::new(make_default_strategy());
        buf.clear();
        Ok(Self {
            user_player,
            game,
            input_buf: buf,
            ai,
        })
    }

    pub(crate) fn make_turn(&mut self) -> Result<Option<SessionResult>> {
        let result = if self.user_player == self.game.current_player {
            match ask_user_turn(&mut self.input_buf, &self.game.board)? {
                UserReaction::Turn(idx) => self.game.make_turn(idx),
                UserReaction::Quit => return Ok(Some(SessionResult::Aborted)),
            }
        } else {
            let turn = self.ai.guide(&self.game.board_repr);
            self.game.make_turn(turn)
        };

        Ok(
            result
                .map(|outcome| match outcome {
                    TurnResult::WinOf(player) if player == self.user_player => {
                        GameResult::Win
                    },
                    TurnResult::WinOf(_) => GameResult::Lose,
                    TurnResult::Draw => GameResult::Draw,
                })
                .map(SessionResult::Finished)
        )
    }
}
