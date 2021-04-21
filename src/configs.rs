use super::*;
use std::fs::write;

/// Set's <string> to the value in <search> in <yaml>; doesn't change the original string if it
/// isn't found or the yaml isn't Yaml::Hash
pub fn get_yaml(string: &mut String, yaml: &Yaml, search: &str) {
    if let Yaml::Hash(hash) = yaml {
        if let Some(Yaml::String(s)) = hash.get(&Yaml::String(search.to_string())) {
            *string = s.to_string();
        }
    }
}

pub mod fstab;
pub mod hostname;
pub mod users;
