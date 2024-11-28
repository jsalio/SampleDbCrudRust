pub mod implements;
pub mod models;
pub mod traits;

use left_pad::{leftpad, leftpad_with};
use models::card_info::CardInfo;
use models::configuration::Configuration;
use models::my_database::MyDatabase;
use traits::global::GobalFn;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let table: MyDatabase<CardInfo> = MyDatabase::new("card-info".to_string());
    let new_record = CardInfo {
        client_name: "John Doe".to_string(),
        configuration: vec![Configuration {
            handle: Some(123),
            id: 1,
            is_required: true,
            is_template: false,
            name: "Sample Configuration".to_string(),
            status: "active".to_string(),
        }],
        creation_date: "2024-11-27".to_string(),
        creator_email: "johndoe@example.com".to_string(),
        creator_name: "John Doe".to_string(),
        creator_phone: "1234567890".to_string(),
        current_document_in_process: 0,
        expiry_date: "2024-12-31".to_string(),
        process_document_requirement: 0,
        process_id: "process-123".to_string(),
        process_name: "Sample Process".to_string(),
        status: "active".to_string(),
    };

    let id = table.add_record(new_record).await;
    if id != Uuid::nil() {
        println!("Registro creado con éxito. ID: {}", id);
    } else {
        eprintln!("Hubo un problema al crear el registro.");
    }

    let result = table.get_record(&id.to_string()).await;

    match result {
        Ok(data) => match data {
            Some(record) => {
                println!("Registro encontrado: {:?}", record);
                println!("nombre del cliente: {}", record.client_name);
                println!("nombre del proceso: {}", record.process_name);
                println!("fecha de creación: {}", record.creation_date);
            }
            None => {
                println!("Registro no encontrado.");
            }
        },
        Err(e) => eprintln!("Error fetching data: {:?}", e),
        
    }

    match &table.list_record().await {
        Ok(data) => {
            println!("id                                      client");
            for (key, value) in data {
                println!(
                    "{}                                 {:?}",
                    key, value.client_name
                );
            }
        }
        Err(e) => eprintln!("Error fetching data: {:?}", e),
    }
}
