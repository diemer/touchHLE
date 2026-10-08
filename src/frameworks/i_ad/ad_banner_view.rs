/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */
//! `ADBannerView`.

use crate::dyld::{ConstantExports, HostConstant};
use crate::frameworks::core_graphics::CGSize;
use crate::frameworks::foundation::ns_string::to_rust_string;
use crate::objc::{id, objc_classes, todo_objc_setter, ClassExports};

const ADBannerContentSizeIdentifier320x50: &str = "ADBannerContentSizeIdentifier320x50";
const ADBannerContentSizeIdentifier480x32: &str = "ADBannerContentSizeIdentifier480x32";
const ADBannerContentSizeIdentifierPortrait: &str = "ADBannerContentSizeIdentifierPortrait";
const ADBannerContentSizeIdentifierLandscape: &str = "ADBannerContentSizeIdentifierLandscape";

pub const CONSTANTS: ConstantExports = &[
    (
        "_ADBannerContentSizeIdentifier320x50",
        HostConstant::NSString(ADBannerContentSizeIdentifier320x50),
    ),
    (
        "_ADBannerContentSizeIdentifier480x32",
        HostConstant::NSString(ADBannerContentSizeIdentifier480x32),
    ),
    (
        "_ADBannerContentSizeIdentifierPortrait",
        HostConstant::NSString(ADBannerContentSizeIdentifierPortrait),
    ),
    (
        "_ADBannerContentSizeIdentifierLandscape",
        HostConstant::NSString(ADBannerContentSizeIdentifierLandscape),
    ),
];

pub const CLASSES: ClassExports = objc_classes! {

(env, this, _cmd);

@implementation ADBannerView: UIView

+ (CGSize)sizeFromBannerContentSizeIdentifier:(id)identifier { // NSString*
    let identifier = to_rust_string(env, identifier);
    if identifier.contains("Landscape") || identifier.contains("480x32") {
        CGSize { width: 480.0, height: 32.0 }
    } else {
        CGSize { width: 320.0, height: 50.0 }
    }
}

// No ad will ever arrive, so the banner stays empty and unloaded.
- (bool)isBannerLoaded {
    false
}

- (())setDelegate:(id)delegate {
    todo_objc_setter!(this, delegate);
}

- (())setCurrentContentSizeIdentifier:(id)identifier { // NSString*
    todo_objc_setter!(this, to_rust_string(env, identifier));
}

- (())setRequiredContentSizeIdentifiers:(id)identifiers { // NSSet*
    todo_objc_setter!(this, identifiers);
}

@end

};
