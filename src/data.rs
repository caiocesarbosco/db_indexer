use std::cmp::Ordering;
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Result, Formatter};


#[derive(Serialize, Deserialize, Ord, Eq, Debug, Clone)]
pub enum Data <'a>{
    StringVal(&'a str),
    NumberVal(i64)
}

impl<'a> Data<'a> {

    pub fn new_from_str(string_val: &'a str) -> Data {
        Data::StringVal(string_val)
    }

    pub fn new_from_number(number_val: i64) -> Data<'a> {
        Data::NumberVal(number_val)
    }
}

impl<'a> From<Data<'a>> for Vec<u8> {
    fn from(data: Data<'a>) -> Vec<u8> {
        let vec_data: Vec<u8>;
        match data {
            Data::StringVal(str_data) => vec_data = str_data.as_bytes().to_vec(),
            Data::NumberVal(number_data) => vec_data = number_data.to_be_bytes().to_vec(),
        }
        vec_data           
    }
}

impl <'a>PartialOrd for Data<'a> {
    fn partial_cmp(&self, other: &Data<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <'a>PartialEq for Data<'a> {
    fn eq(&self, other: &Data<'a>) -> bool {
        match (self, other) {
            (&Data::NumberVal(ref a), &Data::NumberVal(ref b)) => a == b,
            (&Data::StringVal(ref a), &Data::StringVal(ref b)) => a == b,
            _ => false,
        }
    }
}

impl <'a>Display for Data<'a> {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            &Data::NumberVal(ref a) => write!(f, "{}", a),
            &Data::StringVal(ref a) => write!(f, "{:?}", a),
        }
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_string_data_creation() {
        let string_val: &str = "My String Test";
        let data = Data::new_from_str(string_val);
        assert_eq!(data, Data::StringVal(string_val));
    }

    #[test]
    fn test_number_data_creation() {
        let number_val: i64 = 999;
        let data = Data::new_from_number(number_val);
        assert_eq!(data, Data::NumberVal(number_val));
    }

    #[test]
    fn test_convertion_for_byte_array() {
        let string_val: &str = "My String Test";
        let number_val: i64 = 999;
        let number_data = Data::new_from_number(number_val);
        let str_data = Data::new_from_str(string_val);
        let str_data_as_vec: Vec<u8> = str_data.into();
        let number_data_as_vec: Vec<u8> = number_data.into();
        assert_eq!(str_data_as_vec, Vec::from(string_val));
        assert_eq!(number_data_as_vec, Vec::from(number_val.to_be_bytes()));
    }

}

