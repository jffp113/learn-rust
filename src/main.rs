mod server;



fn main() {
    let s = server::Server::new();
    s.start(3000);
}