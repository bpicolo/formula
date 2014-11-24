/*
    String validators
*/
use std::string::String;


pub struct Integer;
pub struct Range {
    min: Option<int>,
    max: Option<int>
}
pub struct Length {
    min: Option<uint>,
    max: Option<uint>
}

pub trait Validator {
    fn validate(&self, s: &String) -> bool;
}


impl Validator for Integer {
    fn validate(&self, s: &String) -> bool {
        let val: Option<int> = from_str(s.as_slice());
        return val.is_some();
    }
}


impl Validator for Range {
    fn validate(&self, s: &String) -> bool {
        let val: Option<int> = from_str(s.as_slice());
        return (self.min.is_none() || self.min <= val) &&
            (self.max.is_none() || self.max >= val);
    }
}


impl Validator for Length {
    fn validate(&self, s: &String) -> bool {
        let val = Some(s.len());
        return (self.min.is_none() || self.min <= val) &&
            (self.max.is_none() || self.max >= val);
    }
}


#[test]
fn test_validate_integer() {
    let v = Integer;
    assert!(v.validate(&String::from_str("0")));
    assert!(v.validate(&String::from_str("1")));
    assert!(v.validate(&String::from_str("-1")));
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
    assert!(!v.validate(&String::from_str("4444")));
}
