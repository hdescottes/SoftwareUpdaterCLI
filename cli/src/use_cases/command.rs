/*use std::process::Command;

pub fn run_command(cmd: &str) -> String {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", cmd])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_else(|_| "Erreur d'exécution".to_string())
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
            .unwrap_or_else(|_| "Erreur d'exécution".to_string())
    }
}*/