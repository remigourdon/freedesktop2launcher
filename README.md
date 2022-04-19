# freedesktop2launcher

Quick Rust utility program to find desktop entry files (as defined by [freedesktop](https://specifications.freedesktop.org/desktop-entry-spec/desktop-entry-spec-latest.html)) on the system, parse them using the Pop! OS maintained [freedesktop-desktop-entry crate](https://crates.io/crates/freedesktop-desktop-entry) and return the simplified data as tab-delimited lines.

This data is used as part of my [application launcher script](https://github.com/remigourdon/scripts/blob/master/payloads/applications).
