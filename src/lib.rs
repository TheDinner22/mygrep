use std::{env, fs};
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let file_content = fs::read_to_string(config.file_name)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.look_for, &file_content)
    } else {
        search(&config.look_for, &file_content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}
pub struct Config {

    pub look_for: String,
    pub file_name: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(argv: &Vec<String>) -> Result<Config, &'static str> {
        if argv.len() < 3 {
            return Err("not enough arguments!");
        }

        let look_for = argv[1].clone();
        let file_name = argv[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { look_for, file_name, ignore_case })
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

pub fn search_case_insensitive<'a>(
    look_for: &str,
    content: &'a str
) -> Vec<&'a str> {
    let look_for = &look_for.to_lowercase();
    let mut res = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(look_for) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let look_for = "RUSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(look_for, contents));
    }

    #[test]
    fn case_sensitive() {
        let look_for = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(look_for, contents));
    }
}
