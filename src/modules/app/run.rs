use crate::modules::qemu::{self};
use crate::utils::error::Error;
use clap::Args;
use dirs;
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};
use std::str::FromStr;
use crate::utils::resource::ResourceValue;

#[derive(Args)]
pub struct RunCommand {
    name: Option<String>,
}

impl RunCommand {
    pub fn exec(&self) -> Result<(), Error> {
        let config_dir = dirs::home_dir()
            .ok_or_else(|| {
                Error::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Home directory not found",
                ))
            })?
            .join(".chromeos-launcher");
        let machines_dir = config_dir.join("machines");
        let last_run_file = config_dir.join("last_run");

        let vm_name = match &self.name {
            Some(name) => name.clone(),
            None => {
                if !last_run_file.exists() {
                    return Err(Error::Io(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "No machine name specified and no last-run machine found.",
                    )));
                }
                fs::read_to_string(&last_run_file)?.trim().to_string()
            }
        };

        let config_file = machines_dir.join(&vm_name);
        if !config_file.exists() {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Configuration for '{}' not found.", vm_name),
            )));
        }

        let config_content = fs::read_to_string(&config_file)?;
        let config: HashMap<_, _> = config_content
            .lines()
            .filter_map(|line| {
                let mut parts = line.splitn(2, '=');
                Some((parts.next()?.trim(), parts.next()?.trim()))
            })
            .collect();

        let memory_str = config.get("MEMORY").unwrap_or(&"4G");
        let cpu_cores_str = config.get("CPU_CORES").unwrap_or(&"2");
        let cpu_model = config.get("CPU_MODEL").unwrap_or(&"host");
        let disk_path = config.get("DISK_PATH").unwrap();
        let iso_path = config.get("ISO_PATH");
        let ovmf_code_str = config.get("OVMF_CODE").map(|s| s.to_string());

        let memory = ResourceValue::from_str(memory_str)?;
        let cpu_cores = ResourceValue::from_str(cpu_cores_str)?;

        run_qemu(
            &vm_name,
            if iso_path.is_some() {
                "install"
            } else {
                "normal"
            },
            iso_path.map(|s| s.to_string()),
            disk_path,
            &cpu_cores,
            &memory,
            cpu_model,
            ovmf_code_str,
        )?;

        Ok(())
    }
}

pub fn run_qemu(
    vm_name: &str,
    mode: &str,
    iso_path: Option<String>,
    disk_path: &str,
    cpu_cores: &ResourceValue,
    memory: &ResourceValue,
    cpu_model: &str,
    ovmf_code: Option<String>,
) -> Result<(), Error> {
    let qemu_config = qemu::detect_arch()?;

    let total_mem_kb = sys_info::mem_info()
        .map_err(|e| {
            Error::Io(std::io::Error::other(format!(
                "Failed to get memory info: {}",
                e
            )))
        })?
        .total;
    let total_cores = sys_info::cpu_num().map_err(|e| {
        Error::Io(std::io::Error::other(format!(
            "Failed to get CPU info: {}",
            e
        )))
    })? as u64;

    let resolved_mem = qemu::resolve_value(memory, total_mem_kb, Some("G"));
    let resolved_cores = qemu::resolve_value(cpu_cores, total_cores, None);

    let config_dir = dirs::home_dir()
        .ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Home directory not found",
            ))
        })?
        .join(".chromeos-launcher");
    let machines_dir = config_dir.join("machines");
    let last_run_file = config_dir.join("last_run");

    let ovmf_vars_copy = machines_dir.join(format!("{}.vars", vm_name));
    if !ovmf_vars_copy.exists() {
        fs::copy(&qemu_config.ovmf_vars_template, &ovmf_vars_copy)?;
    }

    let ovmf_code_path = if let Some(path) = ovmf_code {
        std::path::PathBuf::from(path)
    } else {
        qemu_config.ovmf_code_path
    };

    let ovmf_code_arg = format!(
        "if=pflash,format=raw,readonly=on,file={}",
        ovmf_code_path.display()
    );
    let ovmf_vars_arg = format!("if=pflash,format=raw,file={}", ovmf_vars_copy.display());
    let disk_arg = format!("format=raw,file={}", disk_path);

    let mut qemu_args = vec![
        "-m",
        &resolved_mem,
        "-smp",
        &resolved_cores,
        "-cpu",
        cpu_model,
        "-enable-kvm",
        "-device",
        "virtio-vga-gl",
        "-display",
        "sdl,gl=on",
        "-usb",
        "-device",
        "usb-tablet",
        "-audiodev",
        "sdl,id=audio0",
        "-device",
        "intel-hda",
        "-device",
        "hda-output,audiodev=audio0",
        "-drive",
        &ovmf_code_arg,
        "-drive",
        &ovmf_vars_arg,
        "-drive",
        &disk_arg,
    ];

    if mode == "install" {
        if let Some(iso) = iso_path.as_ref() {
            qemu_args.push("-cdrom");
            qemu_args.push(iso);
            qemu_args.push("-boot");
            qemu_args.push("order=d");
        }
    }

    println!("---");
    println!("Starting QEMU for '{}'...", vm_name);
    println!("  QEMU Binary: {}", qemu_config.binary);
    println!("  Memory: {}", resolved_mem);
    println!("  CPU Cores: {}", resolved_cores);
    println!("  Disk: {}", disk_path);
    println!("  UEFI Code: {}", ovmf_code_path.display());
    if mode == "install" {
        if let Some(iso) = iso_path.as_ref() {
            println!("  ISO (Install Mode): {}", iso);
        }
    }
    println!("---");

    fs::write(&last_run_file, vm_name)?;

    Command::new(&qemu_config.binary)
        .args(qemu_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}