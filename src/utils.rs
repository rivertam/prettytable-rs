//! Internal only utilities
use std::io::{Error, ErrorKind, Write};
use std::str;

use unicode_width::UnicodeWidthStr;

use super::format::Alignment;

#[cfg(not(windows))]
pub static NEWLINE: &'static [u8] = b"\n";
#[cfg(windows)]
pub static NEWLINE: &'static [u8] = b"\r\n";

/// Internal utility for writing data into a string
pub struct StringWriter {
	string: String
}

impl StringWriter {
	/// Create a new `StringWriter`
	pub fn new() -> StringWriter {
		return StringWriter{string: String::new()};
	}

	/// Return a reference to the internally written `String`
	pub fn as_string(&self) -> &str {
		return &self.string;
	}
}

impl Write for StringWriter {
	fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
		let string = match str::from_utf8(data) {
			Ok(s) => s,
			Err(e) => return Err(Error::new(ErrorKind::Other, format!("Cannot decode utf8 string : {}", e)))
		};
		self.string.push_str(string);
		return Ok(data.len());
	}

	fn flush(&mut self) -> Result<(), Error> {
		// Nothing to do here
		return Ok(());
	}
}

/// Align/fill a string and print it to `out`
pub fn print_align<T: Write+?Sized>(out: &mut T, align: Alignment, text: &str, fill: char, size: usize, skip_right_fill: bool) -> Result<(), Error> {
	let text_len = UnicodeWidthStr::width(text);
	let mut nfill = if text_len < size { size - text_len } else { 0 };
	let n = match align {
		Alignment::LEFT => 0,
		Alignment:: RIGHT => nfill,
		Alignment:: CENTER => nfill/2
	};
	if n > 0 {
		try!(out.write(&vec![fill as u8; n]));
		nfill -= n;
	}
	try!(out.write(text.as_bytes()));
	if nfill > 0 && ! skip_right_fill {
		try!(out.write(&vec![fill as u8; nfill]));
	}
	return Ok(());
}

#[cfg(test)]
mod tests {
	use super::*;
	use format::Alignment;
	use std::io::Write;

	#[test]
	fn string_writer() {
		let mut out = StringWriter::new();
		out.write("foo".as_bytes()).unwrap();
		out.write(" ".as_bytes()).unwrap();
		out.write("".as_bytes()).unwrap();
		out.write("bar".as_bytes()).unwrap();
		assert_eq!(out.as_string(), "foo bar");
	}

	#[test]
	fn fill_align() {
		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::RIGHT, "foo", '*', 10, false).unwrap();
		assert_eq!(out.as_string(), "*******foo");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::LEFT, "foo", '*', 10, false).unwrap();
		assert_eq!(out.as_string(), "foo*******");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::CENTER, "foo", '*', 10, false).unwrap();
		assert_eq!(out.as_string(), "***foo****");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::CENTER, "foo", '*', 1, false).unwrap();
		assert_eq!(out.as_string(), "foo");
	}

	#[test]
	fn skip_right_fill() {
		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::RIGHT, "foo", '*', 10, true).unwrap();
		assert_eq!(out.as_string(), "*******foo");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::LEFT, "foo", '*', 10, true).unwrap();
		assert_eq!(out.as_string(), "foo");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::CENTER, "foo", '*', 10, true).unwrap();
		assert_eq!(out.as_string(), "***foo");

		let mut out = StringWriter::new();
		print_align(&mut out, Alignment::CENTER, "foo", '*', 1, false).unwrap();
		assert_eq!(out.as_string(), "foo");
	}
}
