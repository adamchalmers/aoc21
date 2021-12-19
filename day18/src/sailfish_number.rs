pub struct Number {
    pub l: Element,
    pub r: Element,
}

pub enum Element {
    Literal(u8),
    Pair(Box<Number>),
}
