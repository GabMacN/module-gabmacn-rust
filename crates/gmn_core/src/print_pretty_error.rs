//! Pretty terminal message rendering utilities for errors and status output.
//!
//! This module provides a small, focused API for rendering structured terminal messages
//! with a consistent visual style (borders, colors, icons, timestamp, and optional
//! sections like context/hints).
//!
//! It is intended for human-facing CLI output where readability matters.
//!
//! # What this module gives you
//!
//! - A single generic entry point: [`print_pretty_message`]
//! - Convenience wrappers by level:
//!   - [`print_pretty_error`]
//!   - [`print_pretty_warning`]
//!   - [`print_pretty_info`]
//!   - [`print_pretty_success`]
//!   - [`print_pretty_input`]
//! - A string-rendering function for tests/snapshots/log piping:
//!   - [`pretty_message_to_string`]
//!
//! # Message model
//!
//! Every rendered message is composed of:
//!
//! - **level**: visual profile (icon + color + label)
//! - **title**: short, high-signal headline
//! - **code**: stable short identifier (`AUTH-401`, `DB-003`, etc.)
//! - **message**: primary body text
//! - **context** *(optional)*: additional surrounding detail
//! - **hint** *(optional)*: actionable next step for the user
//! - **location** *(optional)*: source/function/path indicator
//!
//! # Output behavior
//!
//! - Printed output goes to **stderr** (not stdout).
//! - Width is auto-computed and constrained by terminal width when available.
//! - Content is wrapped to stay readable in narrow terminals.
//! - ANSI styling is used for terminals that support color.
//! - The printing API intentionally does not return an error; rendering failures are
//!   treated as best-effort display concerns.
//!
//! # Usage examples
//!
//! Basic error:
//!
//! ```rust,no_run
//! use gmn_core::print_pretty_error::print_pretty_error;
//!
//! print_pretty_error(
//!     "Authentication Failed",
//!     "AUTH-401",
//!     "The provided token is invalid or expired.",
//!     Some("Attempted to access protected route: /v1/account/profile"),
//!     Some("Refresh the token and retry the request."),
//!     Some("auth::middleware::verify_token"),
//! );
//! ```
//!
//! Generic level-driven message:
//!
//! ```rust,no_run
//! use gmn_core::print_pretty_error::{print_pretty_message, PrettyMessageLevel};
//!
//! print_pretty_message(
//!     PrettyMessageLevel::Info,
//!     "Cache Warmup",
//!     "CACHE-INIT",
//!     "Preloading 42 templates into memory.",
//!     None,
//!     None,
//!     Some("startup::cache"),
//! );
//! ```
//!
//! Render to string for testing/snapshots:
//!
//! ```rust
//! use gmn_core::print_pretty_error::{pretty_message_to_string, PrettyMessageLevel};
//!
//! let rendered = pretty_message_to_string(
//!     PrettyMessageLevel::Success,
//!     "Build Finished",
//!     "BUILD-OK",
//!     "All artifacts were generated successfully.",
//!     None,
//!     Some("Run `cargo test` to validate behavior."),
//!     Some("ci::build"),
//! ).expect("message should render");
//!
//! assert!(rendered.contains("SUCCESS"));
//! assert!(rendered.contains("BUILD-OK"));
//! ```
//!
//! # Choosing good message values
//!
//! - Keep **title** short (`"Validation Failed"`), not paragraph-length.
//! - Keep **code** stable and machine-searchable.
//! - Keep **message** user-centered and specific.
//! - Use **context** for situational details.
//! - Use **hint** for actionable next steps.
//! - Use **location** to aid maintainers/debuggers.
//!
//! # Notes for library integrators
//!
//! If you already have a richer domain error type, map it into this message model close
//! to your boundary layer (CLI / API adapter / service shell). This keeps domain logic
//! independent of terminal formatting concerns.
//!
//! **Author:** @gabmacn

