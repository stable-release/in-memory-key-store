use memory_store::{config::Config, repl::runtime};


fn main() {
    let config = Config::build().unwrap();

    runtime(config).unwrap()
}