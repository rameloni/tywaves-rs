use super::spec::Hgldd;
use std::path::Path;

pub const HGLDD_EXTENSION: &str = "dd";

/// Remove eventual comments from the HGLDD content since they are not supported by json deserialization.
#[inline]
pub fn drop_comments(hgldd_str: &str) -> String {
    hgldd_str
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<&str>>()
        .join("\n")
}

/// Parse an HGLDD string with multiple HGLDDs
#[inline]
fn parse_hgldds(hgldd_str: &str) -> Vec<Hgldd> {
    // Skip the comment line (if any)
    let hgldd_str = drop_comments(hgldd_str);
    let deserializer = serde_json::Deserializer::from_reader(hgldd_str.as_bytes());
    let iterator = deserializer.into_iter::<serde_json::Value>();
    iterator
        .map(|x| serde_json::from_value(x.unwrap()).unwrap())
        .collect()
}

/// Parse an HGLDD file
#[inline]
pub fn parse_hgldd_file(hgldd_path: &Path) -> Vec<Hgldd> {
    let hgldd_str = std::fs::read_to_string(hgldd_path).unwrap();
    parse_hgldds(&hgldd_str)
}

#[inline]
/// Parse a directory containing HGLDD files
pub fn parse_hgldd_dir(hgldd_dir_path: &Path) -> Vec<Hgldd> {
    // Read the directory and parse all the files
    let files = std::fs::read_dir(hgldd_dir_path).unwrap();
    let mut hgldds = Vec::new();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        // Check if the file is an HGLDD file
        if path.is_file() {
            if path.extension().unwrap() == HGLDD_EXTENSION {
                hgldds.append(&mut parse_hgldd_file(&path));
            }
        }
    }
    hgldds
}
