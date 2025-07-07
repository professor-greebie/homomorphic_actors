use env_logger::{Builder, Env};
use log::info;


fn get_loggings() {
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    Builder::from_env(env).init()
}


#[tokio::main]
 async fn main() {
    get_loggings();
    info!("Starting the homomorphic actors example...");
    
}
