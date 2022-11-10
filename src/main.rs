
mod util;
mod model;
mod parser;
mod type_check;
mod unpack_term;
mod find_term;
mod search;
mod serial;

use crate::parser::{parse};
use crate::model::proof::{Proof};
use crate::model::expression::{CCExpression};
use crate::model::statement::{Statement};
use crate::serial::{flagderiv_output};
use crate::find_term::{find_term};
use argparse::{ArgumentParser, StoreTrue, Store};


fn remove_dup<T: PartialEq+Clone>(lst: Vec<T>) -> Vec<T> {
    let mut output: Vec<T> = vec![];
    for item in lst {
        if !output.contains(&item) {
            output.push(item.clone());
        }
    }
    return output;
}

fn make_fake_context(ex: &CCExpression) -> Vec<Statement> {
    remove_dup(ex.free_var().iter().map(
        |x|
        Statement {
            subject: CCExpression::Var(x.to_string()),
            s_type: CCExpression::Star
        }).collect())
}

fn find_proof(expr: &str) -> Result<Proof, String> {
    let t0 = parse(expr);
    match t0 {
        Some(t1) => {
            let ctx = make_fake_context(&t1);
            find_term(&t1, &ctx, &[])
        },
        None => Err(format!("failed to parse: ({})", expr))
    }
}

fn main() {
    let mut flagderiv: bool = false;
    let mut expr = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Find term (LaTeX syntax)");
        ap.refer(&mut flagderiv)
            .add_option(&["--flagderiv"], StoreTrue,
            "Use flagderiv for proof typesetting");
        ap.refer(&mut expr)
            .add_argument("expr", Store,
            "Find a term for this type").required();
        ap.parse_args_or_exit();
    }

    let proof_r = find_proof(&expr);

    match proof_r {
        Ok(proof) => {
            if !flagderiv {
                println!("{}", proof.lines.last().unwrap().to_latex());
            } else {
                let fd_output = flagderiv_output(&proof);
                match fd_output {
                    Ok(p_str) => { println!("{}", p_str); },
                    Err(msg) => { eprintln!("{}", msg); },
                }
            }
        },
        Err(msg) => {
            eprintln!("{}", msg);
        }
    }
}
