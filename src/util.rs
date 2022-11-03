

pub fn next_unused_var(used: &[String]) -> String {
    for ch in 'a'..'z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}

pub fn next_unused_cap_var(used: &[String]) -> String {
    for ch in 'A'..'Z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}

