pub fn to_upper_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

pub fn capitalize_first_char(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let capitalized_first_char = first_char.to_uppercase().collect::<String>();
        let rest_of_string = input.chars().skip(1).collect::<String>();
        return capitalized_first_char + &rest_of_string;
    }
    input.to_string()
}