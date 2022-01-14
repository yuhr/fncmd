/// An intermediate type to abstract over various return types from subcommand
/// functions.
pub struct ExitCode(i32);

pub trait IntoExitCode {
	fn into_exit_code(self) -> ExitCode;
}

impl<T: std::process::Termination> IntoExitCode for T {
	fn into_exit_code(self) -> ExitCode {
		ExitCode(self.report())
	}
}

impl std::process::Termination for ExitCode {
	fn report(self) -> i32 {
		self.0
	}
}