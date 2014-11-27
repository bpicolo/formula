use validator::{
    Validator,
};

pub struct Field<'a> {
    pub value: Option<String>,
    pub validators: Vec<Box<Validator + 'a>>,
    pub required: bool,
}


impl<'a> Field<'a> {

    pub fn new() -> Field<'a> {
        Field{value: None, validators: vec!(), required: false}
    }

    pub fn with_validators(v: Vec<Box<Validator + 'a>>,) -> Field<'a> {
        Field{value: None, validators: v, required: false}
    }

    pub fn set(&mut self, s: String) {
        self.value = Some(s)
    }

    pub fn validate(&self) -> bool {
        return self.matches_require() && self.matches_validators();
    }

    fn matches_require(&self) -> bool {
        return self.value.is_some() || !self.required;
    }

    fn matches_validators(&self) -> bool {
        return self.validators.iter().all(|v| v.validate(
            self.value.as_ref().unwrap()
        ));
    }

}


#[cfg(test)]
mod test {
    use validator::{Validator, Integer, Range};
    use super::{Field};

    #[test]
    fn test_field() {
        let mut field = Field {
            value: Some(String::from_str("10")),
            validators: vec!(box Integer as Box<Validator>, box Range{min: Some(7), max: Some(500)}),
            required: false,
        };
        assert!(field.validate());
        assert!(field.matches_validators());

        field.value = Some(String::from_str("6"));
        assert!(!field.validate());

        field.value = None;
        assert!(field.matches_require());

        field.required = true;
        assert!(!field.matches_require());
    }

}
