use std::collections::HashMap;

use input::Input;


pub struct HtmlForm<'a> {
    pub inputs: HashMap<&'a str, Input<'a>>,
    _has_errors: bool,
    _validated: bool,
}


impl<'a> HtmlForm<'a> {
    pub fn new() -> HtmlForm<'a> {
        HtmlForm{inputs: HashMap::new(), _has_errors: false, _validated: false}
    }
}


pub trait Form {
    fn validate(&mut self) -> bool;
    fn has_errors(&mut self) -> bool;
}


impl<'a> Form for HtmlForm<'a> {
    fn validate(&mut self) -> bool {
        let is_valid = self.inputs.values().all(|input| input.field.validate());
        self._has_errors = !is_valid;
        is_valid
    }

    fn has_errors(&mut self) -> bool {
        if !self._validated {self.validate();}
        self._has_errors
    }
}


#[cfg(test)]
mod test {

    use validate::field::Field;
    use input::Input;
    use super::{Form, HtmlForm};
    use validate::validator::{Validator, Integer, Range, Length};

    #[test]
    fn test_a_form() {
        let mut f = HtmlForm::new();
        f.inputs.insert("name", Input::with_field(
            Field::with_validators(
                vec!(
                    box Length{min: Some(1), max: Some(10)} as Box<Validator>,
                )
            )
        ));
        f.inputs.insert("quantity", Input::with_field(
            Field::with_validators(
                vec!(
                    box Integer as Box<Validator>,
                    box Range{min: Some(5), max: Some(20)},
                )
            )
        ));

        // Test some valid things
        f.inputs.get_mut(&"name").unwrap().field.set(
            String::from_str("John")
        );
        f.inputs.get_mut(&"quantity").unwrap().field.set(
            String::from_str("10")
        );
        assert!(f.validate());

        f.inputs.get_mut(&"quantity").unwrap().field.set(
            String::from_str("40")
        );
        assert!(!f.validate());

        f.inputs.get_mut(&"quantity").unwrap().field.set(
            String::from_str("10")
        );
        f.inputs.get_mut(&"name").unwrap().field.set(
            String::from_str("My Name is Super Looong")
        );
        assert!(!f.validate());
    }

}
