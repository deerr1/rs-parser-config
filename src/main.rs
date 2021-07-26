use std::env;
use std::fs;
use std::str::Lines;
use std::collections::HashMap;

pub struct VaultConfiguration {
    pub map: HashMap<String, String>,
}

impl VaultConfiguration {
    ///Creates a new structure containing key-value pairs from a file.
    ///The name of the virtual environment parameter, which contains the path to the file, is taken as an input parameter.
    ///
    ///# Example
    ///
    /// ```
    /// let VAULT_CONF: VaultConfiguration = VaultConfiguration::new("PATH_TO_FILE");
    /// ```
    pub fn new(name_key_path: &str) -> Self{
        let map = match env::var(&name_key_path) {
            Ok(path) => get_map(&path),
            Err(_) => HashMap::new()
        };
        Self{map}
    }

    /// The value is increased by the key stored in the structure, otherwise the value stored in the virtual environment by the key.
    ///
    ///# Examlpe
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use std::env;
    ///
    /// let mut config = VaultConfiguration{map: HashMap::new()}
    ///
    /// config.map.insert("db_port".to_string(), "8000".to_string())
    ///
    /// assert_eq!(config.get_val("db_port"), "8000".to_string())
    /// ```
    pub fn get_val(&self, name: &str) -> String{
        match self.map.get(name) {
            Some(value) => value.to_string(),
            None => env::var(name).unwrap().to_string()
        }
    }
}

fn get_map(path: &str) -> HashMap<String, String>{
    parse_text_to_map(fs::read_to_string(path).unwrap().lines())
}

fn parse_text_to_map(array_text: Lines) -> HashMap<String, String> {
    array_text.map(|l| l.split(": ").collect::<Vec<&str>>()).map(|x| (x[0].to_string(), x[1].to_string())).collect()
}