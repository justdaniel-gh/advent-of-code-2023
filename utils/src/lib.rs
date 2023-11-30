#![allow(dead_code)]

use std::{
    cmp::Ordering,
    fmt::{self, Display},
    fs,
    path::Path,
};

fn load_puzzle<T, F: FnOnce(String) -> T>(puzzle_path: &Path, parser: F) -> T {
    parser(String::from_utf8(fs::read(puzzle_path).expect("Unable to open input!")).unwrap())
}

pub fn load_puzzle_data<T, F: FnOnce(String) -> T>(day: u32, parser: F) -> T {
    let puzzle_filename = format!("puzzles/day{day}.txt");
    let puzzle_path = Path::new(&puzzle_filename);
    load_puzzle(puzzle_path, parser)
}

pub fn load_puzzle_test<T, F: FnOnce(String) -> T>(day: u32, parser: F) -> T {
    let puzzle_filename = format!("../puzzles/day{day}_test.txt");
    let puzzle_path = Path::new(&puzzle_filename);
    load_puzzle(puzzle_path, parser)
}

// Thank you Francis Gagné! : https://stackoverflow.com/a/42356713
pub trait SliceExt {
    type Item;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> SliceExt for [T] {
    type Item = T;

    fn get_two_mut(&mut self, index0: usize, index1: usize) -> (&mut Self::Item, &mut Self::Item) {
        match index0.cmp(&index1) {
            Ordering::Less => {
                let mut iter = self.iter_mut();
                let item0 = iter.nth(index0).unwrap();
                let item1 = iter.nth(index1 - index0 - 1).unwrap();
                (item0, item1)
            }
            Ordering::Equal => panic!("[T]::get_two_mut(): received same index twice ({index0})"),
            Ordering::Greater => {
                let mut iter = self.iter_mut();
                let item1 = iter.nth(index1).unwrap();
                let item0 = iter.nth(index0 - index1 - 1).unwrap();
                (item0, item1)
            }
        }
    }
}

/*
    Traits
*/
pub trait Grid {
    type Item;

    fn get_cell(&self, x: isize, y: isize) -> Option<&Self::Item>;
    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Self::Item>;

    fn first_cell_coord(&self) -> (isize, isize);
    fn last_cell_coord(&self) -> (isize, isize);

    fn get_row(&self, y: isize) -> Option<&[Self::Item]>;
}

pub trait Growable {
    type Item;

    fn get_cell_or_add(&mut self, x: isize, y: isize) -> &Self::Item;
    fn get_cell_or_add_mut(&mut self, x: isize, y: isize) -> &mut Self::Item;
}

pub trait GrowableGrid<T>: Growable<Item = T> + Grid<Item = T> {}

#[derive(Debug, Default, Clone)]
pub struct StaticGrid<T> {
    pub cells: Vec<T>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl<T> StaticGrid<T>
where
    T: Default + Clone,
{
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        StaticGrid {
            cells: vec![Default::default(); num_rows * num_cols],
            num_rows,
            num_cols,
        }
    }

    pub fn row(&self, row_ndx: usize) -> &[T] {
        &self.cells[row_ndx * self.num_cols..(row_ndx * self.num_cols) + self.num_cols]
    }

    pub fn row_mut(&mut self, row_ndx: usize) -> &mut [T] {
        &mut self.cells[row_ndx * self.num_cols..(row_ndx * self.num_cols) + self.num_cols]
    }

    pub fn col(&self, col_ndx: usize) -> Vec<&T> {
        let mut ret_cells: Vec<&T> = Vec::new();
        for cell in self.cells.iter().skip(col_ndx).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }

    pub fn col_mut(&mut self, col_ndx: usize) -> Vec<&mut T> {
        let mut ret_cells: Vec<&mut T> = Vec::new();
        for cell in self.cells.iter_mut().skip(col_ndx).step_by(self.num_cols) {
            ret_cells.push(cell);
        }
        ret_cells
    }

    pub fn cell_iter(&self) -> core::slice::Iter<'_, T> {
        self.cells.iter()
    }

