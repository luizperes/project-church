
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

		let a = Term::var("a");
		let b = Term::var("b");
		Term::lam("a", Term::lam("b", enc(n, &a, &b)))
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
			Term::Lam(ref id, ref tm) => format!("(λ{}.{})", id, tm.to_string()),
			Term::Var(ref id) => format!("{}", id)
		}
	}
}

fn main()
{
	// Temporary tests
	let t = Term::app(Term::lam("i", Term::var("i")), Term::var("i"));
	println!("t = {}", t.to_string());
	let z = Term::nat(0);
	let o = Term::nat(1);
	let w = Term::nat(2);
	println!("z = {}", z.to_string());
	println!("o = {}", o.to_string());
	println!("w = {}", w.to_string());
}
