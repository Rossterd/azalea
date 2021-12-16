use azalea_client::connect::join_server;
use tokio::runtime::Runtime;

async fn bot() {
    let address = "95.111.249.143:10000";
    let _response = join_server(&address.try_into().unwrap()).await.unwrap();
    // println!("{}", response.description.to_ansi(None));
    println!("connected");
}

fn main() {
    println!("Hello, world!");

    let io_loop = Runtime::new().unwrap();
    io_loop.block_on(bot());
}
