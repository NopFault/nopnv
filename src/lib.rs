use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

pub struct Env {
    pub vars: HashMap<String, String>,
    pub arrays: HashMap<String, Vec<String>>
}

impl Env {
    pub fn new() -> Self {
        Env {
            vars: HashMap::new(),
            arrays: HashMap::new()
        }
    }

    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut env = Env::new();
        if let Ok(lines) = Self::read_lines(filename) {
            for line in lines {
                if let Ok(line) = line {
                    let trimmed = line.trim();
                    if !trimmed.starts_with('#') && !trimmed.is_empty() {
                        let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                         if parts.len() == 2 {
                            let key = String::from(parts[0]);
                            let value = env.expand_value(parts[1]);
                            if value.contains(',') {
                                env.arrays.insert(key, value.split(',').map(|s| String::from(s)).collect());
                            } else {
                                env.vars.insert(key, value);
                            }

                        }
                    }
                }
            }
        }
        Ok(env)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.vars.get(key)
    }

    pub fn get_array(&self, key: &str) -> Option<&Vec<String>> {
        self.arrays.get(key)
    }

    pub fn expand_value(&self, value: &str) -> String {
        let re = Regex::new(r"\$\{?([A-Z_][A-Z0-9_]*)\}?").unwrap();
        let mut expanded = String::from(value);
        for cap in re.captures_iter(value) {
            if let Some(var_name) = cap.get(1) {
                if let Some(var_value) = self.vars.get(var_name.as_str()) {
                    expanded = expanded.replace(&cap[0], var_value);
                }
            }
        }
        expanded
    }

    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where 
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
}


