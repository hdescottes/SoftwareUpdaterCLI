use std::fs;
use crate::app::user_input::UserInput;
use crate::domain::custom_app::CustomApp;
use crate::storage::store::save_apps;
use crate::use_cases::remove_select::RemoveSelect;
use crate::use_cases::command::run_command;
use semver::Version;
use std::path::Path;
use serde_json::Value;

pub fn list_custom(apps: &[CustomApp]) {
    if apps.is_empty() {
        println!("Aucun logiciel personnalisé enregistré.");
        return;
    }

    println!("\n🗂 Logiciels suivis :");

    for app in apps {
        println!("📦 {}", app.name);

        let current = read_version(&app.current_version_path);
        let latest = run_command(&app.latest_version_command).trim().to_string();

        println!(" Version actuelle : {}", current);
        println!(" Version cible : {}", latest);
        println!(" Commande MAJ : {}", app.update_command);

        if is_update_available(&current, &latest) {
            println!(" 🔄 Mise à jour disponible !");
        } else {
            println!(" ✅ À jour");
        }
    }
}

fn read_version(json_path: &str) -> String {
    let possible_keys = ["version", "appVersion"];

    fs::read_to_string(json_path)
        .ok()
        .and_then(|c| serde_json::from_str::<Value>(&c).ok())
        .and_then(|json| {
            possible_keys
                .iter()
                .find_map(|key| {
                    json.get(*key)
                        .and_then(|v| v.as_str())
                        .map(str::to_string)
                })
        })
        .unwrap_or_default()
}

fn is_update_available(current: &str, latest: &str) -> bool {
    let v1 = Version::parse(current.trim()).ok();
    let v2 = Version::parse(latest.trim()).ok();

    match (v1, v2) {
        (Some(c), Some(l)) => c < l,
        _ => false,
    }
}

pub fn add_custom<I: UserInput>(
    apps: &mut Vec<CustomApp>,
    input: &I,
    file_path: &Path,
) {
    let name = input.ask("Nom du logiciel");
    let update_command = input.ask("Commande pour mettre à jour");
    let current_version_path = input.ask("Chemin de la version actuelle");
    let latest_version_command = input.ask("Commande pour obtenir la dernière version");

    apps.push(CustomApp {
        name,
        update_command,
        current_version_path,
        latest_version_command,
    });

    save_apps(apps, file_path);
    println!("Logiciel ajouté avec succès.");
}

pub fn remove_custom<S: RemoveSelect>(
    apps: &mut Vec<CustomApp>,
    input: &S,
    file_path: &Path
) {
    if apps.is_empty() {
        println!("Aucun logiciel à supprimer.");
        return;
    }

    let items: Vec<String> = apps.iter().map(|a| a.name.clone()).collect();
    let selection = input.select(&items);

    apps.remove(selection);
    save_apps(&apps, file_path);
    println!("Logiciel supprimé.");
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1.0.0", "1.1.0" => true)]
    #[test_case("1.0.0", "1.0.0" => false)]
    fn test_is_update_available(current: &str, target: &str) -> bool {
        is_update_available(current, target)
    }
}