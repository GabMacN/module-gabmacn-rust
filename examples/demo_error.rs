/*
*  .d8888b.           888      888b     d888                   888b    888
* d88P  Y88b          888      8888b   d8888                   8888b   888
* 888    888          888      88888b.d88888                   88888b  888
* 888         8888b.  88888b.  888Y88888P888  8888b.   .d8888b 888Y88b 888
* 888  88888     "88b 888 "88b 888 Y888P 888     "88b d88P"    888 Y88b888
* 888    888 .d888888 888  888 888  Y8P  888 .d888888 888      888  Y88888
* Y88b  d88P 888  888 888 d88P 888   "   888 888  888 Y88b.    888   Y8888
*  "Y8888P88 "Y888888 88888P"  888       888 "Y888888  "Y8888P 888    Y888

* All bugs and glitches proudly made by @GabMacN.

* @GabMacN 2026

* GitHub: GabMacN
* Discord: gabmacn
* Youtube: @GabMacN
* Instagram: @gabmacn

* Path: src/examples/demo_error.rs
* Name: demo_error.rs
* Description: Demonstrates the error handling and pretty printing features of the GabMacN Rust module.
*/

use gabmacn::errors::{GMNError, codes};

fn main() {
	println!("--- Testing Error Output ---");

	let err = GMNError::core(codes::GMNCoreErrorCode::Unknown, None, None);
	err.pretty_print();

	let custom_error_kind = gabmacn::errors::GMNErrorKind::Custom("GMN_CUSTOM_001");
	let custom_err = GMNError::custom(
		custom_error_kind,
		Some("Custom Error Title"),
		Some("A custom error description!"),
		Some("This is a custom error message."),
		Some("Some Context"),
	);

	custom_err.pretty_print();
}
