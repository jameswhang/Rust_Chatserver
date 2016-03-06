#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct MultiIndex (pub usize, pub usize);

impl MultiIndex {
    fn is_vertical(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 == i2.0;
        let stacked = i1.1 + 1 == i2.1 || i1.1 - 1 == i2.1;
        beside && stacked
    }

    fn is_horizontal(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 + 1 == i2.0 || i1.0 - 1 == i2.0;
        let stacked = i1.1  == i2.1 ;
        beside && stacked
    }

    fn is_diagonal(i1 : MultiIndex, i2 : MultiIndex) -> bool {
        let beside = i1.0 + 1 == i2.0 || i1.0 - 1 == i2.0;
        let stacked = i1.1 + 1 == i2.1 || i1.1 - 1 == i2.1;
        beside && stacked
    }
}

impl Into<(usize, usize)> for MultiIndex {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

impl Into<MultiIndex> for (usize, usize) {
    fn into(self) -> MultiIndex {
        MultiIndex(self.0, self.1)
    }
}
