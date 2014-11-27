use std::collections::HashMap;

use field::Field;
use html::{Html, render_html};

pub struct Input<'a> {
    pub attributes: HashMap<String, String>,
    pub field: Field<'a>,
}

impl<'a> Input<'a> {
    fn new() -> Input<'a> {
        Input{attributes: HashMap::new(), field: Field::new()}
    }

    pub fn with_field(f: Field<'a>) -> Input<'a> {
        Input{attributes: HashMap::new(), field: f}
    }
}


impl<'a> Html for Input<'a> {

    fn tag(&self) -> String {
        return String::from_str("input");
    }

    fn closing_tag(&self) -> bool {true}

    fn attrs(&self) -> String {
        let mut a = vec!();
        for (key, val) in self.attributes.iter() {
            a.push(
                key.clone() + String::from_str("=\"") + val.clone() + String::from_str("\"")
            );
        }
        a.connect(" ")
    }

    fn render(&self) -> String {
        render_html(self)
    }
}


#[cfg(test)]
mod test {
    use field::Field;
    use html::{Html, render_html};
    use std::collections::HashMap;
    use super::Input;
    use validator::{Validator, Integer, Range};

    #[test]
    fn test_empty_field() {
        let mut input = Input{field: Field::new(), attributes: HashMap::new()};
        input.attributes.insert(
            String::from_str("class"),
            String::from_str("some-classes and some-more-classes"),
        );
        input.attributes.insert(
            String::from_str("data-id"),
            String::from_str("123"),
        );
    }

}
