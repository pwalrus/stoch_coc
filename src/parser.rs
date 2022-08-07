
use crate::model::expression::CCExpression;

fn all_alpha_num(tokens: &Vec<String>) -> bool {
    let meta_token: Vec<String> = vec![
        String::from("."), 
        String::from(":"),
        String::from("("),
        String::from(")"),
        String::from("\\lambda"),
        String::from("\\prod")];
    let assessment: Option<bool> = tokens.into_iter().map(
        |t| !meta_token.contains(t)
        ).reduce(|a, b| a && b);
    if let Some(x) = assessment {
        return x;
    }
    return false
}

fn is_balanced(tokens: &Vec<String>) -> bool {
    let mut balance: i32 = 0;
    for token in tokens {
        if token == "(" {
            balance += 1;
        } else if token == ")" {
            balance -= 1;
        }
    }

    return balance == 0
}

fn tokenize(expr: &str) -> Vec<String> {
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

struct Consumed {
    expr: CCExpression,
    remain: Vec<String>
}

fn consume_var(tokens: &Vec<String>) -> Option<Consumed> {
    if all_alpha_num(&tokens[0..1].to_vec()) {
        return Some(Consumed { expr: CCExpression::Var(tokens[0].clone()), 
            remain: tokens[1..].to_vec()})
    } else {
        return None
    }

}


fn consume_expressions(tokens: &Vec<String>) -> Vec<CCExpression> {
    if tokens.len() == 0 {
        return vec![];
    } else if let Some(x) = consume_var(tokens) {
        let remain = consume_expressions(&x.remain);

        if x.remain.len() == 0 {
            return vec![x.expr];
        } else if remain.len() > 0 {
            return [vec![x.expr], remain].concat();
        }
    }
    return vec![]
}

fn find_expression(tokens: &Vec<String>) -> Option<CCExpression> {
    let exprs = consume_expressions(tokens);
    if exprs.len() == 0 {
        return None;
    }

    let mut output = exprs[0].clone();

    for expr in &exprs[1..] {
        output = CCExpression::Application(Box::new(output), 
                                           Box::new(expr.clone()));
    }
    return Some(output)
}

fn parse(expr: &str) -> Option<CCExpression> {
    let tokens = tokenize(expr);
    let candidates = find_expression(&tokens);
    return candidates
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

    #[test]
    fn parse_simple1() {
        //let tree = parse(&String::from("\\lambda x : A . y"));
        let tree = parse(&String::from("x y"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("x y"))
        }
    }

}
