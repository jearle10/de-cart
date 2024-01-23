use crate::state::Store;
use candid::{CandidType, Deserialize};
use std::collections::HashMap;

pub(crate) type MerchantId = String;

#[derive(Clone, Debug, Default, Deserialize, CandidType)]
pub struct Merchant {
    pub(crate) id: MerchantId, // This should be the Principle id of the merchant,
    pub(crate) name: String,
    pub(crate) email: String,
}

impl Merchant {
    fn total_bytes(&self) -> usize {
        let mut bytes = 0;
        bytes += self.id.len();
        bytes += self.name.len();
        bytes += self.email.len();
        bytes
    }
}

#[derive(Deserialize, Debug, CandidType, Default, Clone)]
pub struct MerchantStore {
    pub merchants: HashMap<MerchantId, Merchant>,
}

impl MerchantStore {
    pub fn new() -> Self {
        Self {
            merchants: HashMap::new(),
        }
    }
}

impl Store<Merchant> for MerchantStore {
    fn get(&self, merchant_id: MerchantId) -> Option<&Merchant> {
        self.merchants.get(&merchant_id)
    }

    fn add(&mut self, merchant: Merchant) -> Option<Merchant> {
        self.merchants
            .insert(merchant.id.to_string(), merchant.clone());
        Some(merchant)
    }

    fn get_all(&self) -> Vec<Merchant> {
        self.merchants.values().cloned().collect()
    }

    fn update(&mut self, merchant: Merchant) -> Option<Merchant> {
        self.merchants.insert(merchant.id.to_string(), merchant)
    }

    fn delete(&mut self, merchant_id: MerchantId) -> Option<Merchant> {
        self.merchants.remove(merchant_id.as_str())
    }
}

impl MerchantStore {
    pub(crate) fn total_bytes(&self) -> usize {
        let mut total_bytes = 0;
        self.merchants
            .iter()
            .for_each(|(id, merchant)| total_bytes += id.len() + merchant.total_bytes());
        total_bytes
    }

    pub(crate) fn total_merchants(&self) -> String {
        self.merchants.iter().len().to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_adds_a_merchant() {
        let mut store = MerchantStore::new();

        let new_merchant = Merchant {
            id: "2vxsx-fae".to_string(),
            ..Merchant::default()
        };

        ic_cdk::println!("{:?}", new_merchant);
        store.add(new_merchant.clone());

        let merchants: Vec<&String> = store.merchants.keys().collect();
        let length = merchants.len();

        assert_eq!(length, 1);
    }

    #[test]
    fn it_gets_a_merchant() {
        let mut store = MerchantStore::new();

        let new_merchant = Merchant {
            id: "2vxsx-fae".to_string(),
            ..Merchant::default()
        };

        store.add(new_merchant.clone());
        let merchant = store.get("2vxsx-fae".to_string());
        assert_eq!(merchant.unwrap().id, "2vxsx-fae".to_string());
    }

    #[test]
    fn it_gets_all_merchants() {
        let mut store = MerchantStore::new();

        let new_merchant1 = Merchant {
            id: "one".to_string(),
            ..Merchant::default()
        };

        let new_merchant2 = Merchant {
            id: "two".to_string(),
            ..Merchant::default()
        };

        let new_merchant3 = Merchant {
            id: "three".to_string(),
            ..Merchant::default()
        };

        store.add(new_merchant1.clone());
        store.add(new_merchant2.clone());
        store.add(new_merchant3.clone());

        let merchants = store.get_all();
        assert_eq!(merchants.len(), 3);
    }

    #[test]
    fn it_updates_a_merchant() {
        let mut store = MerchantStore::new();

        let new_merchant = Merchant {
            id: "2vxsx-fae".to_string(),
            ..Merchant::default()
        };

        ic_cdk::println!("{:?}", new_merchant);
        store.add(new_merchant.clone());

        let updated_merchant = Merchant {
            id: "2vxsx-fae".to_string(),
            email: "jian@email.com".to_string(),
            ..Merchant::default()
        };

        store.update(updated_merchant.clone());

        assert_eq!(
            store.merchants.get("2vxsx-fae").unwrap().email,
            "jian@email.com".to_string()
        );
    }

    #[test]
    fn it_deletes_a_merchant() {
        let mut store = MerchantStore::new();

        let new_merchant = Merchant {
            id: "2vxsx-fae".to_string(),
            ..Merchant::default()
        };

        ic_cdk::println!("{:?}", new_merchant);
        store.add(new_merchant.clone());
        store.delete("2vxsx-fae".to_string());

        let merchants: Vec<&String> = store.merchants.keys().collect();
        let length = merchants.len();

        assert_eq!(length, 0);
    }

    #[test]
    fn it_calculates_bytes_of_merchant() {
        let merchant = Merchant {
            id: "1".to_string(),
            name: "1".to_string(),
            email: "1".to_string(),
        };

        let merchant_storage = merchant.total_bytes();
        assert_eq!(merchant_storage, 3)
    }

    #[test]
    fn it_calculates_bytes_of_merchant_store() {
        let merchant1 = Merchant {
            id: "1".to_string(),
            name: "1".to_string(),
            email: "1".to_string(),
        };

        let merchant2 = Merchant {
            id: "2".to_string(),
            name: "2".to_string(),
            email: "2".to_string(),
        };

        let mut store = MerchantStore::default();

        store.add(merchant1);
        store.add(merchant2);

        let merchant_store_storage = store.total_bytes();
        assert_eq!(merchant_store_storage, 8)
    }
}
