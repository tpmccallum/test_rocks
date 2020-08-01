// Dependencies
use byte_array::ByteArray;

// Bring into scope
use rocksdb::DB;

// Set config
let db_path = "/media/nvme/ssvm_database";

// Open DB instance
let db = DB::open_default(db_path).unwrap();

// Create key and value
let mut key = ByteArray::new();
let mut value = ByteArray::new();
key.write((1234 as u8));
value.write((5678 as u8));


// Store
db.put(key, value).unwrap();
println("Value of : {:?}, Stored to key: {:?}", value, key);

//Load
let loaded_data = db.get(&key).unwrap().unwrap();
println("Value of : {:?}, Loaded from key: {:?}", value, key);