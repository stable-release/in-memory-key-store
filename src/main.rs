use memory_store::{config::Config, memory_store::{runtime}};

fn main() {
    let config = Config::build().unwrap();

    runtime(config).unwrap()
}