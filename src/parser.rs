
use crate::model::expression::CCExpression;
use crate::model::statement::Statement;
use crate::model::judgement::Judgement;
use crate::model::def::Definition;
use crate::util::{*};

fn all_alpha_num(tokens: &[String]) -> bool {
    let meta_token: Vec<String> = vec![
        String::from(","),
        String::from("\\vdash"),
        String::from("."),
        String::from(":"),
        String::from("("),
        String::from(")"),
        String::from("{"),
        String::from("}"),
        String::from("\\lambda"),
        String::from("\\ast"),
        String::from("\\square"),
        String::from("\\independent"),
        String::from("\\to"),
        String::from("\\langle"),
        String::from("\\rangle"),
        String::from("\\perp"),
        String::from("\\neg"),
        String::from("\\vee"),
        String::from("\\wedge"),
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
    return is_balanced_custom(tokens,
            &["(".to_string(), "\\langle".to_string()],
            &[")".to_string(), "\\rangle".to_string()]);
}

fn is_balanced_custom(tokens: &[String], left: &[String], right: &[String]) -> bool {
    let mut balance: i32 = 0;
    for token in tokens {
        if left.contains(token) {
            balance += 1;
        } else if right.contains(token) {
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
        if start > idx {
        } else if c == ':' && idx + 1 < expr.len() && &expr[idx..idx+2] == ":=" {
            if found {
                output.push(String::from(&expr[start..idx]));
                found = false;
            }
            output.push(String::from(":="));
            start = idx + 2;
        } else if expr[idx..].starts_with("=_{") {
            output.push(String::from("=_{"));
            let mut end_idx: usize = idx;
            for (idx2, c2) in expr[idx+3..].chars().enumerate() {
                if end_idx == idx {
                    let sub_tok = tokenize(&expr[idx+3..idx+3+idx2]);
                    if c2 == '}' && is_balanced_custom(
                        &expr[idx+3..idx+3+idx2].chars().map(|x| x.to_string()).collect::<Vec<String>>(),
                        &['{'.to_string()], &['}'.to_string()]) {
                        end_idx = idx2 + idx + 3;
                        output.extend(sub_tok);
                        output.push(String::from("}"));
                        start = end_idx + 1;
                    }
                }
            }
        } else if ['.', ':', '(', ')', ','].contains(&c) {
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

fn section_multi_delim<'a>(tokens: &'a[String], delims: &'a[String]) -> Vec<Vec<&'a[String]>> {
    let mut output = vec![];

    if delims.len() == 0 {
        let new_row = vec![tokens];
        output.push(new_row);
        return output;
    }

    for (idx, c) in tokens.iter().enumerate() {
        if c == &delims[0] {
            let sub_sol = section_multi_delim(&tokens[idx+1..], &delims[1..]);
            for sub in sub_sol {
                let new_row = [vec![&tokens[0..idx]], sub].concat();
                output.push(new_row);
            }
        }
    }

    return output;
}

fn comma_delim_expressions(tokens: &[String]) -> Option<Vec<CCExpression>> {
    let mut output = vec![];
    let mut last: usize = 0;
    for (idx, token) in tokens.iter().enumerate() {
        if idx >= last && token == "," {
            if let Some(expr) = find_expression(&tokens[last..idx]) {
                output.push(expr);
                last = idx + 1;
            }
        } else if idx >= last && idx == tokens.len() - 1 {
            if let Some(expr) = find_expression(&tokens[last..]) {
                output.push(expr);
                return Some(output);
            }
        }
    }

    return None;
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

struct PrimConsumer {}

impl TokenConsumer for PrimConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "\\independent" {
            return None
        }
        return Some(Consumed {
            expr: CCExpression::Prim,
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

struct ToConsumer {}

impl TokenConsumer for ToConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() > 2 {
            for (idx1, token1) in tokens.iter().enumerate() {
                if token1 == "\\to" {
                    let ante = find_expression(&tokens[0..idx1]);
                    let cnsq = find_expression(&tokens[idx1+1..]);
                    match (ante, cnsq) {
                        (Some(a), Some(c)) => {
                            let var = next_unused_var(&c.free_var());
                            let expr = CCExpression::TypeAbs(
                                var.to_string(),
                                Box::new(a.clone()),
                                Box::new(c.clone())
                                );
                            return Some(Consumed {
                                expr: expr,
                                remain: vec![]
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
        return None;
    }
}

struct DefConsumer {}

impl TokenConsumer for DefConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() <= 2 || tokens[1] != "\\langle" {
            return None
        }
        let delim = ["\\langle".to_string(), "\\rangle".to_string()];
        let sections = section_multi_delim(tokens, &delim);
        for section in sections {
            if section.len() >= 2 {
                let name = section[0][0].clone();
                let o_args = comma_delim_expressions(section[1]);
                if o_args.is_none() { return None; }
                let output = CCExpression::Def(
                    name,
                    o_args.unwrap()
                    );
                return Some(Consumed {
                    expr: output,
                    remain: if section.len() > 2 { section[2].to_vec() } else { vec![] }
                });
            }
        }

        return None;
    }
}

struct EqualsConsumer {}

impl EqualsConsumer {

    fn fab_arrow_type(lhs: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let absts = Statement::abstractions(rhs);
        let a_terms: Vec<String> = absts.iter().map(|stmt| stmt.subject.to_latex()).collect();
        let var = next_unused_var(&[rhs.free_var(), a_terms].concat());
        let expr = CCExpression::TypeAbs(
            var.to_string(),
            Box::new(lhs.clone()),
            Box::new(rhs.clone())
            );
        return expr;
    }

    fn fab_application_type(lhs: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let expr = CCExpression::Application(
            Box::new(lhs.clone()),
            Box::new(rhs.clone())
            );
        return expr;
    }

    fn fab_equality(lhs: &CCExpression, e_type: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let l_absts = Statement::abstractions(lhs);
        let l_terms: Vec<String> = l_absts.iter().map(|stmt| stmt.subject.to_latex()).collect();
        let r_absts = Statement::abstractions(rhs);
        let r_terms: Vec<String> = r_absts.iter().map(|stmt| stmt.subject.to_latex()).collect();
        let e_absts = Statement::abstractions(e_type);
        let e_terms: Vec<String> = e_absts.iter().map(|stmt| stmt.subject.to_latex()).collect();
        let var = next_unused_cap_var(&[lhs.free_var(), e_type.free_var(),
            rhs.free_var(), l_terms, e_terms, r_terms].concat());
        let prop_type = Self::fab_arrow_type(e_type, &CCExpression::Star);
        let l_prop = Self::fab_application_type(&CCExpression::Var(var.to_string()), lhs);
        let r_prop = Self::fab_application_type(&CCExpression::Var(var.to_string()), rhs);
        let implication = Self::fab_arrow_type(&l_prop, &r_prop);
        let expr = CCExpression::TypeAbs(
            var.to_string(),
            Box::new(prop_type),
            Box::new(implication)
            );
        return expr;
    }
}

impl TokenConsumer for EqualsConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        let delim = ["=_{".to_string(), "}".to_string()];
        let sections = section_multi_delim(tokens, &delim);
        for section in sections {
            if section.len() == 3 {
                let lhs = find_expression(&section[0]);
                let e_type = find_expression(&section[1]);
                let rhs = find_expression(&section[2]);
                match (lhs, e_type, rhs) {
                    (Some(l), Some(e), Some(r)) => {
                        return Some(Consumed {
                            expr: EqualsConsumer::fab_equality(&l, &e, &r),
                            remain: vec![]
                        });
                    },
                    (_, _, _) => {}
                }
            }
        }
        return None;
    }
}

struct PerpConsumer {}

impl TokenConsumer for PerpConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "\\perp" {
            return None;
        }
        return Some(Consumed {
            expr: CCExpression::TypeAbs(
                      "x".to_string(),
                      Box::new(CCExpression::Star),
                      Box::new(CCExpression::Var("x".to_string()))
                      ),
            remain: tokens[1..].to_vec()
        });
    }
}

struct NegConsumer {}

impl TokenConsumer for NegConsumer {
    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        if tokens.len() == 0 || tokens[0] != "\\neg" {
            return None;
        }
        for (idx, _) in tokens.iter().enumerate() {
            if idx >= 1 {
                if let Some(arg_expr) = find_expression(&tokens[1..idx+1]) {
                    let contra = CCExpression::TypeAbs(
                        "x".to_string(),
                        Box::new(CCExpression::Star),
                        Box::new(CCExpression::Var("x".to_string())));
                    let arrow = CCExpression::TypeAbs(
                        "x".to_string(),
                        Box::new(arg_expr),
                        Box::new(contra));
                    return Some(Consumed {
                        expr: arrow,
                        remain: tokens[idx+1..].to_vec()
                    });
                }
            }
        }
        return None;
    }
}


struct VeeWedgeConsumer {}

impl VeeWedgeConsumer {

    fn fab_arrow_type(lhs: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let absts = Statement::abstractions(rhs);
        let a_terms: Vec<String> = absts.iter().map(|stmt| stmt.subject.to_latex()).collect();
        let var = next_unused_var(&[rhs.free_var(), a_terms].concat());
        let expr = CCExpression::TypeAbs(
            var.to_string(),
            Box::new(lhs.clone()),
            Box::new(rhs.clone())
            );
        return expr;
    }

    fn fab_or_type(lhs: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let var = next_unused_cap_var(&[lhs.free_var(), rhs.free_var()].concat());
        let var_expr = CCExpression::Var(var.to_string());
        let first = VeeWedgeConsumer::fab_arrow_type(lhs, &var_expr);
        let second = VeeWedgeConsumer::fab_arrow_type(rhs, &var_expr);
        let mid = VeeWedgeConsumer::fab_arrow_type(&second, &var_expr);
        let last = VeeWedgeConsumer::fab_arrow_type(&first, &mid);

        let expr = CCExpression::TypeAbs(
            var.to_string(),
            Box::new(CCExpression::Star),
            Box::new(last)
            );
        return expr;
    }

    fn fab_and_type(lhs: &CCExpression, rhs: &CCExpression) -> CCExpression {
        let var = next_unused_cap_var(&[lhs.free_var(), rhs.free_var()].concat());
        let var_expr = CCExpression::Var(var.to_string());
        let second = VeeWedgeConsumer::fab_arrow_type(rhs, &var_expr);
        let first = VeeWedgeConsumer::fab_arrow_type(lhs, &second);
        let last = VeeWedgeConsumer::fab_arrow_type(&first, &var_expr);

        let expr = CCExpression::TypeAbs(
            var.to_string(),
            Box::new(CCExpression::Star),
            Box::new(last)
            );
        return expr;
    }
}

impl TokenConsumer for VeeWedgeConsumer {

    fn consume(&self, tokens: &[String]) -> Option<Consumed> {
        let delims = [["\\vee".to_string()], ["\\wedge".to_string()]];
        if tokens.len() > 2 {
            for delim in delims {
                let sections = section_multi_delim(tokens, &delim);
                for section in sections {
                    if section.len() == 2 {
                        let lhs = find_expression(&section[0]);
                        let rhs = find_expression(&section[1]);
                        match (lhs, rhs) {
                            (Some(a), Some(c)) => {
                                return Some(Consumed {
                                    expr: if delim[0] == "\\vee" {
                                        VeeWedgeConsumer::fab_or_type(&a, &c)
                                    } else {
                                        VeeWedgeConsumer::fab_and_type(&a, &c)
                                    },
                                    remain: vec![]
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        return None;
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
        &PrimConsumer{},
        &DefConsumer{},
        &PerpConsumer{},
        &NegConsumer{},
        &VeeWedgeConsumer{},
        &AbsConsumer{},
        &ToConsumer{},
        &EqualsConsumer{}
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
            match (subject, s_type) {
                (Some(s), Some(t)) => {
                    return Some(Statement {
                        subject: s,
                        s_type: t
                    });
                },
                _ => {}
            }
        }
    }
    return None
}

fn find_def_name(tokens: &[String]) -> Option<(String, Vec<String>)> {
    let mut args: Vec<String> = vec![];
    let mut last: usize = 0;
    let mut name: Option<String> = None;

    if !is_balanced(tokens) { return None; }
    if tokens.len() > 2 && ["(".to_string(), "\\langle".to_string()].contains(&tokens[1]) {
            name = Some(tokens[0].clone());
            last = 2;
    }

    for (idx, token) in tokens.iter().enumerate() {
        if idx >= last && token == "," {
            args.push(tokens[last..idx].join(" "));
            last = idx + 1;
        } else if idx >= last && [")".to_string(), "\\rangle".to_string()].contains(token) {
            args.push(tokens[last..idx].join(" "));
            last = tokens.len();
        } else if idx >= last {
            if !all_alpha_num(&tokens[idx..idx+1]) {
                return None;
            }
        }
    }

    return Some((name.unwrap(), args));
}

fn find_definition(tokens: &[String]) -> Option<Definition> {
    for (idx1, token1) in tokens.iter().enumerate() {
        if token1 == "\\vartriangleright" {
            let ctx = find_context(&tokens[0..idx1]);
            if let Some(c) = ctx {
                for (idx2, token2) in tokens.iter().enumerate() {
                    if token2 == ":=" {
                        let def_name = find_def_name(&tokens[idx1+1..idx2]);
                        let body = find_statement(&tokens[idx2+1..]);

                        match (def_name, body) {
                            (Some(d), Some(b)) => {
                                return Some(Definition {
                                    context: c,
                                    name: d.0,
                                    args: d.1,
                                    body: b
                                });
                            }
                            _ => {}
                        }
                    }
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
        if token == "," {
            let stmt = find_statement(&tokens[last..idx]);
            if let Some(s) = stmt {
                output.push(s);
                last = idx + 1;
            }
        }
    }
    let stmt = find_statement(&tokens[last..]);
    if let Some(s) = stmt {
        output.push(s);
        return Some(output);
    }

    return None
}

pub fn parse_definition(expr: &str) -> Option<Definition> {
    let tokens = tokenize(expr);
    return find_definition(&tokens);
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
            if let Some(s) = statement {
                if let Some(c) = context {
                    return Some(Judgement {
                        defs: vec![],
                        context: c,
                        statement: s
                    });
                } else if idx == 0 {
                    return Some(Judgement {
                        defs: vec![],
                        context: vec![],
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
    fn tokenize_section() {
        let tokens = tokenize(&String::from(" \\lambda x:A y.B x "));
        let delim = ["\\lambda".to_string(), ":".to_string(), ".".to_string()];
        let sections = section_multi_delim(&tokens, &delim);
        assert_eq!(sections, vec![vec![
                   vec![],
                   vec!["x".to_string()],
                   vec!["A".to_string(), "y".to_string()],
                   vec!["B".to_string(), "x".to_string()]
        ]]);
    }

    #[test]
    fn tokenize_section2() {
        let tokens = tokenize(&String::from(" A \\vee B \\vee C "));
        let delim = ["\\vee".to_string()];
        let sections = section_multi_delim(&tokens, &delim);
        assert_eq!(sections, vec![
                   vec![
                   vec!["A".to_string()],
                   vec!["B".to_string(), "\\vee".to_string(), "C".to_string()],
                   ],
                   vec![
                   vec!["A".to_string(), "\\vee".to_string(), "B".to_string()],
                   vec!["C".to_string()],
        ]]);
    }

    #[test]
    fn tokenize_equality() {
        let tokens = tokenize(&String::from(" x =_{A_{2}} y "));
        assert_eq!(tokens, [
                   String::from("x"),
                   String::from("=_{"),
                   String::from("A_{2}"),
                   String::from("}"),
                   String::from("y")
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
    fn parse_prim() {
        let tree = parse(&String::from("\\independent"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\independent"));
            assert!(matches!(x, CCExpression::Prim {..}));
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
            assert_eq!(x.to_latex(), "A \\to B");
            assert!(matches!(x, CCExpression::TypeAbs {..}));
        }
    }

    #[test]
    fn parse_statement1() {
        let tree = parse_statement(&String::from("\\lambda q: A. r : \\prod x:A.B "));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\lambda q : A . r : A \\to B"));
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

    #[test]
    fn parse_judgement2() {
        let tree = parse_judgement(&String::from("\\vdash \\ast : \\square"));
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), String::from("\\vdash \\ast : \\square"));
            assert!(matches!(x, Judgement{..}));
        }
    }

    #[test]
    fn parse_definition() {
        let def1 = "x : A \\vartriangleright ex \\langle x \\rangle := x : A";
        let tokens = tokenize(&def1);
        assert_eq!(tokens, vec![
                   "x", ":", "A", "\\vartriangleright",
                   "ex", "\\langle", "x", "\\rangle", ":=", "x", ":", "A"
        ]);
        let tree = find_definition(&tokens);
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), def1);
        }
    }

    #[test]
    fn parse_expr_definition() {
        let def1 = "ex \\langle a c, b \\rangle x";
        let tokens = tokenize(&def1);
        assert_eq!(tokens, vec![
                   "ex", "\\langle", "a", "c", ",", "b", "\\rangle", "x"
        ]);
        let tree = find_expression(&tokens);
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), def1);
        }
    }

    #[test]
    fn parse_to_arrow() {
        let expr = "A \\to B a";
        let tokens = tokenize(&expr);
        assert_eq!(tokens, vec![
                   "A", "\\to", "B", "a",
        ]);
        let tree = parse(&expr);
        assert_ne!(tree, None);
        if let Some(x) = tree {
            assert_eq!(x.to_latex(), "A \\to B a");
        }
    }

    #[test]
    fn arrow_bracket_conventions() {
        let samples = [
            "A \\to B",
            "A \\to B \\to C",
            "(A \\to B) \\to C",
            "A \\to B \\to C \\to D",
            "A \\to (B \\to C) \\to D"
        ];
        for s in samples {
            assert_eq!(parse(&s).unwrap().to_latex(), s);
        }
    }

    #[test]
    fn perp_bracket_conventions() {
        let samples = [
            "\\perp",
            "\\perp A",
            "A \\perp",
            "\\perp \\to A",
            "\\neg A",
            "\\neg A B",
            "\\neg (A B)"
        ];
        for s in samples {
            assert_eq!(parse(&s).unwrap().to_latex(), s);
        }
    }

    #[test]
    fn vee_wedge_bracket_conventions() {
        let samples = [
            "A \\vee B",
            "A \\vee B \\vee C",
            "A \\vee (B \\wedge C)",
            "A \\wedge B",
            "A \\wedge B \\wedge C",
            "(A \\vee B) \\wedge C",
            "A \\vee (B \\wedge C)",
        ];
        for s in samples {
            assert_eq!(parse(&s).unwrap().to_latex(), s);
        }
    }

    #[test]
    fn parse_equality() {
        let expr = "x =_{A} y";
        let expr2 = "\\prod P : A \\to \\ast . (P x) \\to (P y)";
        let tokens = tokenize(&expr);
        let tree = find_expression(&tokens);
        let tokens2 = tokenize(&expr2);
        let tree2 = find_expression(&tokens2);
        assert_ne!(tree, None);
        assert_ne!(tree2, None);
        assert!(tree.unwrap().alpha_equiv(&tree2.unwrap()));
    }
}
