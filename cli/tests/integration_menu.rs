mod mocks;

use crate::mocks::mock_inputs::MockInput;
use crate::mocks::mock_menu::MockMenu;
use crate::mocks::mock_remove::MockRemove;
use cli::app::menu::{custom_menu_with, handle_menu_selection};
use cli::domain::custom_app::CustomApp;
use std::fs;
use tempfile::NamedTempFile;
use test_case::test_case;

#[test_case(0 => true; "list does not stop loop")]
#[test_case(2 => true; "remove does not stop loop")]
#[test_case(3 => false; "quit stops loop")]
fn test_handle_menu_selection_with_empty_apps_list(selection: usize) -> bool {
    let mut apps = vec![];
    let input = MockInput::new(vec![]);
    let remove = MockRemove::new(vec![]);
    handle_menu_selection(selection, &mut apps, &input, &remove, "path".as_ref())
}

#[test]
fn test_custom_menu_list_apps() {
    let input = MockInput::new(vec![]);

    // Séquence :
    // 0 → Lister
    // 3 → Quitter
    let menu = MockMenu::new(vec![0, 3]);
    let remove = MockRemove::new(vec![]);

    let temp_file = NamedTempFile::new().unwrap();
    fs::copy("tests/fixtures/apps_data.json", temp_file.path()).unwrap();
    let path = temp_file.path();

    custom_menu_with(&input, &menu, &remove, path);

    let content_result = fs::read_to_string(path).unwrap();
    let result: Vec<CustomApp> = serde_json::from_str(&content_result).unwrap();

    assert_eq!(result.len(), 2);
}

#[test]
fn test_custom_menu_add_app() {
    let input = MockInput::new(vec![
        "TestApp",
        "update cmd",
        "current version path",
        "latest version cmd",
    ]);

    // Séquence :
    // 1 → Ajouter
    // 3 → Quitter
    let menu = MockMenu::new(vec![1, 3]);
    let remove = MockRemove::new(vec![]);

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path();

    custom_menu_with(&input, &menu, &remove, path);

    let content = fs::read_to_string(path).unwrap();
    let result: Vec<CustomApp> = serde_json::from_str(&content).unwrap();

    assert_eq!(result.len(), 1);
    assert!(content.contains("TestApp"));
    assert!(content.contains("update cmd"));
    assert!(content.contains("current version path"));
    assert!(content.contains("latest version cmd"));
}

#[test]
fn test_custom_menu_delete_app() {
    let input = MockInput::new(vec![]);

    // Séquence :
    // 2 → Supprimer
    // 3 → Quitter
    let menu = MockMenu::new(vec![2, 3]);
    let remove = MockRemove::new(vec![0]);

    let temp_file = NamedTempFile::new().unwrap();
    fs::copy("tests/fixtures/apps_data.json", temp_file.path()).unwrap();
    let path = temp_file.path();
    let content = fs::read_to_string(temp_file.path()).unwrap();
    let apps: Vec<CustomApp> = serde_json::from_str(&content).unwrap();
    let name = &apps[0].name;

    custom_menu_with(&input, &menu, &remove, path);

    let content_result = fs::read_to_string(path).unwrap();
    let result: Vec<CustomApp> = serde_json::from_str(&content_result).unwrap();

    assert!(result.iter().find(|a| a.name == *name).is_none());
    assert_eq!(result.len(), 1);
}