use chrono::Local;
use colored::*; // Keep for user content styling
use std::io::{self, Write};
use terminal_size::{Height, Width, terminal_size};
use unicode_width::UnicodeWidthChar;
use wrap_ansi::{WrapOptions, wrap_ansi};

// CONSTANTS
const MIN_CONTENT_WIDTH: usize = 40;
const MAX_CONTENT_WIDTH: usize = 140;
const FRAME_MARGIN: usize = 4; // breathing room around content
const RESET: &str = "\x1b[0m";

/// Semantic message level used to select styling and label.
///
/// This enum controls:
///
/// - border color
/// - icon glyph
/// - level label text
///
/// It does **not** change the structural layout; all levels share the same layout.
///
/// # Variants
///
/// - [`PrettyMessageLevel::Error`]: fatal/problem state
/// - [`PrettyMessageLevel::Warning`]: recoverable issue
/// - [`PrettyMessageLevel::Info`]: neutral informational update
/// - [`PrettyMessageLevel::Success`]: positive completion/confirmation
/// - [`PrettyMessageLevel::Input`]: prompt-like interaction context
#[derive(Clone, Copy, Debug)]
pub enum PrettyMessageLevel {
	/// Use for fatal conditions, failed operations, validation errors, or anything that should stand out immediately.
	Error,
	/// Use for non-fatal issues where execution may continue.
	Warning,
	/// Use for neutral, operator-friendly progress or status updates.
	Info,
	/// Use for successful completion messages and positive confirmations.
	Success,
	///
	Input,
}

#[derive(Clone, Copy)]
struct Frame {
	border_v: &'static str,
	border_tl: &'static str,
	border_tr: &'static str,
	border_bl: &'static str,
	border_br: &'static str,
	border_joint_left: &'static str,
	border_joint_right: &'static str,
	line_color: &'static str,
	line_dim_color: &'static str,
	icon: &'static str,
	label: &'static str,
	label_color: Color,
}

// A static buffer of spaces for zero-allocation padding
static SPACES: &str = "                                                                        ";

// Pre-rendered frames per message level to keep the hot path allocation-free.
const FRAME_ERROR: Frame = Frame {
	border_v: "\x1b[31m│\x1b[0m",
	border_tl: "\x1b[31m╭\x1b[0m",
	border_tr: "\x1b[31m╮\x1b[0m",
	border_bl: "\x1b[31m╰\x1b[0m",
	border_br: "\x1b[31m╯\x1b[0m",
	border_joint_left: "\x1b[31m├\x1b[0m",
	border_joint_right: "\x1b[31m┤\x1b[0m",
	line_color: "\x1b[31m",
	line_dim_color: "\x1b[31;2m",
	icon: "✖",
	label: "ERROR",
	label_color: Color::Red,
};

const FRAME_WARNING: Frame = Frame {
	border_v: "\x1b[33m│\x1b[0m",
	border_tl: "\x1b[33m╭\x1b[0m",
	border_tr: "\x1b[33m╮\x1b[0m",
	border_bl: "\x1b[33m╰\x1b[0m",
	border_br: "\x1b[33m╯\x1b[0m",
	border_joint_left: "\x1b[33m├\x1b[0m",
	border_joint_right: "\x1b[33m┤\x1b[0m",
	line_color: "\x1b[33m",
	line_dim_color: "\x1b[33;2m",
	icon: "⚠",
	label: "WARNING",
	label_color: Color::Yellow,
};

const FRAME_INFO: Frame = Frame {
	border_v: "\x1b[34m│\x1b[0m",
	border_tl: "\x1b[34m╭\x1b[0m",
	border_tr: "\x1b[34m╮\x1b[0m",
	border_bl: "\x1b[34m╰\x1b[0m",
	border_br: "\x1b[34m╯\x1b[0m",
	border_joint_left: "\x1b[34m├\x1b[0m",
	border_joint_right: "\x1b[34m┤\x1b[0m",
	line_color: "\x1b[34m",
	line_dim_color: "\x1b[34;2m",
	icon: "ℹ",
	label: "INFO",
	label_color: Color::Blue,
};

