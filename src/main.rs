use log::info;
use r2d2::Pool;
use redis::{Commands, Client};

fn main() {
    env_logger::init();
    info!("start");

    let redis_host = std::env::var("REDIS_HOST").unwrap();
    let client = Client::open(format!("redis://{}", redis_host)).unwrap();
    let pool = Pool::builder().build(client).unwrap();

    try_get(pool.clone());
    try_set(pool.clone());
    try_get(pool.clone());

    info!("finish");
}

fn try_get(pool:Pool<Client>){
    let mut client = pool.get().unwrap();
    let val: String = client.get("hoge").unwrap_or_else(|_| "null".to_string());
    info!("get {}", val);
}

fn try_set(pool:Pool<Client>){
    let mut client = pool.get().unwrap();
    let _:() = client.set("hoge", "fuga").unwrap();
    info!("set");
}