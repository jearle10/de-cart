use std::collections::HashMap;
/*
 All data stores should implement this trait to allow
 consistent CRUD access to required parts of state
*/
pub trait Store<T> {
    fn get(&self, item_id: String) -> Option<&T>;
    fn add(&mut self, item: T) -> Option<T>;
    fn get_all(&self) -> Vec<T>;
    fn update(&mut self, item: T) -> Option<T>;
    fn delete(&mut self, item_id: String) -> Option<T>;
}
