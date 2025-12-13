use memory_store::{config::Config, repl::runtime};


fn main() {
    let config = Config::build().unwrap();

    match runtime(config) {
        Ok(()) => (),
        Err(e) => eprintln!("{}", e)
    }
}