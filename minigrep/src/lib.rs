use std::error::Error;
use std::fs;
use std::string;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_arg() {
        let arg: Vec<String>  = vec![String::from("hello")];
        assert_eq!(Config::new(&arg), Err("Not enough arguments. Arguments: query, filename"));
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	println!("With text:\n{}", contents);

    Ok(())
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("Not enough arguments. Arguments: query, filename");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}
