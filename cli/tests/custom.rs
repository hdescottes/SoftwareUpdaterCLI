use cli::custom;
use test_case::test_case;

#[test_case(0 => true; "list does not stop loop")]
#[test_case(2 => true; "remove does not stop loop")]
#[test_case(3 => false; "quit stops loop")]
fn test_handle_menu_selection(selection: usize) -> bool {
    let mut apps = vec![];
    custom::handle_menu_selection(selection, &mut apps)
}