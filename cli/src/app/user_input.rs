use dialoguer::Input;

pub trait UserInput {
    fn ask(&self, prompt: &str) -> String;
}

pub struct DialoguerInput;

impl UserInput for DialoguerInput {
    fn ask(&self, prompt: &str) -> String {
        Input::new()
            .with_prompt(prompt)
            .interact_text()
            .unwrap()
    }
}