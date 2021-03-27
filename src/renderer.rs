use hex;
use regex::Regex;
use skia_safe::{canvas::Canvas, Paint, Rect as SkRect};

#[derive(Debug)]
struct ColourParseError {}

#[derive(Copy, Clone)]
struct ArgbColour {
    a: u8,
    r: u8,
    g: u8,
    b: u8,
}
impl ArgbColour {
    fn from_hex(hex: &str) -> Result<ArgbColour, ColourParseError> {
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

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    stroke_colour: ArgbColour,
    fill_colour: ArgbColour
}

impl Rect {
    fn to_skia_rect(&self) -> SkRect {
        SkRect::new(self.x, self.y, self.x + self.width, self.y + self.height)
    }
}

pub fn draw_ui(canvas: &mut Canvas) {
    let black = ArgbColour::from_hex("#000").unwrap();
    let rects = vec! {
        Rect { x: 0.0, y: 0.0, width: 300.0, height: 400.0, stroke_colour: black, fill_colour: black.invert() },
        Rect { x: 300.0, y: 0.0, width: 500.0, height: 400.0, stroke_colour: black.invert(), fill_colour: black },
        Rect { x: 150.0, y: 0.0, width: 250.0, height: 160.0, stroke_colour: ArgbColour::from_hex("#F00").unwrap(), fill_colour: ArgbColour::from_hex("#FFF").unwrap() },
    };
    let mut paint = Paint::default();
    paint.set_anti_alias(true);
    paint.set_stroke_width(4.0);
    for rect in &rects {
        let colour = rect.fill_colour;
        paint.set_argb(colour.a, colour.r, colour.g, colour.b);
        canvas.draw_rect(rect.to_skia_rect(), &paint);
    }
}