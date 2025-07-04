use crate::utils::error::Error;
use clap::Args;
use dirs;
use std::fs;
use std::io::{self, Write};

#[derive(Args)]
pub struct RemoveCommand {
    name: String,
}

impl RemoveCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")))?
            .join(".chromeos-launcher");
        let machines_dir = config_dir.join("machines");
        let config_file = machines_dir.join(&self.name);

        if !config_file.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("VM '{}' not found", self.name),
            )));
        }

        let config_content = fs::read_to_string(&config_file)?;
        let disk_path_line = config_content.lines().find(|line| line.starts_with("DISK_PATH="));
        let disk_path = disk_path_line.map(|line| line.split('=').nth(1).unwrap().trim());

        print!("Are you sure you want to remove the VM '{}'? [y/N] ", self.name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() != "y" {
            println!("Removal cancelled.");
            return Ok(());
        }

        if let Some(disk_path) = disk_path {
            if fs::metadata(disk_path).is_ok() {
                print!("Do you also want to delete the disk file '{}'? [y/N] ", disk_path);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim().to_lowercase() == "y" {
                    println!("Deleting disk file: {}", disk_path);
                    fs::remove_file(disk_path)?;
                }
            }
        }

        println!("Deleting VM configuration for '{}'.", self.name);
        fs::remove_file(config_file)?;

        let ovmf_vars = machines_dir.join(format!("{}.vars", self.name));
        if ovmf_vars.exists() {
            fs::remove_file(ovmf_vars)?;
        }

        let last_run_file = config_dir.join("last_run");
        if last_run_file.exists() {
            let last_run_vm = fs::read_to_string(&last_run_file)?;
            if last_run_vm.trim() == self.name {
                fs::remove_file(last_run_file)?;
            }
        }

        println!("Successfully removed '{}'.", self.name);

        Ok(())
    }
}
