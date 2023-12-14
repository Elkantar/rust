#[derive(PartialEq, Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(&self, first: u8, second: u8) -> Color {
        let mut cloned = self.clone();
        
        if first == cloned.r {
            if second == cloned.g {
                cloned.r = self.g;
                cloned.g = self.r;
            } else if second == cloned.b {
                cloned.r = self.b;
                cloned.b = self.r;
            } else if second == cloned.a {
                cloned.r = self.a;
                cloned.a = self.r;
            }
        } else if first == cloned.g {
            if second == cloned.r {
                cloned.g = self.r;
                cloned.r = self.g;
            } else if second == cloned.b {
                cloned.g = self.b;
                cloned.b = self.g;
            } else if second == cloned.a {
                cloned.g = self.a;
                cloned.a = self.g;
            }
        } else if first == cloned.b {
            if second == cloned.r {
                cloned.b = self.r;
                cloned.r = self.b;
            } else if second == cloned.g {
                cloned.b = self.g;
                cloned.g = self.b;
            } else if second == cloned.a {
                cloned.b = self.a;
                cloned.a = self.b;
            }
        } else if first == cloned.a {
            if second == cloned.r {
                cloned.a = self.r;
                cloned.r = self.a;
            } else if second == cloned.g {
                cloned.a = self.g;
                cloned.g = self.a;
            } else if second == cloned.b {
                cloned.a = self.b;
                cloned.b = self.a;
            }
        }

        cloned
    }
}