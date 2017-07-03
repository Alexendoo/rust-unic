// Copyright 2015 The Servo Project Developers.
// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

extern crate unic;


use unic::bidi::BidiInfo;
use unic::normal::StrNormalForm;
use unic::ucd::{Age, BidiClass, CharAge, CharBidiClass, StrBidiClass};
use unic::ucd::normal::compose;

#[cfg_attr(rustfmt, rustfmt_skip)]
#[test]
fn test_sample() {

    // Age

    assert_eq!(Age::of('A'), Age::Since{ major: 1, minor: 1 });
    assert_eq!(Age::of('\u{A0000}'), Age::Unassigned);
    assert_eq!(Age::of('\u{10FFFF}'), Age::Since{ major: 2, minor: 0 });

    assert_eq!('🦊'.age(), Age::Since{ major: 9, minor: 0 });
    assert_eq!('🦊'.age().to_unicode_version().unwrap().major(), 9);
    assert_eq!('🦊'.age().to_unicode_version().unwrap().minor(), 0);
    assert_eq!('🦊'.age().to_unicode_version().unwrap().micro(), 0);

    // Bidi

    let text = concat![
        "א",
        "ב",
        "ג",
        "a",
        "b",
        "c",
    ];

    assert!(!text.has_bidi_explicit());
    assert!(text.has_rtl());
    assert!(text.has_ltr());

    assert_eq!(text.chars().nth(0).unwrap().bidi_class(), BidiClass::R);
    assert!(!text.chars().nth(0).unwrap().is_ltr());
    assert!(text.chars().nth(0).unwrap().is_rtl());

    assert_eq!(text.chars().nth(3).unwrap().bidi_class(), BidiClass::L);
    assert!(text.chars().nth(3).unwrap().is_ltr());
    assert!(!text.chars().nth(3).unwrap().is_rtl());

    let bidi_info = BidiInfo::new(text, None);
    assert_eq!(bidi_info.paragraphs.len(), 1);

    let para = &bidi_info.paragraphs[0];
    assert_eq!(para.level.number(), 1);
    assert_eq!(para.level.is_rtl(), true);

    let line = para.range.clone();
    let display = bidi_info.reorder_line(para, line);
    assert_eq!(
        display,
        concat![
            "a",
            "b",
            "c",
            "ג",
            "ב",
            "א",
        ]
    );

    // Normalization

    assert_eq!(compose('A', '\u{030A}'), Some('Å'));

    let s = "ÅΩ";
    let c = s.nfc().collect::<String>();
    assert_eq!(c, "ÅΩ");

}
