use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

#[derive(Clone, Debug)]
pub struct Board<T> {
    cells: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Cell {
    Wall         = b'#',
    Player       = b'@',
    PlayerOnGoal = b'+',
    Box          = b'$',
    BoxOnGoal    = b'*',
    Goal         = b'.',
    Floor        = b' ',
}

impl Default for Cell {
    fn default() -> Cell {
        Cell::Floor
    }
}

impl Board<Cell> {
    pub fn dump_ascii(&self) {
        for line in self.cells.chunks(self.width) {
            for &b in line { print!("{}", b as u8 as char); }
            print!("\n");
        }
    }
}

impl Board<bool> {
    pub fn new_test_board() -> Self {
        let mut board = Board::new(2, 3);
        board[2][0] = true;
        board[1][0] = true;
        board[0][0] = true;
        board[2][1] = true;
        board
    }
}

impl<T> Board<T> {
    pub fn new(width: usize, height: usize) -> Self where T: Clone + Default {
        Board {
            cells: vec![T::default(); width * height],
            width: width,
            height: height,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl<T> Index<usize> for Board<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        &self.cells[start..start + self.width]
    }
}

impl<T> IndexMut<usize> for Board<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        &mut self.cells[start..start + self.width]
    }
}

impl<T> IntoIterator for Board<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Board<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Board<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.iter_mut()
    }
}
