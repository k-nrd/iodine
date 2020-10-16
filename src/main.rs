use clap::{App, Arg};
use iodine_lib::{interpreter, repl::REPL, BoxError};

fn main() -> Result<(), BoxError> {
    let matches = App::new("Iodine REPL")
        .version("0.0.1")
        .author("Gustavo K. <g.konrad@outlook.com>")
        .about("A language inspired by Box Nystrom's Lox")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Sets an optional input file")
                .required(false),
        )
        .get_matches();

    if let Some(path) = matches.value_of("file") {
        // Get the file into a buffer, pass the buffer
        interpreter::eval(std::fs::read_to_string(path)?)?;
    } else {
        REPL::new().run()?;
    }
    Ok(())
}
