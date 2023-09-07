#[derive(Clone, Copy)]
pub enum Pirate {
    Barbary,
    Bucaneer,
}


impl Pirate {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Self::Bucaneer),
            '0' => Some(Self::Barbary),
            _ => None,
        }
    }
}