const FRAME_SUCCESS: Frame = Frame {
	border_v: "\x1b[32m│\x1b[0m",
	border_tl: "\x1b[32m╭\x1b[0m",
	border_tr: "\x1b[32m╮\x1b[0m",
	border_bl: "\x1b[32m╰\x1b[0m",
	border_br: "\x1b[32m╯\x1b[0m",
	border_joint_left: "\x1b[32m├\x1b[0m",
	border_joint_right: "\x1b[32m┤\x1b[0m",
	line_color: "\x1b[32m",
	line_dim_color: "\x1b[32;2m",
	icon: "✔",
	label: "SUCCESS",
	label_color: Color::Green,
};

const FRAME_INPUT: Frame = Frame {
	border_v: "\x1b[36m│\x1b[0m",
	border_tl: "\x1b[36m╭\x1b[0m",
	border_tr: "\x1b[36m╮\x1b[0m",
	border_bl: "\x1b[36m╰\x1b[0m",
	border_br: "\x1b[36m╯\x1b[0m",
	border_joint_left: "\x1b[36m├\x1b[0m",
	border_joint_right: "\x1b[36m┤\x1b[0m",
	line_color: "\x1b[36m",
	line_dim_color: "\x1b[36;2m",
	icon: "⌨",
	label: "INPUT",
	label_color: Color::Cyan,
};

fn frame_for(level: PrettyMessageLevel) -> &'static Frame {
	match level {
		PrettyMessageLevel::Error => &FRAME_ERROR,
		PrettyMessageLevel::Warning => &FRAME_WARNING,
		PrettyMessageLevel::Info => &FRAME_INFO,
		PrettyMessageLevel::Success => &FRAME_SUCCESS,
		PrettyMessageLevel::Input => &FRAME_INPUT,
	}
}

fn terminal_width_limit() -> usize {
	if let Some((Width(w), Height(_h))) = terminal_size() {
		return w.saturating_sub(2) as usize; // leave a small gutter
	}

	if let Ok(cols) = std::env::var("COLUMNS")
		&& let Ok(parsed) = cols.parse::<usize>()
	{
		return parsed.saturating_sub(2);
	}

	100 // sensible default when terminal size is unknown
}

fn measure_lines(max_len: &mut usize, indent: usize, text: &str) {
	for line in text.lines() {
		let len = indent + visible_len(line);
		if len > *max_len {
			*max_len = len;
		}
	}
}

fn compute_content_width(
	frame: &Frame,
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) -> usize {
	let timestamp = Local::now().format("%H:%M:%S");
	let title_up = title.to_uppercase();

	let header_left_len = visible_len(&format!(" {} {} {}", frame.icon, frame.label, title_up));
	let header_right_len = visible_len(&format!("[{}] {} ", code, timestamp));
	let mut max_len = header_left_len + header_right_len;

	if let Some(loc) = location {
		max_len = max_len.max(visible_len(&format!("   ‣at {}", loc)));
	}

	measure_lines(&mut max_len, 2, message);

	if let Some(ctx) = context {
		measure_lines(&mut max_len, 2, ctx);
	}

	if let Some(h) = hint {
		measure_lines(&mut max_len, 5, h);
	}

	// Add breathing room and clamp to sensible bounds / terminal width
	let desired = max_len.saturating_add(FRAME_MARGIN);
	let term_cap = terminal_width_limit();
	desired.clamp(MIN_CONTENT_WIDTH, MAX_CONTENT_WIDTH).min(term_cap.max(MIN_CONTENT_WIDTH))
}

