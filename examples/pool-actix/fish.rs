use std::fmt;

#[derive(Debug)]
pub enum Fish {
    Hauki,
    Ahven,
    Kissankala
}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /*let s = match self {
            Fish::Hauki => "Hauki",
            Fish::Ahven => "Ahven",
            Fish::Kissankala => "Kissankala"
        };*/
        write!(f, "{:?}", self)
    }
}
