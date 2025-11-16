#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Token {
    Atom(char),
    Op(char),
    Eof,
}