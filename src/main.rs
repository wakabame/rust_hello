use clap::{App, Arg};

fn main() {
    let matches: clap::ArgMatches = App::new("My RPN Program")
        .version("1.0.0")
        .author("Your Name")
        .about("Super awesome sample RPN caluclator")
        .arg(
            Arg::new("formula_file")
                .about("Formula written in RPN")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of vervosity")
                .short('v')
                .long("verbose")
                .required(false),
        )
        .get_matches();

    match matches.value_of("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified."),
    }

    let verbose = matches.is_present("verbose");
    println!("Is verbosity specified?: {}", verbose);
}
