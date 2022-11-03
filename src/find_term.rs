
use crate::model::statement::{Statement};
use crate::model::expression::{CCExpression};
use crate::model::def::{Definition};
use crate::model::proof::{Proof};
use crate::model::partial::{Goal, PartialSol};
use crate::type_check::{check_proof};
use crate::unpack_term::{unpack_term};

use crate::search::proof_model::{ProofSearchModel};
use crate::search::control::{SearchControl};


fn do_search(partial: PartialSol,
             defs: &[Definition]) -> Result<PartialSol, String> {

    let control = SearchControl {
        model: Box::new(ProofSearchModel {
            defs: defs.to_vec()
        })
    };

    control.search(partial)
}

pub fn find_term(s_type: &CCExpression, context: &[Statement], defs: &[Definition]) -> Result<Proof, String> {
    let g1 = Goal::Initial(s_type.clone(), vec![]);
    let partial = PartialSol{
        context: context.to_vec(),
        goals: vec![g1],
    };
    let res = do_search(partial, defs);

    match res {
        Ok(out_partial) => {
            let lines_o = out_partial.goals.last().unwrap();
            if let Goal::Final(lines) = lines_o {
                let term: &CCExpression = &lines.last().unwrap().statement.subject;
                println!("found term: {}", term.to_latex());

                let full_lines = unpack_term(term, context, defs);
                if full_lines.is_err() {
                    return Err(full_lines.unwrap_err());
                }
                let refs_o = check_proof(&[], &full_lines.as_ref().unwrap());
                match refs_o {
                    Ok(refs) => Ok(Proof { lines: full_lines.as_ref().unwrap().to_vec(), refs: refs }),
                    Err(x) => {
                        println!("{}", lines.iter().map(|x| x.to_latex()).collect::<Vec<String>>().join("\n"));
                        Err(x)
                    }
                }
            } else {
                Err(format!("returned goal not final: {:?}", lines_o))
            }
        },
        Err(e) => Err(e)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};

    #[test]
    fn simple_find() {
        let jdg = parse_judgement("A : \\ast, x : A \\vdash x : A").unwrap();
        let t1 = jdg.statement.s_type.clone();
        let stmt1 = jdg.context[0].clone();
        let stmt2 = jdg.context[1].clone();
        let term = find_term(&t1, &[stmt1, stmt2], &[]);

        if let Ok(x) = term {
            assert_eq!(x.lines.last().unwrap().to_latex(), jdg.to_latex());
            let str_lines: Vec<String> = x.lines.iter().map(|x| x.to_latex()).collect();
            assert_eq!(str_lines,
                       ["\\vdash \\ast : \\square",
                       "A : \\ast \\vdash A : \\ast",
                       "A : \\ast, x : A \\vdash x : A"]);
        } else {
            println!("term not found: {:?}", term);
            panic!();
        }
    }

    #[test]
    fn find_on_easy_tautology() {
        let jdg = parse_judgement("A : \\ast \\vdash \\lambda a : A . a : A \\to A").unwrap();
        let t1 = jdg.statement.s_type.clone();
        let stmt1 = jdg.context[0].clone();
        let term = find_term(&t1, &[stmt1], &[]);

        if let Ok(x) = term {
            assert_eq!(x.lines.last().unwrap().to_latex(), jdg.to_latex());
            let str_lines: Vec<String> = x.lines.iter().map(|x| x.to_latex()).collect();
            assert_eq!(str_lines,
                       ["\\vdash \\ast : \\square",
                        "A : \\ast \\vdash A : \\ast",
                        "A : \\ast, a : A \\vdash a : A",
                        "A : \\ast, a : A \\vdash A : \\ast",
                        "A : \\ast \\vdash A \\to A : \\ast",
                        "A : \\ast \\vdash \\lambda a : A . a : A \\to A"
                       ]);
        } else {
            println!("term not found: {:?}", term);
            panic!();
        }
    }

    /*
    #[test]
    fn find_and_elim_no_def() {
        let jdg = parse_judgement("A : \\ast, B : \\ast \\vdash x : (A \\wedge B) \\to A").unwrap();
        let t1 = jdg.statement.s_type.clone();
        let stmt1 = jdg.context[0].clone();
        let term = find_term(&t1, &[stmt1], &[]);

        match term {
            Ok(proof) => {
                println!("proof good: {:?}", proof);
                assert_eq!(proof.lines.last().unwrap().to_latex(), "");
                panic!();
            },
            Err(msg) => {
                println!("{}", msg);
                panic!();
            }
        }
    }
    */
}
