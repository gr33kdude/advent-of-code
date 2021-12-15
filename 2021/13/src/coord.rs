struct Coord {
    row : int,
    col : int,
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (Ordering::Equal, y_ord) => y_ord,
            (x_ord, _) => x_ord,
        }
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        (self.row, self.col) == (other.row, other.col)
    }
}
impl Eq for Coord {}