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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub origin: String,
    pub destination: String,
    pub text: String
}
