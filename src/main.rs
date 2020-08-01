// Bring into scope
use rocksdb::DB;


fn main() {
// Set config
let db_path = "/media/nvme/ssvm_database";

// Open DB instance
let db = DB::open_default(db_path).unwrap();


// Store
db.put(&b"1234"[..], &b"5678"[..]).unwrap();
println!("Value stored ... ");

//Load
let loaded_data = db.get(&b"1234"[..]).unwrap().unwrap();
println!("Value of : {:?}, was successfully loaded.", loaded_data);
}