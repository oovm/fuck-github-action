use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use wax::{Glob};
use anyhow::Result;

#[derive(Copy, Clone, Debug, Default)]
pub struct ActionWalker {}

impl ActionWalker {
    pub fn walk(&self) {
        if let Err(e) = self.try_walk() {
            eprintln!("{e:?}")
        }
    }
    fn try_walk(&self) -> Result<()> {
        let glob = Glob::new("**/*.{json,json5,toml}")?;
        for entry in glob.walk(".github/") {
            self.convert(entry?.path())?
        }
        Ok(())
    }
    fn convert(&self, entry: &Path) -> Result<()> {
        if let Some(s) = entry.file_name() {
            println!("{s:?} is fucking github action", )
        }
        match entry.extension().and_then(|s| s.to_str()) {
            Some("json") | Some("json5") => {
                let yaml: serde_yaml::Value = json5::from_str(&read_to_string(entry)?)?;
                let mut file = File::create(entry.with_extension("yml"))?;
                file.write_all(serde_yaml::to_string(&yaml)?.as_bytes())?
            }
            Some("toml") => {
                let yaml: serde_yaml::Value = toml::from_str(&read_to_string(entry)?)?;
                let mut file = File::create(entry.with_extension("yml"))?;
                file.write_all(serde_yaml::to_string(&yaml)?.as_bytes())?
            }
            _ => {}
        }
        Ok(())
    }
}
