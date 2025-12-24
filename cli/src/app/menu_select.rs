use dialoguer::Select;

pub trait MenuSelect {
    fn select(&self, options: &[&str]) -> usize;
}

pub struct DialoguerMenuSelect;

impl MenuSelect for DialoguerMenuSelect {
    fn select(&self, options: &[&str]) -> usize {
        Select::new()
            .with_prompt("Que souhaitez-vous faire ?")
            .items(options)
            .default(0)
            .interact()
            .unwrap()
    }
}