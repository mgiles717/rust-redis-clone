mod server;
mod storage;

fn main() {
    // Need to handle Result<()> for this 
    // let _ = server::init_server();
    //
    let mut unit = storage::Storage::new();
    let key = "Hello";
    let value = "thisexamplestring";

    unit.set(key, value);
    let result = unit.get(key);
}
