use simple_kv::handler;
use simple_kv::store::KvStore;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addrs = "0.0.0.0:8088";

    let listener = TcpListener::bind(&addrs).await?;
    println!("listen on: {}", addrs);

    let store = KvStore::new();

    loop {
        let (stream, client_addr) = listener.accept().await?;
        println!("client: {} connected", client_addr);

        let mut inner_store = store.clone();
        match handler::stream_handle(stream, &mut inner_store).await {
            Ok(_) => {
                println!("client: {} disconnect", client_addr);
            }
            Err(e) => {
                println!("have error: {}", e);
            }
        }
    }
}
