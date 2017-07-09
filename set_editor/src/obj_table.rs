use std::collections::HashMap;
use std::path::Path;
use std::fs::File;

use serde_json;

#[derive(Clone,Debug,Deserialize)]
pub struct ObjectTable(HashMap<u16, HashMap<u16, String>>);

impl ObjectTable {
    pub fn from_file(path: &Path) -> Result<ObjectTable, String> {
        let json_file = File::open(path).map_err(|e| e.to_string())?;
        serde_json::from_reader::<_, ObjectTable>(json_file).map_err(|e| e.to_string())
    }

    pub fn lookup(&self, level: u16, object: u16) -> Option<&String> {
        self.0.get(&level).and_then(|h| h.get(&object))
    }
}

