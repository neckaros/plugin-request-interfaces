use serde::Serialize;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
pub enum RequestError {

	UnableToParseCookieString(String, String)

}

// region:    --- Error Boilerplate

impl core::fmt::Display for RequestError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for RequestError {}