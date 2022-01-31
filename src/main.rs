mod data;
mod indexer;
mod db;
mod errors;

use indexer::{Key, Entry, Indexer};
use data::Data;

fn main() {
    let db_path = "/tmp/Only1/Table";
    Indexer::new(db_path);
    let tab: [(Key,Entry);4] = [
        (0.into(), Entry::new(Data::new_from_str("String Z"), Data::new_from_number(4))),
        (2.into(), Entry::new(Data::new_from_str("String B"), Data::new_from_number(9))),
        (10.into(), Entry::new(Data::new_from_str("String C"), Data::new_from_number(75))),
        (5.into(), Entry::new(Data::new_from_str("q"), Data::new_from_number(3)))];
    
    let mut indexer = Indexer::new("/tmp/only1/test2");
    for row in tab {
        indexer.entry(row.0, row.1);
    }

    indexer.sort_by_string_col("/tmp/Only1/SortStringTable");
    indexer.sort_by_num_col("/tmp/Only1/SortNumberTable");
}
