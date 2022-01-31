use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::fmt::{Display, Result, Formatter};
use serde::{Serialize, Deserialize};

use crate::data::Data;
use crate::db::{Database, KVStore};

#[derive(Ord, Eq, Hash, Clone, Debug)]
pub struct Key {
    key: Vec<u8>
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.key)
    }
}

impl From<i64> for Key {
    fn from(key:i64) -> Key {
        Key {
            key: key.to_be_bytes().to_vec()
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        &self.key[..]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Entry<'a> {
    #[serde(borrow)]
    string_col: Data<'a>,
    num_col: Data<'a>
}

impl <'a>Entry<'a> {
    pub fn new(str_col: Data<'a>, number_col: Data<'a>) -> Entry<'a> {
        Entry {
            string_col: str_col,
            num_col: number_col
        }
    }
}


pub struct Indexer<'a> {
    db: Database,
    str_col_indexer: BTreeMap<Data<'a>, Key>,
    num_col_indexer: BTreeMap<Data<'a>, Key>
}

impl <'a>Indexer<'a> {

    pub fn new(db_path: &str) -> Indexer<'a> {
        Indexer {
            db: KVStore::initialize(db_path),
            str_col_indexer: BTreeMap::new(),
            num_col_indexer: BTreeMap::new()
        }
    }

    pub fn entry(&mut self, key: Key, value: Entry<'a>) {
        self.db.save(key.clone(), value.clone());
        self.str_col_indexer.insert(value.string_col, key.clone());
        self.num_col_indexer.insert(value.num_col, key);
    }

    pub fn index_by_string_col(&self) -> BTreeMap<Data<'a>, Key> {
        self.str_col_indexer.clone()
    }

    pub fn index_by_num_col(&self) -> BTreeMap<Data<'a>, Key> {
        self.num_col_indexer.clone()
    }

    pub fn sort_by_string_col(&self, path: &str) -> Database {
        let sort_db: Database = KVStore::initialize(path);
        for (value, key) in &self.str_col_indexer {
            sort_db.save_col(key.clone(), value.clone());
        }
        sort_db
    }

    pub fn sort_by_num_col(&self, path: &str) -> Database {
        
        let sort_db: Database = KVStore::initialize(path);
        for (value, key) in &self.num_col_indexer {
            sort_db.save_col(key.clone(), value.clone());
        }
        sort_db
    }


}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_indexing_by_string_col() {
        let tab: [(Key,Entry);4] = [
            (0.into(), Entry::new(Data::new_from_str("String Z"), Data::new_from_number(4))),
            (2.into(), Entry::new(Data::new_from_str("String B"), Data::new_from_number(9))),
            (10.into(), Entry::new(Data::new_from_str("String C"), Data::new_from_number(75))),
            (5.into(), Entry::new(Data::new_from_str("q"), Data::new_from_number(3)))];
        
        let mut indexer = Indexer::new("/tmp/only1/test1");
        for row in tab {
            indexer.entry(row.0, row.1);
        }
        
        let mut start_iter = indexer.str_col_indexer.iter();

        assert_eq!(start_iter.next(), Some((&(Data::new_from_str("String B")), &(2.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_str("String C")), &(10.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_str("String Z")), &(0.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_str("q")), &(5.into()))));
    }

    #[test]
    fn test_indexing_by_number_col() {
        let tab: [(Key,Entry);4] = [
            (0.into(), Entry::new(Data::new_from_str("String Z"), Data::new_from_number(4))),
            (2.into(), Entry::new(Data::new_from_str("String B"), Data::new_from_number(9))),
            (10.into(), Entry::new(Data::new_from_str("String C"), Data::new_from_number(75))),
            (5.into(), Entry::new(Data::new_from_str("q"), Data::new_from_number(3)))];
        
        let mut indexer = Indexer::new("/tmp/only1/test2");
        for row in tab {
            indexer.entry(row.0, row.1);
        }
        
        let mut start_iter = indexer.num_col_indexer.iter();

        assert_eq!(start_iter.next(), Some((&(Data::new_from_number(3)), &(5.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_number(4)), &(0.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_number(9)), &(2.into()))));
        assert_eq!(start_iter.next(), Some((&(Data::new_from_number(75)), &(10.into()))));
    }

}

