/// Alias for the type `[u8; 3]` for documentation purposes.
pub type Color = [u8; 3];

/// The colour #0000FF
pub const BLUE: Color = [0x00, 0x00, 0xff];

/// The colour #007FFF
pub const AZURE: Color = [0x00, 0x7f, 0xff];

/// The colour #00FFFF
pub const CYAN: Color = [0x00, 0xff, 0xff];

/// The colour #00FF7F
pub const SPRING: Color = [0x00, 0xff, 0x7f];

/// The colour #00FF00
pub const GREEN: Color = [0x00, 0xff, 0x00];

/// The colour #7FFF00
pub const CHARTREUSE: Color = [0x7f, 0xff, 0x00];

/// The colour #FFFF00
pub const YELLOW: Color = [0xff, 0xff, 0x00];

/// The colour #FF7F00
pub const ORANGE: Color = [0xff, 0x7f, 0x00];

/// The colour #FF0000
pub const RED: Color = [0xff, 0x00, 0x00];

/// The colour #FF007F
pub const ROSE: Color = [0xff, 0x00, 0x7f];

/// The colour #FF00FF
pub const MAGENTA: Color = [0xff, 0x00, 0xff];

/// The colour #7F00FF
pub const VIOLET: Color = [0x7f, 0x00, 0xff];

/// The colour #000000
pub const BLACK: Color = [0x00, 0x00, 0x00];

/// An array of all the 12 pre-defined colours for easy indexing.
pub const COLORS: [Color; 12] = [
    BLUE, AZURE, CYAN, SPRING, GREEN, CHARTREUSE, YELLOW, ORANGE, RED, ROSE, MAGENTA, VIOLET,
];
