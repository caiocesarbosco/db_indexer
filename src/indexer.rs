use std::collections::BTreeMap;
use crate::data::Data;
use std::cmp::Ordering;

#[derive(Ord, Eq, Hash, Clone)]
pub struct Key {
    key: Vec<u8>
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

#[derive(Clone)]
pub struct Entry<'a> {
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
    index: BTreeMap<Key, Entry<'a>>,
    str_col_indexer: BTreeMap<Data<'a>, Key>,
    num_col_indexer: BTreeMap<Data<'a>, Key>
}

impl <'a>Indexer<'a> {

    pub fn new() -> Indexer<'a> {
        Indexer {
            index: BTreeMap::new(),
            str_col_indexer: BTreeMap::new(),
            num_col_indexer: BTreeMap::new()
        }
    }

    pub fn entry(&mut self, key: Key, value: Entry<'a>) {
        self.index.insert(key.clone(), value.clone());
        self.str_col_indexer.insert(value.string_col, key.clone());
        self.num_col_indexer.insert(value.num_col, key);
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_inserting_data() {
        let tab: [(Key,Entry);4] = [(0.into(), Entry::new(Data::new_from_str("String A"), Data::new_from_number(4))), (2.into(), Entry::new(Data::new_from_str("String B"), Data::new_from_number(9))), (10.into(), Entry::new(Data::new_from_str("String C"), Data::new_from_number(75))), (0.into(), Entry::new(Data::new_from_str("q"), Data::new_from_number(3)))];
        let mut indexer = Indexer::new();
        for row in tab {
            indexer.entry(row.0, row.1);
        }
        assert_eq!(indexer.index.len(), 3);
    }

}

