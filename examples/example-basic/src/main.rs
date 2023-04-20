/// Prints greeting message.
#[fncmd::fncmd]
fn main(
	/// Greeting message.
	greeting: String,
	/// Name of someone to greet.
	name: Option<String>,
	/// Whether to use “!” instead of “.” at the end of the message.
	#[opt]
	bang: bool,
) {
	if let Some(name) = name {
		println!("{greeting}, {name}{}", if bang { "!" } else { "." });
	} else {
		println!("{greeting}{}", if bang { "!" } else { "." });
	}
}