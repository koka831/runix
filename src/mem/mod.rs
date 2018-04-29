
use core::ops::{Index, IndexMut};


pub struct Page {
    n_page: usize,
}

pub struct Entry {
    flg: usize,
}

pub struct PageTable {
    entry: [Entry; 512],
}

pub struct MemoryManager {

}

impl Index<usize> for PageTable {
    type Output = Entry;

    fn index(&self, idx: usize) -> &Output {
        &self.entry[idx]
    }
}

impl IndexMut<usize> for PageTable {
    type Output = Entry;
    fn index_mut(&mut self, idx: usize) -> &mut Output {
        &mut self.entry[idx]
    }
}
