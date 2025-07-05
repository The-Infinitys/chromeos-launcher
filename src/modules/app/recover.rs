use crate::modules::app::run::run_qemu;
use crate::utils::error::Error;
use crate::utils::resource::ResourceValue;
use clap::Args;
use dirs;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Args)]
pub struct RecoverCommand {
    #[clap(long)]
    name: String,
    #[clap(long)]
    iso: String,
}

impl RecoverCommand {
    pub fn exec(&self) -> Result<(), Error> {
        println!("Recovering VM '{}' with ISO '{}'...", self.name, self.iso);

        let config_dir = dirs::home_dir()
            .ok_or_else(|| {
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            })?
            .join(".chromeos-launcher");
        let machines_dir = config_dir.join("machines");

        let config_file = machines_dir.join(&self.name);
        if !config_file.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Configuration for '{}' not found.", self.name),
            )));
        }

        let config_content = fs::read_to_string(&config_file)?;
        let config: HashMap<_, _> = config_content
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                Some((
                    parts.next()?.trim(),
                    parts.next()?.trim().trim_matches('\''),
                ))
            })
            .collect();

        let memory_str = config.get("MEMORY").unwrap_or(&"4G");
        let cpu_cores_str = config.get("CPU_CORES").unwrap_or(&"2");
        let cpu_model = config.get("CPU_MODEL").unwrap_or(&"host");
        let disk_path = config.get("DISK_PATH").unwrap();
        let ovmf_code_str = config.get("OVMF_CODE").map(|s| s.to_string());

        let memory = ResourceValue::from_str(memory_str)?;
        let cpu_cores = ResourceValue::from_str(cpu_cores_str)?;

        run_qemu(
            &self.name,
            "install",
            Some(self.iso.clone()),
            None, // No recovery path in recover command
            disk_path,
            &cpu_cores,
            &memory,
            cpu_model,
            ovmf_code_str,
            false, // No 3D accel in recover command
        )?;

        Ok(())
    }
}
