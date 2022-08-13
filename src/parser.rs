
use crate::model::expression::CCExpression;
use crate::model::judgement::Statement;
use crate::model::judgement::Judgement;


fn all_alpha_num(tokens: &[String]) -> bool {
    let meta_token: Vec<String> = vec![
        String::from(","), 
        String::from("\\vdash"), 
        String::from("."), 
        String::from(":"),
        String::from("("),
        String::from(")"),
        String::from("\\lambda"),
        String::from("\\ast"),
        String::from("\\square"),
        String::from("\\prod")];
    let assessment: Option<bool> = tokens.into_iter().map(
        |t| !meta_token.contains(t)
        ).reduce(|a, b| a && b);
    if let Some(x) = assessment {
        return x;
    }
    return false
}

fn is_balanced(tokens: &[String]) -> bool {
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
        if ['.', ':', '(', ')', ','].contains(&c) {
            if found {
                output.push(String::from(&expr[start..idx]));
                found = false;
            }
            output.push(String::from(c));
            start = idx + 1;
        }
        else if c == '\\' {
            if found {
                output.push(String::from(&expr[start..idx]));
            }
            found = true;
            start = idx;
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

trait TokenConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed>;
}

struct VarConsumer {}

impl TokenConsumer for VarConsumer {

    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if all_alpha_num(&tokens[0..1]) {
            return Some(Consumed { 
                expr: CCExpression::Var(tokens[0].clone()), 
                remain: tokens[1..].to_vec()})
        } else {
            return None
        }
    }
}

struct ParenConsumer {}

impl TokenConsumer for ParenConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "(" {
            return None;
        }
        for (idx, token) in tokens.iter().enumerate() {
            if token == ")" && is_balanced(&tokens[0..idx+1]) {
                let inner = find_expression(&tokens[1..idx]);
                if let Some(x) = inner {
                    return Some(Consumed {
                        expr: x,
                        remain: tokens[idx+1..].to_vec()
                    });
                }
            }
        }

        return None
    }
}

struct StarConsumer {}

impl TokenConsumer for StarConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "\\ast" {
            return None
        }

        return Some(Consumed { 
            expr: CCExpression::Star, 
            remain: tokens[1..].to_vec()})
    }
}

struct SqConsumer {}

impl TokenConsumer for SqConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "\\square" {
            return None
        }
        return Some(Consumed { 
            expr: CCExpression::Sq,
            remain: tokens[1..].to_vec()})
    }
}

struct AbsConsumer {}

impl TokenConsumer for AbsConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() < 4 || 
            (tokens[0] != "\\lambda" && tokens[0] != "\\prod") {
            return None;
        }
        for (idx1, token1) in tokens.iter().enumerate() {
            if token1 == ":" && all_alpha_num(&tokens[1..idx1]) {
                for (idx2, token2) in tokens.iter().enumerate() {
                    if idx2 > idx1 + 1 && token2 == "." {
                        let type_expr = find_expression(
                            &tokens[idx1+1..idx2]);
                        let ret_expr = find_expression(&tokens[idx2+1..]);
                        if let Some(t) = type_expr {
                            if let Some(ret) = ret_expr {
                                if tokens[0] == "\\lambda" {
                                    return Some(Consumed {
                                        expr: CCExpression::Abs(
                                            tokens[1].clone(),
                                            Box::new(t),
                                            Box::new(ret)
                                        ),
                                        remain: vec![]
                                    });
                                } else {
                                    return Some(Consumed {
                                        expr: CCExpression::TypeAbs(
                                            tokens[1].clone(),
                                            Box::new(t),
                                            Box::new(ret)
                                        ),
                                        remain: vec![]
                                    });
                                }
                            }
                       }

                    }
                }
            }

        }

        return None
    }
}

fn consume_expressions(tokens: &[String]) -> Vec<CCExpression> {
    if tokens.len() == 0 {
        return vec![];
    }
    let consumers: Vec<&dyn TokenConsumer> = vec![
        &VarConsumer{},
        &ParenConsumer{},
        &StarConsumer{},
        &SqConsumer{},
        &AbsConsumer{}
    ];

    for consumer in consumers {
        if let Some(x) = consumer.consume(tokens) {
            let remain = consume_expressions(&x.remain);

            if x.remain.len() == 0 {
                return vec![x.expr];
            } else if remain.len() > 0 {
                return [vec![x.expr], remain].concat();
            }
        }
    }
    return vec![]
}

