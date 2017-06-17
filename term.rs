
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

		lam("a", lam("b", enc(n, var("a"), var("b"))))
	}

	pub fn eval(self) -> Term
	{
		match self
		{
			//TODO
		}
	}
}

impl ToString for Term
{
	// Recursively output the term
	pub fn to_string(&self) -> String
	{
		match self
		{
			Term::App(tf, ta) => format!("({} {})", tf.to_string(), ta.to_string()),
			Term::Lam(id, tm) => format!("λ{}.{}", id, tm.to_string()),
			Term::Var(id) => id
		}
	}
}
