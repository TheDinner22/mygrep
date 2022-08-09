use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.file_name)?;

    for line in search(&config.look_for, &file_content) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub look_for: String,
    pub file_name: String,
}

impl Config {
    pub fn new(argv: &Vec<String>) -> Result<Config, &'static str> {
        if argv.len() < 3 {
            return Err("not enough arguments!");
        }

        let look_for = argv[1].clone();
        let file_name = argv[2].clone();

        Ok(Config { look_for, file_name })
    }
}

pub fn search<'a>(look_for: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in content.lines() {
        if line.contains(look_for) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let look_for = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(look_for, contents));
    }
}
