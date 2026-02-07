mod server;
mod storage;

use storage::Storage;

fn main() {
    // Need to handle Result<()> for this 
    let mut _storage = Storage::new();
    let _ = server::init_server();
}
