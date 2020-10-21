use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::BoxError;

pub fn eval(input: String) -> Result<(), BoxError> {
    let tokens = Lexer::new(input.as_bytes()).tokenize()?;
    let ast = Parser::new(tokens).generate_ast()?;

    for expr in ast {
        println!("{:#?}", expr);
    }

    Ok(())
}
