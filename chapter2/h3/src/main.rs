use std::fs;
use text_colorizer::*;
use regex::Regex;



fn main() {
    let args = Arguments::from_env_args();

    let mut text = match fs::read_to_string(&args.input_filename) {
        Ok(text) => text,
        Err(_) => quit("Could not read the input file"),
    };
    
    match text.replace_regex(&args.search_regex, &args.replace_with) {
        Ok(_) => println!("All matched occurences were replaced"),
        Err(e) => quit(format!("Regex Error: {e}")),
    };

    match fs::write(&args.output_filename, &text) {
        Ok(_) => println!("Success, written: {}", &args.output_filename),
        Err(_) => quit(format!("Couldn't write output file: {}",
                               &args.output_filename)),
    }
}



#[derive(Debug)]
struct Arguments {
    input_filename: String,
    output_filename: String,
    search_regex: String,
    replace_with: String,
}

impl<'a> Arguments {
    pub fn new<C: AsRef<str>>(
        input_filename: C,
        output_filename: C,
        search_regex: C,
        replace_with: C,
    ) -> Self
    {
        Self {
            input_filename: String::from(input_filename.as_ref()),
            output_filename: String::from(output_filename.as_ref()),
            search_regex: String::from(search_regex.as_ref()),
            replace_with: String::from(replace_with.as_ref()),
        }
    }

    pub fn from_env_args() -> Self {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 5 {
            eprint!("Exactly 4 arguments needed: \n{} ", args[0]);
            eprintln!("{}",
                "<filename> <output_filename> <regex> <replace_with>".green());
            std::process::exit(1);
        }

        Self::new(&args[1], &args[2], &args[3], &args[4])
    }
}


trait ReplaceRegex {
    fn replace_regex(&mut self, regex: &str, with: &str)
    -> Result<(), regex::Error>;
}

impl ReplaceRegex for String {
    fn replace_regex(&mut self, regex: &str, with: &str)
    -> Result<(), regex::Error>
    {
        let reg = Regex::new(regex)?;
        *self = reg.replace_all(self, with).to_string();
        Ok(())
    }
}


fn quit(message: impl AsRef<str>) -> ! {
    eprintln!("{}\n{}", "Error:".red(), message.as_ref().purple());
    std::process::exit(1);
}