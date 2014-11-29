/*
    String validators
*/
pub struct Integer<'a>;
pub struct Float<'a>;
pub struct Range<'a> {
    pub min: Option<int>,
    pub max: Option<int>
}
pub struct Length<'a> {
    pub min: Option<uint>,
    pub max: Option<uint>
}
pub struct Regex<'a> {
    pub re: String,
}

pub trait Validator {
    fn validate(&self, s: &String) -> bool;
}


impl<'a> Validator for Integer<'a> {
    fn validate(&self, s: &String) -> bool {
        let val: Option<int> = from_str(s.as_slice());
        return val.is_some();
    }
}


impl<'a> Integer<'a> {
    pub fn foo(self) -> Box<Validator + 'a> {
        box self as Box<Validator>
    }
}


impl<'a> Validator for Float<'a> {
    fn validate(&self, s: &String) -> bool {
        let val = from_str::<f64>(s.as_slice());
        return val.is_some();
    }
}


impl<'a> Validator for Range<'a> {
    fn validate(&self, s: &String) -> bool {
        let val: Option<int> = from_str(s.as_slice());
        return (self.min.is_none() || self.min <= val) &&
            (self.max.is_none() || self.max >= val);
    }
}


// impl<'a> Validator for Regex<'a> {
//     fn validate(&self, s: &String) -> bool {
//     }
// }


impl<'a> Validator for Length<'a> {
    fn validate(&self, s: &String) -> bool {
        let val = Some(s.len());
        return (self.min.is_none() || self.min <= val) &&
            (self.max.is_none() || self.max >= val);
    }
}


#[cfg(test)]
mod test {

    use super::{Validator, Integer, Range, Float, Length};

    #[test]
    fn test_validate_integer() {
        let v = Integer;
        assert!(v.validate(&String::from_str("0")));
        assert!(v.validate(&String::from_str("1")));
        assert!(v.validate(&String::from_str("-1")));
        assert!(v.validate(&String::from_str("4294967295")));
        assert!(!v.validate(&String::from_str("1.0")));
        assert!(!v.validate(&String::from_str("1.4")));
        assert!(!v.validate(&String::from_str("Seven")));
        assert!(!v.validate(&String::from_str("")));
    }
    #[test]
    fn test_multiple_validate_number() {
        let v: Vec<Box<Validator>> = vec![box Integer as Box<Validator>, box Range{min: Some(5), max:Some(100)},];
        assert!(v.iter().all(|v| v.validate(&String::from_str("6"))));
        assert!(!v.iter().all(|v| v.validate(&String::from_str("5.5"))));
        assert!(!v.iter().all(|v| v.validate(&String::from_str("101"))));
    }

    #[test]
    fn test_validate_float() {
        let v = Float;
        assert!(v.validate(&String::from_str("0")));
        assert!(v.validate(&String::from_str("1")));
        assert!(v.validate(&String::from_str("-1")));
        assert!(v.validate(&String::from_str("1.0")));
        assert!(v.validate(&String::from_str("1.4")));
        assert!(v.validate(&String::from_str("4294967295")));
        assert!(!v.validate(&String::from_str("Seven")));
        assert!(!v.validate(&String::from_str("")));
    }

    #[test]
    fn test_validate_range() {
        let v = Range{min:Some(1), max:Some(5)}; //Hehe, someone
        assert!(v.validate(&String::from_str("1")));
        assert!(v.validate(&String::from_str("3")));
        assert!(v.validate(&String::from_str("5")));
        assert!(!v.validate(&String::from_str("0")));
        assert!(!v.validate(&String::from_str("6")));

        let v = Range{min: None, max: Some(10)};
        assert!(v.validate(&String::from_str("-1")));
        assert!(v.validate(&String::from_str("1")));
        assert!(v.validate(&String::from_str("3")));
        assert!(v.validate(&String::from_str("5")));
        assert!(!v.validate(&String::from_str("11")));

        let v = Range{min: None, max: None};
        assert!(v.validate(&String::from_str("-1")));
        assert!(v.validate(&String::from_str("1")));
        assert!(v.validate(&String::from_str("3")));
        assert!(v.validate(&String::from_str("5")));
        assert!(v.validate(&String::from_str("11")));
        assert!(v.validate(&String::from_str("4294967295")));
    }

    #[test]
    fn test_validate_length() {
        let v = Length{min: None, max: None};
        assert!(v.validate(&String::from_str("")));
        assert!(v.validate(&String::from_str("This doesn't matter =|")));

        let v = Length{min: Some(2), max: None};
        assert!(v.validate(&String::from_str("Tw")));
        assert!(!v.validate(&String::from_str("")));
        assert!(!v.validate(&String::from_str("1")));

        let v = Length{min: None, max: Some(3)};
        assert!(v.validate(&String::from_str("333")));
        assert!(v.validate(&String::from_str("22")));
        assert!(!v.validate(&String::from_str("But I love you ;_;")));
    }
}
