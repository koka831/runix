

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