    /// Returns an iterator moving in the specified direction, starting at (returning first) the x,y coord
    pub fn direction_iter_at(
        &self,
        x: isize,
        y: isize,
        direction: Direction,
    ) -> DirectionIter<'_, T> {
        DirectionIter {
            grid: self,
            direction,
            next_x: x,
            next_y: y,
        }
    }
}

impl<T> Grid for StaticGrid<T> 
where
    T: Default + Clone,
{
    type Item = T;

    fn get_cell(&self, x: isize, y: isize) -> Option<&Self::Item> {
        if x >= self.num_cols as isize || y >= self.num_rows as isize {
            None
        } else {
            self.cells.get((y as usize * self.num_cols) + x as usize)
        }
    }

    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Self::Item> {
        self.cells
            .get_mut((y as usize * self.num_cols) + x as usize)
    }

    fn first_cell_coord(&self) -> (isize, isize) {
        (0, 0)
    }

    fn last_cell_coord(&self) -> (isize, isize) {
        (self.num_cols as isize - 1, self.num_rows as isize - 1)
    }

    fn get_row(&self, y: isize) -> Option<&[Self::Item]> {
        Some(self.row(y as usize))
    }
}

impl<T> fmt::Display for StaticGrid<T>
where
    T: Display + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = String::new();

        for row_ndx in 0..self.num_rows {
            let row_str: String = self.row(row_ndx).iter().map(ToString::to_string).collect();
            rows.push_str(&row_str);
            rows.push('\n');
        }

        write!(f, "{rows}")
    }
}

/*
    Enums
*/
#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

/*
    Structs
*/
pub struct DirectionIter<'a, T> {
    grid: &'a dyn Grid<Item = T>,
    direction: Direction,
    next_x: isize,
    next_y: isize,
}

impl<'a, T> Iterator for DirectionIter<'a, T>
where
    T: Default + Clone,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.grid.get_cell(self.next_x, self.next_y) {
            Some(item) => {
                match self.direction {
                    Direction::North => {
                        self.next_y -= 1;
                    }
                    Direction::South => {
                        self.next_y += 1;
                    }
                    Direction::East => {
                        self.next_x += 1;
                    }
                    Direction::West => {
                        self.next_x -= 1;
                    }
                    Direction::NorthEast => {
                        self.next_x += 1;
                        self.next_y -= 1;
                    }
                    Direction::NorthWest => {
                        self.next_x -= 1;
                        self.next_y -= 1;
                    }
                    Direction::SouthEast => {
                        self.next_x += 1;
                        self.next_y += 1;
                    }
                    Direction::SouthWest => {
                        self.next_x -= 1;
                        self.next_y += 1;
                    }
                }
                Some(item)
            }
            None => None,
        }
    }
}

pub struct DirectionIterMut<'a, T>
{
    grid: &'a mut dyn Grid<Item = T>,
    direction: Direction,
    next_x: isize,
    next_y: isize,
}

impl<'a, T> Iterator for DirectionIterMut<'a, T>
where
    T: Default + Clone,
{
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.grid.get_cell_mut(self.next_x, self.next_y) {
            Some(item) => {
                match self.direction {
                    Direction::North => {
                        self.next_y -= 1;
                    }
                    Direction::South => {
                        self.next_y += 1;
                    }
                    Direction::East => {
                        self.next_x += 1;
                    }
                    Direction::West => {
                        self.next_x -= 1;
                    }
                    Direction::NorthEast => {
                        self.next_x += 1;
                        self.next_y -= 1;
                    }
                    Direction::NorthWest => {
                        self.next_x -= 1;
                        self.next_y -= 1;
                    }
                    Direction::SouthEast => {
                        self.next_x += 1;
                        self.next_y += 1;
                    }
                    Direction::SouthWest => {
                        self.next_x -= 1;
                        self.next_y += 1;
                    }
                }

                unsafe {
                    Some(&mut *(item as *mut T))
                }
            }
            None => None,
        }
    }
}

pub struct SubGridIterMut<'a, T>
{
    grid: &'a mut dyn GrowableGrid<T>,
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
    ndx: isize,
}

