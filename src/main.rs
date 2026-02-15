use std::env::args;

pub mod program; 
pub mod cmd_line;
pub mod data;
pub mod features;

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
i18n!("locales", fallback = "en");

fn main()
{
    let args = args().collect();

    // rust_i18n::set_locale("fr");

    program::start_program(args);
}