use face;
use face::MarkupElement;
use std::collections::HashMap;

fn main() {
    let doc = MarkupElement {
        node_name: String::from("imuroot"),
        number: 0,
        attributes: vec![(String::from("version"), String::from("0.0.0"))].into_iter().collect(),
        children: vec![
            MarkupElement {
                node_name: String::from("box"),
                number: 1,
                attributes: vec![(String::from("height"), String::from("40dp"))].into_iter().collect(),
                children: vec![]
            },
            MarkupElement {
                node_name: String::from("box"),
                number: 2,
                attributes: vec![(String::from("height"), String::from("2*"))].into_iter().collect(),
                children: vec![]
            },
            MarkupElement {
                node_name: String::from("box"),
                number: 3,
                attributes: HashMap::new(),
                children: vec![]
            },
            MarkupElement {
                node_name: String::from("box"),
                number: 4,
                attributes: vec![(String::from("height"), String::from("20dp"))].into_iter().collect(),
                children: vec![]
            }
        ],
    };
    face::run_face_window(doc);
}
