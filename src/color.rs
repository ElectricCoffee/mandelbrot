use serde::{Deserialize, Serialize};

pub type Color = [u8; 3];
/// The colour offset lets us specify an offset from a colour in either a positive or negative direction.
/// So an offset of `[-3, 0, 4]` would indicate that the resulting red component should decrease by 3 and the blue should increase by 4.
pub type ColorOffset = [i16; 3];

/// The `Coloring` enum supplies colouring information to the generator.
/// It consists of two distinct cases: a `Palette` and a `Range`.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Coloring {
    /// Defines a vector of colours which makes up the palette used.
    Palette(Vec<Color>),
    /// Defines a palette generator, which creates a palette
    /// based on a desired starting colour, a desired end colour,
    /// and how many steps it should take to get there.
    ///
    /// The resulting palette is 1 longer than the steps mentioned, because the starting colour isn't counted.
    ///
    /// **NOTE:** This algorithm doesn't take shared colour components into account, if you want to start at say `#ff0000` and end at `#0000ff`
    /// the green component (which stays 00 between both) won't change.
    Range {
        start: Color,
        end: Color,
        steps: i32,
    },
}

const fn to_color_offset(color: Color) -> ColorOffset {
    [color[0] as i16, color[1] as i16, color[2] as i16]
}

const fn from_color_offset(offset: ColorOffset) -> Color {
    [offset[0] as u8, offset[1] as u8, offset[2] as u8]
}

impl Coloring {
    // there has to be a better way...
    /// Builds a vector of colours out of a `Coloring`.
    /// If fed a `Palette`, it simply returns the contained `Vec`, otherwise it builds a new vec based on the parameters.
    ///
    /// Example
    /// ```
    /// let range = Coloring::Range { start: [255, 0, 0], end: [0, 127, 0], steps: 4}.build();
    /// let expected = vec![[255, 0, 0], [192, 31, 0], [129, 62, 0], [66, 93, 0], [0, 127, 0]];
    ///
    /// assert_eq!(range, expected);
    /// ```
    pub fn build(self) -> Vec<Color> {
        match self {
            Coloring::Palette(vec) => vec,

            Coloring::Range { start, end, steps } => {
                // early returns for convenience/performance
                if steps == 0 {
                    return vec![start];
                }

                if steps == 1 {
                    return vec![start, end];
                }

                // to get rid of those pesky underflows, we need to make it signed
                // but to avoid data loss, we need double the width
                let start = to_color_offset(start);
                let end = to_color_offset(end);
                let steps = steps as i16;

                let rgb_steps: Vec<i16> = end
                    .iter()
                    .zip(start.iter())
                    .map(|(s, e)| s - e)
                    .map(|d| d / steps)
                    .collect();

                let mut progress = start;
                let mut result = vec![start];

                for _ in 1..steps {
                    progress[0] += rgb_steps[0];
                    progress[1] += rgb_steps[1];
                    progress[2] += rgb_steps[2];

                    result.push(progress);
                }

                result.push(end);

                result.into_iter().map(from_color_offset).collect()
            }
        }
    }
}

#[test]
fn test_range() {
    let range = Coloring::Range {
        start: [255, 0, 0],
        end: [0, 127, 0],
        steps: 4,
    }
    .build();
    let expected = vec![
        [255, 0, 0],
        [192, 31, 0],
        [129, 62, 0],
        [66, 93, 0],
        [0, 127, 0],
    ];
    assert_eq!(range, expected);
}
