use imaperror::IMAPError;
use regex::bytes::Regex;
// use std::str;
use std::str;

pub fn validate_greeting(response: &[u8]) -> Result<(), IMAPError> {
    lazy_static! {
        static ref greeting_re: Regex = Regex::new("(?i)OK").unwrap();
    };
    if greeting_re.is_match(response) {
        Ok(())
    } else {
        Err(IMAPError::GreetingError(str::from_utf8(response).unwrap().to_owned()))
    }
}

pub fn validate_login(response: &[u8]) -> Result<(), IMAPError> {
    lazy_static! {
        static ref login_re: Regex = Regex::new("(?i)OK LOGIN").unwrap();
    };
    if login_re.is_match(response) {
        Ok(())
    } else {
        Err(IMAPError::GreetingError(str::from_utf8(response).unwrap().to_owned()))
    }
}

pub fn validate_select(response: &[u8]) -> Result<(), IMAPError> {
    lazy_static! {
        static ref select_re: Regex = Regex::new("(?i)OK SELECT").unwrap();
    };
    if select_re.is_match(response) {
        Ok(())
    } else {
        Err(IMAPError::GreetingError(str::from_utf8(response).unwrap().to_owned()))
    }
}

pub fn validate_logout(response: &[u8]) -> Result<(), IMAPError> {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_login() {
        let login_str = b"OK LOGIN";
        super::validate_login(&login_str[..]).unwrap();
        let login_str = b"ok login";
        super::validate_login(&login_str[..]).unwrap();
    }
}
