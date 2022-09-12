
use super::expression::{CCExpression};
use super::statement::{Statement};
use super::def::{Definition};
use std::collections::HashMap;


fn context_map(lhs: &[Statement], rhs: &[Statement]) -> Option<HashMap<String, String>> {
    let mut output: HashMap<String, String> = HashMap::new();
    if lhs.len() != rhs.len() {
        return None;
    }
    for (x, y) in lhs.iter().zip(rhs.iter()) {
        if x.s_type != y.s_type {
            return None;
        }
        if x.subject != y.subject {
            output.insert(x.subject.to_latex(), y.subject.to_latex());
        }
    }

    return Some(output);
}

#[derive(PartialEq,Debug,Clone)]
pub struct Judgement {
    pub defs: Vec<Definition>,
    pub context: Vec<Statement>,
    pub statement: Statement
}

impl Judgement {

    pub fn same_or_weaker(&self, rhs: &Judgement) -> bool {
        if self.statement != rhs.statement {
            return false;
        }
        return rhs.context.iter().all(|x| self.context.contains(x));
    }

    pub fn to_latex(&self) -> String {
        let output = self.context.iter().map(
                |x| x.to_latex()
            ).reduce(
                |a, b| a + ", " + &b
            );

        let stmt = String::from("\\vdash ") + &self.statement.to_latex(); 
           
        return match output {
            Some(x) => x + " " + &stmt,
            None => stmt
        };
    }

    pub fn alpha_equiv(&self, rhs: &Judgement) -> bool {
        let cm = context_map(&self.context, &rhs.context);
        if let Some(cmap) = cm {
            let mut rhs_sub = rhs.statement.subject.clone();
            let mut rhs_type = rhs.statement.s_type.clone();
            for (k, v) in &cmap {
                let new_var = CCExpression::Var(k.clone());
                rhs_sub = rhs_sub.substitute(v, &new_var);
                rhs_type = rhs_type.substitute(v, &new_var);
            }

            return self.statement.alpha_equiv(&Statement{
                subject: rhs_sub,
                s_type: rhs_type
            });
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_latex_simple_judgement() {
        let expr1 = CCExpression::Var(String::from("banana"));
        let expr2 = CCExpression::Var(String::from("A"));
        let stmt1 = Statement { subject: expr1, s_type: expr2 };
        let expr3 = CCExpression::Var(String::from("orange"));
        let expr4 = CCExpression::Var(String::from("B"));
        let stmt2 = Statement { subject: expr3, s_type: expr4 };
        let expr5 = CCExpression::Var(String::from("potato"));
        let expr6 = CCExpression::Var(String::from("C"));
        let stmt3 = Statement { subject: expr5, s_type: expr6 };
        let judge = Judgement {
            defs: vec![],
            context: vec![stmt1.clone(), stmt2],
            statement: stmt3.clone()
        };
        let judge2 = Judgement {
            defs: vec![],
            context: vec![stmt1],
            statement: stmt3
        };
        assert_eq!(judge.to_latex(), String::from(
                "banana : A, orange : B \\vdash potato : C"
                ));
        assert!(judge.same_or_weaker(&judge2));
        assert!(judge.same_or_weaker(&judge));
        assert!(judge2.same_or_weaker(&judge2));
        assert!(!judge2.same_or_weaker(&judge));
    }

    #[test]
    fn alpha_equiv_simple_judgement() {
        let expr1 = CCExpression::Var(String::from("A"));
        let expr2 = CCExpression::Star;
        let stmt1 = Statement { subject: expr1, s_type: expr2 };
        let expr3 = CCExpression::Var(String::from("B"));
        let expr4 = CCExpression::Star;
        let stmt2 = Statement { subject: expr3, s_type: expr4 };
        let jdg1 = Judgement {
            defs: vec![],
            context: vec![stmt1.clone()],
            statement: stmt1
        };
        let jdg2 = Judgement {
            defs: vec![],
            context: vec![stmt2.clone()],
            statement: stmt2
        };
        assert_eq!(jdg1.to_latex(), "A : \\ast \\vdash A : \\ast");
        assert_eq!(jdg2.to_latex(), "B : \\ast \\vdash B : \\ast");
        let cm = context_map(&jdg1.context, &jdg2.context);
        if let Some(cmap) = cm {
            assert_eq!(cmap.len(), 1);
            assert_eq!(cmap[&"A".to_string()], "B");
        } else {
            panic!();
        }
        assert!(jdg1.alpha_equiv(&jdg2));
        assert!(jdg2.alpha_equiv(&jdg1));
    }
}

