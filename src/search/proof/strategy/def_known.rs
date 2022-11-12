
use crate::model::statement::{Statement};
use crate::model::judgement::{Judgement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::partial::{Goal};
use super::base::{ProofStrat};

pub struct DefKnown {}

fn matching_args(context: &[Statement],
                 decided: &[Statement],
                 undecided: &[Statement]) -> Vec<Vec<Statement>> {
    if undecided.len() == 0 {
        vec![decided.to_vec()]
    } else {
        context.iter().filter(
            |stmt| stmt.s_type == undecided[0].s_type
        ).map(|stmt| {
            let new_decide: Vec<Statement> = [decided, &[stmt.clone()]].concat();
            let new_undecide: Vec<Statement> = undecided[1..].iter().map(
                |stmt1| Statement {
                    subject: stmt1.subject.clone(),
                    s_type: stmt1.s_type.substitute(
                        undecided[0].subject.var_str().as_ref().unwrap(),
                        &stmt.subject)
                }).collect();
            matching_args(context, &new_decide, &new_undecide)
        }).flatten().collect()
    }
}

fn match_single_def(def: &Definition,
                    context: &[Statement]) -> Vec<Vec<Statement>> {
    return matching_args(context, &[], &def.arg_statements());
}

fn find_matches(defs: &[Definition],
                context: &[Statement]) -> Vec<(Definition, Vec<Statement>)> {
    defs.iter().map(|def| {
        match_single_def(def, context).iter().map(
            |subs| (def.clone(), subs.to_vec())
            ).collect::<Vec<(Definition, Vec<Statement>)>>()
        }).flatten().collect()
}

fn type_already_known(new_type: &CCExpression,
                      context: &[Statement]) -> bool {
    context.iter().any(|stmt| stmt.s_type.alpha_equiv(new_type))
}

fn make_new_type(def: &Definition, subs: &[Statement]) -> CCExpression {
    let mut new_type = def.body.s_type.clone();
    subs.iter().zip(&def.args).for_each(
        |(sub, arg)| { new_type = new_type.substitute(arg, &sub.subject); });
    new_type
}


fn make_goal(ex: &CCExpression,
             context: &[Statement],
             inner_context: &[Statement],
             def: &Definition,
             subs: &[Statement]) -> Goal {
    let new_type = make_new_type(def, subs);

    let new_stmt = Statement {
        subject: CCExpression::Def(
                     def.name.to_string(),
                     subs.iter().map(|s| s.subject.clone()).collect()),
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

impl ProofStrat for DefKnown {
    fn sub_goals(&self, ex: &CCExpression,
                 context: &[Statement],
                 inner_context: &[Statement],
                 concs: &[Judgement],
                 defs: &[Definition]) -> Result<Vec<Goal>, String> {

        let full_context: Vec<Statement> = self.full_context(context, inner_context);
        let usable_conc: Vec<Statement> = self.usable_conc(&full_context, concs);
        let all_known: Vec<Statement> = [full_context, usable_conc].concat();
        let matches = find_matches(&defs, &all_known);

        println!("matches: {}", matches.iter().map(
                |(def, subs)| def.to_latex() + "\n\t" + &subs.iter().map(
                    |x| x.to_latex()).collect::<Vec<String>>().join("\n\t")
                ).collect::<Vec<String>>().join("\n"));

        let goals: Vec<Goal> = matches.iter().filter(
            |(def, subs)| !type_already_known(&make_new_type(&def, &subs), &all_known)
            ).map(
            |(def, subs)| make_goal(ex, context, inner_context, def, subs)
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
    use crate::parser::{parse_judgement, parse_definition};

    #[test]
    fn test_def_known_strat() {
        let jdg: Judgement = parse_judgement(
            "D:\\ast \\vdash y: \\neg \\neg D \\to D"
            ).unwrap();
        assert_eq!(jdg.context[0].to_latex(), "D : \\ast");
        let def = parse_definition("A : \\ast \\vartriangleright lem \\langle A \\rangle := \\independent : \\neg A \\vee A").unwrap();
        assert_eq!(def.body.to_latex(), "\\independent : \\neg A \\vee A");
        let strat = DefKnown {};
        let ex = &jdg.statement.s_type;
        let context = &jdg.context;
        let res = strat.sub_goals(ex, context, &[], &[], &[def]);

        match res {
            Ok(lst) => {
                assert_eq!(lst.len(), 1);
                if let Goal::Unpacked(inst, ex, subs, inner) = &lst[0] {
                    assert_eq!(inst.to_latex(), "sub_{1}");
                    assert_eq!(ex.to_latex(), "\\neg \\neg D \\to D");
                    assert_eq!(subs.len(), 2);
                    assert_eq!(inner.len(), 0);
                    if let Goal::Final(jdgs) = &subs[0] {
                        assert_eq!(jdgs[0].to_latex(), "D : \\ast \\vdash lem \\langle D \\rangle : \\neg D \\vee D");
                    } else { panic!(); }
                    if let Goal::Initial(ex, _) = &subs[1] {
                        assert_eq!(ex.to_latex(), "\\neg \\neg D \\to D");
                    } else { panic!(); }
                } else { panic!(); }
            },
            Err(_) => { panic!(); }
        }
    }
}
