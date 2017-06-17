
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term
{
    // Basic Untyped Lambda Calculus
    App(Box<Term>, Box<Term>),
    Lam(String, Box<Term>),
    Var(String),
    
    //TODO: Let, LC types
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
	pub fn nat(num: uint) -> Term
	{
		fn enc(n: uint, v1: Term, v2: Term) -> Term
		{
			match n
			{
				0 => v1,
				_ => app(v2, enc(n - 1, v1, v2))
			}
		}

		lam("i", lam("j", enc(n, var("i"), var("j"))))
	}

	pub fn eval(self) -> Term
	{
		match self
		{
			//TODO
		}
	}
}
