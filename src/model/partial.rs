
use super::statement::{Statement};
use super::judgement::{Judgement};
use super::expression::{CCExpression};

#[derive(Clone)]
enum Goal {
    Initial(CCExpression),
    Unpacked(CCExpression, Vec<Goal>),
    Final(Vec<Judgement>)
}

impl Goal {
    pub fn to_latex(&self) -> String {
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

impl PartialSol {
    pub fn to_latex(&self) -> String {
        let c_str: String = self.context.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join(", ");
        let g_str: String = self.goals.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join("\n");
        return c_str + "\n" + &g_str;
    }
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
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\n?? : \\prod x : A . A");
    }

    #[test]
    fn unpacked_to_latex() {
        let t1 = CCExpression::Var("A".to_string());
        let t2 = CCExpression::TypeAbs(
            "x".to_string(),
            Box::new(t1.clone()),
            Box::new(t1.clone()));
        let stmt1 = Statement {
            s_type: CCExpression::Star,
            subject: t1.clone() 
        };

        let g1 = Goal::Initial(t2.clone());
        let g2 = Goal::Unpacked(t2.clone(), vec![
                                Goal::Initial(t1.clone())
        ]);
        assert_eq!(g1.to_latex(), "?? : \\prod x : A . A");
        assert_eq!(g2.to_latex(), "?? : A\n?? : \\prod x : A . A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g2]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\n?? : A\n?? : \\prod x : A . A");
    }

    #[test]
    fn final_to_latex() {
        let x1 = CCExpression::Var("x".to_string());
        let t1 = CCExpression::Var("A".to_string());
        let x2 = CCExpression::Abs("x".to_string(),
            Box::new(t1.clone()),
            Box::new(x1.clone()));
        let t2 = CCExpression::TypeAbs(
            "x".to_string(),
            Box::new(t1.clone()),
            Box::new(t1.clone()));
        let stmt1 = Statement {
            s_type: CCExpression::Star,
            subject: t1.clone() 
        };
        let jdg = Judgement {
            defs: vec![],
            context: vec![stmt1.clone()],
            statement: Statement {
                s_type: t2.clone(),
                subject: x2.clone()
            }
        };

        let g1 = Goal::Final(vec![jdg]);
        assert_eq!(g1.to_latex(), "A : \\ast \\vdash \\lambda x : A . x : \\prod x : A . A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\nA : \\ast \\vdash \\lambda x : A . x : \\prod x : A . A");
    }
}

