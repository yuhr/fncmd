pub struct Termination(i32);

impl<T: std::process::Termination> From<T> for Termination {
	fn from(t: T) -> Self {
		Termination(t.report())
	}
}