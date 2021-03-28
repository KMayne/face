use crate::renderer;
use std::collections::HashMap;
use renderer::Rect;
use crate::renderer::ArgbColour;

pub enum PrimaryLength {
    Dp(f32),
    Star(f32),
    Content
}
pub enum SecondaryLength {
    Dp(f32),
    Fill
}

pub struct MarkupElement {
    pub node_name: String,
    pub number: i32,
    pub attributes: HashMap<String, String>,
    pub children: Vec<MarkupElement>,
}

#[derive(Copy, Clone)]
enum MeasuredLength {
    Dp(f32),
    Star(f32),
}
impl MeasuredLength {
    fn to_dp(&self, star_unit_value: f32) -> f32 {
        match self {
            MeasuredLength::Dp(dp) => *dp,
            MeasuredLength::Star(star) => *star * star_unit_value,
        }
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
        }
    }
    pub fn add(len_a: CombinedMeasuredLength, len_b: CombinedMeasuredLength) -> CombinedMeasuredLength {
        CombinedMeasuredLength {
            dp: len_a.dp + len_b.dp,
            star: len_a.star + len_b.star,
        }
    }
}

enum LayoutDirection {
    Row,
    Column,
}

struct MeasuredElement {
    number: i32,
    width: MeasuredLength,
    height: MeasuredLength,
    layout_direction: LayoutDirection,
    children: Vec<MeasuredElement>,
}
impl MeasuredElement {
    fn primary_measure(&self, layout_direction: &LayoutDirection) -> MeasuredLength {
        match layout_direction {
            LayoutDirection::Row => self.height,
            LayoutDirection::Column => self.width,
        }
    }

    fn secondary_measure(&self, layout_direction: &LayoutDirection) -> MeasuredLength {
        match layout_direction {
            LayoutDirection::Row => self.width,
            LayoutDirection::Column => self.height,
        }
    }
}

#[derive(Debug, PartialEq)]
struct ArrangedElement {
    number: i32,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    children: Vec<ArrangedElement>,
}

impl ArrangedElement {
    fn flatten(&self) -> Vec<Rect> {
        let mut elements = vec![self.to_rect()];
        for child in self.children.iter() {
            let x = &mut child.flatten();
            elements.append(x)
        }
        elements
    }

    fn to_rect(&self) -> Rect {
        Rect {
            left: self.x as f32,
            top: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
            stroke_colour: ArgbColour::from_hex("#000").unwrap(),
            fill_colour: ArgbColour::from_hex("#F00").unwrap(),
        }
    }
}

pub fn generate_layout(_root_elem: &MarkupElement, width: f32, height: f32) -> Vec<Rect> {
    // Dimension - (_dp | content | fill | _*)
    // * only allowed on primary measure
    // fill only allowed on secondary measure
    // * cannot appear below content in visual tree
    // content cannot appear on a leaf node (could produce a warning?)
    // root element always fills the display area
    // rootElem.layout = { x: 0, y: 0, width, height };
    // rootElem.children.forEach(measureElem);
    // arrangeChildren(rootElem);
    // let measured_tree = MeasuredElement {
    //     number: 0,
    //     width: MeasuredLength::Star(1),
    //     height: MeasuredLength::Star(1),
    //     children: root_elem.children.iter().map(measure_element_tree).collect()
    // };
    let measured_tree = MeasuredElement {
        number: 0,
        width: MeasuredLength::Star(1.0),
        height: MeasuredLength::Star(1.0),
        layout_direction: LayoutDirection::Row,
        children: vec![
            MeasuredElement {
                number: 1,
                width: MeasuredLength::Star(1.0),
                height: MeasuredLength::Star(1.0),
                layout_direction: LayoutDirection::Row,
                children: vec![],
            }
        ],
    };
    // let arranged_tree = arrange_layout_tree(&measured_tree, 0, 0, width, height);
    let arranged_tree = arrange_layout_tree(&measured_tree, 0.0, 0.0, width, height);
    arranged_tree.flatten()
}

fn measure_element_tree(_elem: &MarkupElement) -> MeasuredElement {
    // measure_element_tree(elem)
    MeasuredElement {
        number: 0,
        width: MeasuredLength::Star(1.0),
        height: MeasuredLength::Star(1.0),
        layout_direction: LayoutDirection::Row,
        children: vec![],
    }
}

fn arrange_layout_tree(elem: &MeasuredElement, x: f32, y: f32, width: f32, height: f32) -> ArrangedElement {
    let content_primary_measure = elem.children.iter()
        .fold(CombinedMeasuredLength::zero(),
              |acc, child| acc.combine(&child.primary_measure(&elem.layout_direction)));
    let star_unit_value = (match elem.layout_direction {
        LayoutDirection::Row => height,
        LayoutDirection::Column => width,
    } - content_primary_measure.dp) /  content_primary_measure.star;
    let is_row = match elem.layout_direction {
        LayoutDirection::Row => true,
        LayoutDirection::Column => false,
    };
    let mut primary_offset = if is_row { y } else { x };
    let mut arranged_children = vec![];
    for child in elem.children.iter() {
        let primary_measure = child.primary_measure(&elem.layout_direction)
            .to_dp(star_unit_value);
        let secondary_measure =
            if let MeasuredLength::Dp(dp) = child.secondary_measure(&elem.layout_direction)
            { dp } else { if is_row { width } else { height } };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element_is_one_big_rect() {}

    #[test]
    fn single_elem_measured_tree_can_be_arranged() {
        let measured_tree = MeasuredElement {
            number: 0,
            width: MeasuredLength::Star(1.0),
            height: MeasuredLength::Star(1.0),
            layout_direction: LayoutDirection::Row,
            children: vec![
                MeasuredElement {
                    number: 1,
                    layout_direction: LayoutDirection::Row,
                    width: MeasuredLength::Star(1.0),
                    height: MeasuredLength::Star(1.0),
                    children: vec![],
                }
            ],
        };
        let width = 333.0;
        let height = 777.0;
        let result = arrange_layout_tree(&measured_tree, 0.0, 0.0, width, height);

        let expected_arranged_tree = ArrangedElement {
            number: 0,
            x: 0.0,
            y: 0.0,
            width,
            height,
            children: vec![
                ArrangedElement {
                    number: 1,
                    x: 0.0,
                    y: 0.0,
                    width,
                    height,
                    children: vec![],
                }
            ],
        };
        assert_eq!(result, expected_arranged_tree);
    }
}