/// Measure visible display width of a potentially ANSI-styled string.
///
/// This function walks characters and ignores terminal CSI color sequences
/// (`\x1b[...m`) while counting printable width, using Unicode display width rules.
///
/// It is used to:
///
/// - align borders correctly
/// - compute horizontal padding
/// - avoid visual drift when color codes are present
///
/// The implementation is allocation-free and optimized for hot rendering paths.
fn visible_len(s: &str) -> usize {
	let mut len = 0;
	let mut in_esc = false;

	for c in s.chars() {
		if c == '\x1b' {
			in_esc = true;
			continue;
		}

		if in_esc {
			if c == 'm' {
				in_esc = false;
			}
			continue;
		}

		len += UnicodeWidthChar::width(c).unwrap_or(0);
	}

	len
}

/// Write `width` spaces into the provided writer without allocating a new string.
///
/// Uses a static reusable buffer chunk and writes it repeatedly.
fn write_padding(w: &mut impl Write, width: usize) -> io::Result<()> {
	let mut remaining = width;
	while remaining > 0 {
		let len = remaining.min(SPACES.len());
		w.write_all(&SPACES.as_bytes()[..len])?;
		remaining -= len;
	}
	Ok(())
}

fn draw_row(
	writer: &mut impl Write,
	frame: &Frame,
	content_width: usize,
	content: &str,
) -> io::Result<()> {
	let vis_len = visible_len(content);
	let padding = content_width.saturating_sub(vis_len);

	writer.write_all(frame.border_v.as_bytes())?;
	writer.write_all(content.as_bytes())?;
	write_padding(writer, padding)?;
	writer.write_all(frame.border_v.as_bytes())?;
	writer.write_all(b"\n")
}

fn write_horizontal(writer: &mut impl Write, color: &str, width: usize) -> io::Result<()> {
	writer.write_all(color.as_bytes())?;
	for _ in 0..width {
		writer.write_all("─".as_bytes())?;
	}
	writer.write_all(RESET.as_bytes())
}

fn draw_horizontal_line(
	writer: &mut impl Write,
	frame: &Frame,
	content_width: usize,
	joints: bool,
) -> io::Result<()> {
	if joints {
		writer.write_all(frame.border_joint_left.as_bytes())?;
		write_horizontal(writer, frame.line_dim_color, content_width)?;
		writer.write_all(frame.border_joint_right.as_bytes())?;
		writer.write_all(b"\n")
	} else {
		writer.write_all(frame.border_v.as_bytes())?;
		write_horizontal(writer, frame.line_dim_color, content_width)?;
		writer.write_all(frame.border_v.as_bytes())?;
		writer.write_all(b"\n")
	}
}

