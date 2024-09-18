use std::{env, fs};

use std::error::Error;

#[derive(Debug)]
pub enum SearchCase {
    Sensitive,
    Insensitive,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub search_case: SearchCase,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query_arg) => query_arg,
            None => return Err("Didn't Query Argument: Query Filename [IGNORE_CASE: 0|1]"),
        };
        let file_path = match args.next() {
            Some(file_arg) => file_arg,
            None => return Err("Didn't Filename Argument: Query Filename [IGNORE_CASE: 0|1]"),
        };
        if let Some(case_arg) = args.next() {
            unsafe {
                env::set_var("IGNORE_CASE", case_arg);
            }
        }

        let search_case = match env::var("IGNORE_CASE") {
            Ok(val) => {
                if val == "1" {
                    SearchCase::Insensitive
                } else {
                    SearchCase::Sensitive
                }
            }
            Err(e) => {
                eprintln!("IGNORE_CASE environment variable: {e}, Applying default search case: 'Sensitive'.");
                SearchCase::Sensitive
            }
        };
        Ok(Config {
            query,
            file_path,
            search_case,
        })
    }
}

pub trait Summary {
    fn summarize(&self);
}

impl Summary for Config {
    fn summarize(&self) {
        println!(
            "Looking for '{}' with case:'{:?}' in '{}'.",
            self.query, self.search_case, self.file_path
        )
    }
}

pub fn search<'content>(
    query: &str,
    contents: &'content str,
    case_sensitive: &SearchCase,
) -> Option<Vec<&'content str>> {
    match case_sensitive {
        SearchCase::Sensitive => Some(
            contents
                .lines()
                .filter(|line| line.contains(query))
                .collect(),
        ),
        SearchCase::Insensitive => Some(
            contents
                .lines()
                .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
                .collect(),
        ),
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    match search(&config.query, &contents, &config.search_case) {
        Some(result) => {
            println!("\n#########################################\n");
            for line in result {
                println!("{line:?}");
            }

            println!("\n#########################################");
        }
        _ => eprintln!("'{}' not found in '{}'.", config.query, config.file_path),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            Some(vec!["safe, fast, productive."]),
            search(query, contents, &SearchCase::Sensitive)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "/
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            Some(vec!["Rust:", "Trust me."]),
            search(query, contents, &SearchCase::Insensitive)
        );
    }
}
