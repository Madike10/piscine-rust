pub use chrono::{Utc, NaiveDate};

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

#[derive(Debug)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}
impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError{
            form_values: (field_name,  field_value),
            date : Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}


impl Form {
    pub fn new(first_name: String, last_name: String, birth: NaiveDate, birth_location: String, password: String,) -> Form {
        Form{
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }
    
    pub fn validate(&self) -> Result<Vec<FormError>, FormError> {
        let mut form_output = Vec::new();
        if self.first_name.is_empty() {
            form_output.push(FormError{
                form_values: ("first_name".to_string(), self.first_name.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "No user name".to_string(),
            });
        }
        if self.password.len() < 8 {
            form_output.push(FormError {
                form_values: ("password".to_string(), self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "must be at least 8 characters".to_string(),
            });
        }
        if!self.password.chars().any(|c| c.is_digit(10)) ||!self.password.chars().any(|c| c.is_alphabetic()) ||!self.password.chars().any(|c|!c.is_alphanumeric()) {
            form_output.push(FormError {
                form_values: ("password".to_string(), self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)".to_string(),
            });
        }
    
        if form_output.is_empty() {
            Ok(Vec::new())
        } else {
            Err(form_output.remove(0)) // Assuming you want to return the first error encountered
        }
    }
}

