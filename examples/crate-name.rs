/// Description of the command line tool
#[fncmd::fncmd]
pub fn main(
	/// Argument foo
	#[opt(short, long)]
	foo: String,
	/// Argument bar
	#[opt(short, long)]
	bar: Option<String>,
) {
	println!("{:?} {:?}", foo, bar);
}