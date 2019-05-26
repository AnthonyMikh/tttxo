use rand::{thread_rng, rngs::ThreadRng, seq::SliceRandom};

use crate::battlefield::*;

trait Heuristic {
    /// Check the battlefield for presence of possibilities of good turn
    /// and return the index corresponding to this turn if there is one.
    /// The cell with returned index (if it is present) should be empty
    fn hint(&mut self, field: &Battlefield) -> Option<usize>;

    /// Chain two heuristics. Apply first heuristic
    /// and, if it fails, apply second heuristic
    fn or<H: Heuristic>(self, second: H) -> Or<Self, H> where Self: Sized
    {
        Or { first: self, second }
    }

    /// Chain heuristic with a strategy.
    /// If heuristic fails to supply a hint, use guidance of strategy instead
    fn otherwise<S: Strategy>(self, strategy: S) -> Otherwise<Self, S> where Self: Sized {
        Otherwise { heuristic: self, strategy }
    }
}

fn close_own_line(line: IndexedLine) -> Option<usize> {
    match line {
        ([i, _, _], [__, O_, O_]) |
        ([_, i, _], [O_, __, O_]) |
        ([_, _, i], [O_, O_, __]) => Some(i),
        _ => None,
    }
}

fn close_enemy_line(line: IndexedLine) -> Option<usize> {
    match line {
        ([i, _, _], [__, E_, E_]) |
        ([_, i, _], [E_, __, E_]) |
        ([_, _, i], [E_, E_, __]) => Some(i),
        _ => None,
    }
}

#[test]
fn test_closing_lines() {
    assert_eq!(close_own_line(([0, 1, 2], [O_, __, O_])), Some(1));
    assert_eq!(close_own_line(([0, 4, 8], [__, O_, O_])), Some(4));
    assert_eq!(close_own_line(([2, 5, 8], [O_, E_, __])), None);

    assert_eq!(close_enemy_line(([2, 4, 6], [E_, E_, __])), Some(6));
    assert_eq!(close_enemy_line(([1, 4, 7], [__, E_, E_])), Some(1));
    assert_eq!(close_enemy_line(([2, 4, 6], [E_, __, O_])), None);
}

struct CloseOwnLine;

impl Heuristic for CloseOwnLine {
    fn hint(&mut self, field: &Battlefield) -> Option<usize> {
        field.all_lines().iter().find_map(|&line| close_own_line(line))
    }
}

struct CloseEnemyLine;

impl Heuristic for CloseEnemyLine {
    fn hint(&mut self, field: &Battlefield) -> Option<usize> {
        field.all_lines().iter().find_map(|&line| close_enemy_line(line))
    }
}

struct TakeCenterOnEmptyBoard;

impl Heuristic for TakeCenterOnEmptyBoard {
    fn hint(&mut self, field: &Battlefield) -> Option<usize> {
        if field.0.iter().all(|fig| fig.is_none()) {
            Some(4)
        } else {
            None
        }
    }
}

pub(crate) struct Or<H1, H2> {
    first: H1,
    second: H2,
}

impl<H1, H2> Heuristic for Or<H1, H2>
where
    H1: Heuristic, H2: Heuristic
{
    fn hint(&mut self, field: &Battlefield) -> Option<usize> {
        self.first.hint(field).or_else(|| self.second.hint(field))
    }
}

pub(crate) trait Strategy {
    /// Inspect the battlefield and return the index corresponding to turn.
    /// The cell with returned index (if it is present) should be empty
    fn guide(&mut self, field: &Battlefield) -> usize;
}

struct Otherwise<H, S> {
    heuristic: H,
    strategy: S,
}

impl<H: Heuristic, S: Strategy> Strategy for Otherwise<H, S> {
    fn guide(&mut self, field: &Battlefield) -> usize {
        self.heuristic.hint(field).unwrap_or_else(|| self.strategy.guide(field))
    }
}

struct RandomFree {
    rng: ThreadRng,
    places: Vec<usize>,
}

impl RandomFree {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
            places: Vec::with_capacity(9),
        }
    }
}

impl Strategy for RandomFree {
    fn guide(&mut self, field: &Battlefield) -> usize {
        self.places.extend(
            field.0.iter()
                .enumerate()
                .filter_map(|(i, fig)| {
                    if fig.is_none() {
                        Some(i)
                    } else {
                        None
                    }
                })
        );
        let turn = *self.places.choose(&mut self.rng).expect("no free cells on battlefield");
        self.places.clear();
        turn
    }
}

pub(crate) fn make_default_strategy() -> impl Strategy {
    CloseOwnLine
        .or(CloseEnemyLine)
        .or(TakeCenterOnEmptyBoard)
        .otherwise(RandomFree::new())
}