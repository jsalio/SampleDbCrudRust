use std::collections::HashMap;

use uuid::Uuid;

 pub struct MyDatabase<T> {
    pub data: HashMap<Uuid, T>,
    pub path: String,
    pub firebase_config_file: String,
}