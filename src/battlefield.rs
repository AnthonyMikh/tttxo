#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(crate) enum Figure {
    Own,
    Enemy,
}

impl Figure {
    /// Change Own into Enemy and vice-versa
    pub(crate) fn reverse(&mut self) {
        use Figure::*;

        *self = match *self {
            Own => Enemy,
            Enemy => Own,
        }
    }
}

#[test]
fn test_figure_reverse() {
    use Figure::*;

    let mut fig = Own;
    fig.reverse();
    assert_eq!(fig, Enemy);

    let mut fig = Enemy;
    fig.reverse();
    assert_eq!(fig, Own);
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// Represents the board of tic-tac-toe
/// from players's point of view
pub(crate) struct Battlefield(pub [Option<Figure>; 9]);

pub(crate) type IndexedLine = ([usize; 3], [Option<Figure>; 3]);

impl Battlefield {
    fn index(&self, i:usize, j: usize, k: usize) -> IndexedLine {
        let table = self.0;
        ([i, j, k], [table[i], table[j], table[k]])
    }

    /// Get all rows with corresponding indices
    pub(crate) fn rows(&self) -> [IndexedLine; 3] {
        [
            self.index(0, 1, 2),
            self.index(3, 4, 5),
            self.index(6, 7, 8),
        ]
    }

    /// Get all columns with corresponding indices
    pub(crate) fn cols(&self) -> [IndexedLine; 3] {
        [
            self.index(0, 3, 6),
            self.index(1, 4, 7),
            self.index(2, 5, 8),
        ]
    }

    /// Get all diagonals with corresponding indices
    pub(crate) fn diags(&self) -> [IndexedLine; 2] {
        [self.index(0, 4, 8), self.index(2, 4, 6)]
    }

    /// Get all lines (rows, columns and diagonals) with corresponding indices
    pub(crate) fn all_lines(&self) -> [IndexedLine; 8] {
        let [r1, r2, r3] = self.rows();
        let [c1, c2, c3] = self.cols();
        let [d1, d2] = self.diags();
        [r1, r2, r3, c1, c2, c3, d1, d2]
    }
}

#[allow(non_upper_case_globals)]
pub(in super) const __: Option<Figure> = None;
pub(in super) const O_: Option<Figure> = Some(Figure::Own);
pub(in super) const E_: Option<Figure> = Some(Figure::Enemy);


#[test]
fn test_accessors() {
    const TABLE: Battlefield = Battlefield([
        __, O_, E_,
        __, __, E_,
        O_, __, O_,
    ]);

    assert_eq!(
        TABLE.rows(),
        [
            ([0, 1, 2], [__, O_, E_]),
            ([3, 4, 5], [__, __, E_]),
            ([6, 7, 8], [O_, __, O_]),
        ]
    );

    assert_eq!(
        TABLE.cols(),
        [
            ([0, 3, 6], [__, __, O_]),
            ([1, 4, 7], [O_, __, __]),
            ([2, 5, 8], [E_, E_, O_]),
        ]
    );

    assert_eq!(
        TABLE.diags(),
        [
            ([0, 4, 8], [__, __, O_]),
            ([2, 4, 6], [E_, __, O_]),
        ]
    );

    assert_eq!(
        TABLE.all_lines(),
        [
            ([0, 1, 2], [__, O_, E_]),
            ([3, 4, 5], [__, __, E_]),
            ([6, 7, 8], [O_, __, O_]),

            ([0, 3, 6], [__, __, O_]),
            ([1, 4, 7], [O_, __, __]),
            ([2, 5, 8], [E_, E_, O_]),

            ([0, 4, 8], [__, __, O_]),
            ([2, 4, 6], [E_, __, O_]),
        ]
    );
}
