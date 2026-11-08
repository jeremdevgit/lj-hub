use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub dpi: u32,
    pub report_rate_ms: u32,
    pub color: String,
    pub notes: String,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            dpi: 800,
            report_rate_ms: 1,
            color: "00AEEF".into(),
            notes: String::new(),
        }
    }
}

impl Profile {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() || self.name.len() > 80 {
            return Err("Profile name must have 1–80 characters".into());
        }
        self.validate_dpi()?;
        self.validate_rate()?;
        self.validate_color()?;
        if self.notes.len() > 4096 {
            return Err("Notes are too long".into());
        }
        Ok(())
    }

    pub fn validate_dpi(&self) -> Result<(), String> {
        if !(100..=32000).contains(&self.dpi) {
            return Err("DPI must be between 100 and 32000".into());
        }
        Ok(())
    }
    pub fn validate_rate(&self) -> Result<(), String> {
        if ![1, 2, 4, 8].contains(&self.report_rate_ms) {
            return Err("Report interval must be 1, 2, 4 or 8 ms".into());
        }
        Ok(())
    }
    pub fn validate_color(&self) -> Result<(), String> {
        if self.color.len() != 6 || !self.color.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("Color must contain six hexadecimal digits".into());
        }
        Ok(())
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Saved {
    pub version: u32,
    pub language: String,
    #[serde(default)]
    pub rgb_control_enabled: bool,
    pub profiles: Vec<Profile>,
}

impl Default for Saved {
    fn default() -> Self {
        Self {
            version: 1,
            language: "fr".into(),
            rgb_control_enabled: false,
            profiles: vec![Profile::default()],
        }
    }
}

impl Saved {
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("Unsupported profile file version".into());
        }
        if self.profiles.is_empty() || self.profiles.len() > 100 {
            return Err("Profile count must be 1–100".into());
        }
        for profile in &self.profiles {
            profile.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalid_profiles_rejected() {
        let mut p = Profile {
            color: "hello; rm".into(),
            ..Default::default()
        };
        assert!(p.validate().is_err());
        p = Profile::default();
        p.dpi = 0;
        assert!(p.validate().is_err());
        p = Profile::default();
        p.report_rate_ms = 3;
        assert!(p.validate().is_err());
    }

    #[test]
    fn old_settings_keep_rgb_control_disabled() {
        let json = r#"{"version":1,"language":"fr","profiles":[{"name":"Default","dpi":800,"report_rate_ms":1,"color":"00AEEF","notes":""}]}"#;
        let saved: Saved = serde_json::from_str(json).unwrap();
        assert!(!saved.rgb_control_enabled);
        assert!(saved.validate().is_ok());
    }
}
