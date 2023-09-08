use int_enum::IntEnum;

/// Color enum
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Color {
    BLACK = 0,
    RED = 1,
    GREEN = 2,
    YELLOW = 3,
    BLUE = 4,
    MAGENTA = 5,
    CYAN = 6,
    WHITE = 7,
}

/// Struct that indicates the position at which a
struct ColorMarker {
    index: usize,
    fg: Color,
}

/// Struct to handle the output of the git status.
pub struct ColoredString {
    raw: String,
    color_markers: Vec<ColorMarker>,
}

// 3 to change the foreground color as non-bright color.
const ANSII_ESCAPE_START: &str = "\x0033[3";
const ANSII_ESCAPE_END: &str = "m";
const ANSII_ESCAPE_LEN: usize = ANSII_ESCAPE_START.len() +
                               1 + // Color on one character.
                               ANSII_ESCAPE_END.len();

fn ansii_escape_str(fg: Color) -> String {
    let mut ret = String::with_capacity(ANSII_ESCAPE_LEN);
    ret += ANSII_ESCAPE_START;
    ret += &fg.int_value().to_string();
    ret += ANSII_ESCAPE_END;

    return ret;
}

impl ColoredString {
    pub fn new() -> ColoredString {
        ColoredString {
            raw: String::new(),
            color_markers: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> ColoredString {
        ColoredString {
            raw: String::with_capacity(capacity),
            color_markers: Vec::new(),
        }
    }

    pub fn set_fg_color(&mut self, fg: Color) {
        let color_marker = ColorMarker {
            index: self.raw.len(),
            fg,
        };
        self.color_markers.push(color_marker)
    }

    #[inline]
    pub fn push(&mut self, ch: char) {
        self.raw.push(ch)
    }

    #[inline]
    pub fn push_str(&mut self, string: &str) {
        self.raw.push_str(string)
    }

    pub fn colored(&self) -> String {
        let mut index: usize = 0;
        let mut ret = String::with_capacity(
            self.raw.len() + ((self.color_markers.len() + 1) * ANSII_ESCAPE_LEN),
        );

        for color_marker in self.color_markers.iter() {
            ret += &self.raw[index..color_marker.index];
            ret += &ansii_escape_str(color_marker.fg);

            index = color_marker.index;
        }

        ret += &self.raw[index..self.raw.len()];

        return ret;
    }
}
