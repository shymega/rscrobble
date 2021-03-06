// This file is part of Scrobblers.

// Scrobblers is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Scrobblers is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Scrobblers.  If not, see <http://www.gnu.org/licenses/>

//! The library for Scrobblers.

#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

pub mod logging;

extern crate sources;
extern crate targets;
extern crate rpc;
extern crate db;
