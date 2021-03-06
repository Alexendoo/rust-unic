// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use source::common::unicode_version::UnicodeVersion;
use source::utils::read;

lazy_static! {
    pub static ref UNICODE_VERSION: UnicodeVersion = {
        read("data/idna/ReadMe.txt").parse().unwrap()
    };
}
