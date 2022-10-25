use std::ops;
use std::hash::{Hash, Hasher};
use super::statement::{Statement};
use super::judgement::{Judgement};
use super::expression::{CCExpression};

#[derive(Debug,PartialEq,Eq)]
pub struct GoalCount {
    pub i: u32,
    pub u: u32,
    pub f: u32,
}

impl ops::Add<GoalCount> for GoalCount {
    type Output = GoalCount;

    fn add(self, rhs: GoalCount) -> GoalCount {
        GoalCount {
            i: self.i + rhs.i,
            u: self.u + rhs.u,
            f: self.f + rhs.f
        }
    }
}

impl GoalCount {
    pub fn blank() -> GoalCount {
        GoalCount {i: 0, u: 0, f: 0}
    }
}

pub struct WithConc {
    pub conc: Vec<Statement>,
    pub goal: Goal
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Goal {
    Initial(CCExpression, Vec<Statement>),
    Unpacked(CCExpression, CCExpression, Vec<Goal>),
    Final(Vec<Judgement>)
}

impl Goal {
    pub fn to_latex(&self) -> String {
        match self {
            Goal::Initial(ex, _) => format!("?? : {}", ex.to_latex()),
            Goal::Unpacked(_, ex, lst) => {
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

    pub fn count(&self) -> GoalCount {
        match self {
            Goal::Initial(_, _) => GoalCount {i: 1, u: 0, f:0},
            Goal::Unpacked(_, _, lst) => {
                GoalCount {i: 0, u: 1, f:0} +
                    lst.iter().map(
                        |x| x.count()
                        ).fold(
                            GoalCount::blank(),
                            |a, b| a + b
                            )
            },
            Goal::Final(_) => GoalCount {i: 0, u: 0, f:1},
        }
    }

    pub fn replace(&self, old_g: &Goal, new_g: &Goal) -> Goal {
        match self {
            Goal::Unpacked(term, ex, lst) => {
                Goal::Unpacked(term.clone(), ex.clone(),
                lst.iter().map(
                    |x| if x == old_g { new_g.clone() }
                    else { x.replace(old_g, new_g) }
                    ).collect()
                )
            },
            _ => self.clone()
        }
    }

    pub fn active(&self, concs: &[Statement]) -> Vec<WithConc> {
        match self {
            Goal::Initial(_, _) => vec![WithConc{conc: concs.to_vec(), goal: self.clone()}],
            Goal::Unpacked(_, _, lst) => {
                lst.iter().map(|x| x.active(concs)).flatten().collect()
            },
            _ => vec![]
        }
    }
}

#[derive(Clone,PartialEq,Eq)]
pub struct PartialSol {
    pub context: Vec<Statement>,
    pub goals: Vec<Goal>
}

impl PartialSol {
    pub fn to_latex(&self) -> String {
        let c_str: String = self.context.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join(", ");
        let g_str: String = self.goals.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join("\n");
        return c_str + "\n" + &g_str;
    }

    pub fn count(&self) -> GoalCount {
        return self.goals.iter().map(
            |x| x.count()
            ).fold(
                GoalCount::blank(),
                |a, b| a + b
                );
    }

    pub fn replace(&self, old_g: &Goal, new_g: &Goal) -> PartialSol {
        PartialSol {
            context: self.context.clone(),
            goals: self.goals.iter().map(
                |x| if x == old_g { new_g.clone() }
                else { x.replace(old_g, new_g) }
                ).collect()
        }
    }

    pub fn active(&self) -> Vec<WithConc> {
        return self.goals.iter().map(
            |g| g.active(&[])
            ).flatten().collect();
    }
}

impl Hash for PartialSol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.goals.last().unwrap().to_latex().hash(state);
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
                                                     Box::new(t1.clone())),
                                                     vec![]);
        assert_eq!(g1.to_latex(), "?? : A \\to A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\n?? : A \\to A");
        assert_eq!(partial.count(), GoalCount {i: 1, u: 0, f:0});
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

        let g1 = Goal::Initial(t2.clone(), vec![]);
        let g2 = Goal::Unpacked(CCExpression::Var("x".to_string()),
                                t2.clone(), vec![
                                Goal::Initial(t1.clone(), vec![])
        ]);
        let g3 = g2.replace(&Goal::Initial(t1.clone(), vec![]), &Goal::Initial(t2.clone(), vec![]));
        assert_eq!(g1.to_latex(), "?? : A \\to A");
        assert_eq!(g2.to_latex(), "?? : A\n?? : A \\to A");
        assert_eq!(g3.to_latex(), "?? : A \\to A\n?? : A \\to A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g2]
        };
        let partial2 = partial.replace(&Goal::Initial(t1.clone(), vec![]), &Goal::Initial(t2.clone(), vec![]));
        assert_eq!(partial.to_latex(), "A : \\ast\n?? : A\n?? : A \\to A");
        assert_eq!(partial2.to_latex(), "A : \\ast\n?? : A \\to A\n?? : A \\to A");
        assert_eq!(partial.count(), GoalCount {i: 1, u: 1, f:0});
        assert_eq!(partial2.count(), GoalCount {i: 1, u: 1, f:0});
        let act = partial.active();
        assert_eq!(act.len(), 1);
        assert_eq!(act.last().unwrap().goal.to_latex(), "?? : A");
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
        assert_eq!(g1.to_latex(), "A : \\ast \\vdash \\lambda x : A . x : A \\to A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\nA : \\ast \\vdash \\lambda x : A . x : A \\to A");
        assert_eq!(partial.count(), GoalCount {i: 0, u: 0, f:1});
    }
}

