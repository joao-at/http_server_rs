mod http;
use http::Server;

fn main() -> std::io::Result<()> {
    Server::new()?.run()
}