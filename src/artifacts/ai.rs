use crate::artifacts::data_center::DataCenter;
use crate::artifacts::desire_variables::DesireVariables;
use std::any::Any;
use std::collections::HashMap;
use std::io::Write;
use std::{fmt, fs};

use serde::{Deserialize, Serialize};

pub enum DbCrudResponses {
    AddFailed,
    GetFailed,
    DeleteFailed(String),
}

impl DbCrudResponses {
    fn get(&self) -> String {
        match self {
            DbCrudResponses::AddFailed => {
                String::from("Internal error! Should never happen. Failed to add.")
            }
            DbCrudResponses::GetFailed => String::from("No item of provided key exists."),
            DbCrudResponses::DeleteFailed(key) => format!("Item of key {} not exist.", key),
        }
    }
}

struct DbFile<'a> {
    my_db_of_relationships: &'a MyDbOfRelationships<'a>,
}
type DataBaseT<'a> = &'a mut HashMap<String, String>;
const RELATIONAL_DB_NAME: &str = "data center";

struct MyDbOfRelationships<'a> {
    pub data_center: DataBaseT<'a>,
}

impl<'a> MyDbOfRelationships<'a> {
    fn new(data_center: DataBaseT) -> MyDbOfRelationships {
        let my_db_of_relationships = MyDbOfRelationships { data_center };

        my_db_of_relationships
            .data_center
            .insert(RELATIONAL_DB_NAME.to_string(), Default::default());
        my_db_of_relationships
    }

    fn add(&mut self, key: String, item: String) -> bool {
        if let Some(_) = self.data_center.insert(key, item) {
            true
        } else {
            false
        }
    }

    fn get(&mut self, key: &str) -> String {
        if let Some(item) = self.data_center.get(key) {
            String::from(item)
        } else {
            DbCrudResponses::GetFailed.get()
        }
    }

    fn delete(&mut self, key: &str) -> String {
        if let Some(item) = self.data_center.remove(key) {
            item
        } else {
            String::from("Failed.")
        }
    }

    fn commit(&self) -> bool {
        let mut file = fs::File::create("data_center.json").unwrap();
        match serde_json::to_writer(file, self.data_center) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;
    use std::ops::Deref;

    #[test]
    fn database_initialized() {
        // region instantiation
        let db = &mut Default::default();
        MyDbOfRelationships::new(db);
        // endregion instantiation

        if let Some(item) = db.get(RELATIONAL_DB_NAME) {
            dbg!("Successfully instantiated database.");
        } else {
            panic!("Failed");
        }
    }

    #[test]
    fn through_and_through() {
        // region instantiation
        let db = &mut Default::default();
        let mut dbr = MyDbOfRelationships::new(db);
        // endregion instantiation

        let mut data: DataBaseT = &mut Default::default();
        data.insert("test".to_string(), "eh".to_string());
        let key = "myUniqueKey";

        let serialized_data = serde_json::to_string(data).unwrap();
        // let serialized_data: DataBaseT = serde_json::from_str(&serialized_data).unwrap();

        dbr.add(key.parse().unwrap(), String::from(&serialized_data));

        let res = dbr.get(key);

        let serialized_data: HashMap<String, String> =
            serde_json::from_str(&serialized_data).unwrap();
        dbg!(serialized_data);

        if dbr.commit() {
            dbg!("Successfully committed.");
        } else {
            panic!("failed to");
        }

        // if dbr.get(key) {
        //     dbg!("item: ");
        //     dbg!(item);
        // } else {
        //     dbg!("Failed");
        // }
    }
}
