/// Contains all the possible input tokens
pub enum Token {
    Plus,
    Minus,
    Slash,
    Star,
}

impl Token {
    pub const fn is_binary(&self) -> bool {
        match self {
            Self::Plus => true,
            Self::Minus => true,
            Self::Slash => true,
            Self::Star => true,
        }
    }
}
