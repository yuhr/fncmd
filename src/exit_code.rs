/// An intermediate type to abstract over various return types from subcommand
/// functions.
pub struct ExitCode(std::process::ExitCode);

pub trait IntoExitCode {
	fn into_exit_code(self) -> ExitCode;
}

impl<T: std::process::Termination> IntoExitCode for T {
	fn into_exit_code(self) -> ExitCode { ExitCode(self.report()) }
}

impl std::process::Termination for ExitCode {
	fn report(self) -> std::process::ExitCode { self.0 }
}