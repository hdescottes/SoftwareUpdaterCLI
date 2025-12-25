use dialoguer::Select;

pub trait RemoveSelect {
    fn select(&self, items: &Vec<String>) -> usize;
}

pub struct DialoguerRemoveSelect;

impl RemoveSelect for DialoguerRemoveSelect {
    fn select(&self, items: &Vec<String>) -> usize {
        Select::new()
            .with_prompt("Quel logiciel voulez-vous supprimer ?")
            .items(items)
            .default(0)
            .interact()
            .unwrap()
    }
}