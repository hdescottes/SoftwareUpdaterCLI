use cli::use_cases::remove_select::RemoveSelect;

pub struct MockRemove {
    selections: Vec<usize>,
    index: std::cell::Cell<usize>,
}

impl MockRemove {
    pub fn new(selections: Vec<usize>) -> Self {
        Self {
            selections,
            index: std::cell::Cell::new(0),
        }
    }
}

impl RemoveSelect for MockRemove {
    fn select(&self, _: &Vec<String>) -> usize {
        let i = self.index.get();
        self.index.set(i + 1);
        self.selections[i]
    }
}