// Allow because this is a non-exposed internal function with many args for flexibility.
#[allow(clippy::too_many_arguments)]
fn render_pretty_message(
	handle: &mut impl Write,
	frame: &Frame,
	content_width: usize,
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) -> io::Result<()> {
	macro_rules! draw {
        ($func:ident $(, $arg:expr )* ) => {
            $func(handle, frame, content_width $(, $arg )* )?
        };
    }

	handle.write_all(b"\n")?;
	handle.write_all(frame.border_tl.as_bytes())?;
	write_horizontal(handle, frame.line_color, content_width)?;
	handle.write_all(frame.border_tr.as_bytes())?;
	handle.write_all(b"\n")?;

	let timestamp = Local::now().format("%H:%M:%S");
	let title_up = title.to_uppercase();
	let left_part = format!(
		" {} {} {}",
		frame.icon.color(frame.label_color),
		format!("{}:", frame.label).color(frame.label_color).bold(),
		title_up.as_str().bold()
	);
	let right_part =
		format!("[{}] {} ", code.bold(), timestamp).truecolor(100, 100, 100).to_string();

	let left_len = visible_len(&format!(" {} {}: {}", frame.icon, frame.label, title_up));
	let right_len = visible_len(&format!("[{}] {} ", code, timestamp));
	let space_needed = content_width.saturating_sub(left_len + right_len);

	handle.write_all(frame.border_v.as_bytes())?;
	handle.write_all(left_part.as_bytes())?;
	write_padding(handle, space_needed)?;
	handle.write_all(right_part.as_bytes())?;
	handle.write_all(frame.border_v.as_bytes())?;
	handle.write_all(b"\n")?;

	if let Some(loc) = location {
		draw!(
			draw_row,
			&format!(
				"   {}{} {}",
				"‣".black(),
				"at".black().italic(),
				loc.bright_blue().underline().italic()
			)
		);
	}

	draw!(draw_horizontal_line, false);

	// Setup ANSI-aware wrapping options
	let wrap_opts = WrapOptions::builder().word_wrap(true).hard_wrap(false).build();
	let wrap_width = content_width.saturating_sub(4).max(10);

	draw!(draw_row, "");

	// 1. Wrap the main message
	let wrapped_message = wrap_ansi(message, wrap_width, Some(wrap_opts.clone()));
	for line in wrapped_message.lines() {
		draw!(draw_row, &format!("  {}", line));
	}

	draw!(draw_row, "");

	// 2. Wrap the context
	if let Some(ctx) = context {
		draw!(draw_row, &format!("  {}", "CONTEXT:".truecolor(100, 100, 100)));

		// Apply the default style FIRST. wrap_ansi will distribute it across lines.
		// If the user passed their own colors, their inner codes will override this!
		let default_ctx = ctx.italic().truecolor(150, 150, 150).to_string();
		let wrapped_context = wrap_ansi(&default_ctx, wrap_width, Some(wrap_opts.clone()));

		for line in wrapped_context.lines() {
			// Print it raw! Let the embedded ANSI do the talking.
			draw!(draw_row, &format!("  {}", line));
		}
		draw!(draw_row, "");
	}

	// 3. Wrap the hint
	if let Some(h) = hint {
		draw!(draw_horizontal_line, false);
		draw!(draw_row, &format!("  {}", "➜  HINT".yellow().bold()));

		let hint_wrap_width = content_width.saturating_sub(6).max(10);

		// Default yellow.
		let default_hint = h.yellow().to_string();
		let wrapped_hint = wrap_ansi(&default_hint, hint_wrap_width, Some(wrap_opts));

		for line in wrapped_hint.lines() {
			// Print it raw!
			draw!(draw_row, &format!("     {}", line));
		}
	}

	handle.write_all(frame.border_bl.as_bytes())?;
	write_horizontal(handle, frame.line_color, content_width)?;
	handle.write_all(frame.border_br.as_bytes())?;
	handle.write_all(b"\n\n")?;

	Ok(())
}

