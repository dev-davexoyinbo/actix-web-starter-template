use rand::{Rng, distr::Alphanumeric};

pub fn generate_random_string(length: u8) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
} // end function generate_random_string

pub fn generate_random_numeric_string(length: u8) -> String {
    let mut rng = rand::rng();
    (0..length)
        .map(|_| rng.random_range(0u8..=9u8).to_string())
        .collect()
} // end function generate_random_string

pub mod pagination {
    use std::borrow::Cow;

    use validator::ValidationError;

    pub fn default_pagination_page() -> u64 {
        1
    }

    pub fn default_pagination_per_page() -> u64 {
        20
    }

    pub fn default_pagination_sort_by() -> String {
        "created_at".to_string()
    }

    pub fn default_pagination_sort_direction() -> String {
        "desc".to_string()
    }

    pub fn validate_sort_direction(sort_direction: &str) -> Result<(), ValidationError> {
        let valid_fields = ["asc", "desc"];

        if valid_fields.contains(&sort_direction) {
            Ok(())
        } else {
            let fields = valid_fields.join(", ");

            let message = format!(
                "Invalid sort_by value: {}. Valid values are: {}",
                sort_direction, fields
            );

            Err(ValidationError::new("").with_message(Cow::Owned(message)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_string() {
        // Test a range of lengths, including edge cases
        for length in [0, 1, 5, 10, 50, 100].iter() {
            let random_string = generate_random_string(*length);

            // Check if the string has the correct length
            assert_eq!(
                random_string.len(),
                *length as usize,
                "Failed for length: {}",
                length
            );

            // Check if all characters are alphanumeric
            assert!(
                random_string.chars().all(|c| c.is_ascii_alphanumeric()),
                "String contains non-alphanumeric characters for length: {}",
                length
            );
        }
    }

    #[test]
    fn test_generate_random_numeric_string() {
        // Test a range of lengths, including edge cases
        for length in [0, 1, 5, 10, 50, 100].iter() {
            let random_numeric_string = generate_random_numeric_string(*length);

            // Check if the string has the correct length
            assert_eq!(
                random_numeric_string.len(),
                *length as usize,
                "Failed for length: {}",
                length
            );

            // Check if all characters are numeric
            assert!(
                random_numeric_string.chars().all(|c| c.is_ascii_digit()),
                "String contains non-numeric characters for length: {}",
                length
            );
        }
    }
}
