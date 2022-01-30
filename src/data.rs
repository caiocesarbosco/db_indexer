#[derive(PartialEq, Debug)]
pub struct Data <'a> {
    string_col: &'a str,
    number_col: i64
}

impl<'a> Data<'a> {

    pub fn new(string_val: &'a str, number_val: i64) -> Data {
        Data {
            string_col: string_val,
            number_col: number_val
        }
    }

    pub fn get_string_val(&self) -> &'a str {
        self.string_col
    }

    pub fn get_number_val(&self) -> i64 {
        self.number_col
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_data_creation() {
        let string_val: &str = "My String Test";
        let number_val: i64 = 999;
        let data = Data::new(string_val, number_val);
        assert_eq!(data, Data {string_col: string_val, number_col: number_val});
    }

    #[test]
    fn test_string_col_getter() {
        let string_val: &str = "My String Test";
        let number_val: i64 = 999;
        let data = Data::new(string_val, number_val);
        assert_eq!(data.get_string_val(), string_val);
    }    

    #[test]
    fn test_number_col_getter() {
        let string_val: &str = "My String Test";
        let number_val: i64 = 999;
        let data = Data::new(string_val, number_val);
        assert_eq!(data.get_number_val(), number_val);
    }

}

