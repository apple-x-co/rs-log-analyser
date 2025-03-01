use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    file_path: String,

    #[clap(short, long)]
    pattern: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    match Regex::new(&args.pattern) {
        Ok(re) => {
            let file = File::open(args.file_path)?;
            let reader = io::BufReader::new(file);

            let mut map:HashMap<String, u32> = HashMap::new();

            for line in reader.lines() {
                let line = line?;
                if !re.is_match(line.as_str()) {
                    continue;
                }

                for cap in re.captures_iter(line.as_str()) {
                    let s = cap.get(0).unwrap().as_str().to_string();

                    let i = map.get(&s);
                    if i.is_none() {
                        map.insert(s, 1);

                        continue;
                    }

                    map.insert(s, i.unwrap() + 1);
                }
            }

            for (key, value) in &map {
                println!("{}\t{}", key, value);
            }
        }
        Err(e) => {
            eprintln!("Error compiling regex: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
