// FREECHAIN STRUCTS

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        return Self{username, password}
    }
}

pub struct Database {
    pub data: HashMap<String, User>
}

impl Database {
    pub fn new() -> Self {
        return Self{data: HashMap::new()}
    }
    pub fn pull(&self, username: &str) -> Option<User>{
        return self.data.get(username).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub origin: String,
    pub destination: String,
    pub text: String
}
