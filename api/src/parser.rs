use std::error::Error;
use std::fs;
use std::path::Path;

use crate::packages::PackageInfo;

struct Requirement {
    key: InfoKey,
    value: String,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum InfoKey {
    PRGNAM,
    VERSION,
    REQUIRES,
    Other(String),
}

impl InfoKey {
    pub fn from_part(key: &str) -> Self {
        match key {
            "PRGNAM" => InfoKey::PRGNAM,
            "VERSION" => InfoKey::VERSION,
            "REQUIRES" => InfoKey::REQUIRES,
            _ => InfoKey::Other(key.to_string()),
        }
    }
}

pub fn parse_file(file: &Path) -> Result<PackageInfo, Box<dyn Error>> {
    let mut package_info = PackageInfo::new();
    let contents = fs::read_to_string(file)?;

    for line in contents.lines() {
        if let Some(requirement) = requirement_from_line(line) {
            match requirement.key {
                InfoKey::PRGNAM => package_info.name = Some(requirement.value),
                InfoKey::VERSION => package_info.version = Some(requirement.value),
                InfoKey::REQUIRES => {
                    let requires: Vec<String> = requirement
                        .value
                        .split_whitespace()
                        .map(String::from)
                        .collect();
                    package_info.requires = requires;
                }
                _ => {}
            }
        }
    }
    Ok(package_info)
}

fn requirement_from_line(line: &str) -> Option<Requirement> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if let [k, v] = parts.as_slice() {
        let info_k = InfoKey::from_part(k.trim());
        let value = trim_value(v);
        let requirement = Requirement { key: info_k, value };
        return Some(requirement);
    }
    None
}

fn trim_value(v: &str) -> String {
    return v.trim().replace('\"', "");
}
