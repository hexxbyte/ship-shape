use pulldown_cmark::{Event, Parser, Tag, TagEnd};

/// Close the current line, leaving exactly one trailing newline.
fn end_line(text: &mut String) {
	trim_trailing_space(text);
	if !text.is_empty() {
		text.push('\n');
	}
}

/// Close the current block, leaving exactly one blank line after it.
fn end_block(text: &mut String) {
	trim_trailing_space(text);
	if !text.is_empty() {
		text.push_str("\n\n");
	}
}

/// Move to the start of a line, keeping any blank line that is already there.
fn start_line(text: &mut String) {
	if !text.is_empty() && !text.ends_with('\n') {
		text.push('\n');
	}
}

fn trim_trailing_space(text: &mut String) {
	text.truncate(text.trim_end_matches([' ', '\n']).len());
}

/// Convert markdown to plain text suitable for display in a read-only `TextCtrl`.
///
/// Headings, paragraphs, code blocks, and lists each become a block separated by a blank line.
/// List items carry a `- ` or `N. ` marker and are indented two spaces per nesting level. Wrapped
/// source lines are joined with a space. Every other construct contributes only the text it wraps,
/// so emphasis markers and link targets are dropped.
#[must_use]
pub fn markdown_to_text(markdown: &str) -> String {
	let mut text = String::new();
	// One entry per open list: `None` for a bullet list, `Some(n)` for the next ordinal of an
	// ordered one.
	let mut lists: Vec<Option<u64>> = Vec::new();
	for event in Parser::new(markdown) {
		match event {
			Event::Text(s) | Event::Code(s) => text.push_str(&s),
			Event::SoftBreak => text.push(' '),
			Event::HardBreak | Event::End(TagEnd::Item) => end_line(&mut text),
			Event::Start(Tag::List(first_ordinal)) => lists.push(first_ordinal),
			Event::End(TagEnd::List(_)) => {
				lists.pop();
				if lists.is_empty() {
					end_block(&mut text);
				}
			}
			Event::Start(Tag::Item) => {
				start_line(&mut text);
				for _ in 1..lists.len() {
					text.push_str("  ");
				}
				match lists.last_mut() {
					Some(Some(ordinal)) => {
						text.push_str(&ordinal.to_string());
						text.push_str(". ");
						*ordinal += 1;
					}
					_ => text.push_str("- "),
				}
			}
			Event::End(TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::CodeBlock) => end_block(&mut text),
			_ => {}
		}
	}
	text.trim().to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn changelog_section_keeps_a_blank_line_before_the_next_heading() {
		let markdown =
			"### Added\n\n- First change that wraps\n  onto a second line\n- Second change\n\n### Fixed\n\n- A fix\n";
		assert_eq!(
			markdown_to_text(markdown),
			"Added\n\n- First change that wraps onto a second line\n- Second change\n\nFixed\n\n- A fix"
		);
	}

	#[test]
	fn wrapped_list_item_joins_its_lines_with_a_space() {
		assert_eq!(markdown_to_text("- wrapped\n  text"), "- wrapped text");
	}

	#[test]
	fn nested_list_items_are_indented() {
		assert_eq!(markdown_to_text("- outer\n  - inner\n"), "- outer\n  - inner");
	}

	#[test]
	fn ordered_list_items_keep_their_numbers() {
		assert_eq!(markdown_to_text("2. second\n3. third\n"), "2. second\n3. third");
	}

	#[test]
	fn loose_list_renders_like_a_tight_list() {
		assert_eq!(markdown_to_text("- one\n\n- two\n"), "- one\n- two");
	}

	#[test]
	fn paragraph_after_a_list_starts_a_new_block() {
		assert_eq!(markdown_to_text("- one\n\nAfter.\n"), "- one\n\nAfter.");
	}

	#[test]
	fn hard_break_starts_a_new_line() {
		assert_eq!(markdown_to_text("one  \ntwo"), "one\ntwo");
	}

	#[test]
	fn fenced_code_block_becomes_its_own_block() {
		assert_eq!(markdown_to_text("Before.\n\n```\nrun --now\n```\n\nAfter."), "Before.\n\nrun --now\n\nAfter.");
	}

	#[test]
	fn inline_code_keeps_its_text() {
		assert_eq!(markdown_to_text("Use `--silent` now."), "Use --silent now.");
	}

	#[test]
	fn blank_markdown_produces_an_empty_string() {
		assert_eq!(markdown_to_text("\n\n   \n"), "");
	}
}
