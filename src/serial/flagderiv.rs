
use crate::model::proof::Proof;
use crate::model::statement::Statement;
use crate::model::proof::{LineRef};


fn valid_context_change(lhs: &[Statement], rhs: &[Statement]) -> bool {
    lhs.iter().zip(rhs).all(|(x,y)| x == y)
}

fn step(idx: i32, stmt: &Statement, rf: &LineRef) -> String {
    format!("\t\\step*{{$({})$}}{{{}}}{{{}}}", idx, stmt.to_latex(), rf.to_latex())
}

fn conclude(leap: i32, idx: i32, stmt: &Statement, rf: &LineRef) -> String {
    format!("\t\\conclude*[{}]{{$({})$}}{{{}}}{{{}}}", leap, idx, stmt.to_latex(), rf.to_latex())
}

fn assume(stmt: &Statement) -> String {
    format!("\t\\assume*{{}}{{{}}}{{}}", stmt.to_latex())
}

pub fn flagderiv_output(proof: &Proof) -> Result<String,String> {
    let s_token = "\\begin{flagderiv}".to_string();
    let e_token = "\\end{flagderiv}".to_string();
    let mut current_ctx: &Vec<Statement> = &vec![];
    let mut output: Vec<String> = vec![];
    for (idx, (jdg, rf)) in proof.lines.iter().zip(&proof.refs).enumerate() {
        if !valid_context_change(&current_ctx, &jdg.context) {
            return Err(format!("invalid context change: [{}] to [{}]", Statement::ctx_str(&current_ctx), Statement::ctx_str(&jdg.context)));
        }
        if current_ctx.len() < jdg.context.len() {
            for stmt in jdg.context[current_ctx.len()..].iter() {
                output.push(assume(&stmt));
            }
        }
        if current_ctx.len() > jdg.context.len() {
            output.push(conclude(current_ctx.len() as i32 - jdg.context.len() as i32,
            idx as i32, &jdg.statement, &rf));
        } else {
            output.push(step(idx as i32, &jdg.statement, &rf));
        }
        current_ctx = &jdg.context;
    }
    Ok(std::iter::once(s_token).chain(output).chain(std::iter::once(e_token)).collect::<Vec<String>>().join("\n"))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{parse_judgement};
    use crate::unpack_term::{unpack_term};
    use crate::type_check::{check_proof};

    #[test]
    fn simple_identity_proof() {
        let jdg = parse_judgement("A:\\ast \\vdash \\lambda x : A . x : A \\to A").unwrap();
        let lines = unpack_term(&jdg.statement.subject, &jdg.context, &[]).unwrap();
        let refs = check_proof(&[], &lines).unwrap();
        let proof = Proof { lines: lines, refs: refs };
        let output = flagderiv_output(&proof).unwrap();
        let str_lines: Vec<String> = output.split("\n").map(|x| x.trim().to_string()).collect();
        println!("{}", output);
        assert_eq!(str_lines, [
           "\\begin{flagderiv}",
           "\\step*{$(0)$}{\\ast : \\square}{sort}",
           "\\assume*{}{A : \\ast}{}",
           "\\step*{$(1)$}{A : \\ast}{var 0}",
           "\\assume*{}{a : A}{}",
           "\\step*{$(2)$}{a : A}{var 1}",
           "\\step*{$(3)$}{A : \\ast}{weak 1,1}",
           "\\conclude*[1]{$(4)$}{A \\to A : \\ast}{form 1,3}",
           "\\step*{$(5)$}{\\lambda x : A . x : A \\to A}{abst 2,4}",
           "\\end{flagderiv}"
        ]);
    }
}

