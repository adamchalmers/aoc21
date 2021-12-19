pub struct Pair {
    pub l: Element,
    pub r: Element,
}

pub enum Element {
    Num(u8),
    Pair(Box<Pair>),
}
