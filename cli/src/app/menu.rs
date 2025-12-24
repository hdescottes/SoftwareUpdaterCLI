use crate::app::menu_select::{DialoguerMenuSelect, MenuSelect};
use crate::app::user_input::{DialoguerInput, UserInput};
use crate::domain::custom_app::CustomApp;
use crate::storage::store::load_apps;
use crate::use_cases::custom_app::{add_custom, list_custom, remove_custom};
use std::path::Path;

const FILE_PATH: &str = "cli/apps.json";

pub fn custom_menu() {
    let input = DialoguerInput;
    let menu = DialoguerMenuSelect;

    custom_menu_with(
        &input,
        &menu,
        FILE_PATH.as_ref(),
    );
}

pub fn custom_menu_with<I, M>(
    input: &I,
    menu: &M,
    file_path: &Path,
) where
    I: UserInput,
    M: MenuSelect,
{
    let mut apps = load_apps(file_path);

    loop {
        let options = [
            "Lister les logiciels",
            "Ajouter un logiciel",
            "Supprimer un logiciel",
            "Quitter",
        ];

        let selection = menu.select(&options);

        if !handle_menu_selection(selection, &mut apps, input, file_path) {
            break;
        }
    }
}

pub fn handle_menu_selection<I: UserInput>(
    selection: usize,
    apps: &mut Vec<CustomApp>,
    input: &I,
    file_path: &Path,
) -> bool {
    match selection {
        0 => {
            list_custom(apps);
            true
        }
        1 => {
            add_custom(apps, input, file_path);
            true
        }
        2 => {
            remove_custom(apps, file_path);
            true
        }
        3 => false,
        _ => unreachable!(),
    }
}