
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct ProdElim {}

fn find_products<'a>(context: &'a [Statement],
                     u_concs: &'a [Statement]) -> Vec<&'a Statement> {
    context.iter().chain(u_concs).filter(
        |x| x.s_type.is_arrow().is_none()
        ).filter_map(
            |x| match x.s_type {
                CCExpression::TypeAbs(_, _, _) => Some(x),
                _ => None
            }
            ).collect()
}

fn find_matches<'a>(prods: &[&Statement],
                    context: &'a [Statement],
                    u_concs: &'a [Statement]) -> Vec<(CCExpression, CCExpression, CCExpression)> {
    context.iter().chain(u_concs).map(|stmt| {
        prods.iter().filter_map(|prod| {
            match &prod.s_type {
                CCExpression::TypeAbs(arg, a_type, ret) => {
                    if **a_type == stmt.s_type {
                        Some((stmt.subject.clone(),
                        prod.subject.clone(),
                        ret.substitute(&arg, &stmt.subject)))
                    } else {
                        None
                    }
                },
                _ => None
            }
        })
    }).flatten().collect()
}

fn make_goal(arg: &CCExpression,
             p: &CCExpression,
             new_type: &CCExpression,
             ex: &CCExpression,
             context: &[Statement],
             inner_context: &[Statement],
             ) -> Goal {
    let new_term = CCExpression::Application(
        Box::new(p.clone()),
        Box::new(arg.clone()));
    let new_stmt = Statement {
        subject: new_term,
        s_type: new_type.clone()
    };
    let g_fin = Goal::Final(
        vec![Judgement {
            defs: vec![],
            context: [context, inner_context].concat(),
            statement: new_stmt
        }]);
    let g_init = Goal::Initial(ex.clone(), inner_context.to_vec());
    Goal::Unpacked(
        CCExpression::Var("sub_{1}".to_string()),
        ex.clone(),
        vec![g_fin, g_init],
        inner_context.to_vec()
        )
}

impl ProofStrat for ProdElim {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Judgement],
                 _: &[Definition]) -> Result<Vec<Goal>, String> {

        let full_context: Vec<Statement> = self.full_context(context, inner_context);
        let usable_conc: Vec<Statement> = self.usable_conc(&full_context, concs);
        let prods = find_products(&full_context, &usable_conc);
        let matches = find_matches(&prods, &full_context, &usable_conc);

        println!("matches: {:?}", matches);

        let goals: Vec<Goal> = matches.iter().map(
            |(arg, p, new_type)| make_goal(arg, p, new_type, ex, context, inner_context)
            ).collect();

        if goals.len() > 0 {
            Ok(goals)
        } else {
            Err("No appropriate products to instantiate".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn test_prod_elim_strat() {
        let jdg: Judgement = parse_judgement(
            "A:\\ast, x: \\prod Q:\\ast. Q \\to Q \\vdash y: A \\to A"
            ).unwrap();
        assert_eq!(jdg.context[1].to_latex(), "x : \\prod Q : \\ast . Q \\to Q");
        let strat = ProdElim {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Unpacked(inst, ex, subs, inner) = &lst[0] {
                    assert_eq!(inst.to_latex(), "sub_{1}");
                    assert_eq!(ex.to_latex(), "A \\to A");
                    assert_eq!(subs.len(), 2);
                    assert_eq!(inner.len(), 0);
                    if let Goal::Final(jdgs) = &subs[0] {
                        assert_eq!(jdgs[0].to_latex(), "A : \\ast, x : \\prod Q : \\ast . Q \\to Q \\vdash x A : A \\to A");
                    } else { panic!(); }
                    if let Goal::Initial(ex, _) = &subs[1] {
                        assert_eq!(ex.to_latex(), "A \\to A");
                    } else { panic!(); }
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}
