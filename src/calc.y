%start Expr
%avoid_insert "INT"
%%
Expr -> Result<Expr, ()>:
      Expr '+' Term { Ok(Expr::Add{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Expr '-' Term { Ok(Expr::Sub{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term { $1 }
    ;

Term -> Result<Expr, ()>:
      Term '*' Factor { Ok(Expr::Mul{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Term '/' Factor { Ok(Expr::Div{ span: $span, lhs: Box::new($1?), rhs: Box::new($3?) }) }
    | Factor { $1 }
    ;

Factor -> Result<Expr, ()>:
      '(' Expr ')' { $2 }
    | '-' Factor { Ok(Expr::Neg{ span: $span, lhs: Box::new($2?) }) }
    | 'INT' { Ok(Expr::Number{ span: $span }) }
    ;

Unmatched -> ():
      "UNMATCHED" { }
    ;
%%

use lrpar::Span;

#[derive(Debug)]
pub enum Expr {
    Add {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Sub {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Mul {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Div {
        span: Span,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Neg {
        span: Span,
        lhs: Box<Expr>,
    },
    Number {
        span: Span
    }
}