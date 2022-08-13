use super::expression::CCExpression;
use super::judgement::{Judgement, Statement};


fn next_unused_var(context: &[Statement]) -> String {
    let used: Vec<String> = context.iter().filter_map(|stmt| {
        match &stmt.subject {
            CCExpression::Var(x) => Some(x.clone()),
            _ =>  None
        }
    }).collect();
    for ch in 'a'..'z' {
        if !used.contains(&ch.to_string()) {
            return ch.to_string();
        }
    }
    return String::from("x");
}


trait DerRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement>;
    fn name(&self) -> String;
}

struct SortRule {}

impl DerRule for SortRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement> {
        if let Some(_) = lhs { return None; }
        if let Some(_) = rhs { return None; }
        let stmt = Statement {
            subject: CCExpression::Star,
            s_type: CCExpression::Sq
        };
        return Some(Judgement {
            context: vec![],
            statement: stmt
        })
    }

    fn name(&self) -> String {
        return String::from("sort");
    }
    
}

struct VarRule {}

impl DerRule for VarRule {
    fn apply(&self, lhs: Option<Judgement>, rhs: Option<Judgement>) -> Option<Judgement> {
        if let Some(_) = rhs { return None; }
        if let Some(in_judge) = lhs {
            let stmt = &in_judge.statement;
            if let CCExpression::Star = &stmt.s_type {
                if let CCExpression::Var(_) = &stmt.subject {
                    let next = next_unused_var(&in_judge.context);
                    let new_stmt = Statement {
                        s_type: stmt.subject.clone(),
                        subject: CCExpression::Var(next) 
                    };
                    return Some(Judgement {
                        context: [
                            in_judge.context,
                            vec![new_stmt.clone()]].concat(),
                        statement: new_stmt
                    });
                }
            }
        }
        return None;
    }

    fn name(&self) -> String {
        return String::from("var");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_sort() {
        let rule = SortRule {};
        let stmt = Statement {
            subject: CCExpression::Star,
            s_type: CCExpression::Sq
        };
        let output = rule.apply(None, None);
        assert_eq!(rule.name(), "sort");
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(judge) = output {
            assert_eq!(judge.statement, stmt);
        } else {
            panic!();
        }
    }

    #[test]
    fn tokenize_var() {
        let rule = VarRule {};
        let stmt = Statement {
            subject: CCExpression::Var(String::from("A")),
            s_type: CCExpression::Star
        };
        let jdg = Judgement {
            context: vec![],
            statement: stmt
        };
        let output = rule.apply(Some(jdg), None);
        assert_eq!(rule.name(), "var");
        assert_ne!(output, None);
        assert!(matches!(output, Some(Judgement { .. })));
        if let Some(x) = output {
            assert_eq!(&x.to_latex(), "a : A \\vdash a : A");
        }
    }
}

