mod battlefield;
mod game;
mod strategy;
mod ask;
mod game_session;

use game_session::{GameSession, SessionResult, print_game_board};
use ask::ask_yes_no;

// REVIEW: style: not important, but I personally prefer clearer
fn main() -> std::io::Result<()> {
    println!("Welcome to tttxo - your tic-tac-toe game!");
    let mut buf = String::new();
    if ask_yes_no(&mut buf, "Do you want to play a game? ([y]/n): ", "Invalid input")?.no() {
        return Ok(())
    }

    loop {
        // REVIEW: creating a new buffer for each session seems redundant (not that it's important in this case, lol)
        let mut session = GameSession::new_session(String::new())?;
        loop {
            println!();
            print_game_board(&session.game.board);
            println!();
            match session.make_turn()? {
                None => continue,
                Some(SessionResult::Aborted) => break,
                Some(SessionResult::Finished(result)) => {
                    println!();
                    print_game_board(&session.game.board);
                    println!();
                    println!("{}", result);
                    break
                },
            }
        }
        if ask_yes_no(&mut buf, "One more game? ([y]/n): ", "Invalid input")?.no() {
            println!("Thanks for playing!");
            break
        }
    }

    Ok(())
}
