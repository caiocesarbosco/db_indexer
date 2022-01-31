use rocksdb::DB;
use serde_json;

use crate::indexer::{Key, Entry};
use crate::errors::Error;
use crate::data::Data;

pub trait KVStore {
    fn initialize(file_path: &str) -> Self;
    fn save(&self, k: Key, v: Entry) -> bool;
    fn save_col(&self, k: Key, v: Data) -> bool;
    fn find(&self, k: Key)-> Result<Option<String>, Error>;
}

pub struct Database {
    db: DB,
}

impl KVStore for Database {
    fn initialize(file_path: &str) -> Self {
        Database {
            db: DB::open_default(file_path).unwrap()
        }
    }

    fn save(&self, k: Key, v: Entry) -> bool {
        self.db.put(k, serde_json::to_string(&v).unwrap().as_bytes()).is_ok()
    }
    
    fn save_col(&self, k: Key, v: Data) -> bool {
        self.db.put(k, serde_json::to_string(&v).unwrap().as_bytes()).is_ok()
    }

    fn find(&self, k: Key) -> Result<Option<String>, Error> {
        match self.db.get(k) {
            Ok(Some(v)) => Ok(Some(String::from_utf8(v).unwrap())),
            Ok(None) => Ok(None),
            Err(_e) => Err(Error::InternalError) 
        }
    }
}
