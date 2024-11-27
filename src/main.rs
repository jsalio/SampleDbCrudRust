use std::{collections::HashMap, future::Future, path, pin::Pin, str};
use firebase_rs::{Firebase, RequestError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;
use left_pad::{leftpad, leftpad_with};


trait GobalFn<T> {
    fn add_record(&self, record: T) -> Uuid;
    fn list_record(&self) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>>>>;
    fn get_record(&self, id: Uuid) -> T;
    fn update_record(&self, id: Uuid, record: T) -> T;
}

struct MyDatabase<T> {
    data: HashMap<Uuid, T>,
    path: String,
}


impl<T: Clone + Default + Serialize + DeserializeOwned + std::fmt::Debug> GobalFn<T> for MyDatabase<T> where T:Clone {
    fn add_record(&self, record: T) -> Uuid {
        let id = Uuid::new_v4();

        // Aquí modificarías self.data si tuviera mutabilidad
        id
    }

    fn list_record(&self) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>>>> {
        let firebase = self.build_connection();
        Box::pin(async move { firebase.get::<HashMap<String, T>>().await })
    }

    fn get_record(&self, id: Uuid) -> T {
        T::default()
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
            database_url: "https://fir-crm-4a06a-default-rtdb.firebaseio.com".to_string(),
            project_id: "fir-crm-4a06a".to_string(),
            storage_bucket: "fir-crm-4a06a.firebasestorage.app".to_string(),
            messaging_sender_id: "456718419550".to_string(),
            app_id: "1:456718419550:web:5d2a0f56fa082edea156cf".to_string(),
        };
        let firebase = Firebase::new(&firebase_config.database_url)
        .expect("Failed to initialize Firebase");
        firebase
    }

    fn new(path:String) -> Self {
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


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CardInfo {
    pub client_name: String,
    pub configuration: Vec<Configuration>,
    pub creation_date: String,
    pub creator_email: String,
    pub creator_name: String,
    pub creator_phone: String,
    pub current_document_in_process: i64,
    pub expiry_date: String,
    pub process_document_requirement: i64,
    pub process_id: String,
    pub process_name: String,
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub handle: Option<i64>,
    pub id: i64,
    pub is_required: bool,
    pub is_template: bool,
    pub name: String,
    pub status: String,
}
#[derive(Debug)]
struct FirebaseConfig {
    api_key: String,
    auth_domain: String,
    database_url: String,
    project_id: String,
    storage_bucket: String,
    messaging_sender_id: String,
    app_id: String,
}


async fn add_dumy_record (db:&Firebase) {
    let id = Uuid::new_v4().to_string();
    let card_info = CardInfo {
        client_name: "Test".to_string(),
        configuration: vec![Configuration {
            handle: Some(1),
            id: 1,
            is_required: true,
            is_template: true,
            name: "Test".to_string(),
            status: "Test".to_string(),
        }],
        creation_date: "2021-09-01".to_string(),
        creator_email: "".to_string(),
        creator_name: "".to_string(),
        creator_phone: "".to_string(),
        current_document_in_process: 1,
        expiry_date: "2021-09-01".to_string(),
        process_document_requirement: 1,
        process_id: "1".to_string(),
        process_name: "Test".to_string(),
        status: "Test".to_string(),
    };
    let mut binding = db.at("card-info");
    let result = binding.set_with_key(&id,&card_info);
    match result.await {
        Ok(_) => println!("Record added successfully"),
        Err(e) => eprintln!("Error adding record: {}", e),
    }
}

async fn list_data_set (db:&Firebase) {
    match db.at("card-info").get::<HashMap<String, CardInfo>>().await {
        Ok(data) => {
            println!("id                                      client");
            for (key, value) in data {
                println!("{}                                 {:?}",  key, value.client_name);
            }
        },
        Err(e) => eprintln!("Error fetching data: {}", e),
    }
}
#[tokio::main]
async fn main() {
    // let firebase_config = FirebaseConfig {
    //     api_key: "AIzaSyDuFNWfJFkeB5odXMvNIbE9xTzDd-2XKZ0".to_string(),
    //     auth_domain: "fir-crm-4a06a.firebaseapp.com".to_string(),
    //     database_url: "https://fir-crm-4a06a-default-rtdb.firebaseio.com".to_string(),
    //     project_id: "fir-crm-4a06a".to_string(),
    //     storage_bucket: "fir-crm-4a06a.firebasestorage.app".to_string(),
    //     messaging_sender_id: "456718419550".to_string(),
    //     app_id: "1:456718419550:web:5d2a0f56fa082edea156cf".to_string(),
    // };

    let table: MyDatabase<CardInfo> = MyDatabase::new("card-info".to_string());

    match table.list_record().await {
        Ok(data) => {
            println!("id                                      client");
            for (key, value) in data {
                println!("{}                                 {:?}", key, value.client_name);
            }
        }
        Err(e) => eprintln!("Error fetching data: {:?}", e),
    }

    // // Initialize Firebase
    // let firebase = Firebase::new(&firebase_config.database_url)
    //     .expect("Failed to initialize Firebase");

    // println!("Firebase initialized with URL: {}", firebase_config.database_url);

    // match firebase.at("card-info").get::<HashMap<String, CardInfo>>().await {
    //     Ok(data) => {
    //         println!("Data: {:?}", data.len());
    //         for (key, value) in data {
    //             println!("Key: {}, Value: {:?}", key, value);
    //         }
    //     },
    //     Err(e) => eprintln!("Error fetching data: {}", e),
    // }
    //  add_dumy_record(&firebase).await;
    //  list_data_set(&firebase).await;
    // println!("Hello, world!");
}
