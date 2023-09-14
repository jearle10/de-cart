use std::collections::HashMap;
use std::io::Error;

/*
 All data stores should implement this trait to allow
 consistent CRUD access to required parts of state
 */
pub trait Store <T> {
    fn get(&self, id : String) -> Option<T>;
    fn add(&mut self, item : T);
    fn get_all(&self) -> Result<HashMap<String,T>, Error >;
    fn update(&self, id: String , item : Error);
    fn delete(&self , id : String);
}