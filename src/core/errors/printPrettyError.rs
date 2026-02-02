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

* @GabMacN 2025

* GitHub: GabMacN
* Discord: gabmacn
* Youtube: @GabMacN
* Instagram: @gabmacn

* Name: errors/printPrettyError.rs
* Description: Utility functions for printing pretty-formatted messages (errors, warnings, info, success, input).
*/
use chrono::Local;
use colored::*; // Keep for user content styling
use std::io::{self, Write};
use terminal_size::{Height, Width, terminal_size};
use textwrap::Options;
use unicode_width::UnicodeWidthChar;

// CONSTANTS
const MIN_CONTENT_WIDTH: usize = 40;
const MAX_CONTENT_WIDTH: usize = 140;
const FRAME_MARGIN: usize = 4; // breathing room around content
const RESET: &str = "\x1b[0m";

#[derive(Clone, Copy)]
pub enum PrettyMessageLevel {
	Error,
	Warning,
	Info,
	Success,
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
	desired
		.clamp(MIN_CONTENT_WIDTH, MAX_CONTENT_WIDTH)
		.min(term_cap.max(MIN_CONTENT_WIDTH))
}

/// OPTIMIZATION 1: Manual ANSI Stripping (Zero Allocation)
/// Replaces Regex. Iterates bytes to count visible characters, skipping CSI sequences.
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

/// OPTIMIZATION 2: Zero-Allocation Padding
/// Writes a slice of the static SPACES string directly to the buffer.
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
	let right_part = format!("[{}] {} ", code.bold(), timestamp)
		.truecolor(100, 100, 100)
		.to_string();

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

	let wrap_opts = Options::new(content_width.saturating_sub(4).max(10));
	draw!(draw_row, "");

	for line in textwrap::wrap(message, &wrap_opts) {
		draw!(draw_row, &format!("  {}", line.white()));
	}

	draw!(draw_row, "");

	if let Some(ctx) = context {
		draw!(
			draw_row,
			&format!("  {}", "CONTEXT:".truecolor(100, 100, 100))
		);
		for line in textwrap::wrap(ctx, &wrap_opts) {
			draw!(
				draw_row,
				&format!("  {}", line.italic().truecolor(150, 150, 150))
			);
		}
		draw!(draw_row, "");
	}

	if let Some(h) = hint {
		draw!(draw_horizontal_line, false);
		draw!(draw_row, &format!("  {}", "➜  HINT".yellow().bold()));

		let hint_wrap_opts = Options::new(content_width - 6);
		for line in textwrap::wrap(h, &hint_wrap_opts) {
			draw!(draw_row, &format!("     {}", line.yellow()));
		}
	}

	handle.write_all(frame.border_bl.as_bytes())?;
	write_horizontal(handle, frame.line_color, content_width)?;
	handle.write_all(frame.border_br.as_bytes())?;
	handle.write_all(b"\n\n")?;

	Ok(())
}

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

pub fn print_pretty_error(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(
		PrettyMessageLevel::Error,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
}

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

pub fn print_pretty_info(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(
		PrettyMessageLevel::Info,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
}

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

pub fn print_pretty_input(
	title: &str,
	code: &str,
	message: &str,
	context: Option<&str>,
	hint: Option<&str>,
	location: Option<&str>,
) {
	print_pretty_message(
		PrettyMessageLevel::Input,
		title,
		code,
		message,
		context,
		hint,
		location,
	);
}
