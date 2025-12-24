use cli::app::menu_select::MenuSelect;

pub struct MockMenu {
    selections: Vec<usize>,
    index: std::cell::Cell<usize>,
}

impl MockMenu {
    pub fn new(selections: Vec<usize>) -> Self {
        Self {
            selections,
            index: std::cell::Cell::new(0),
        }
    }
}

impl MenuSelect for MockMenu {
    fn select(&self, _: &[&str]) -> usize {
        let i = self.index.get();
        self.index.set(i + 1);
        self.selections[i]
    }
}