
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term
{
    // Basic Untyped Lambda Calculus
    App(Box<Term>, Box<Term>),
    Lam(Box<Term>),
    Var(u64),
}


#[macro_export] // Create an application term
macro_rules! app {($l:expr,$t:expr) => (Term::App(Box::new($l),Box::new($t)))}

#[macro_export] // Create a lambda term
macro_rules! lam {($t:expr) => (Term::Lam(Box::new($t)))}

#[macro_export] // Create a variable term
macro_rules! var {($i:expr) => (Term::Var($i))}


impl Term
{
    // Safe function wrappers for the macros above
    pub fn app(lb: Term, tm: Term) -> Term {app!(lb, tm)}
    pub fn lam(tm: Term) -> Term {lam!(tm)}
    pub fn var(id: u64) -> Term {var!(id)}

    // Create a church-encoded natural number from an unsigned integer
    pub fn nat(n: u64) -> Term
    {
        fn e(n: u64) -> Term {if n == 0 {var!(0)} else {app!(var!(1), e(n-1))}}
        lam!(lam!(e(n)))
    }
}


impl std::fmt::Display for Term
{
    // Recursively display the terms
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Term::App(ref lb, ref tm) => write!(f, "({} {})", lb, tm),
            Term::Lam(ref tm) => write!(f, "λ{}", tm),
            Term::Var(id) => write!(f, "{}", id)
        }
    }
}


fn main()
{
    // Temporary output test
    println!("S = {}", lam!(lam!(lam!(app!(app!(var!(2), var!(0)), app!(var!(1), var!(0)))))));
    println!("K = {}", lam!(lam!(var!(1))));
    println!("I = {}", lam!(var!(0)));
    println!("nat(0) = {}", Term::nat(0));
    println!("nat(1) = {}", Term::nat(1));
    println!("nat(2) = {}", Term::nat(2));
}
