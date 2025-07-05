use crate::modules::qemu::{self};
use crate::utils::error::Error;
use crate::utils::resource::ResourceValue;
use clap::Args;
use dirs;
use std::collections::HashMap;
use std::fs;
use std::process::{Command, Stdio};
use std::str::FromStr;

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
                Some((
                    parts.next()?.trim(),
                    parts.next()?.trim().trim_matches('\''),
                ))
            })
            .collect();

        let memory_str = config.get("MEMORY").unwrap_or(&"4G");
        let cpu_cores_str = config.get("CPU_CORES").unwrap_or(&"2");
        let cpu_model = config.get("CPU_MODEL").unwrap_or(&"host");
        let disk_path = config.get("DISK_PATH").unwrap().to_string();
        let ovmf_code_str = config.get("OVMF_CODE").map(|s| s.to_string());
        let recovery_path = config.get("RECOVERY_PATH").map(|s| s.to_string());
        let use_3d_accel = config
            .get("USE_3D_ACCEL")
            .and_then(|s| s.parse::<bool>().ok())
            .unwrap_or(false);

        let memory = ResourceValue::from_str(memory_str)?;
        let cpu_cores = ResourceValue::from_str(cpu_cores_str)?;

        run_qemu(
            &vm_name,
            "run",
            None,
            recovery_path,
            &disk_path,
            &cpu_cores,
            &memory,
            cpu_model,
            ovmf_code_str,
            use_3d_accel,
        )?;

        Ok(())
    }
}

pub fn run_qemu(
    vm_name: &str,
    mode: &str,
    iso_path: Option<String>,
    recovery_path: Option<String>,
    disk_path: &str,
    cpu_cores: &ResourceValue,
    memory: &ResourceValue,
    cpu_model: &str,
    ovmf_code: Option<String>,
    use_3d_accel: bool,
) -> Result<(), Error> {
    let qemu_config = qemu::detect_arch()?;

    let total_mem_kb = sys_info::mem_info()
        .map_err(|e| Error::Io(std::io::Error::other(format!("Failed to get memory info: {}", e))))?
        .total;
    let total_cores = sys_info::cpu_num()
        .map_err(|e| Error::Io(std::io::Error::other(format!("Failed to get CPU info: {}", e))))?
        as u64;

    let resolved_mem = qemu::resolve_value(memory, total_mem_kb, Some("G"));
    let resolved_cores = qemu::resolve_value(cpu_cores, total_cores, None);

    let config_dir = dirs::home_dir()
        .ok_or_else(|| Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Home directory not found")))?
        .join(".chromeos-launcher");
    let last_run_file = config_dir.join("last_run");

    let mut qemu_args = Vec::<String>::new();

    if mode == "install" {
        if let Some(ref iso) = iso_path {
            let disk_arg = format!("format=qcow2,file={}", disk_path);
            qemu_args.extend(vec![
                "-display".to_string(),
                "sdl,show-cursor=on".to_string(),
                "-boot".to_string(),
                "order=d".to_string(),
                "-cdrom".to_string(),
                iso.to_string(),
                "-drive".to_string(),
                disk_arg,
                "-m".to_string(),
                resolved_mem.clone(),
                "-enable-kvm".to_string(),
                "-smp".to_string(),
                resolved_cores.clone(),
                "-usb".to_string(),
                "-device".to_string(),
                "usb-tablet".to_string(),
            ]);
        } else {
            return Err(Error::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Install mode requires an ISO path.",
            )));
        }
    } else {
        let machines_dir = config_dir.join("machines");
        let ovmf_vars_copy = machines_dir.join(format!("{}.vars", vm_name));
        if !ovmf_vars_copy.exists() {
            fs::copy(&qemu_config.ovmf_vars_template, &ovmf_vars_copy)?;
        }

        let ovmf_code_path = if let Some(path) = ovmf_code {
            std::path::PathBuf::from(path)
        } else {
            qemu_config.ovmf_code_path.clone()
        };

        let ovmf_code_arg = format!("if=pflash,format=raw,readonly=on,file={}", ovmf_code_path.display());
        let ovmf_vars_arg = format!("if=pflash,format=raw,file={}", ovmf_vars_copy.display());
        let disk_arg = format!("format=qcow2,file={}", disk_path);

        qemu_args.extend(vec![
            "-drive".to_string(), ovmf_code_arg,
            "-drive".to_string(), ovmf_vars_arg,
            "-display".to_string(), "sdl,show-cursor=on,gl=on".to_string(),
            "-usb".to_string(),
            "-device".to_string(), "usb-tablet".to_string(),
        ]);

        if let Some(rec_path) = recovery_path {
            qemu_args.push("-drive".to_string());
            qemu_args.push(format!("format=raw,file={}", rec_path));
        }

        qemu_args.push("-drive".to_string());
        qemu_args.push(disk_arg);

        qemu_args.extend(vec![
            "-m".to_string(), resolved_mem.clone(),
            "-enable-kvm".to_string(),
            "-smp".to_string(), resolved_cores.clone(),
            "-audiodev".to_string(), "sdl,id=audio0".to_string(),
            "-device".to_string(), "intel-hda".to_string(),
            "-device".to_string(), "hda-output,audiodev=audio0".to_string(),
            "-cpu".to_string(), cpu_model.to_string(),
        ]);

        if use_3d_accel {
            qemu_args.extend(vec!["-vga".to_string(), "virtio".to_string()]);
        } else {
            let (xres, yres) = (1280, 800);
            qemu_args.extend(vec![
                "-device".to_string(),
                format!("virtio-vga-gl,xres={},yres={}", xres, yres),
            ]);
        }
    }

    println!("---");
    println!("Starting QEMU for '{}'...", vm_name);
    println!("  QEMU Binary: {}", qemu_config.binary);
    println!("  Memory: {}", resolved_mem);
    println!("  CPU Cores: {}", resolved_cores);
    println!("  Disk: {}", disk_path);

    if mode != "install" {
        println!("  UEFI Code: {}", qemu_config.ovmf_code_path.display());
        if use_3d_accel {
            println!("  3D Acceleration: Enabled");
        }
    }
    if let Some(iso) = iso_path.as_ref() {
        println!("  ISO (Install Mode): {}", iso);
    }
    println!("---");

    fs::write(&last_run_file, vm_name)?;

    let qemu_args_str: Vec<&str> = qemu_args.iter().map(|s| s.as_str()).collect();
    Command::new(&qemu_config.binary)
        .args(&qemu_args_str)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    Ok(())
}

