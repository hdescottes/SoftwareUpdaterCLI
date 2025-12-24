use cli::app::user_input::UserInput;

pub struct MockInput {
    answers: Vec<String>,
    index: std::cell::Cell<usize>,
}

impl MockInput {
    pub fn new(answers: Vec<&str>) -> Self {
        Self {
            answers: answers.into_iter().map(String::from).collect(),
            index: std::cell::Cell::new(0),
        }
    }
}

impl UserInput for MockInput {
    fn ask(&self, _: &str) -> String {
        let i = self.index.get();
        self.index.set(i + 1);
        self.answers[i].clone()
    }
}