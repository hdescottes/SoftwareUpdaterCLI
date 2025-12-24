mod mocks;

use crate::mocks::mock_inputs::MockInput;
use crate::mocks::mock_menu::MockMenu;
use cli::app::menu::{custom_menu_with, handle_menu_selection};
use std::fs;
use tempfile::NamedTempFile;
use test_case::test_case;

#[test_case(0 => true; "list does not stop loop")]
#[test_case(2 => true; "remove does not stop loop")]
#[test_case(3 => false; "quit stops loop")]
fn test_handle_menu_selection(selection: usize) -> bool {
    let mut apps = vec![];
    let input = MockInput::new(vec![]);
    handle_menu_selection(selection, &mut apps, &input, "path".as_ref())
}

#[test]
fn custom_menu_add_custom() {
    // Inputs simulés pour add_custom
    let input = MockInput::new(vec![
        "TestApp",
        "update cmd",
        "current version cmd",
        "latest version cmd",
    ]);

    // Séquence :
    // 1 → Ajouter
    // 3 → Quitter
    let menu = MockMenu::new(vec![1, 3]);

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    custom_menu_with(&input, &menu, path);

    let content = fs::read_to_string(path).unwrap();

    assert!(content.contains("TestApp"));
    assert!(content.contains("update cmd"));
    assert!(content.contains("current version cmd"));
    assert!(content.contains("latest version cmd"));
}