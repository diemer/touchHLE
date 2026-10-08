/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! iAd
//!
//! The iAd service was discontinued in 2016, so this is only enough of the
//! framework to keep apps that embed a banner running.

mod ad_banner_view;

pub const DYLIB: crate::dyld::HostDylib = crate::dyld::HostDylib {
    path: "/System/Library/Frameworks/iAd.framework/iAd",
    aliases: &[],
    class_exports: &[ad_banner_view::CLASSES],
    constant_exports: &[ad_banner_view::CONSTANTS],
    function_exports: &[],
};
