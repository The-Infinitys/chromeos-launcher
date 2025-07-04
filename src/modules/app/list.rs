use crate::utils::error::Error;
use dirs;
use std::fs;
use std::collections::HashMap;

pub fn list() -> Result<(), Error> {
    let machines_dir = dirs::home_dir()
        .ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        })?
        .join(".chromeos-launcher/machines");

    if !machines_dir.exists() {
        println!("No virtual machines found.");
        return Ok(());
    }

    let entries = fs::read_dir(machines_dir)?;
    println!("Existing VMs:");
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let vm_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let config_content = fs::read_to_string(&path)?;
            let config: HashMap<_, _> = config_content
                .lines()
                .filter_map(|line| {
                    let mut parts = line.splitn(2, '=');
                    Some((parts.next()?.trim(), parts.next()?.trim()))
                })
                .collect();

            let memory = config.get("MEMORY").unwrap_or(&"N/A");
            let cpu_cores = config.get("CPU_CORES").unwrap_or(&"N/A");
            let ovmf_code = config.get("OVMF_CODE").unwrap_or(&"N/A");

            println!("  - Name: {}", vm_name);
            println!("    Memory: {}", memory);
            println!("    CPU Cores: {}", cpu_cores);
            println!("    OVMF Code: {}", ovmf_code);
        }
    }

    Ok(())
}