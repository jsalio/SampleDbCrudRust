use std::{collections::HashMap, future::Future, pin::Pin};

use firebase_rs::{Firebase, RequestError};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::{models::{firebase_config::FirebaseConfig, my_database::MyDatabase}, traits::global::GobalFn};

impl<T: Clone + Default + Serialize + DeserializeOwned + std::fmt::Debug + 'static> GobalFn<T> for MyDatabase<T> where T:Clone {

    fn add_record(&self, record: T) -> Pin<Box<dyn Future<Output = Uuid>>> {
        let mut firebase = self.build_connection().at(&self.path);

        Box::pin(async move {
            let id = Uuid::new_v4();
            match firebase.set_with_key(&id.to_string(), &record).await {
                Ok(_) => {
                    id
                }
                Err(e) => {
                    eprintln!("Error al guardar en Firebase: {:?}", e);
                    Uuid::nil()
                }
            }
        })
    }
    
    fn list_record(&self) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>>>> {
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
        let firebase_config = FirebaseConfig {
            api_key: "AIzaSyDuFNWfJFkeB5odXMvNIbE9xTzDd-2XKZ0".to_string(),
            auth_domain: "fir-crm-4a06a.firebaseapp.com".to_string(),
            database_url: "uri".to_string(),
            project_id: "fir-crm-4a06a".to_string(),
            storage_bucket: "fir-crm-4a06a.firebasestorage.app".to_string(),
            messaging_sender_id: "456718419550".to_string(),
            app_id: "1:456718419550:web:5d2a0f56fa082edea156cf".to_string(),
        };
        let firebase = Firebase::new(&firebase_config.database_url)
        .expect("Failed to initialize Firebase");
    println!("Firebase initialized {}", firebase_config.database_url);
        firebase
    }

   pub fn new(path:String) -> Self {
        MyDatabase {
            data: HashMap::new(),
            path
        }
        
    }
}

impl<T> Default for MyDatabase<T>   {
    fn default() -> Self {
        MyDatabase {
            data: HashMap::new(),
            path: "".to_string()
        }
    }
}
