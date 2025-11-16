use crate::data::{load_apps, save_apps, CustomApp};
use crate::utils::run_command;
use dialoguer::{Input, Select};
use semver::Version;


pub fn custom_menu() {
    let mut apps = load_apps();

    loop {
        println!("\n=== Gestionnaire de logiciels personnalisés ===");
        let options = vec![
            "Lister les logiciels",
            "Ajouter un logiciel",
            "Supprimer un logiciel",
            "Quitter",
        ];

        let selection = Select::new()
            .with_prompt("Que souhaitez-vous faire ?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();
        
        match selection {
            0 => list_custom(&apps),
            1 => add_custom(&mut apps),
            2 => remove_custom(&mut apps),
            3 => break,
            _ => unreachable!(),
        }
    }
}


fn list_custom(apps: &[CustomApp]) {
    if apps.is_empty() {
        println!("Aucun logiciel personnalisé enregistré.");
        return;
    }
    
    println!("\n🗂 Logiciels suivis :");
    
    for app in apps {
        println!("📦 {}", app.name);
        
        let current = run_command(&app.current_version_command).trim().to_string();
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


fn is_update_available(current: &str, latest: &str) -> bool {
    let v1 = Version::parse(current.trim()).ok();
    let v2 = Version::parse(latest.trim()).ok();
    
    match (v1, v2) {
        (Some(c), Some(l)) => c < l,
        _ => false,
    }
}


fn add_custom(apps: &mut Vec<CustomApp>) {
    let name: String = Input::new()
        .with_prompt("Nom du logiciel")
        .interact_text()
        .unwrap();
    
    let update_command: String = Input::new()
        .with_prompt("Commande pour mettre à jour")
        .interact_text()
        .unwrap();

    let current_version_command: String = Input::new()
        .with_prompt("Commande pour obtenir la version actuelle")
        .interact_text()
        .unwrap();

    let latest_version_command: String = Input::new()
        .with_prompt("Commande pour obtenir la dernière version")
        .interact_text()
        .unwrap();
    
    apps.push(CustomApp {
        name,
        update_command,
        current_version_command,
        latest_version_command,
    });
    save_apps(&apps);
    println!("Logiciel ajouté avec succès.");
}


fn remove_custom(apps: &mut Vec<CustomApp>) {
    if apps.is_empty() {
        println!("Aucun logiciel à supprimer.");
        return;
    }
    
    let items: Vec<String> = apps.iter().map(|a| a.name.clone()).collect();
    let selection = Select::new()
        .with_prompt("Quel logiciel voulez-vous supprimer ?")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();
    
    apps.remove(selection);
    save_apps(&apps);
    println!("Logiciel supprimé.");
}