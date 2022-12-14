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

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct WithConc {
    pub conc: Vec<Judgement>,
    pub goal: Goal
}

#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Goal {
    Initial(CCExpression, Vec<Statement>),
    Unpacked(CCExpression, CCExpression, Vec<Goal>, Vec<Statement>),
    Final(Vec<Judgement>)
}

impl Goal {
    fn ctx_to_latex(&self, ctx: &[Statement]) -> String {
        if ctx.len() > 0 {
            format!(" (ctx: {})", ctx.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join(", "))
        } else {
            "".to_string()
        }
    }

    pub fn to_latex(&self) -> String {
        self.to_latex_ind(0)
    }

    pub fn to_latex_ind(&self, indent: u32) -> String {
        let ind: String = " ".repeat(indent as usize);
        match self {
            Goal::Initial(ex, ctx) => format!("{}?? : {}{}", ind, ex.to_latex(), self.ctx_to_latex(ctx)),
            Goal::Unpacked(_, ex, lst, _) => {
                lst.iter().map(|x| x.to_latex_ind(indent + 1)
                               ).collect::<Vec<String>>().join("\n") +
                    "\n" + &format!("{}?? : {}", ind, ex.to_latex())
            },
            Goal::Final(lst) => {
                lst.iter().map(|x| format!("{}{}", ind, x.to_latex())
                               ).collect::<Vec<String>>().join("\n")
            }
        }
    }

    pub fn count(&self) -> GoalCount {
        match self {
            Goal::Initial(_, _) => GoalCount {i: 1, u: 0, f:0},
            Goal::Unpacked(_, _, lst, _) => {
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
            Goal::Unpacked(term, ex, lst, ctx) => {
                Goal::Unpacked(term.clone(), ex.clone(),
                lst.iter().map(
                    |x| if x == old_g { new_g.clone() }
                    else { x.replace(old_g, new_g) }
                    ).collect(),
                    ctx.to_vec()
                )
            },
            _ => self.clone()
        }
    }

    pub fn active(&self, concs: &[Judgement]) -> Vec<WithConc> {
        match self {
            Goal::Initial(_, _) => vec![WithConc{conc: concs.to_vec(), goal: self.clone()}],
            Goal::Unpacked(_, _, lst, _) => {
                let mut accum: Vec<Judgement> = concs.to_vec();
                let mut blocks: Vec<WithConc> = vec![];
                for x in lst {
                    match x {
                        Goal::Final(jdgs) => {
                            accum = [accum, jdgs.to_vec()].concat();
                        },
                        _ => {
                            blocks = [blocks, x.active(&accum)].concat();
                        }
                    }
                }
                blocks
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
        let stmt2 = Statement {
            s_type: CCExpression::Var("A".to_string()),
            subject: CCExpression::Var("a".to_string())
        };
        let g1 = Goal::Initial(CCExpression::TypeAbs("x".to_string(),
                                                     Box::new(t1.clone()),
                                                     Box::new(t1.clone())),
                                                     vec![stmt2]);
        assert_eq!(g1.to_latex(), "?? : A \\to A (ctx: a : A)");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\n?? : A \\to A (ctx: a : A)");
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
        ], vec![]);
        let g3 = g2.replace(&Goal::Initial(t1.clone(), vec![]), &Goal::Initial(t2.clone(), vec![]));
        assert_eq!(g1.to_latex(), "?? : A \\to A");
        assert_eq!(g2.to_latex(), " ?? : A\n?? : A \\to A");
        assert_eq!(g3.to_latex(), " ?? : A \\to A\n?? : A \\to A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g2]
        };
        let partial2 = partial.replace(&Goal::Initial(t1.clone(), vec![]), &Goal::Initial(t2.clone(), vec![]));
        assert_eq!(partial.to_latex(), "A : \\ast\n ?? : A\n?? : A \\to A");
        assert_eq!(partial2.to_latex(), "A : \\ast\n ?? : A \\to A\n?? : A \\to A");
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

        let g1 = Goal::Final(vec![jdg.clone()]);
        assert_eq!(g1.to_latex(), "A : \\ast \\vdash \\lambda x : A . x : A \\to A");
        let partial = PartialSol{
            context: vec![stmt1],
            goals: vec![g1.clone()]
        };
        assert_eq!(partial.to_latex(), "A : \\ast\nA : \\ast \\vdash \\lambda x : A . x : A \\to A");
        assert_eq!(partial.count(), GoalCount {i: 0, u: 0, f:1});

        let g2 = Goal::Initial(CCExpression::Var("A".to_string()), vec![]);
        let g3 = Goal::Initial(CCExpression::Var("B".to_string()), vec![]);
        let g4 = Goal::Unpacked(CCExpression::Star,
                                CCExpression::Star,
                                vec![g2.clone(), g1, g3.clone()], vec![]);

        let active = g4.active(&[]);

        assert_eq!(active, [
                   WithConc { conc: vec![], goal: g2},
                   WithConc { conc: vec![jdg], goal: g3}
        ]);
    }
}

