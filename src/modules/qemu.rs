use crate::utils::error::Error;
use crate::utils::resource::ResourceValue;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub struct QemuConfig {
    pub binary: String,
    pub ovmf_code_path: PathBuf,
    pub ovmf_vars_template: PathBuf,
}

pub fn detect_arch() -> Result<QemuConfig, Error> {
    let arch = std::env::consts::ARCH;
    let (qemu_binary, ovmf_dir_name) = match arch {
        "x86_64" => ("qemu-system-x86_64", "OVMF"),
        "aarch64" => ("qemu-system-aarch64", "AAVMF"),
        _ => {
            return Err(Error::Io(std::io::Error::other(format!(
                "Unsupported architecture: {}",
                arch
            ))));
        }
    };

    let ovmf_dir = PathBuf::from(format!("/usr/share/{}", ovmf_dir_name));
    if !ovmf_dir.exists() {
        return Err(Error::Io(std::io::Error::new(
            ErrorKind::NotFound,
            format!("OVMF/AAVMF directory not found: {}", ovmf_dir.display()),
        )));
    }

    let ovmf_code_path = find_ovmf_file(&ovmf_dir, "CODE")?;
    let ovmf_vars_template = find_ovmf_file(&ovmf_dir, "VARS")?;

    Ok(QemuConfig {
        binary: qemu_binary.to_string(),
        ovmf_code_path,
        ovmf_vars_template,
    })
}

fn find_ovmf_file(ovmf_dir: &PathBuf, file_type: &str) -> Result<PathBuf, Error> {
    for entry in fs::read_dir(ovmf_dir)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
            if file_name.contains(file_type) && file_name.ends_with(".fd") {
                return Ok(path);
            }
        }
    }
    Err(Error::Io(std::io::Error::new(
        ErrorKind::NotFound,
        format!(
            "OVMF/AAVMF {} file not found in {}",
            file_type,
            ovmf_dir.display()
        ),
    )))
}

pub fn resolve_value(value: &ResourceValue, total: u64, unit: Option<&str>) -> String {
    match value {
        ResourceValue::Absolute(val) => {
            if let Some(u) = unit {
                match u {
                    "M" => format!("{}M", (val / (1024 * 1024))),
                    "G" => format!("{}G", (val / (1024 * 1024 * 1024))),
                    _ => val.to_string(),
                }
            } else {
                val.to_string()
            }
        }
        ResourceValue::Percentage(percentage) => {
            let result = (total * (*percentage as u64) / 100) as f64;
            match unit {
                Some("M") => format!("{}M", (result / 1024.0).round() as u64),
                Some("G") => format!("{}G", (result / 1024.0 / 1024.0).round() as u64),
                _ => format!("{}", result.round() as u64),
            }
        }
    }
}
