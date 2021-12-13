use std::io::{self, BufRead, Write};

use lrlex::{lrlex_mod, DefaultLexeme};
use lrpar::{lrpar_mod, NonStreamingLexer, Span};

lrlex_mod!("calc.l");
lrpar_mod!("calc.y");

use calc_y::Expr;

fn main() {
    let lexerdef = calc_l::lexerdef();
    let stdin = io::stdin();
    loop {
        print!(">>> ");
        io::stdout().flush().ok();
        match stdin.lock().lines().next() {
            Some(Ok(ref l)) => {
                if l.trim().is_empty() {
                    continue;
                }
                let lexer = lexerdef.lexer(l);
                let (res, errs) = calc_y::parse(&lexer);
                for e in errs {
                    println!("{}", e.pp(&lexer, &calc_y::token_epp));
                }
                if let Some(Ok(r)) = res {
                    // We have a successful parse.
                    match eval(&lexer, r) {
                        Ok(i) => println!("Result: {}", i),
                        Err((span, msg)) => {
                            let ((line, col), _) = lexer.line_col(span);
                            eprintln!(
                                "Evaluation error at line {} column {}, '{}' {}.",
                                line,
                                col,
                                lexer.span_str(span),
                                msg
                            )
                        }
                    }
                }
            }
            _ => break,
        }
    }
}

fn eval(
    lexer: &dyn NonStreamingLexer<DefaultLexeme<u32>, u32>,
    e: Expr,
) -> Result<i64, (Span, &'static str)> {
    match e {
        Expr::Add { span, lhs, rhs } => eval(lexer, *lhs)?
            .checked_add(eval(lexer, *rhs)?)
            .ok_or((span, "overflowed")),

        Expr::Sub { span, lhs, rhs } => eval(lexer, *lhs)?
            .checked_sub(eval(lexer, *rhs)?)
            .ok_or((span, "overflowed")),

        Expr::Mul { span, lhs, rhs } => eval(lexer, *lhs)?
            .checked_mul(eval(lexer, *rhs)?)
            .ok_or((span, "overflowed")),

        Expr::Div { span, lhs, rhs } => eval(lexer, *lhs)?
            .checked_div(eval(lexer, *rhs)?)
            .ok_or((span, "overflowed")),

        Expr::Neg { span, lhs } => eval(lexer, *lhs)?.checked_neg().ok_or((span, "overflowed")),

        Expr::Number { span } => lexer
            .span_str(span)
            .parse::<i64>()
            .map_err(|_| (span, "cannot be represented as a i64")),
    }
}
