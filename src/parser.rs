
use crate::model::expression::CCExpression;


fn tokenize(expr: &String) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    let mut start: usize = 0;
    let mut found: bool = false;
    for (idx, c) in expr.chars().enumerate() {
        if ['.', ':', '(', ')'].contains(&c) {
            if found {
                output.push(String::from(&expr[start..idx]));
                found = false;
            }
            output.push(String::from(c));
            start = idx + 1;
        }
        else if !found && !c.is_whitespace() {
            found = true;
            start = idx;
        }
        else if found && c.is_whitespace() {
            output.push(String::from(&expr[start..idx]));
            found = false;
        }
    }
    if found {
        output.push(String::from(&expr[start..]));
    }

    return output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_simple() {
        let tokens = tokenize(&String::from("xss sdg dsd"));
        assert_eq!(tokens, [
                   String::from("xss"),
                   String::from("sdg"),
                   String::from("dsd")
        ]);
    }

    #[test]
    fn tokenize_symbol1() {
        let tokens = tokenize(&String::from(" \\lambda x:A.B "));
        assert_eq!(tokens, [
                   String::from("\\lambda"),
                   String::from("x"),
                   String::from(":"),
                   String::from("A"),
                   String::from("."),
                   String::from("B")
        ]);
    }
}
