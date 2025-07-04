use crate::utils::error::Error;
use crate::utils::shell::is_available;
use clap::Args;
use dirs;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use crate::modules::app::run::run_qemu;
use crate::utils::resource::ResourceValue;

#[derive(Args)]
pub struct NewCommand {
    #[clap(long)]
    name: String,
    #[clap(long)]
    iso: String,
    #[clap(long)]
    disk: String,
    #[clap(long, default_value = "64G")]
    disk_size: String,
    #[clap(long, default_value = "2", value_parser = ResourceValue::from_str)]
    cpu_cores: ResourceValue,
    #[clap(long, default_value = "4G", value_parser = ResourceValue::from_str)]
    memory: ResourceValue,
    #[clap(long, default_value = "host")]
    cpu_model: String,
    #[clap(long)]
    ovmf_code: Option<String>,
}

impl NewCommand {
    pub fn exec(&self) -> Result<(), Error> {
        println!("Creating new VM...");

        let config_dir = dirs::home_dir()
            .ok_or_else(|| {
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            })?
            .join(".chromeos-launcher");
        let machines_dir = config_dir.join("machines");
        fs::create_dir_all(&machines_dir)?;

        let config_file = machines_dir.join(&self.name);
        if config_file.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("VM '{}' already exists", self.name),
            )));
        }

        let disk_path = PathBuf::from(&self.disk);
        if !disk_path.exists() {
            if !is_available("qemu-img") {
                println!("qemu-img is not installed. Please install it to create a disk image.");
                return Ok(());
            }
            println!(
                "Creating new disk image at '{}' with size {}",
                self.disk, self.disk_size
            );
            Command::new("qemu-img")
                .arg("create")
                .arg("-f")
                .arg("raw")
                .arg(&self.disk)
                .arg(&self.disk_size)
                .status()?;
        }

        let config_content = format!(
            "VM_NAME={}\nDISK_PATH={}\nCPU_CORES={}\nMEMORY={}\nCPU_MODEL={}\nOVMF_CODE={}\n",
            self.name, self.disk, self.cpu_cores.to_string(), self.memory.to_string(), self.cpu_model, self.ovmf_code.as_deref().unwrap_or("")
        );
        fs::write(config_file, config_content)?;

        println!("Configuration for '{}' created successfully.", self.name);
        println!("Starting installation...");

        run_qemu(
            &self.name,
            "install",
            Some(self.iso.clone()),
            &self.disk,
            &self.cpu_cores,
            &self.memory,
            &self.cpu_model,
            self.ovmf_code.clone(),
        )?;

        Ok(())
    }
}
