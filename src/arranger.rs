use crate::renderer::{ArgbColour, Rect};
use crate::measurer::{MeasuredLength, MeasuredElement};
use crate::layout::LayoutDirection;

#[derive(Debug, PartialEq)]
pub struct ArrangedElement {
    pub(crate) number: i32,
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub width: f32,
    pub height: f32,
    pub(crate) children: Vec<ArrangedElement>,
}

impl ArrangedElement {
    pub(crate) fn flatten(&self) -> Vec<Rect> {
        let mut elements = vec![self.to_rect()];
        for child in self.children.iter() {
            let x = &mut child.flatten();
            elements.append(x)
        }
        elements
    }

    fn to_rect(&self) -> Rect {
        let colours = vec![
            ArgbColour::from_hex("#F2DC5D").unwrap(),
            ArgbColour::from_hex("#F2A359").unwrap(),
            ArgbColour::from_hex("#DB9065").unwrap(),
            ArgbColour::from_hex("#A4031F").unwrap(),
            ArgbColour::from_hex("#240B36").unwrap(),
            ArgbColour::from_hex("#71A2B6").unwrap(),
            ArgbColour::from_hex("#C4F1BE").unwrap(),
        ];

        Rect {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
            stroke_colour: ArgbColour::from_hex("#000").unwrap(),
            fill_colour: colours[(self.number as usize) % colours.len()],
        }
    }
}

pub fn arrange_layout_tree(elem: &MeasuredElement, x: f32, y: f32, width: f32, height: f32) -> ArrangedElement {
    let content_primary_measure = elem.children.iter()
        .fold(CombinedMeasuredLength::zero(),
              |acc, child| acc.combine(&child.primary_measure(&elem.layout_direction)));
    let star_unit_value = (match elem.layout_direction {
        LayoutDirection::Row => height,
        LayoutDirection::Column => width,
    } - content_primary_measure.dp) / content_primary_measure.star;
    let is_row = match elem.layout_direction {
        LayoutDirection::Row => true,
        LayoutDirection::Column => false,
    };
    let mut primary_offset = if is_row { y } else { x };
    let mut arranged_children = vec![];
    for child in elem.children.iter() {
        let primary_measure = child.primary_measure(&elem.layout_direction)
            .apply_star_value(star_unit_value);
        let secondary_measure = match child.secondary_measure(&elem.layout_direction) {
            MeasuredLength::Dp(dp) => dp,
            MeasuredLength::Star(_) => panic!("Star not supported on secondary measure"),
            MeasuredLength::Fill => width,
        };
        arranged_children.push(arrange_layout_tree(
            child,
            if is_row { x } else { x + primary_offset },
            if is_row { y + primary_offset } else { y },
            if is_row { secondary_measure } else { primary_measure },
            if is_row { primary_measure } else { secondary_measure }));
        primary_offset += primary_measure;
    }

    ArrangedElement {
        number: elem.number,
        x,
        y,
        width,
        height,
        children: arranged_children,
    }
}

struct CombinedMeasuredLength {
    dp: f32,
    star: f32,
}

impl CombinedMeasuredLength {
    pub fn zero() -> CombinedMeasuredLength {
        CombinedMeasuredLength {
            dp: 0.0,
            star: 0.0,
        }
    }

    pub fn combine(&self, len: &MeasuredLength) -> CombinedMeasuredLength {
        match len {
            MeasuredLength::Dp(dp) => CombinedMeasuredLength { dp: self.dp + dp, ..*self },
            MeasuredLength::Star(star) => CombinedMeasuredLength { star: self.star + star, ..*self },
            MeasuredLength::Fill => panic!("Fill not allowed on primary measure")
        }
    }
    pub fn add(len_a: CombinedMeasuredLength, len_b: CombinedMeasuredLength) -> CombinedMeasuredLength {
        CombinedMeasuredLength {
            dp: len_a.dp + len_b.dp,
            star: len_a.star + len_b.star,
        }
    }
}