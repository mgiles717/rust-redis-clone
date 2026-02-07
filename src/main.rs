mod server;

fn main() {
    // Need to handle Result<()> for this 
    let _ = server::init_server();
}
