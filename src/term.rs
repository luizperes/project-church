
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term
{
    // Basic Untyped Lambda Calculus
    App(Box<Term>, Box<Term>),
    Lam(String, Box<Term>),
    Var(String),

    /*
    TODO:
    Let(String, Box<Term>, Box<Term>),

    // Church-encoded Numerals
    Nat(u32),
    Add(Box<Term>, Box<Term>),
    Sub(Box<Term>, Box<Term>),
    Mul(Box<Term>, Box<Term>),
    Div(Box<Term>, Box<Term>),
    Pow(Box<Term>, Box<Term>),
    */
}

impl Term
{
    // Create a new application node
    pub fn app(fun: Term, arg: Term) -> Term
    {
        Term::App(Box::new(fun), Box::new(arg))
    }

    // Create a new function (lambda) node
    pub fn lam<S: Into<String>>(id: S, trm: Term) -> Term
    {
        Term::Lam(id.into(), Box::new(trm))
    }

    // Create a new variable node
    pub fn var<S: Into<String>>(id: S) -> Term
    {
        Term::Var(id.into())
    }

    // Converts an unsigned int into church-encoded natural
    pub fn nat(n: u32) -> Term
    {
        fn enc(n: u32, v1: &Term, v2: &Term) -> Term
        {
            match n
            {
                0 => v1.clone(),
                _ => Term::app(v2.clone(), enc(n - 1, v1, v2))
            }
        }

        let (a, b) = (Term::var("a"), Term::var("b"));
        Term::lam("a", Term::lam("b", enc(n, &a, &b)))
    }
}

impl fmt::Display for Term
{
    // Recursively display the terms
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            Term::App(ref tf, ref ta) => write!(f, "({} {})", tf, ta),
            Term::Lam(ref id, ref tm) => write!(f, "(λ{}.{})", id, tm),
            Term::Var(ref id) => write!(f, "{}", id)
        }
    }
}

fn main()
{
    // Temporary output test
    println!("S = {}", Term::lam("x", Term::lam("y", Term::lam("z", Term::app(Term::app(Term::var("x"), Term::var("z")), Term::app(Term::var("y"), Term::var("z")))))));
    println!("K = {}", Term::lam("x", Term::lam("y", Term::var("x"))));
    println!("I = {}", Term::lam("x", Term::var("x")));
    
    println!("nat(0) = {}", Term::nat(0));
    println!("nat(1) = {}", Term::nat(1));
    println!("nat(2) = {}", Term::nat(2));
}
