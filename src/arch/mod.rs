// This file is part of libfringe, a low-level green threading library.
// Copyright (c) Nathan Zadoks <nathan@nathan7.eu>,
//               whitequark <whitequark@whitequark.org>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
pub use self::imp::*;

// rust-lang/rust#25544
// #[cfg_attr(target_arch = "x86",    path = "x86.rs")]
// #[cfg_attr(target_arch = "x86_64", path = "x86_64.rs")]
// mod imp;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod imp;

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod imp;
