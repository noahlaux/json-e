use failure::Fail;

#[derive(Debug, Fail, Eq, PartialEq)]
pub enum Error {
    #[fail(display = "Syntax Error: {}", _0)]
    SyntaxError(String)
}
