use std::io::Write;

use crate::error::CargoResult;
use clap::builder::styling::Style;
use clap_cargo::style::HEADER;

pub fn status(action: &str, message: impl std::fmt::Display) -> CargoResult<()> {
    print(action, message, HEADER, true)
}

/// Print a styled error message.
pub fn error(message: impl std::fmt::Display) -> CargoResult<()> {
    let report = &[annotate_snippets::Group::with_title(
        annotate_snippets::Level::ERROR.primary_title(message.to_string()),
    )];
    print_report(report)
}

pub fn note(message: impl std::fmt::Display) -> CargoResult<()> {
    let report = &[annotate_snippets::Group::with_title(
        annotate_snippets::Level::NOTE.secondary_title(message.to_string()),
    )];
    print_report(report)
}

/// Print a message with a colored title in the style of Cargo shell messages.
pub fn print(
    status: &str,
    message: impl std::fmt::Display,
    style: Style,
    justified: bool,
) -> CargoResult<()> {
    let mut stderr = anstream::stderr().lock();
    if justified {
        write!(stderr, "{style}{status:>12}{style:#}")?;
    } else {
        write!(stderr, "{style}{status}{style:#}:")?;
    }

    writeln!(stderr, " {message:#}")?;

    Ok(())
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
