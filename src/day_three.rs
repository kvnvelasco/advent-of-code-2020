use std::ops::Index;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Tree,
    Clear,
}

/// A structure that allows us to get an arbitrary index into it by tiling a seed state repeatedly
/// until we get to the required index. Math works out to be index % seed.length
#[derive(Eq, PartialEq, Clone, Debug)]
struct ForestRow {
    seed: Vec<Tile>,
}

impl ForestRow {
    fn from_str(source: &'static str) -> Self {
        let seed = source
            .chars()
            .map(|character| {
                if character == '#' {
                    Tile::Tree
                } else {
                    Tile::Clear
                }
            })
            .collect();
        Self { seed }
    }
}

impl Index<usize> for ForestRow {
    type Output = Tile;

    fn index(&self, index: usize) -> &Self::Output {
        let new_index = index % self.seed.len();
        self.seed
            .get(new_index)
            .expect("Modulo operation should guarantee output")
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    rows: Vec<ForestRow>,
}

impl Map {
    fn get(&self, (x, y): (usize, usize)) -> Option<Tile> {
        self.rows.get(y).map(|tile| tile[x])
    }

    fn from_string_iterator(source: impl Iterator<Item = &'static str>) -> Self {
        let rows = source.map(ForestRow::from_str).collect();
        Self { rows }
    }
}

fn a(input: impl Iterator<Item = &'static str>, (delta_x, delta_y): (usize, usize)) -> usize {
    let map = Map::from_string_iterator(input);
    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    while let Some(value) = map.get((x, y)) {
        if value == Tile::Tree {
            count += 1
        }
        x += delta_x;
        y += delta_y;
    }

    count
}

fn b(source: &'static str) -> usize {
    let slope_list = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let value = slope_list
        .iter()
        .map(|&pair| a(source.lines(), pair))
        .product();
    value
}

#[cfg(test)]
mod tests {
    use crate::day_three::Tile::{Clear, Tree};
    use crate::day_three::{a, b, ForestRow, Map};

    #[test]
    fn forest_rows_are_parseable() {
        let row = ForestRow::from_str(".#..#.##.");
        assert_eq!(
            row,
            ForestRow {
                seed: vec![Clear, Tree, Clear, Clear, Tree, Clear, Tree, Tree, Clear]
            }
        );
    }

    #[test]
    fn forest_rows_are_indexable() {
        let row = ForestRow::from_str(".#..#.##.");
        assert_eq!(Tree, row[4]);
        assert_eq!(Clear, row[9]);
        assert_eq!(Tree, row[13]);
    }

    #[test]
    fn maps_are_indexable() {
        let map = Map::from_string_iterator(include_str!("inputs/day_tree_sanity.txt").lines());
        assert_eq!(map.get((0, 0)), Some(Clear));
        assert_eq!(map.get((0, 1)), Some(Tree));
        assert_eq!(map.get((0, 2)), Some(Clear));
        assert_eq!(map.get((0, 3)), Some(Tree));
        assert_eq!(map.get((124124, 1)), Some(Tree));

        // Even rows have trees
        assert_eq!(map.get((55463180, 5)), Some(Tree));
        // Even rows have trees
        assert_eq!(map.get((55461, 5)), Some(Clear));
    }

    #[test]
    fn a_naive_test_case() {
        let count = a(include_str!("inputs/day_three_test.txt").lines(), (3, 1));

        assert_eq!(count, 7)
    }

    #[test]
    fn a_production_test_case() {
        let count = a(include_str!("inputs/day_three.txt").lines(), (3, 1));

        assert_eq!(count, 148)
    }

    #[test]
    fn b_test_case_production_test_case() {
        let count = b(include_str!("inputs/day_three_test.txt"));

        assert_eq!(count, 336)
    }

    #[test]
    fn b_production_Case() {
        let count = b(include_str!("inputs/day_three.txt"));

        assert_eq!(count, 727923200)
    }
}
