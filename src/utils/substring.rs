use crate::error::Error;

#[derive(Debug)]
pub struct SubString<'a> {
    original_string: &'a str,
    start: usize,
    end: usize,
}

impl SubString<'_> {
    pub fn new(original_string: &str, start: usize, end: usize) -> Result<SubString, Error> {
        if start >= original_string.len() {
            return Err(Error::Message("the start position too large".to_string()));
        }
        if end >= original_string.len() {
            return Err(Error::Message("the end position too large".to_string()));
        }
        if start >= end {
            return Err(Error::Message(
                "the start position is larger than the end position".to_string(),
            ));
        }
        Ok(SubString {
            original_string,
            start,
            end,
        })
    }

    pub fn to_str(&self) -> &str {
        &self.original_string[self.start..self.end]
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn increase_end(&mut self, increment : usize) -> Result<(), Error> {
        if self.end + increment >= self.original_string.len() {
            return Err(Error::Message("the end position would become too large".to_string()));
        }
        self.end += increment;
        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ctor() {
        let s = "abcdefgh";
        let res = SubString::new(s, 1, 5);
        assert!(res.is_ok());
        let res_val = res.unwrap();
        assert_eq!(res_val.get_start(), 1);
        assert_eq!(res_val.get_end(), 5);
        assert_eq!(res_val.to_str(), "bcde");
    }

    #[test]
    fn test_ctor_bad_start_value() {
        assert!(SubString::new("ab", 3, 3).is_err());
    }

    #[test]
    fn test_ctor_bad_end_value() {
        assert!(SubString::new("ab", 1, 3).is_err());
    }

    #[test]
    fn test_ctor_bad_start_grater_than_end() {
        assert!(SubString::new("ab", 1, 0).is_err());
    }

    #[test]
    fn test_inc_end() {
        let s = "abcdefgh";
        let res = SubString::new(s, 1, 5);
        assert!(res.is_ok());
        let mut res_val = res.unwrap();
        assert!(res_val.increase_end(2).is_ok());
        assert_eq!(res_val.get_start(), 1);
        assert_eq!(res_val.get_end(), 7);
        assert_eq!(res_val.to_str(), "bcdefg");
    }

}
