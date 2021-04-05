use std::collections::HashMap;

use regex::Regex;

use renderer::Rect;

use crate::{arranger, measurer, renderer};
use crate::markup::{MarkupElement, MarkupLength};
use crate::measurer::{MeasuredElement, MeasuredLength};
use crate::renderer::ArgbColour;

pub enum  PrimaryLength {
    Dp(f32),
    Star(f32),
    Content
}
pub enum SecondaryLength {
    Dp(f32),
    Fill
}

#[derive(PartialEq, Debug)]
pub enum LayoutDirection {
    Row,
    Column,
}

pub fn generate_layout(root_elem: &MarkupElement, width: f32, height: f32) -> Vec<Rect> {
    // Dimension - (_dp | content | fill | _*)
    // * only allowed on primary measure
    // fill only allowed on secondary measure
    // * cannot appear below content in visual tree
    // content cannot appear on a leaf node (could produce a warning?)
    // root element always fills the display area

    let measured_tree = MeasuredElement {
        number: root_elem.number,
        width: MeasuredLength::Star(1.0),
        height: MeasuredLength::Star(1.0),
        layout_direction: root_elem.get_layout_direction(),
        children: root_elem.children.iter().map(measurer::measure_element_tree).collect()
    };
    arranger::arrange_layout_tree(&measured_tree, 0.0, 0.0, width, height).flatten()
}

#[cfg(test)]
mod tests {
    use crate::arranger::ArrangedElement;

    use super::*;

    #[test]
    fn single_element_is_one_big_rect() {}

    #[test]
    fn single_elem_markup_tree_can_be_measured() {
        let single_elem_doc = MarkupElement {
            node_name: String::from("imuroot"),
            number: 0,
            attributes: vec![(String::from("version"), String::from("0.0.0"))].into_iter().collect(),
            children: vec![
                MarkupElement {
                    node_name: String::from("box"),
                    number: 1,
                    attributes: HashMap::new(),
                    children: vec![]
                }
            ],
        };
        let result = measurer::measure_element_tree(&single_elem_doc);

        let expected_measured_tree = MeasuredElement {
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
        assert_eq!(result, expected_measured_tree);
    }

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
        let result = arranger::arrange_layout_tree(&measured_tree, 0.0, 0.0, width, height);

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