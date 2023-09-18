use std::collections::HashMap;
/*
 All data stores should implement this trait to allow
 consistent CRUD access to required parts of state
 */
pub trait Store <T, L> {
    fn get(&self, merchant_id : String , id : String) -> Option<T>;
    fn add(&mut self, item : T) -> Option<T>;
    fn get_all(&self, merchant_id : String) -> Option<L>;
    fn update(&mut self, item : T) -> Option<T>;
    fn delete(&mut self , merchant_id : String , id : String) -> Option<T>;
}