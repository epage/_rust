use std::io::Write;

use crate::error::CargoResult;

/// Print a styled error message.
pub fn error(message: impl std::fmt::Display) -> CargoResult<()> {
    let report = &[annotate_snippets::Group::with_title(
        annotate_snippets::Level::ERROR.primary_title(message.to_string()),
    )];
    print_report(report)
}

/// Prints the passed in [`Report`][annotate_snippets::Report] to stderr
pub fn print_report(report: annotate_snippets::Report<'_>) -> CargoResult<()> {
    let decor_style = if cargo_term_unicode().unwrap_or_else(supports_unicode::supports_unicode) {
        annotate_snippets::renderer::DecorStyle::Unicode
    } else {
        annotate_snippets::renderer::DecorStyle::Ascii
    };
    let rendered = annotate_snippets::Renderer::styled()
        .decor_style(decor_style)
        .render(report);
    let mut stderr = anstream::stderr().lock();
    stderr.write_all(rendered.as_bytes())?;
    stderr.write_all(b"\n")?;
    Ok(())
}

fn cargo_term_unicode() -> Option<bool> {
    std::env::var_os("CARGO_TERM_UNICODE").map(|v| v == "true")
}
