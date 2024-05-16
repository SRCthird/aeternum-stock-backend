
pub fn validate_string(input: &Option<String>) -> bool {
    match input {
        Some(input) => {
            if input.contains(";") || input.contains("%") {
                return false;
            }
        }
        None => (),
    }
    true
}
