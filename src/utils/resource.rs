use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum ResourceValue {
    Absolute(u128),
    Percentage(u8),
}

impl FromStr for ResourceValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            let percentage_str = s.trim_end_matches('%');
            let percentage = percentage_str
                .parse::<u8>()
                .map_err(|_| format!("Invalid percentage value: {}", s))?;
            if percentage > 100 {
                return Err(format!("Percentage cannot exceed 100%: {}", s));
            }
            Ok(ResourceValue::Percentage(percentage))
        } else {
            let re_mem = Regex::new(r"^(\d+)([GMK]?)$").unwrap();
            if let Some(caps) = re_mem.captures(s) {
                let value = caps[1]
                    .parse::<u128>()
                    .map_err(|_| format!("Invalid absolute value: {}", s))?;
                let unit = caps.get(2).map_or("", |m| m.as_str());
                let multiplier = match unit {
                    "G" => 1024 * 1024 * 1024,
                    "M" => 1024 * 1024,
                    "K" => 1024,
                    "" => 1, // No unit, assume bytes for memory or raw count for CPU
                    _ => return Err(format!("Unknown unit: {}", unit)),
                };
                Ok(ResourceValue::Absolute(value * multiplier))
            } else {
                s.parse::<u128>()
                    .map(|val| ResourceValue::Absolute(val))
                    .map_err(|_| format!("Invalid resource value: {}", s))
            }
        }
    }
}

impl ToString for ResourceValue {
    fn to_string(&self) -> String {
        match self {
            ResourceValue::Absolute(val) => val.to_string(),
            ResourceValue::Percentage(val) => format!("{}%", val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_absolute_memory() {
        assert_eq!(
            ResourceValue::from_str("4G").unwrap(),
            ResourceValue::Absolute(4 * 1024 * 1024 * 1024)
        );
        assert_eq!(
            ResourceValue::from_str("512M").unwrap(),
            ResourceValue::Absolute(512 * 1024 * 1024)
        );
        assert_eq!(
            ResourceValue::from_str("1024K").unwrap(),
            ResourceValue::Absolute(1024 * 1024)
        );
        assert_eq!(
            ResourceValue::from_str("100").unwrap(),
            ResourceValue::Absolute(100)
        );
    }

    #[test]
    fn test_from_str_percentage() {
        assert_eq!(
            ResourceValue::from_str("50%").unwrap(),
            ResourceValue::Percentage(50)
        );
        assert_eq!(
            ResourceValue::from_str("100%").unwrap(),
            ResourceValue::Percentage(100)
        );
    }

    #[test]
    fn test_from_str_invalid() {
        assert!(ResourceValue::from_str("101%").is_err());
        assert!(ResourceValue::from_str("abc").is_err());
        assert!(ResourceValue::from_str("10GB").is_err());
    }

    #[test]
    fn test_to_string_absolute() {
        assert_eq!(
            ResourceValue::Absolute(4 * (1024 * 1024 * 1024) as u128).to_string(),
            (4 * 1024 * 1024 * 1024u128).to_string()
        );
        assert_eq!(ResourceValue::Absolute(2).to_string(), "2");
    }

    #[test]
    fn test_to_string_percentage() {
        assert_eq!(ResourceValue::Percentage(50).to_string(), "50%");
    }
}
