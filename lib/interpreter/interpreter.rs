use crate::lexer::Lexer;
use crate::BoxError;

pub fn eval(input: String) -> Result<(), BoxError> {
    let tokens = Lexer::new(input.as_bytes()).tokenize()?;

    for expr in tokens {
        println!("{:#?}", expr);
    }

    Ok(())
}