/// Iterate over all cells within a SubGrid of an existing grid
/// Expands underlying grid to meet size
impl<'a, T> SubGridIterMut<'a, T> {
    pub fn new(grid: &'a mut dyn GrowableGrid<T>, start_x: isize, start_y: isize, end_x: isize, end_y: isize) -> Self {
        println!("first: {:?} - last: {:?}", grid.first_cell_coord(), grid.last_cell_coord());
        grid.get_cell_or_add(start_x, start_y);
        grid.get_cell_or_add(end_x, end_y);
        println!("first: {:?} - last: {:?}", grid.first_cell_coord(), grid.last_cell_coord());
        println!("New sub grid: {start_x},{start_y} - {end_x},{end_y}");
        SubGridIterMut {
            grid,
            start_x,
            start_y,
            end_x,
            end_y,
            ndx: 0,
        }
    }
}

impl<'a, T> Iterator for SubGridIterMut<'a, T>
where
    T: Default + Clone,
{
    type Item = (&'a mut T, (isize, isize));

    fn next(&mut self) -> Option<Self::Item> {
        let next_x = self.start_x + (self.ndx % (self.end_x - self.start_x + 1));
        let next_y = self.start_y + (self.ndx / (self.end_x - self.start_x + 1));
        if next_y > self.end_y {
            return None;
        }
        //println!("{}: {next_x},{next_y}", self.ndx);
        self.ndx += 1;
        self.grid.get_cell_mut(next_x, next_y).map(|item| unsafe { (&mut *(item as *mut T), (next_x, next_y)) })
    }
}

pub struct DynamicGrid<CellType> {
    // [y][x]
    //     -|
    //  -   |    +
    //  ----------
    //      |
    //     +|
    // Negative is relative to start_pos
    cells: Vec<Vec<CellType>>,
    // The coordinate of the start, used to translate position
    start_x: isize,
    start_y: isize,
    // These are the coords of the offset to the center (start) in absolute coords
    center_x: usize,
    center_y: usize,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl<CellType> DynamicGrid<CellType>
where
    CellType: Default + Clone,
{
    pub fn new(start_x: isize, start_y: isize) -> Self {
        DynamicGrid {
            cells: vec![vec![Default::default()]],
            start_x,
            start_y,
            // First cell will always be local 0,0
            center_x: 0,
            center_y: 0,
            num_rows: 1,
            num_cols: 1,
        }
    }

    pub fn direction_iter(
        &self,
        start_x: isize,
        start_y: isize,
        direction: Direction,
    ) -> DirectionIter<'_, CellType> {
        DirectionIter {
            grid: self,
            direction,
            next_x: start_x,
            next_y: start_y,
        }
    }

    pub fn cell_iter(&self) -> std::iter::Flatten<std::slice::Iter<'_, Vec<CellType>>>
    {
        self.cells.iter().flatten()
    }

    pub fn row_iter(&self) -> std::slice::Iter<'_, Vec<CellType>>
    {
        self.cells.iter()
    }

    pub fn sub_grid_iter_mut(&mut self, start_x: isize, start_y: isize, end_x: isize, end_y: isize) -> SubGridIterMut<'_, CellType>
    {
        SubGridIterMut::new(self, start_x, start_y, end_x, end_y)
    }

    // Translate coordinate to local coordinate system
    // Local coordinate system is relative to start value
    //  start_x = 500; start_y = 0
    //  translate_coord(500, 0) -> 0,0
    //  translate_coord(498, -2) -> -2,-2
    //  translate_coord(502, 2) -> 2,2
    fn translate_absolute_to_local(&self, x: isize, y: isize) -> (isize, isize) {
        (x - self.start_x, y - self.start_y)
    }

    // These could be negative, or out of bounds, if they haven't been allocated yet
    fn translate_local_to_indices(&self, local_x: isize, local_y: isize) -> (isize, isize) {
        (
            self.center_x as isize + local_x,
            self.center_y as isize + local_y,
        )
    }

    fn cell_exists(&self, index_x: isize, index_y: isize) -> bool {
        index_x >= 0
            && index_y >= 0
            && index_x < self.num_cols as isize
            && index_y < self.num_rows as isize
    }

    // returns new cells indices
    fn add_cell(&mut self, ndx_x: isize, ndx_y: isize) {
        if ndx_y < 0 {
            // Insert rows above center (at the start)
            let diff_y = ndx_y.unsigned_abs();
            self.center_y += diff_y;
            for _ in 0..diff_y {
                self.cells
                    .insert(0, vec![Default::default(); self.num_cols]);
                self.num_rows += 1;
            }
        } else if ndx_y >= self.num_rows as isize {
            // Insert rows after center (to the end)
            let diff_y = (ndx_y - self.num_rows as isize + 1) as usize;
            for _ in 0..diff_y {
                self.cells.push(vec![Default::default(); self.num_cols]);
                self.num_rows += 1;
            }
        }
        if ndx_x < 0 {
            let diff_x = ndx_x.unsigned_abs();
            self.center_x += diff_x;
            for _ in 0..diff_x {
                for c in self.cells.iter_mut() {
                    c.insert(0, Default::default());
                }
                self.num_cols += 1;
            }
        } else if ndx_x >= self.num_cols as isize {
            let diff_x = (ndx_x - self.num_cols as isize + 1) as usize;
            for _ in 0..diff_x {
                for c in self.cells.iter_mut() {
                    c.push(Default::default());
                }
                self.num_cols += 1;
            }
        }
    }
}

impl<CellType> Grid for DynamicGrid<CellType>
where
    CellType: Default + Clone,
{
    type Item = CellType;

    fn get_cell(&self, x: isize, y: isize) -> Option<&Self::Item> {
        let (local_x, local_y) = self.translate_absolute_to_local(x, y);
        let (ndx_x, ndx_y) = self.translate_local_to_indices(local_x, local_y);
        if self.cell_exists(ndx_x, ndx_y) {
            Some(
                self.cells
                    .get(ndx_y as usize)
                    .unwrap()
                    .get(ndx_x as usize)
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut Self::Item> {
        let (local_x, local_y) = self.translate_absolute_to_local(x, y);
        let (ndx_x, ndx_y) = self.translate_local_to_indices(local_x, local_y);
        if self.cell_exists(ndx_x, ndx_y) {
            Some(
                self.cells
                    .get_mut(ndx_y as usize)
                    .unwrap()
                    .get_mut(ndx_x as usize)
                    .unwrap(),
            )
        } else {
            None
        }
    }

    fn first_cell_coord(&self) -> (isize, isize) {
        (0 - self.center_x as isize + self.start_x, 0 - self.center_y as isize + self.start_y)
    }

    fn last_cell_coord(&self) -> (isize, isize) {
        (self.num_cols as isize - 1 - self.center_x as isize + self.start_x, self.num_rows as isize - 1 - self.center_y as isize + self.start_y)
    }

    fn get_row(&self, y: isize) -> Option<&[Self::Item]> {
        let y_ndx = self.translate_local_to_indices(0, self.translate_absolute_to_local(0, y).1).1;
        Some(self.cells[y_ndx as usize].as_slice())
    }
}

impl<CellType> Growable for DynamicGrid<CellType>
where
    CellType: Default + Clone,
{
    type Item = CellType;

    fn get_cell_or_add(&mut self, x: isize, y: isize) -> &Self::Item {
        let (local_x, local_y) = self.translate_absolute_to_local(x, y);
        let (mut ndx_x, mut ndx_y) = self.translate_local_to_indices(local_x, local_y);
        (ndx_x, ndx_y) = if self.cell_exists(ndx_x, ndx_y) {
            (ndx_x, ndx_y)
        } else {
            self.add_cell(ndx_x, ndx_y);
            self.translate_local_to_indices(local_x, local_y)
        };

        self.cells
            .get(ndx_y as usize)
            .unwrap()
            .get(ndx_x as usize)
            .unwrap()
    }

    fn get_cell_or_add_mut(&mut self, x: isize, y: isize) -> &mut Self::Item {
        let (local_x, local_y) = self.translate_absolute_to_local(x, y);
        let (mut ndx_x, mut ndx_y) = self.translate_local_to_indices(local_x, local_y);
        (ndx_x, ndx_y) = if self.cell_exists(ndx_x, ndx_y) {
            (ndx_x, ndx_y)
        } else {
            self.add_cell(ndx_x, ndx_y);
            self.translate_local_to_indices(local_x, local_y)
        };

        self.cells
            .get_mut(ndx_y as usize)
            .unwrap()
            .get_mut(ndx_x as usize)
            .unwrap()
    }
}

impl<CellType> GrowableGrid<CellType> for DynamicGrid<CellType> 
where
    CellType: Default + Clone,
{}

impl<CellType> Default for DynamicGrid<CellType>
where
    CellType: Display + Default + Clone,
{
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl<CellType> Display for DynamicGrid<CellType>
where
    CellType: Display + Default + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rows = String::new();

        for row_ndx in 0..self.num_rows {
            let row_str: String = self.cells[row_ndx]
                .iter()
                .map(ToString::to_string)
                .collect();
            rows.push_str(&row_str);
            rows.push('\n');
        }

        write!(f, "{rows}")
    }
}

#[derive(Clone, Default, Debug)]
pub struct Point2D<CoordType> {
    pub x: CoordType,
    pub y: CoordType,
}

#[derive(Clone, Default, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[cfg(test)]
mod tests {
    use std::fmt::Display;

    use crate::{DynamicGrid, StaticGrid, Growable};

    /*
    Test Structs
    */
    #[derive(Clone)]
    struct TestCell {
        value: char,
    }

    impl Default for TestCell {
        fn default() -> Self {
            Self { value: '.' }
        }
    }

    impl Display for TestCell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:>1}", self.value)
        }
    }

    #[test]
    fn test_grid() {
        let mut g: StaticGrid<TestCell> = StaticGrid::new(2, 2);
        assert_eq!(g.num_cols, 2);
        assert_eq!(g.num_rows, 2);
        assert_eq!(g.cells.len(), 4);

        for c in g.row_mut(0) {
            c.value = 'a';
        }

        let mut i = g.direction_iter_at(0, 0, crate::Direction::East);
        assert_eq!(i.next().unwrap().value, 'a');
        assert_eq!(i.next().unwrap().value, 'a');
    }

    #[test]
    fn test_dynamic_grid() {
        let mut g = DynamicGrid::<TestCell>::new(500, 0);
        g.get_cell_or_add_mut(500, 0).value = 'S';
        g.get_cell_or_add_mut(498, -2);
        print!("{g}");
        assert_eq!(g.num_rows, 3);
        assert_eq!(g.num_cols, 3);

        g.get_cell_or_add_mut(502, 2);
        print!("{g}");
        assert_eq!(g.num_rows, 5);
        assert_eq!(g.num_cols, 5);
    }

    #[test]
    fn test_iterator() {
        let mut g = DynamicGrid::<TestCell>::new(500, 0);
        g.get_cell_or_add_mut(500, 0).value = '*';
        g.get_cell_or_add_mut(500, -5).value = 'N';
        g.get_cell_or_add_mut(500, 5).value = 'S';
        g.get_cell_or_add_mut(505, 0).value = 'E';
        g.get_cell_or_add_mut(495, 0).value = 'W';
        print!("{g}");
        assert_eq!(g.num_rows, 11);
        assert_eq!(g.num_cols, 11);
        let mut s: String = "".to_string();
        for c in g.direction_iter(500, 0, crate::Direction::South) {
            s.push(c.value);
        }
        assert_eq!(s, "*....S");

        let mut s: String = "".to_string();
        for c in g.direction_iter(500, 0, crate::Direction::North) {
            s.push(c.value);
        }
        assert_eq!(s, "*....N");

        let mut s: String = "".to_string();
        for c in g.direction_iter(500, 0, crate::Direction::East) {
            s.push(c.value);
        }
        assert_eq!(s, "*....E");

        let mut s: String = "".to_string();
        for c in g.direction_iter(500, 0, crate::Direction::West) {
            s.push(c.value);
        }
        assert_eq!(s, "*....W");
    }
}
