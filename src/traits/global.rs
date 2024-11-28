use std::{collections::HashMap, future::Future, pin::Pin};

use firebase_rs::RequestError;
use uuid::Uuid;

pub trait GobalFn<T> {
    fn add_record(&self, record: T) -> Pin<Box<dyn Future<Output = Uuid>>>;
    fn list_record(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<HashMap<String, T>, RequestError>>>>;
    fn get_record(
        &self,
        key: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<T>, RequestError>> + '_>>;
    fn update_record(&self, id: Uuid, record: T) -> T;
}
