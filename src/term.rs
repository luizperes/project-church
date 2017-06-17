
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term
{
    // Basic Untyped Lambda Calculus
    App(Box<Term>, Box<Term>),
    Lam(String, Box<Term>),
    Var(String),
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

        let (x, y) = (Term::var("x"), Term::var("y"));
        Term::lam("x", Term::lam("y", enc(n, &x, &y)))
    }
}


macro_rules! app
{
    ($f:expr, $a:expr) => (Term::App(Box::new($f), Box::new($a)))
}

#[macro_export]
macro_rules! lam
{
    ($i:expr, $a:expr) => (Term::Lam($i.into(), Box::new($a)))
}

#[macro_export]
macro_rules! var
{
    ($i:expr) => (Term::Var($i.into()))
}


impl std::fmt::Display for Term
{
    // Recursively display the terms
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Term::App(ref tf, ref ta) => write!(f, "({} {})", tf, ta),
            Term::Lam(ref id, ref tm) => write!(f, "λ{}.{}", id, tm),
            Term::Var(ref id) => write!(f, "{}", id)
        }
    }
}


fn main()
{
    // Temporary output test
    println!("S = {}", lam!("x", lam!("y", lam!("z", app!(app!(var!("x"), var!("z")), app!(var!("y"), var!("z")))))));
    println!("K = {}", lam!("x", lam!("y", var!("x"))));
    println!("I = {}", lam!("x", var!("x")));
}
