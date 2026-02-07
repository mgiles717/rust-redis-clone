use std::collections::HashMap;

pub struct Storage {
    // Storage data structure that will hold our data in a K,V map 
    // Key: unique object
    // Value: seralizied object
    data: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let _ = self.data.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        return self.data.get(key); // HashMap::get accepts &str for String keys
    }
}
