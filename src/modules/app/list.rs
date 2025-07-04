use crate::utils::error::Error;
use dirs;
use std::fs;

pub fn list() -> Result<(), Error> {
    let machines_dir = dirs::home_dir()
        .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")))?
        .join(".chromeos-launcher/machines");

    if !machines_dir.exists() {
        println!("No virtual machines found.");
        return Ok(());
    }

    let entries = fs::read_dir(machines_dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("{}", path.file_name().unwrap().to_str().unwrap());
        }
    }

    Ok(())
}