/// Print a fully formatted pretty message to **stderr**.
///
/// This is the primary API for message rendering when you want to choose the level
/// dynamically.
///
/// ## Parameters
///
/// - `level`: visual level profile (error/warn/info/success/input)
/// - `title`: concise message headline
/// - `code`: stable short code for searching/correlation
/// - `message`: primary body text
/// - `context`: optional contextual details block
/// - `hint`: optional action-oriented hint block
/// - `location`: optional source/location line
///
/// ## Behavior
///
/// - Computes an adaptive width from content and terminal size.
/// - Wraps body/context/hint text to fit.
/// - Emits to stderr using a buffered writer and flushes before return.
///
/// ## Return value
///
/// Returns `()` and intentionally swallows rendering I/O errors (best-effort UX path).
///
/// ## Example
///
/// ```rust,no_run
/// use gmn_core::print_pretty_error::{print_pretty_message, PrettyMessageLevel};
///
/// print_pretty_message(
///     PrettyMessageLevel::Warning,
///     "Configuration Missing",
///     "CFG-001",
///     "No config file was found in the default location.",
///     Some("Looked in: ./config.toml and ~/.config/myapp/config.toml"),
///     Some("Create a config file or pass --config <path>."),
///     Some("bootstrap::load_config"),
/// );
/// ```
pub fn print_pretty_message(
	level: PrettyMessageLevel,
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	let stderr = io::stderr();
	// OPTIMIZATION 4: Lock once, wrap in Buffer
	let mut handle = io::BufWriter::new(stderr.lock());
	let frame = frame_for(level);
	let content_width = compute_content_width(frame, title, code, message, context, hint, location);
	let _ = render_pretty_message(
		&mut handle,
		frame,
		content_width,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
	let _ = handle.flush();
}

/// Render a pretty message into a `String` instead of writing to stderr.
///
/// Useful for:
///
/// - tests and snapshot assertions
/// - storing formatted terminal output
/// - forwarding formatted text to alternate sinks
///
/// ## Errors
///
/// Returns [`io::Error`] if rendering fails, or if UTF-8 conversion from the internal
/// byte buffer fails.
///
/// ## Example
///
/// ```rust
/// use gmn_core::print_pretty_error::{pretty_message_to_string, PrettyMessageLevel};
///
/// let text = pretty_message_to_string(
///     PrettyMessageLevel::Info,
///     "Server",
///     "SRV-200",
///     "HTTP server is listening on 127.0.0.1:3000.",
///     None,
///     None,
///     Some("main"),
/// ).expect("should render");
///
/// assert!(text.contains("INFO"));
/// assert!(text.contains("SRV-200"));
/// ```
pub fn pretty_message_to_string(
	level: PrettyMessageLevel,
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) -> io::Result<String> {
	let mut buffer = Vec::new();
	let frame = frame_for(level);
	let content_width = compute_content_width(frame, title, code, message, context, hint, location);
	render_pretty_message(
		&mut buffer,
		frame,
		content_width,
		title,
		code,
		message,
		context,
		hint,
		location,
	)?;

	String::from_utf8(buffer).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

/// Convenience wrapper for [`PrettyMessageLevel::Error`].
///
/// Use this for fatal conditions, failed operations, validation errors, or anything
/// that should stand out immediately.
///
/// ## Example
///
/// ```rust,no_run
/// use gmn_core::print_pretty_error::print_pretty_error;
///
/// print_pretty_error(
///     "Payment Failed",
///     "PAY-402",
///     "Unable to charge the selected payment method.",
///     Some("Gateway returned status: declined"),
///     Some("Ask the user to verify card details or choose another method."),
///     Some("billing::charge"),
/// );
/// ```
pub fn print_pretty_error(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(PrettyMessageLevel::Error, title, code, message, context, hint, location);
}

/// Convenience wrapper for [`PrettyMessageLevel::Warning`].
///
/// Use this for non-fatal issues where execution may continue.
///
/// ## Example
///
/// ```rust,no_run
/// use gmn_core::print_pretty_error::print_pretty_warning;
///
/// print_pretty_warning(
///     "Retrying Request",
///     "NET-RETRY",
///     "Initial request timed out. Retrying with backoff.",
///     Some("Attempt 2 of 5"),
///     None,
///     Some("network::client"),
/// );
/// ```
pub fn print_pretty_warning(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(
		PrettyMessageLevel::Warning,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
}

/// Convenience wrapper for [`PrettyMessageLevel::Info`].
///
/// Use this for neutral, operator-friendly progress or status updates.
pub fn print_pretty_info(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(PrettyMessageLevel::Info, title, code, message, context, hint, location);
}

/// Convenience wrapper for [`PrettyMessageLevel::Success`].
///
/// Use this for successful completion messages and positive confirmations.
pub fn print_pretty_success(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(
		PrettyMessageLevel::Success,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
}

/// Convenience wrapper for [`PrettyMessageLevel::Input`].
///
/// Use this level when displaying prompt-adjacent guidance or expected user input.
pub fn print_pretty_input(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(PrettyMessageLevel::Input, title, code, message, context, hint, location);
}
