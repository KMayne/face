use hex;
use regex::Regex;
use skia_safe::{canvas::Canvas, Paint, Rect as SkRect};

#[derive(Debug)]
pub struct ColourParseError {}

#[derive(Copy, Clone)]
pub struct ArgbColour {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}
impl ArgbColour {
    pub fn from_hex(hex: &str) -> Result<ArgbColour, ColourParseError> {
        let re = Regex::new(r"#?([\dA-Za-z]{3}|[\dA-Za-z]{6}|[\dA-Za-z]{8})").unwrap();
        let captures = re.captures(hex);
        let digits = captures.and_then(|c| { c.get(1) }).map(|m| m.as_str());
        match digits {
            Some(digit_string) =>
                match digit_string.len() {
                    3 => {
                        fn parse_hex_digit(char: &char) -> Option<u8> {
                            char.to_digit(16).map(|d| d as u8)
                        }
                        let mut chars = digit_string.chars();
                        let r = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                        let g = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                        let b = parse_hex_digit(&chars.next().unwrap()).unwrap() * 17;
                        Ok(ArgbColour { a: 255, r, g, b })
                    },
                    6 => {
                        hex::decode(digit_string)
                            .map(|b| ArgbColour { a: 255, r: b[0], g: b[1], b: b[2] })
                            .map_err(|_| ColourParseError {})
                    }
                    8 => {
                        hex::decode(digit_string)
                            .map(|b| ArgbColour { a: b[0], r: b[1], g: b[2], b: b[3] })
                            .map_err(|_| ColourParseError {})
                    }
                    _ => Err(ColourParseError {})
                }
            None => Err(ColourParseError {})
        }
    }
    fn invert(&self) -> ArgbColour {
        ArgbColour {
            a: self.a,
            r: self.r ^ 0xFF,
            g: self.g ^ 0xFF,
            b: self.b ^ 0xFF,
        }
    }
}

pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub stroke_colour: ArgbColour,
    pub fill_colour: ArgbColour
}

impl Rect {
    fn to_skia_rect(&self) -> SkRect {
        SkRect::new(self.left, self.top, self.left + self.width, self.top + self.height)
    }
}

pub fn draw_ui(canvas: &mut Canvas, rects: Vec<Rect>) {
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(4.0);
    for rect in &rects {
        let colour = rect.fill_colour;
        paint.set_argb(colour.a, colour.r, colour.g, colour.b);
        canvas.draw_rect(rect.to_skia_rect(), &paint);
    }
}