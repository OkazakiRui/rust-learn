use std::error::Error;
use std::fs::File;
use std::io::Read;

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // 引数の数が足りません
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // テキストは\n{}です
    println!("With text:\n{}", contents);

    Ok(())
}