use std::collections::HashMap;

use regex::Regex;

use crate::layout::LayoutDirection;

pub enum MarkupLength {
    Dp(f32),
    Star(f32),
    Content,
    Fill,
}

pub struct MarkupElement {
    pub node_name: String,
    pub number: i32,
    pub attributes: HashMap<String, String>,
    pub children: Vec<MarkupElement>,
}

impl MarkupElement {
    pub fn get_layout_direction(&self) -> LayoutDirection {
        self.attributes.get("layout-direction").map(|dir_str|
            match &dir_str[..] {
                "column" => LayoutDirection::Column,
                "row" => LayoutDirection::Row,
                _ => panic!("Bad layout-direction on {}", &self.node_name)
            }
        ).unwrap_or(LayoutDirection::Row)
    }

    pub fn get_primary_measure(&self, layout_direction: &LayoutDirection) -> Option<MarkupLength> {
        match layout_direction {
            LayoutDirection::Row => self.get_height(),
            LayoutDirection::Column => self.get_width()
        }
    }

    pub fn get_secondary_measure(&self, layout_direction: &LayoutDirection) -> Option<MarkupLength> {
        match layout_direction {
            LayoutDirection::Row => self.get_width(),
            LayoutDirection::Column => self.get_height()
        }
    }

    pub fn get_width(&self) -> Option<MarkupLength> {
        self.attributes.get("width").and_then(parse_length)
    }

    pub fn get_height(&self) -> Option<MarkupLength> {
        self.attributes.get("height").and_then(parse_length)
    }
}

fn parse_length(length_string: &String) -> Option<MarkupLength> {
    match &length_string[..] {
        "content" => Some(MarkupLength::Content),
        "fill" => Some(MarkupLength::Fill),
        str => {
            let re = Regex::new(r"^(?P<value>[0-9]+)(?P<unit>dp|\*)$").unwrap();
            re.captures(str).map(|c| match (c.name("value"), c.name("unit")) {
                (Some(value_match), Some(unit_match)) => {
                    let unit_str = unit_match.as_str();
                    let value = value_match.as_str().parse::<f32>().unwrap();
                    match &unit_str[..] {
                        "dp" => MarkupLength::Dp(value),
                        "*" => MarkupLength::Star(value),
                        _ => panic!("Bad length unit '{}'", unit_str)
                    }
                }
                (_, _) => panic!("Bad length value '{}'", length_string)
            })
        }
    }
}
