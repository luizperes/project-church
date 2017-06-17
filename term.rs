
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
	pub fn nat(num: u32) -> Term
	{
		fn enc(n: u32, v1: Box<Term>, v2: Box<Term>) -> Term
		{
			match n
			{
				0 => *v1,
				_ => Term::app(*v2, enc(n - 1, v1, v2))
			}
		}

		let a: Term = Term::var("a");
		let b: Term = Term::var("b");
		Term::lam("a", Term::lam("b", enc(num, Box::new(a), Box::new(b))))
	}
}

impl ToString for Term
{
	// Recursively output the term
	fn to_string(&self) -> String
	{
		match *self
		{
			Term::App(ref tf, ref ta) => format!("({} {})", tf.to_string(), ta.to_string()),
			Term::Lam(ref id, ref tm) => format!("λ{}.{}", id, tm.to_string()),
			Term::Var(ref id) => format!("{}", id)
		}
	}
}

fn main()
{
	let t = Term::app(Term::lam("i", Term::var("i")), Term::var("i"));
	println!("t = {}", t.to_string());
}
