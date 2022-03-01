use std::{fs, error::Error};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)
                                .expect("Something went wrong while reading the file");

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}


pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Insufficient arguements.")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let query = "Ecstasy";
        let contents = "\nThere were parties. Ecstasy.\nHennessy. Dancing\naround each other. Bluntness. Blunts\n";

        assert_eq!(vec!["There were parties. Ecstasy."], search(query, contents));

    }
}