
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term
{
    // Basic Untyped Lambda Calculus
    App(Box<Term>, Box<Term>),
    Lam(String, Box<Term>),
    Var(String),
}


#[macro_export] // Create an application term
macro_rules! app {($f:expr,$a:expr) => (Term::App(Box::new($f),Box::new($a)))}

#[macro_export] // Create a lambda term
macro_rules! lam {($i:expr,$a:expr) => (Term::Lam($i.into(),Box::new($a)))}

#[macro_export] // Create a variable term
macro_rules! var {($i:expr) => (Term::Var($i.into()))}


impl Term
{
    // Safe function wrapper for the macros above
    pub fn app(lb: Term, ar: Term) -> Term {app!(lb, ar)}
    pub fn lam<S: Into<String>>(id: S, tm: Term) -> Term {lam!(id, tm)}
    pub fn var<S: Into<String>>(id: S) -> Term {var!(id)}

    // Create a church-encoded natural number from an unsigned integer
    pub fn nat(n: u32) -> Term
    {
        fn enc(n: u32, v1: &Term, v2: &Term) -> Term
        {
            match n
            {
                0 => v1.clone(),
                _ => app!(v2.clone(), enc(n - 1, v1, v2))
            }
        }

        let (x, y) = (var!("x"), var!("y"));
        lam!("x", lam!("y", enc(n, &x, &y)))
    }
}


impl std::fmt::Display for Term
{
    // Recursively display the terms
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Term::App(ref a, ref b) => write!(f, "({} {})", a, b),
            Term::Lam(ref a, ref b) => write!(f, "λ{}.{}", a, b),
            Term::Var(ref a) => write!(f, "{}", a)
        }
    }
}


fn main()
{
    // Temporary output test
    println!("S = {}", lam!("x",lam!("y", lam!("z", app!(app!(var!("x"), var!("z")), app!(var!("y"), var!("z")))))));
    println!("K = {}", lam!("x", lam!("y", var!("x"))));
    println!("I = {}", lam!("x", var!("x")));
}
