
use super::statement::{Statement};
use super::judgement::{Judgement};
use super::expression::{CCExpression};


enum Goal {
    Initial(CCExpression),
    Unpacked(CCExpression, Vec<Goal>),
    Final(Vec<Judgement>)
}

impl Goal {
    fn to_latex(&self) -> String {
        match self {
            Goal::Initial(ex) => format!("?? : {}", ex.to_latex()),
            Goal::Unpacked(ex, lst) => {
                lst.iter().map(|x| x.to_latex()
                               ).collect::<Vec<String>>().join("\n") + 
                    "\n" + &format!("?? : {}", ex.to_latex())
            },
            Goal::Final(lst) => {
                lst.iter().map(|x| x.to_latex()
                               ).collect::<Vec<String>>().join("\n")
            }
        }
    }
}

struct PartialSol {
    context: Vec<Statement>,
    goals: Vec<Goal>
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_to_latex() {
        let t1 = CCExpression::Var("A".to_string());
        let stmt1 = Statement {
            s_type: CCExpression::Star,
            subject: t1.clone() 
        };
        let g1 = Goal::Initial(CCExpression::TypeAbs("x".to_string(),
                                                     Box::new(t1.clone()),
                                                     Box::new(t1.clone())));
        assert_eq!(g1.to_latex(), "?? : \\prod x : A . A");
    }
}

