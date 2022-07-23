mod lib;
use lib::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!(
        "============= file: {} ==== query: {} =============",
        config.filename, config.query
    );

    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
