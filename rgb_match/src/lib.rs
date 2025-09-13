#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        self.r = match self.r {
            x if x == first => second,
            x if x == second => first,
            x => x,
        };
        self.g = match self.g {
            x if x == first => second,
            x if x == second => first,
            x => x,
        };
        self.b = match self.b {
            x if x == first => second,
            x if x == second => first,
            x => x,
        };
        self.a = match self.a {
            x if x == first => second,
            x if x == second => first,
            x => x,
        };

        self
    }
}
