# db_indexer
A Simple Key Value Database Indexer Sample

Rocksdb has been chosen as our Low Level Database.

Database insert each entry following an algorithm which sorts Key's lexically.

Value is a Struct containing a String and a Number.

Indexer must indexing DB by String or Number Field.

Indexer must sort/filter DB by String or Number Field.

The Indexer & DB resources are used by many threads.

Fuzzing Test must cover as many different entries as possible.


Run the Tests: 

cargo test
