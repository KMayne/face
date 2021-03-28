use face;
use face::MarkupElement;

fn main() {
    let single_elem_doc = MarkupElement {
        node_name: String::from("imuroot"),
        number: 0,
        attributes: vec![(String::from("version"), String::from("0.0.0"))].into_iter().collect(),
        children: vec![
            MarkupElement {
                node_name: String::from("box"),
                number: 1,
                attributes: vec![(String::from("width"), String::from("1*"))].into_iter().collect(),
                children: vec![]
            }
        ],
    };
    face::run_face_window(single_elem_doc);
}
