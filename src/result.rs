use std::error::Error as StdError;
use std::result::Result as StdResult;

pub struct Result(StdResult<(), Box<dyn StdError>>);

impl From<Result> for () {
	#[inline]
	fn from(from: Result) -> Self {
		from.0.unwrap()
	}
}

impl From<()> for Result {
	#[inline]
	fn from(from: ()) -> Self {
		Result(Ok(from))
	}
}

impl<E: 'static + StdError> From<Result> for StdResult<(), E> {
	#[inline]
	fn from(from: Result) -> Self {
		from.0
			.map_err(|error| Box::<E>::into_inner(error.downcast().unwrap()))
	}
}

impl<E: 'static + StdError> From<StdResult<(), E>> for Result {
	#[inline]
	fn from(from: StdResult<(), E>) -> Self {
		Result(from.map_err(|error| error.into()))
	}
}