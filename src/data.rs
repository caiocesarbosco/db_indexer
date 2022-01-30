#[derive(PartialEq, Debug)]
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

