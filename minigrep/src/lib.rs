use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("Not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

      Ok(Config { query, filename, case_sensitive })
  }

  // New constructor that takes ownership of the iterator
  pub fn newIterator(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    // Iterator::next to find the query string
    let query = match args.next(){
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    // Iterator::next to find the filename string
    let filename = match args.next(){
      Some(arg) => arg,
      None => return Err("Didn't get a filename string"),
    };
    

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config { query, filename, case_sensitive })
}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  
  let results = if config.case_sensitive {
    searchIterator(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
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

// Use Iterator to search
pub fn searchIterator<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents ="\
Rust:
safe, fast, productive
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive"], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents ="\
Rust:
safe, fast, productive
Pick three.
Trust me.";

    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
  }
}