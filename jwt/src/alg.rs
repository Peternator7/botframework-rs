#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, EnumString, Display, AsRefStr)]
pub enum Algorithm {
    None,
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
}
