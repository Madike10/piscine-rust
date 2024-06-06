#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        let color = self.clone();
        let r = color.r.clone();
        let g = color.g.clone();
        let b = color.b.clone();
        let a = color.a.clone();

        if (r, g) == (first, second) || (g, r) == (first, second) {
            self = Color { r: g, g: r, b, a };
            return self;
        }
        if (r, b) == (first, second) || (b, r) == (first, second) {
            self = Color { r: b, g, b: r, a };
            return self;
        }
        if (r, a) == (first, second) || (a, r) == (first, second) {
            self = Color { r: a, g, b, a: r };
            return self;
        }
        if (g, b) == (first, second) || (b, g) == (first, second) {
            self = Color { r, g: b, b: g, a };
            return self;
        }
        if (g, a) == (first, second) || (a, g) == (first, second) {
            self = Color { r, g: a, b, a: g };
            return self;
        }
        if (b, a) == (first, second) || (a, b) == (first, second) {
            self = Color { r, g, b: a, a: b };
            return self;
        }

        return self;
    }
}
