use field::Field;

pub struct Input<'a> {
    field: Field<'a>,
    classes: Option<Vec<String>>,
    typ: String,
}

fn quoted(s: String) -> String {
    return String::from_str("\"") + s + String::from_str("\"");
}

impl<'a> Input<'a> {

    pub fn render(&self) -> String {
        let mut v: Vec<String> = Vec::new();
        v.push(String::from_str("<input type="));
        v.push(quoted(self.typ.clone()));
        v.push(String::from_str("></input>"));
        v.connect(" ")
    }

}

#[cfg(test)]
mod test {
    use validator::{Validator, Integer, Range};
    use super::Input;
    use field::Field;
    #[test]
    fn test_stuff() {
        let field = Field {
            value: None,
            validators: vec!(box Integer as Box<Validator>, box Range{min: Some(7), max: Some(500)}),
            required: false,
        };
        let input = Input{field:field, classes: None, typ: String::from_str("hidden")};
        println!("{}", input.render());
    }

}
