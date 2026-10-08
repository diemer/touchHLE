/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `NSDateFormatter`.
//!
//! Resources:
//! - Apple's [Introduction to Data Formatting Programming Guide For Cocoa](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/DataFormatting/DataFormatting.html)
//! - [Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-10.html#Date_Format_Patterns)

use crate::frameworks::core_foundation::time::CFAbsoluteTimeGetGregorianDate;
use crate::frameworks::foundation::{ns_string, NSTimeInterval};
use crate::objc::{
    autorelease, id, msg, nil, objc_classes, todo_objc_setter, ClassExports, HostObject, NSZonePtr,
};

struct NSDateFormatterHostObject {
    date_format: Option<id>,
}
impl HostObject for NSDateFormatterHostObject {}

fn day_of_year(year: i32, month: i8, day: i8) -> u32 {
    const DAYS_BEFORE_MONTH: [u32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let month = month.clamp(1, 12);
    let mut result = DAYS_BEFORE_MONTH[month as usize - 1] + day.max(1) as u32;
    if leap && month > 2 {
        result += 1;
    }
    result
}

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation NSDateFormatter: NSObject

+ (id)allocWithZone:(NSZonePtr)_zone {
    let host_object = Box::new(NSDateFormatterHostObject {
        date_format: None,
    });
    env.objc.alloc_object(this, host_object, &mut env.mem)
}

- (())setDateFormat:(id)format { // NSString *
    let date_format: id = msg![env; format copy];
    env.objc.borrow_mut::<NSDateFormatterHostObject>(this).date_format = Some(date_format);
}

- (())setTimeZone:(id)time_zone {
    todo_objc_setter!(this, time_zone);
}

- (id)stringFromDate:(id)date {
    let &NSDateFormatterHostObject {
        date_format
    } = env.objc.borrow(this);
    let mut format = ns_string::to_rust_string(env, date_format.unwrap()).to_string().clone();
    log_dbg!("date_format before: {:?}", format);

    let ti: NSTimeInterval = msg![env; date timeIntervalSinceReferenceDate];
    let greg_date = CFAbsoluteTimeGetGregorianDate(env, ti, nil);
    let year = greg_date.year;
    let month = greg_date.month;
    let day = greg_date.day;
    let hour = greg_date.hours;
    let minute = greg_date.minutes;
    let second = greg_date.seconds;

    let day_of_year = day_of_year(year, month, day);
    let frac_second = second.fract();

    format = format.replace("yyyy", format!("{year:04}").as_str());
    format = format.replace("YYYY", format!("{year:04}").as_str());
    format = format.replace("MM", format!("{month:02}").as_str());
    format = format.replace("dd", format!("{day:02}").as_str());
    format = format.replace("HH", format!("{hour:02}").as_str());
    format = format.replace("mm", format!("{minute:02}").as_str());
    format = format.replace("ss", format!("{second:02}").as_str());
    format = format.replace("DDD", format!("{day_of_year:03}").as_str());
    format = format.replace("DD", format!("{day_of_year:02}").as_str());
    format = format.replace("D", format!("{day_of_year}").as_str());
    format = format.replace("SS", format!("{:02}", (frac_second * 100.0) as u32).as_str());
    format = format.replace("S", format!("{}", (frac_second * 10.0) as u32).as_str());

    // Real NSDateFormatter leaves patterns it doesn't know alone.
    if let Some(pattern) = format.chars().find(|c| c.is_ascii_alphabetic()) {
        log!("Warning: unsubstituted date format pattern {pattern:?} in {format:?}");
    }
    log_dbg!("date_format after: {:?}", format);

    let res = ns_string::from_rust_string(env, format);
    autorelease(env, res)
}

@end

};
