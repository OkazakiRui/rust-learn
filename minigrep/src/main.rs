use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // {}を探しています
    println!("Searching for {}", config.query);
    // {}というファイルの中
    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // テキストは\n{}です
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // 引数の数が足りません
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
