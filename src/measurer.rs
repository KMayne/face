use crate::layout::LayoutDirection;
use crate::markup::{MarkupElement, MarkupLength};
use std::cmp::Ordering;

pub fn measure_element_tree(elem: &MarkupElement) -> MeasuredElement {
    let children: Vec<MeasuredElement> = elem.children.iter().map(measure_element_tree).collect();
    let layout_direction = elem.get_layout_direction();

    let primary_measurement = match elem.get_primary_measure(&layout_direction) {
        Some(MarkupLength::Content) => {
            let total_primary_content_measure: f32 = children.iter()
                .map(|child| child.primary_measure(&layout_direction).dp_or_default(0.0))
                .sum();
            MeasuredLength::Dp(total_primary_content_measure)
        },
        Some(MarkupLength::Dp(dp)) => MeasuredLength::Dp(dp),
        Some(MarkupLength::Star(star)) => MeasuredLength::Star(star),
        Some(MarkupLength::Fill) => panic!("Fill not valid as a primary measure"),
        None => MeasuredLength::Star(1.0)
    };
    let secondary_measurement = match elem.get_secondary_measure(&layout_direction) {
        Some(MarkupLength::Content) => {
            let max_child_secondary_measure: f32 = children.iter()
                .map(|child| child.secondary_measure(&layout_direction).dp_or_default(0.0))
                .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(0.0);
            MeasuredLength::Dp(max_child_secondary_measure)
        },
        Some(MarkupLength::Dp(dp)) => MeasuredLength::Dp(dp),
        Some(MarkupLength::Star(_)) => panic!("Star not valid as a secondary measure"),
        Some(MarkupLength::Fill) => MeasuredLength::Fill,
        None => MeasuredLength::Fill,
    };
    MeasuredElement {
        number: elem.number,
        width: match layout_direction {
            LayoutDirection::Row => secondary_measurement,
            LayoutDirection::Column => primary_measurement
        },
        height: match layout_direction {
            LayoutDirection::Row => primary_measurement,
            LayoutDirection::Column => secondary_measurement
        },
        layout_direction,
        children,
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MeasuredLength {
    Dp(f32),
    Star(f32),
    Fill
}

impl MeasuredLength {
    pub(crate) fn apply_star_value(&self, star_unit_value: f32) -> f32 {
        match self {
            MeasuredLength::Dp(dp) => *dp,
            MeasuredLength::Star(star) => *star * star_unit_value,
            MeasuredLength::Fill => panic!("Fill lengths can't have star values applied")
        }
    }

    fn dp_or_default(&self, default: f32) -> f32 {
        match self {
            MeasuredLength::Dp(dp) => *dp,
            _ => default,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct MeasuredElement {
    pub(crate) number: i32,
    pub(crate) width: MeasuredLength,
    pub(crate) height: MeasuredLength,
    pub(crate) layout_direction: LayoutDirection,
    pub(crate) children: Vec<MeasuredElement>,
}

impl MeasuredElement {
    pub fn primary_measure(&self, layout_direction: &LayoutDirection) -> MeasuredLength {
        match layout_direction {
            LayoutDirection::Row => self.height,
            LayoutDirection::Column => self.width,
        }
    }

    pub fn secondary_measure(&self, layout_direction: &LayoutDirection) -> MeasuredLength {
        match layout_direction {
            LayoutDirection::Row => self.width,
            LayoutDirection::Column => self.height,
        }
    }
}
