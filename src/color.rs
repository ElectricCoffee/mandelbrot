pub const BLUE: [u8; 3] = [0x00, 0x00, 0xff];
pub const AZURE: [u8; 3] = [0x00, 0x7f, 0xff];
pub const CYAN: [u8; 3] = [0x00, 0xff, 0xff];
pub const SPRING: [u8; 3] = [0x00, 0xff, 0x7f];
pub const GREEN: [u8; 3] = [0x00, 0xff, 0x00];
pub const CHARTREUSE: [u8; 3] = [0x7f, 0xff, 0x00];
pub const YELLOW: [u8; 3] = [0xff, 0xff, 0x00];
pub const ORANGE: [u8; 3] = [0xff, 0x7f, 0x00];
pub const RED: [u8; 3] = [0xff, 0x00, 0x00];
pub const ROSE: [u8; 3] = [0xff, 0x00, 0x7f];
pub const MAGENTA: [u8; 3] = [0xff, 0x00, 0xff];
pub const VIOLET: [u8; 3] = [0x7f, 0x00, 0xff];
pub const BLACK: [u8; 3] = [0x00, 0x00, 0x00];
pub const COLORS: [[u8; 3]; 12] = [
    BLUE, AZURE, CYAN, SPRING, GREEN, CHARTREUSE, YELLOW, ORANGE, RED, ROSE, MAGENTA, VIOLET,
];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Color {
    state: u32,
    factor: u32,
}

impl Color {
    pub fn new(factor: u32) -> Color {
        Color { state: 0, factor }
    }
    pub fn new_rgb([r, g, b]: [u8; 3], factor: u32) -> Color {
        let mut state = 0;
        state += r as u32;
        state <<= 8;
        state += g as u32;
        state <<= 8;
        state += b as u32;
        Color { state, factor }
    }
    pub fn to_rgb(&self) -> [u8; 3] {
        let Color { mut state, .. } = self;
        let b = (state % 0x100) as u8;
        state >>= 8;
        let g = (state % 0x100) as u8;
        state >>= 8;
        let r = (state % 0x100) as u8;
        [r, g, b]
    }
}

const MAX_COLOR: u32 = 0x1000000;

impl Iterator for Color {
    type Item = Color;
    fn next(&mut self) -> Option<Self::Item> {
        self.state = (self.state + self.factor) % MAX_COLOR;
        Some(*self)
    }
}

#[cfg(test)]
mod test {
    use super::Color;
    #[test]
    fn test_to_rgb() {
        let color = Color {
            state: 0xaabbcc,
            factor: 0,
        };
        let expected = [0xaa, 0xbb, 0xcc];
        assert_eq!(color.to_rgb(), expected)
    }
    #[test]
    fn test_from_rgb() {
        let expected = Color {
            state: 0xaabbcc,
            factor: 0,
        };
        let color = [0xaa, 0xbb, 0xcc];
        assert_eq!(Color::new_rgb(color, 0), expected)
    }
}