fn find_expression(tokens: &[String]) -> Option<CCExpression> {
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

fn find_statement(tokens: &[String]) -> Option<Statement> {
    for (idx, token) in tokens.iter().enumerate() {
        if token == ":" {
            let subject = find_expression(&tokens[0..idx]);
            let s_type = find_expression(&tokens[idx+1..]);
            if let Some(s) = subject {
                if let Some(t) = s_type {
                    return Some(Statement {
                        subject: s,
                        s_type: t
                    });
                }
            }
        }
    }
    return None
}

fn find_context(tokens: &[String]) -> Option<Vec<Statement>> {
    let mut output: Vec<Statement> = vec![];
    let mut last: usize = 0;

    for (idx, token) in tokens.iter().enumerate() {
        println!("token {}: {}", idx, token);
        if token == "," {
            println!("found comma at {}", idx);
            let stmt = find_statement(&tokens[last..idx]);
            if let Some(s) = stmt {
                println!("found statement: {} .. {}", &tokens[last], &tokens[idx-1]);
                output.push(s);
                last = idx + 1;
            }
        }
    }
    let stmt = find_statement(&tokens[last..]);
    println!("last = {}", last);
    if let Some(s) = stmt {
        output.push(s);
        return Some(output);
    } 

    return None
}


pub fn parse_statement(expr: &str) -> Option<Statement> {
    let tokens = tokenize(expr);
    return find_statement(&tokens);
}

pub fn parse(expr: &str) -> Option<CCExpression> {
    let tokens = tokenize(expr);
    let candidates = find_expression(&tokens);
    return candidates
}

pub fn parse_judgement(expr: &str) -> Option<Judgement> {
    let tokens = tokenize(expr);
    for (idx, token) in tokens.iter().enumerate() {
        if token == "\\vdash" {
            let context = find_context(&tokens[0..idx]);
            let statement = find_statement(&tokens[idx+1..]);
            if let Some(c) = context {
                println!("found context");
                if let Some(s) = statement {
                    return Some(Judgement {
                        context: c,
                        statement: s
                    });
                }
            }
        }
    }
    return None
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
    fn tokenize_symbol2() {
        let tokens = tokenize(&String::from(" x:A,y:B\\vdash z:C "));
        assert_eq!(tokens, [
                   String::from("x"),
                   String::from(":"),
                   String::from("A"),
                   String::from(","),
                   String::from("y"),
                   String::from(":"),
                   String::from("B"),
                   String::from("\\vdash"),
                   String::from("z"),
                   String::from(":"),
                   String::from("C")
        ]);
    }

    #[test]
    fn parse_simple1() {
        //let tree = parse(&String::from("\\lambda x : A . y"));
        let tree = parse(&String::from("x y"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("x y"));
            assert!(matches!(x, CCExpression::Application { .. }));
        }
    }

    #[test]
    fn parse_paren() {
        let tree = parse(&String::from("x (a b) y"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("x (a b) y"));
        }
    }

    #[test]
    fn parse_star() {
        let tree = parse(&String::from("\\ast"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\ast"));
            assert!(matches!(x, CCExpression::Star {..}));
        }
    }

    #[test]
    fn parse_sq() {
        let tree = parse(&String::from("\\square"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\square"));
            assert!(matches!(x, CCExpression::Sq {..}));
        }
    }

    #[test]
    fn parse_abs() {
        let tree = parse(&String::from("\\lambda x:A.y "));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\lambda x : A . y"));
            assert!(matches!(x, CCExpression::Abs {..}));
        }
    }

    #[test]
    fn parse_type_abs() {
        let tree = parse(&String::from("\\prod x:A.B "));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\prod x : A . B"));
            assert!(matches!(x, CCExpression::TypeAbs {..}));
        }
    }

    #[test]
    fn parse_statement1() {
        let tree = parse_statement(&String::from("\\lambda q: A. r : \\prod x:A.B "));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\lambda q : A . r : \\prod x : A . B"));
            assert!(matches!(x, Statement {..}));
        }
    }

    #[test]
    fn parse_judgement1() {
        let tree = parse_judgement(&String::from("x: A, y:B \\vdash x y : C"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("x : A, y : B \\vdash x y : C"));
            assert!(matches!(x, Judgement{..}));
        }
    }
}