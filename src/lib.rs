use std::env;
use std::error::Error;
use std::fs;

pub struct Args {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Args {
    pub fn new(mut args: env::Args) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Args {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(parsed_args: Args) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(parsed_args.filename)?;

    let results = if parsed_args.case_sensitive {
        search(&parsed_args.query, &contents)
    } else {
        search_case_insensitive(&parsed_args.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    return Ok(());
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    return contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    return results;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
