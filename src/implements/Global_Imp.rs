use firebase_rs::{Firebase, RequestError};
use serde::{de::DeserializeOwned, Serialize};
use std::fs::File;
use std::io::Read;
use std::{collections::HashMap, future::Future, pin::Pin};
use uuid::Uuid;

use crate::{
    models::{firebase_config::FirebaseConfig, my_database::MyDatabase},
    traits::global::GobalFn,
};

impl<T: Clone + Default + Serialize + DeserializeOwned + std::fmt::Debug + 'static> GobalFn<T>
    for MyDatabase<T>
where
    T: Clone,
{
    fn add_record(&self, record: T) -> Pin<Box<dyn Future<Output = Uuid>>> {
        let mut firebase = self.build_connection().at(&self.path);

        Box::pin(async move {
            let id = Uuid::new_v4();
            match firebase.set_with_key(&id.to_string(), &record).await {
                Ok(_) => id,
                Err(e) => {
                    eprintln!("Error al guardar en Firebase: {:?}", e);
                    Uuid::nil()
                }
            }
        })
    }

    fn list_record(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>>>> {
        let firebase = self.build_connection().at(&self.path);
        Box::pin(async move { firebase.get::<HashMap<String, T>>().await })
    }

    fn get_record(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<T>, RequestError>> + '_>> {
        let firebase = self.build_connection();
        let path = format!("{}/{}", self.path, key);

        Box::pin(async move {
            let result = firebase.at(&path).get::<T>().await;
            match result {
                Ok(data) => Ok(Some(data)),
                Err(RequestError::NotFoundOrNullBody) => Ok(None),
                Err(err) => Err(err),
            }
        })
    }
    fn update_record(&self, id: Uuid, record: T) -> T {
        T::default()
    }
}

impl<T> MyDatabase<T> {
    fn build_connection(&self) -> Firebase {
        let firebase_config = MyDatabase::<T>::load_firebase_config(&self.firebase_config_file);
        let firebase =
            Firebase::new(&firebase_config.database_url).expect("Failed to initialize Firebase");
        println!("Firebase initialized {}", firebase_config.database_url);
        firebase
    }

    fn load_firebase_config(file_path: &str) -> FirebaseConfig {
        let mut file = File::open(file_path).expect("Failed to open configuration file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read configuration file");

        serde_json::from_str(&contents).expect("Failed to parse JSON configuration")
    }

    pub fn new(path: String, config: String) -> Self {
        MyDatabase {
            data: HashMap::new(),
            path,
            firebase_config_file: config,
        }
    }
}

impl<T> Default for MyDatabase<T> {
    fn default() -> Self {
        MyDatabase {
            data: HashMap::new(),
            path: "".to_string(),
            firebase_config_file: "".to_string(),
        }
    }
}
