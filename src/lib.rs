// THIS FILE IS GENERATED, DO NOT EDIT

//! A wrapper around the [HID Usage Tables (HUT)](https://usb.org/document-library/hid-usage-tables-15).
//!
//! In this document and unless stated otherwise, a reference to
//! - **"HID Section a.b.c"** refers to the [HID Device Class Definition for HID 1.11](https://www.usb.org/document-library/device-class-definition-hid-111)
//! - **"HUT Section a.b.c"** refers to the
//!   [HID Usage Tables (HUT) version 1.5](https://usb.org/document-library/hid-usage-tables-15)
//!
//! This module is created through code generation from the HID Usage Tables.
//!
//! # Terminology
//!
//! See HID Section 5.5: a HID Usage is a 32 bit value comprising of a 16-bit Usage
//! Page (MSB) and a 16-bit Usage ID (LSB) so that:
//!
//! ```
//! # let usage_page: u32 = 0;
//! # let usage_id: u32 = 0;
//! let usage: u32 = (usage_page << 16) | usage_id;
//! ```
//! Note that the HID encoding requires little endian byte order on the wire.
//!
//! In this module:
//!
//! - **"Usage Page"** refers to the 16-bit value. Where the Usage Page is converted
//!   to or from a 32-bit value the Usage Page is in the upper 16 bits of that value and
//!   the lower 16 bits are ignored or set to zero.
//!   ```
//!   # let usage: u32 = 0;
//!   let usage_page: u16 = (usage >> 16) as u16 & 0xffff;
//!   ```
//! - **"Usage ID"** refers to the 16-bit value. Where the Usage ID is converted to
//!   or from a 32-bit value the Usage is in the lower 16 bits of that value and
//!   the upper 16 bits are ignored or set to zero.
//!   ```
//!   # let usage: u32 = 0;
//!   let usage_id: u16 = (usage & 0xffff) as u16;
//!   ```
//! - **"Usage"** refers to the 32-bit value comprising a Usage Page and a Usage.
//!
//! # Converting between types
//!
//! All defined [Usages](Usage) and [UsagePages](UsagePage) implement [AsUsagePage] and (if applicable) [AsUsage] as
//! well as the [`From<u16>`](From), [`From<u32>`](From), [`TryFrom<u16>`](TryFrom), and [`TryFrom<u32>`](TryFrom)
//! conversions so that:
//! ```
//! # use hut::*;
//! let usage_page_value: u16 = 0x01; // Generic Desktop
//! let usage_id_value: u16 = 0x02; // Mouse
//! let usage_value: u32 = ((usage_page_value as u32) << 16) | usage_id_value as u32;
//!
//! // Create a known Usage from a 32-bit value
//! let u: Usage = Usage::try_from(usage_value).unwrap();
//! assert!(matches!(u, Usage::GenericDesktop(GenericDesktop::Mouse)));
//!
//! // Create a known Usage from the Usage Page and Usage ID values
//! let u2 = Usage::new_from_page_and_id(usage_page_value, usage_id_value).unwrap();
//! assert_eq!(u, u2);
//!
//! // Create a known Usage from an individual Usage Page enum item
//! let u3 = Usage::from(GenericDesktop::Mouse);
//! assert_eq!(u, u3);
//!
//! // Create a known Usage from an known Usage Page enum item
//! let gd_mouse = GenericDesktop::try_from(usage_id_value).unwrap();
//! let u4 = Usage::from(gd_mouse);
//! assert_eq!(u, u4);
//!
//! // Convert to and fro the Usage either via u32 or the AsUsage trait
//! let u = GenericDesktop::Mouse;
//! assert_eq!(u32::from(&u), usage_value);
//! assert_eq!(u.usage_value(), usage_value);
//!
//! // Extract the 16-bit Usage ID either via u16 or the AsUsage trait
//! assert_eq!(u16::from(&u), usage_id_value);
//! assert_eq!(u.usage_id_value(), usage_id_value);
//!
//! // Extract the Usage Page from the Usage enum value
//! let up = u.usage_page();
//! assert!(matches!(up, UsagePage::GenericDesktop));
//! let up: UsagePage = UsagePage::from(&u);
//! assert!(matches!(up, UsagePage::GenericDesktop));
//!
//! // Get the Usage Page numeric value is via the AsUsagePage
//! assert_eq!(u16::from(&up), usage_page_value);
//! assert_eq!(up.usage_page_value(), usage_page_value);
//! ```
//!
//! Naming Usages (e.g. [`GenericDesktop::Mouse`]) above works for Defined Usage
//! Pages, Generated Usage Pages (see below) need to be destructured via their
//! individual elements:
//! ```
//! # use hut::*;
//! let usage_page_value: u16 = 0x09; // Button
//! let usage_id_value: u16 = 8; // Button number 8
//! let usage_value: u32 = ((usage_page_value as u32) << 16) | usage_id_value as u32;
//!
//! let u = Usage::try_from(usage_value).unwrap();
//! let button = Usage::Button(Button::Button(8));
//! assert!(matches!(Usage::try_from(usage_value).unwrap(), button));
//! // or via from() or into()
//! let button: Usage = Button::Button(8).into();
//! assert!(matches!(Usage::try_from(usage_value).unwrap(), button));
//! ```
//! Once a Usage is created, the [AsUsagePage] and [AsUsage] traits and conversion to and from
//! [u16] and [u32] work the same as for a Defined Usage Page.
//!
//! # Names of Usage Pages and Usage IDs
//!
//! All defined [Usages](Usage) and [UsagePages](UsagePage) implement `name()` to return a string
//! representing that page or usage:
//!
//! ```
//! # use hut::*;
//! let up = UsagePage::GenericDesktop;
//! assert_eq!(up.name(), "Generic Desktop");
//! let up = UsagePage::SimulationControls;
//! assert_eq!(up.name(), "Simulation Controls");
//!
//! let usage = GenericDesktop::Mouse;
//! assert_eq!(usage.name(), "Mouse");
//! let usage = SimulationControls::CyclicControl;
//! assert_eq!(usage.name(), "Cyclic Control");
//! ```
//!
//! # Generated Usage Pages
//!
//! The HUT differ between "Defined" and "Generated" Usage Pages. The former define Usage ID values
//! and their meanings, the latter define a Usage ID range, with the actual Usage ID simply
//! referring to "nth thing in this usage page". One example for this is the Button Usage Page
//! (0x09) where a Usage ID of 3 means "Button 3".
//!
//! ```
//! # use hut::*;
//! let b = Button::Button(3);
//! let o = Ordinal::Ordinal(23);
//! ```
//!
//! Unlike Defined Usage Pages these Generated Usage Pages need to be destructured in `match`
//! statements:
//!
//! ```
//! # use hut::*;
//! let b = Button::Button(3);
//! match b {
//!     Button::Button(b) => println!("Button {b}"),
//!     _ => {},
//! }
//! ```
//!
//! The following usage pages are Generated:
//!   - Usage Page 0x9 - [Button]
//!   - Usage Page 0xA - [Ordinal]
//!   - Usage Page 0x10 - [Unicode]
//!   - Usage Page 0x81 - [MonitorEnumerated]
//!
//! A further special case of this is the [Unicode] usage page which is not in the HUT
//! document and was inserted during code generation.
//!
//! # Vendor Defined Usage Pages (0xFF00 to 0xFFFF)
//!
//! [Vendor Defined Usage Pages](VendorDefinedPage) and [VendorUsages](VendorDefinedPage::VendorUsage) are not autogenerated and thus
//! follow a different approach: the Usage inside the Usage Page is a simple
//! numeric usage that needs to be destructured in `match` statements.
//!
//! ```
//! # use hut::*;
//! let v = Usage::VendorDefinedPage {
//!     vendor_page: VendorPage::try_from(0xff00 as u16).unwrap(),
//!     usage: VendorDefinedPage::VendorUsage { usage_id: 0x01 },
//! };
//! match v {
//!     Usage::VendorDefinedPage {
//!         vendor_page,
//!         usage,
//!     } => println!("Vendor Usage ID {usage}"),
//!     _ => {},
//! }
//! ```
//!
//! A notable exception is the [Wacom] (`0xFF0D`) which is technically a
//! Vendor-defined page but with defined Usages. Converting from a [UsagePage]
//! or [Usage] numeric value will produce the correct or [Wacom] Usage, not a [VendorDefinedPage::VendorUsage].
//!
//! # Reserved Usage Pages
//!
//! [Reserved Usage Pages](ReservedUsagePage) and [ReservedUsages](ReservedUsagePage::ReservedUsage) are
//! not autogenerated and thus follow a different approach: the Usage inside the Usage Page is a simple
//! numeric usage that needs to be destructured in `match` statements.
//!
//! Unlike the [Vendor Defined Usage Pages](VendorDefinedPage) a [Reserved Usage Page](ReservedPage) may become
//! a defined page in a later version of the HUT standard and thus in a future version of this crate.
//! A caller must not rely on a Reserved Usage Page or Reserved Usage to remain so.
//!
//! The following Usage Pages are reserved as of HUT 1.5 (see HUT Section 3, p15):
//! - `0x13`, `0x15-0x1F`
//! - `0x21-0x3F`
//! - `0x42-0x58`
//! - `0x5A-0x7F`
//! - `0x83-0x83`
//! - `0x86-0x8B`
//! - `0x8F-0x8F`
//! - `0x93-0xF1CF`
//! - `0xF1D1-0xFEFF`
//!
//! # Renames
//!
//! For technical reasons, spaces, (` `), dashes (`-`), and slashes (`/`) are
//! stripped out of Usage Page and Usage names. The string representation via
//! the `Display` trait will have the unmodified value.

#![allow(clippy::identity_op, clippy::eq_op, clippy::match_single_binding)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

use core::ops::BitOr;
#[cfg(feature = "std")]
use std::{fmt, format, string::String, string::ToString};

/// Error raised if conversion between HUT elements fails.
#[derive(Debug)]
pub enum HutError {
    /// The usage page value is not known. Usage Pages
    /// may get added in the future and a future version
    /// of this crate may not raise this error for the same value.
    UnknownUsagePage { usage_page: u16 },
    /// The usage ID value is not known. Usage IDs
    /// may get added in the future and a future version
    /// of this crate may not raise this error for the same value.
    UnknownUsageId { usage_id: u16 },
    /// The value given for a [VendorDefinedPage] is outside the allowed range.
    InvalidVendorPage { vendor_page: u16 },
    /// The value given for a [ReservedUsagePage] is outside the allowed range.
    InvalidReservedPage { reserved_page: u16 },
    /// The 32-bit usage value given cannot be resolved. Usages
    /// may get added in the future and a future version
    /// of this crate may not raise this error for the same value.
    UnknownUsage,
}

impl core::error::Error for HutError {}

impl core::fmt::Display for HutError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            HutError::UnknownUsagePage { usage_page } => {
                write!(fmt, "Unknown Usage Page {}", usage_page)
            }
            HutError::UnknownUsageId { usage_id } => write!(fmt, "Unknown Usage ID {}", usage_id),
            HutError::InvalidVendorPage { vendor_page } => {
                write!(fmt, "Invalid Vendor Page {}", vendor_page)
            }
            HutError::InvalidReservedPage { reserved_page } => {
                write!(fmt, "Invalid Reserved Page {}", reserved_page)
            }
            HutError::UnknownUsage => write!(fmt, "Unknown Usage"),
        }
    }
}

type Result<T> = core::result::Result<T, HutError>;

/// A trait to return the Usage and Usage ID as numeric value
pub trait AsUsage {
    /// Returns the 32-bit Usage numeric value of this Usage
    fn usage_value(&self) -> u32;

    /// Returns the 16-bit Usage Id numeric value of this Usage
    fn usage_id_value(&self) -> u16;

    /// Returns this usage as [Usage]
    fn usage(&self) -> Usage;
}

/// A trait to return the Usage Page as numeric value
pub trait AsUsagePage {
    /// Returns the 16-bit Usage Page value
    fn usage_page_value(&self) -> u16;

    /// Returns the [UsagePage]
    fn usage_page(&self) -> UsagePage;
}

/// A HID UsagePage, see HID Section 5.5. This represents the upper 16 bits in the
/// 32-bit Usage. Where a [UsagePage] is converted to or from 32 bit, the
/// [UsagePage] value are the upper 16 bits only and the lower 16 bits are
/// ignored or set to zero.
/// ```
/// # use hut::*;
/// let usage = Usage::from(GenericDesktop::Mouse);
/// let up: UsagePage = UsagePage::from(&usage);
/// assert!(matches!(up, UsagePage::GenericDesktop));
/// ```
/// Note: this enum is generated from the HUT specification.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum UsagePage {
    /// Usage Page `0x1`: "Generic Desktop",
    /// see [GenericDesktop].
    GenericDesktop,
    /// Usage Page `0x2`: "Simulation Controls",
    /// see [SimulationControls].
    SimulationControls,
    /// Usage Page `0x3`: "VR Controls",
    /// see [VRControls].
    VRControls,
    /// Usage Page `0x4`: "Sport Controls",
    /// see [SportControls].
    SportControls,
    /// Usage Page `0x5`: "Game Controls",
    /// see [GameControls].
    GameControls,
    /// Usage Page `0x6`: "Generic Device Controls",
    /// see [GenericDeviceControls].
    GenericDeviceControls,
    /// Usage Page `0x7`: "Keyboard/Keypad",
    /// see [KeyboardKeypad].
    KeyboardKeypad,
    /// Usage Page `0x8`: "LED",
    /// see [LED].
    LED,
    /// Usage Page `0x9`: "Button",
    /// see [Button].
    Button,
    /// Usage Page `0xA`: "Ordinal",
    /// see [Ordinal].
    Ordinal,
    /// Usage Page `0xB`: "Telephony Device",
    /// see [TelephonyDevice].
    TelephonyDevice,
    /// Usage Page `0xC`: "Consumer",
    /// see [Consumer].
    Consumer,
    /// Usage Page `0xD`: "Digitizers",
    /// see [Digitizers].
    Digitizers,
    /// Usage Page `0xE`: "Haptics",
    /// see [Haptics].
    Haptics,
    /// Usage Page `0xF`: "Physical Input Device",
    /// see [PhysicalInputDevice].
    PhysicalInputDevice,
    /// Usage Page `0x10`: "Unicode",
    /// see [Unicode].
    Unicode,
    /// Usage Page `0x11`: "SoC",
    /// see [SoC].
    SoC,
    /// Usage Page `0x12`: "Eye and Head Trackers",
    /// see [EyeandHeadTrackers].
    EyeandHeadTrackers,
    /// Usage Page `0x14`: "Auxiliary Display",
    /// see [AuxiliaryDisplay].
    AuxiliaryDisplay,
    /// Usage Page `0x20`: "Sensors",
    /// see [Sensors].
    Sensors,
    /// Usage Page `0x40`: "Medical Instrument",
    /// see [MedicalInstrument].
    MedicalInstrument,
    /// Usage Page `0x41`: "Braille Display",
    /// see [BrailleDisplay].
    BrailleDisplay,
    /// Usage Page `0x59`: "Lighting And Illumination",
    /// see [LightingAndIllumination].
    LightingAndIllumination,
    /// Usage Page `0x80`: "Monitor",
    /// see [Monitor].
    Monitor,
    /// Usage Page `0x81`: "Monitor Enumerated",
    /// see [MonitorEnumerated].
    MonitorEnumerated,
    /// Usage Page `0x82`: "VESA Virtual Controls",
    /// see [VESAVirtualControls].
    VESAVirtualControls,
    /// Usage Page `0x84`: "Power",
    /// see [Power].
    Power,
    /// Usage Page `0x85`: "Battery System",
    /// see [BatterySystem].
    BatterySystem,
    /// Usage Page `0x8C`: "Barcode Scanner",
    /// see [BarcodeScanner].
    BarcodeScanner,
    /// Usage Page `0x8D`: "Scales",
    /// see [Scales].
    Scales,
    /// Usage Page `0x8E`: "Magnetic Stripe Reader",
    /// see [MagneticStripeReader].
    MagneticStripeReader,
    /// Usage Page `0x90`: "Camera Control",
    /// see [CameraControl].
    CameraControl,
    /// Usage Page `0x91`: "Arcade",
    /// see [Arcade].
    Arcade,
    /// Usage Page `0xF1D0`: "FIDO Alliance",
    /// see [FIDOAlliance].
    FIDOAlliance,
    /// Usage Page `0xFF0D`: "Wacom",
    /// see [Wacom].
    Wacom,
    /// The Reserved Usage Pages (range: various). See [ReservedUsagePage].
    ReservedUsagePage(ReservedPage),
    /// The Vendor Defined Pages, range `0xFF00 - 0xFFFF`. See [VendorDefinedPage].
    VendorDefinedPage(VendorPage),
}

/// Represents a Reserved Page number value of in the current range of
/// reserved values. See [ReservedUsagePage].
///
/// ```
/// # use hut::*;
/// // 0x3f is currently reserved
/// let vp: ReservedPage = ReservedPage::try_from(0x3f as u16).unwrap();
/// let vp: ReservedPage = ReservedPage::try_from(0x003f1234 as u32).unwrap();
///
/// let usage = Usage::try_from(0x003f1234).unwrap();
/// let up = UsagePage::from(&usage);
/// assert!(matches!(up, UsagePage::ReservedUsagePage(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ReservedPage(u16);

impl From<&ReservedPage> for ReservedPage {
    fn from(v: &ReservedPage) -> ReservedPage {
        ReservedPage(v.0)
    }
}

impl From<&ReservedPage> for u16 {
    fn from(v: &ReservedPage) -> u16 {
        v.0
    }
}

impl From<ReservedPage> for u16 {
    fn from(v: ReservedPage) -> u16 {
        u16::from(&v)
    }
}

impl From<&ReservedPage> for u32 {
    fn from(v: &ReservedPage) -> u32 {
        (v.0 as u32) << 16
    }
}

impl From<ReservedPage> for u32 {
    fn from(v: ReservedPage) -> u32 {
        u32::from(&v)
    }
}

impl TryFrom<u16> for ReservedPage {
    type Error = HutError;

    fn try_from(v: u16) -> Result<ReservedPage> {
        match v {
            p @ 0x13 => Ok(ReservedPage(p)),
            p @ 0x15..=0x1F => Ok(ReservedPage(p)),
            p @ 0x21..=0x3F => Ok(ReservedPage(p)),
            p @ 0x42..=0x58 => Ok(ReservedPage(p)),
            p @ 0x5A..=0x7F => Ok(ReservedPage(p)),
            p @ 0x83..=0x83 => Ok(ReservedPage(p)),
            p @ 0x86..=0x8B => Ok(ReservedPage(p)),
            p @ 0x8F..=0x8F => Ok(ReservedPage(p)),
            p @ 0x93..=0xF1CF => Ok(ReservedPage(p)),
            p @ 0xF1D1..=0xFEFF => Ok(ReservedPage(p)),
            n => Err(HutError::InvalidReservedPage { reserved_page: n }),
        }
    }
}

impl TryFrom<u32> for ReservedPage {
    type Error = HutError;

    fn try_from(v: u32) -> Result<ReservedPage> {
        ReservedPage::try_from((v >> 16) as u16)
    }
}

/// Represents a Vendor Defined Page number value of in the range
/// `0xFF00..=0xFFFF`. See [VendorDefinedPage].
///
/// ```
/// # use hut::*;
/// // The value has to be 0xff00..=0xffff
/// let vp: VendorPage = VendorPage::try_from(0xff00 as u16).unwrap();
/// let vp: VendorPage = VendorPage::try_from(0xff001234 as u32).unwrap();
///
/// let usage = Usage::try_from(0xff001234).unwrap();
/// let up = UsagePage::from(&usage);
/// assert!(matches!(up, UsagePage::VendorDefinedPage(_)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VendorPage(u16);

impl From<&VendorPage> for VendorPage {
    fn from(v: &VendorPage) -> VendorPage {
        VendorPage(v.0)
    }
}

impl From<&VendorPage> for u16 {
    fn from(v: &VendorPage) -> u16 {
        v.0
    }
}

impl From<VendorPage> for u16 {
    fn from(v: VendorPage) -> u16 {
        u16::from(&v)
    }
}

impl From<&VendorPage> for u32 {
    fn from(v: &VendorPage) -> u32 {
        (v.0 as u32) << 16
    }
}

impl From<VendorPage> for u32 {
    fn from(v: VendorPage) -> u32 {
        u32::from(&v)
    }
}

impl TryFrom<u16> for VendorPage {
    type Error = HutError;

    fn try_from(v: u16) -> Result<VendorPage> {
        match v {
            p @ 0xff00..=0xffff => Ok(VendorPage(p)),
            n => Err(HutError::InvalidVendorPage { vendor_page: n }),
        }
    }
}

impl TryFrom<u32> for VendorPage {
    type Error = HutError;

    fn try_from(v: u32) -> Result<VendorPage> {
        VendorPage::try_from((v >> 16) as u16)
    }
}

impl UsagePage {
    /// Returns the Usage Page for the given Usage Page value. This is the
    /// 16-bit Usage Page value only, not the full 32-bit Usage.
    /// ```
    /// # use hut::*;
    /// let usage_value: u16 = 0x1; // GenericDesktop
    /// let usage_page = UsagePage::from_usage_page_value(usage_value).unwrap();
    /// assert!(matches!(UsagePage::GenericDesktop, usage_page));
    /// ```
    pub fn from_usage_page_value(usage_page: u16) -> Result<UsagePage> {
        UsagePage::try_from(usage_page)
    }

    /// Returns the Usage Page for the given Usage numeric value. The Usage Page
    /// must be in the upper 16 bits of the `usage` value and the lower 16 bits
    /// are ignored.
    /// ```
    /// # use hut::*;
    /// let usage_value: u32 = (0x1 << 16) | 0x2;
    /// let usage = UsagePage::from_usage_value(usage_value).unwrap();
    /// assert!(matches!(Usage::from(GenericDesktop::Mouse), usage));
    /// ```
    pub fn from_usage_value(usage: u32) -> Result<UsagePage> {
        let up: u16 = (usage >> 16) as u16;
        UsagePage::try_from(up)
    }

    /// Returns the 32-bit Usage that is this Usage Page combined with
    /// the 16 bits Usage ID.
    ///
    /// ```
    /// # use hut::*;
    /// let up = UsagePage::GenericDesktop;
    /// let usage_id_value: u16 = 0x02; // Mouse
    ///
    /// let usage = up.to_usage_from_value(usage_id_value).unwrap();
    /// assert!(matches!(Usage::from(GenericDesktop::Mouse), usage));
    /// ```
    pub fn to_usage_from_value(&self, usage: u16) -> Result<Usage> {
        let up: u32 = (self.usage_page_value() as u32) << 16;
        let u: u32 = usage as u32;
        Usage::try_from(up | u)
    }

    /// Return a printable name for this usage page
    /// ```
    /// # use hut::*;
    /// let up = UsagePage::GenericDesktop;
    /// assert_eq!(up.name(), "Generic Desktop");
    /// ```
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            UsagePage::GenericDesktop => "Generic Desktop".into(),
            UsagePage::SimulationControls => "Simulation Controls".into(),
            UsagePage::VRControls => "VR Controls".into(),
            UsagePage::SportControls => "Sport Controls".into(),
            UsagePage::GameControls => "Game Controls".into(),
            UsagePage::GenericDeviceControls => "Generic Device Controls".into(),
            UsagePage::KeyboardKeypad => "Keyboard/Keypad".into(),
            UsagePage::LED => "LED".into(),
            UsagePage::Button => "Button".into(),
            UsagePage::Ordinal => "Ordinal".into(),
            UsagePage::TelephonyDevice => "Telephony Device".into(),
            UsagePage::Consumer => "Consumer".into(),
            UsagePage::Digitizers => "Digitizers".into(),
            UsagePage::Haptics => "Haptics".into(),
            UsagePage::PhysicalInputDevice => "Physical Input Device".into(),
            UsagePage::Unicode => "Unicode".into(),
            UsagePage::SoC => "SoC".into(),
            UsagePage::EyeandHeadTrackers => "Eye and Head Trackers".into(),
            UsagePage::AuxiliaryDisplay => "Auxiliary Display".into(),
            UsagePage::Sensors => "Sensors".into(),
            UsagePage::MedicalInstrument => "Medical Instrument".into(),
            UsagePage::BrailleDisplay => "Braille Display".into(),
            UsagePage::LightingAndIllumination => "Lighting And Illumination".into(),
            UsagePage::Monitor => "Monitor".into(),
            UsagePage::MonitorEnumerated => "Monitor Enumerated".into(),
            UsagePage::VESAVirtualControls => "VESA Virtual Controls".into(),
            UsagePage::Power => "Power".into(),
            UsagePage::BatterySystem => "Battery System".into(),
            UsagePage::BarcodeScanner => "Barcode Scanner".into(),
            UsagePage::Scales => "Scales".into(),
            UsagePage::MagneticStripeReader => "Magnetic Stripe Reader".into(),
            UsagePage::CameraControl => "Camera Control".into(),
            UsagePage::Arcade => "Arcade".into(),
            UsagePage::FIDOAlliance => "FIDO Alliance".into(),
            UsagePage::Wacom => "Wacom".into(),
            UsagePage::ReservedUsagePage(reserved_page) => {
                format!("Reserved Usage Page {:04X}", u16::from(reserved_page))
            }
            UsagePage::VendorDefinedPage(vendor_page) => {
                format!("Vendor Defined Page {:04X}", u16::from(vendor_page))
            }
        }
    }
}

impl AsUsagePage for UsagePage {
    /// Returns the 16 bit Usage Page value of this Usage Page
    /// ```
    /// # use hut::*;
    /// let usage_value: u16 = 0x1; // GenericDesktop
    /// let usage_page = UsagePage::from_usage_page_value(usage_value).unwrap();
    /// assert_eq!(usage_page.usage_page_value(), 0x1);
    /// ```
    fn usage_page_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns our current [UsagePage]
    /// ```
    /// # use hut::*;
    /// let usage_value: u16 = 0x1; // GenericDesktop
    /// let usage_page = UsagePage::from_usage_page_value(usage_value).unwrap();
    /// assert!(matches!(usage_page.usage_page(), UsagePage::GenericDesktop));
    /// ```
    /// There is seldom a need to invoke this function, it is merely
    /// implemented to meet the [AsUsagePage] requirements.
    fn usage_page(&self) -> UsagePage {
        UsagePage::try_from(u16::from(self)).unwrap()
    }
}

/// *Usage Page `0x1`: "Generic Desktop"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::GenericDesktop(GenericDesktop::Mouse);
/// let u2 = Usage::new_from_page_and_id(0x1, 0x2).unwrap();
/// let u3 = Usage::from(GenericDesktop::Mouse);
/// let u4: Usage = GenericDesktop::Mouse.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::GenericDesktop));
/// assert_eq!(0x1, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x1 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Mouse", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum GenericDesktop {
    /// Usage ID `0x1`: "Pointer"
    Pointer = 0x1,
    /// Usage ID `0x2`: "Mouse"
    Mouse = 0x2,
    /// Usage ID `0x4`: "Joystick"
    Joystick = 0x4,
    /// Usage ID `0x5`: "Gamepad"
    Gamepad = 0x5,
    /// Usage ID `0x6`: "Keyboard"
    Keyboard = 0x6,
    /// Usage ID `0x7`: "Keypad"
    Keypad = 0x7,
    /// Usage ID `0x8`: "Multi-axis Controller"
    MultiaxisController = 0x8,
    /// Usage ID `0x9`: "Tablet PC System Controls"
    TabletPCSystemControls = 0x9,
    /// Usage ID `0xA`: "Water Cooling Device"
    WaterCoolingDevice = 0xA,
    /// Usage ID `0xB`: "Computer Chassis Device"
    ComputerChassisDevice = 0xB,
    /// Usage ID `0xC`: "Wireless Radio Controls"
    WirelessRadioControls = 0xC,
    /// Usage ID `0xD`: "Portable Device Control"
    PortableDeviceControl = 0xD,
    /// Usage ID `0xE`: "System Multi-Axis Controller"
    SystemMultiAxisController = 0xE,
    /// Usage ID `0xF`: "Spatial Controller"
    SpatialController = 0xF,
    /// Usage ID `0x10`: "Assistive Control"
    AssistiveControl = 0x10,
    /// Usage ID `0x11`: "Device Dock"
    DeviceDock = 0x11,
    /// Usage ID `0x12`: "Dockable Device"
    DockableDevice = 0x12,
    /// Usage ID `0x13`: "Call State Management Control"
    CallStateManagementControl = 0x13,
    /// Usage ID `0x30`: "X"
    X = 0x30,
    /// Usage ID `0x31`: "Y"
    Y = 0x31,
    /// Usage ID `0x32`: "Z"
    Z = 0x32,
    /// Usage ID `0x33`: "Rx"
    Rx = 0x33,
    /// Usage ID `0x34`: "Ry"
    Ry = 0x34,
    /// Usage ID `0x35`: "Rz"
    Rz = 0x35,
    /// Usage ID `0x36`: "Slider"
    Slider = 0x36,
    /// Usage ID `0x37`: "Dial"
    Dial = 0x37,
    /// Usage ID `0x38`: "Wheel"
    Wheel = 0x38,
    /// Usage ID `0x39`: "Hat Switch"
    HatSwitch = 0x39,
    /// Usage ID `0x3A`: "Counted Buffer"
    CountedBuffer = 0x3A,
    /// Usage ID `0x3B`: "Byte Count"
    ByteCount = 0x3B,
    /// Usage ID `0x3C`: "Motion Wakeup"
    MotionWakeup = 0x3C,
    /// Usage ID `0x3D`: "Start"
    Start = 0x3D,
    /// Usage ID `0x3E`: "Select"
    Select = 0x3E,
    /// Usage ID `0x40`: "Vx"
    Vx = 0x40,
    /// Usage ID `0x41`: "Vy"
    Vy = 0x41,
    /// Usage ID `0x42`: "Vz"
    Vz = 0x42,
    /// Usage ID `0x43`: "Vbrx"
    Vbrx = 0x43,
    /// Usage ID `0x44`: "Vbry"
    Vbry = 0x44,
    /// Usage ID `0x45`: "Vbrz"
    Vbrz = 0x45,
    /// Usage ID `0x46`: "Vno"
    Vno = 0x46,
    /// Usage ID `0x47`: "Feature Notification"
    FeatureNotification = 0x47,
    /// Usage ID `0x48`: "Resolution Multiplier"
    ResolutionMultiplier = 0x48,
    /// Usage ID `0x49`: "Qx"
    Qx = 0x49,
    /// Usage ID `0x4A`: "Qy"
    Qy = 0x4A,
    /// Usage ID `0x4B`: "Qz"
    Qz = 0x4B,
    /// Usage ID `0x4C`: "Qw"
    Qw = 0x4C,
    /// Usage ID `0x80`: "System Control"
    SystemControl = 0x80,
    /// Usage ID `0x81`: "System Power Down"
    SystemPowerDown = 0x81,
    /// Usage ID `0x82`: "System Sleep"
    SystemSleep = 0x82,
    /// Usage ID `0x83`: "System Wake Up"
    SystemWakeUp = 0x83,
    /// Usage ID `0x84`: "System Context Menu"
    SystemContextMenu = 0x84,
    /// Usage ID `0x85`: "System Main Menu"
    SystemMainMenu = 0x85,
    /// Usage ID `0x86`: "System App Menu"
    SystemAppMenu = 0x86,
    /// Usage ID `0x87`: "System Menu Help"
    SystemMenuHelp = 0x87,
    /// Usage ID `0x88`: "System Menu Exit"
    SystemMenuExit = 0x88,
    /// Usage ID `0x89`: "System Menu Select"
    SystemMenuSelect = 0x89,
    /// Usage ID `0x8A`: "System Menu Right"
    SystemMenuRight = 0x8A,
    /// Usage ID `0x8B`: "System Menu Left"
    SystemMenuLeft = 0x8B,
    /// Usage ID `0x8C`: "System Menu Up"
    SystemMenuUp = 0x8C,
    /// Usage ID `0x8D`: "System Menu Down"
    SystemMenuDown = 0x8D,
    /// Usage ID `0x8E`: "System Cold Restart"
    SystemColdRestart = 0x8E,
    /// Usage ID `0x8F`: "System Warm Restart"
    SystemWarmRestart = 0x8F,
    /// Usage ID `0x90`: "D-pad Up"
    DpadUp = 0x90,
    /// Usage ID `0x91`: "D-pad Down"
    DpadDown = 0x91,
    /// Usage ID `0x92`: "D-pad Right"
    DpadRight = 0x92,
    /// Usage ID `0x93`: "D-pad Left"
    DpadLeft = 0x93,
    /// Usage ID `0x94`: "Index Trigger"
    IndexTrigger = 0x94,
    /// Usage ID `0x95`: "Palm Trigger"
    PalmTrigger = 0x95,
    /// Usage ID `0x96`: "Thumbstick"
    Thumbstick = 0x96,
    /// Usage ID `0x97`: "System Function Shift"
    SystemFunctionShift = 0x97,
    /// Usage ID `0x98`: "System Function Shift Lock"
    SystemFunctionShiftLock = 0x98,
    /// Usage ID `0x99`: "System Function Shift Lock Indicator"
    SystemFunctionShiftLockIndicator = 0x99,
    /// Usage ID `0x9A`: "System Dismiss Notification"
    SystemDismissNotification = 0x9A,
    /// Usage ID `0x9B`: "System Do Not Disturb"
    SystemDoNotDisturb = 0x9B,
    /// Usage ID `0xA0`: "System Dock"
    SystemDock = 0xA0,
    /// Usage ID `0xA1`: "System Undock"
    SystemUndock = 0xA1,
    /// Usage ID `0xA2`: "System Setup"
    SystemSetup = 0xA2,
    /// Usage ID `0xA3`: "System Break"
    SystemBreak = 0xA3,
    /// Usage ID `0xA4`: "System Debugger Break"
    SystemDebuggerBreak = 0xA4,
    /// Usage ID `0xA5`: "Application Break"
    ApplicationBreak = 0xA5,
    /// Usage ID `0xA6`: "Application Debugger Break"
    ApplicationDebuggerBreak = 0xA6,
    /// Usage ID `0xA7`: "System Speaker Mute"
    SystemSpeakerMute = 0xA7,
    /// Usage ID `0xA8`: "System Hibernate"
    SystemHibernate = 0xA8,
    /// Usage ID `0xA9`: "System Microphone Mute"
    SystemMicrophoneMute = 0xA9,
    /// Usage ID `0xAA`: "System Accessibility Binding"
    SystemAccessibilityBinding = 0xAA,
    /// Usage ID `0xB0`: "System Display Invert"
    SystemDisplayInvert = 0xB0,
    /// Usage ID `0xB1`: "System Display Internal"
    SystemDisplayInternal = 0xB1,
    /// Usage ID `0xB2`: "System Display External"
    SystemDisplayExternal = 0xB2,
    /// Usage ID `0xB3`: "System Display Both"
    SystemDisplayBoth = 0xB3,
    /// Usage ID `0xB4`: "System Display Dual"
    SystemDisplayDual = 0xB4,
    /// Usage ID `0xB5`: "System Display Toggle Int/Ext Mode"
    SystemDisplayToggleIntExtMode = 0xB5,
    /// Usage ID `0xB6`: "System Display Swap Primary/Secondary"
    SystemDisplaySwapPrimarySecondary = 0xB6,
    /// Usage ID `0xB7`: "System Display Toggle LCD Autoscale"
    SystemDisplayToggleLCDAutoscale = 0xB7,
    /// Usage ID `0xC0`: "Sensor Zone"
    SensorZone = 0xC0,
    /// Usage ID `0xC1`: "RPM"
    RPM = 0xC1,
    /// Usage ID `0xC2`: "Coolant Level"
    CoolantLevel = 0xC2,
    /// Usage ID `0xC3`: "Coolant Critical Level"
    CoolantCriticalLevel = 0xC3,
    /// Usage ID `0xC4`: "Coolant Pump"
    CoolantPump = 0xC4,
    /// Usage ID `0xC5`: "Chassis Enclosure"
    ChassisEnclosure = 0xC5,
    /// Usage ID `0xC6`: "Wireless Radio Button"
    WirelessRadioButton = 0xC6,
    /// Usage ID `0xC7`: "Wireless Radio LED"
    WirelessRadioLED = 0xC7,
    /// Usage ID `0xC8`: "Wireless Radio Slider Switch"
    WirelessRadioSliderSwitch = 0xC8,
    /// Usage ID `0xC9`: "System Display Rotation Lock Button"
    SystemDisplayRotationLockButton = 0xC9,
    /// Usage ID `0xCA`: "System Display Rotation Lock Slider Switch"
    SystemDisplayRotationLockSliderSwitch = 0xCA,
    /// Usage ID `0xCB`: "Control Enable"
    ControlEnable = 0xCB,
    /// Usage ID `0xD0`: "Dockable Device Unique ID"
    DockableDeviceUniqueID = 0xD0,
    /// Usage ID `0xD1`: "Dockable Device Vendor ID"
    DockableDeviceVendorID = 0xD1,
    /// Usage ID `0xD2`: "Dockable Device Primary Usage Page"
    DockableDevicePrimaryUsagePage = 0xD2,
    /// Usage ID `0xD3`: "Dockable Device Primary Usage ID"
    DockableDevicePrimaryUsageID = 0xD3,
    /// Usage ID `0xD4`: "Dockable Device Docking State"
    DockableDeviceDockingState = 0xD4,
    /// Usage ID `0xD5`: "Dockable Device Display Occlusion"
    DockableDeviceDisplayOcclusion = 0xD5,
    /// Usage ID `0xD6`: "Dockable Device Object Type"
    DockableDeviceObjectType = 0xD6,
    /// Usage ID `0xE0`: "Call Active LED"
    CallActiveLED = 0xE0,
    /// Usage ID `0xE1`: "Call Mute Toggle"
    CallMuteToggle = 0xE1,
    /// Usage ID `0xE2`: "Call Mute LED"
    CallMuteLED = 0xE2,
}

impl GenericDesktop {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            GenericDesktop::Pointer => "Pointer",
            GenericDesktop::Mouse => "Mouse",
            GenericDesktop::Joystick => "Joystick",
            GenericDesktop::Gamepad => "Gamepad",
            GenericDesktop::Keyboard => "Keyboard",
            GenericDesktop::Keypad => "Keypad",
            GenericDesktop::MultiaxisController => "Multi-axis Controller",
            GenericDesktop::TabletPCSystemControls => "Tablet PC System Controls",
            GenericDesktop::WaterCoolingDevice => "Water Cooling Device",
            GenericDesktop::ComputerChassisDevice => "Computer Chassis Device",
            GenericDesktop::WirelessRadioControls => "Wireless Radio Controls",
            GenericDesktop::PortableDeviceControl => "Portable Device Control",
            GenericDesktop::SystemMultiAxisController => "System Multi-Axis Controller",
            GenericDesktop::SpatialController => "Spatial Controller",
            GenericDesktop::AssistiveControl => "Assistive Control",
            GenericDesktop::DeviceDock => "Device Dock",
            GenericDesktop::DockableDevice => "Dockable Device",
            GenericDesktop::CallStateManagementControl => "Call State Management Control",
            GenericDesktop::X => "X",
            GenericDesktop::Y => "Y",
            GenericDesktop::Z => "Z",
            GenericDesktop::Rx => "Rx",
            GenericDesktop::Ry => "Ry",
            GenericDesktop::Rz => "Rz",
            GenericDesktop::Slider => "Slider",
            GenericDesktop::Dial => "Dial",
            GenericDesktop::Wheel => "Wheel",
            GenericDesktop::HatSwitch => "Hat Switch",
            GenericDesktop::CountedBuffer => "Counted Buffer",
            GenericDesktop::ByteCount => "Byte Count",
            GenericDesktop::MotionWakeup => "Motion Wakeup",
            GenericDesktop::Start => "Start",
            GenericDesktop::Select => "Select",
            GenericDesktop::Vx => "Vx",
            GenericDesktop::Vy => "Vy",
            GenericDesktop::Vz => "Vz",
            GenericDesktop::Vbrx => "Vbrx",
            GenericDesktop::Vbry => "Vbry",
            GenericDesktop::Vbrz => "Vbrz",
            GenericDesktop::Vno => "Vno",
            GenericDesktop::FeatureNotification => "Feature Notification",
            GenericDesktop::ResolutionMultiplier => "Resolution Multiplier",
            GenericDesktop::Qx => "Qx",
            GenericDesktop::Qy => "Qy",
            GenericDesktop::Qz => "Qz",
            GenericDesktop::Qw => "Qw",
            GenericDesktop::SystemControl => "System Control",
            GenericDesktop::SystemPowerDown => "System Power Down",
            GenericDesktop::SystemSleep => "System Sleep",
            GenericDesktop::SystemWakeUp => "System Wake Up",
            GenericDesktop::SystemContextMenu => "System Context Menu",
            GenericDesktop::SystemMainMenu => "System Main Menu",
            GenericDesktop::SystemAppMenu => "System App Menu",
            GenericDesktop::SystemMenuHelp => "System Menu Help",
            GenericDesktop::SystemMenuExit => "System Menu Exit",
            GenericDesktop::SystemMenuSelect => "System Menu Select",
            GenericDesktop::SystemMenuRight => "System Menu Right",
            GenericDesktop::SystemMenuLeft => "System Menu Left",
            GenericDesktop::SystemMenuUp => "System Menu Up",
            GenericDesktop::SystemMenuDown => "System Menu Down",
            GenericDesktop::SystemColdRestart => "System Cold Restart",
            GenericDesktop::SystemWarmRestart => "System Warm Restart",
            GenericDesktop::DpadUp => "D-pad Up",
            GenericDesktop::DpadDown => "D-pad Down",
            GenericDesktop::DpadRight => "D-pad Right",
            GenericDesktop::DpadLeft => "D-pad Left",
            GenericDesktop::IndexTrigger => "Index Trigger",
            GenericDesktop::PalmTrigger => "Palm Trigger",
            GenericDesktop::Thumbstick => "Thumbstick",
            GenericDesktop::SystemFunctionShift => "System Function Shift",
            GenericDesktop::SystemFunctionShiftLock => "System Function Shift Lock",
            GenericDesktop::SystemFunctionShiftLockIndicator => {
                "System Function Shift Lock Indicator"
            }
            GenericDesktop::SystemDismissNotification => "System Dismiss Notification",
            GenericDesktop::SystemDoNotDisturb => "System Do Not Disturb",
            GenericDesktop::SystemDock => "System Dock",
            GenericDesktop::SystemUndock => "System Undock",
            GenericDesktop::SystemSetup => "System Setup",
            GenericDesktop::SystemBreak => "System Break",
            GenericDesktop::SystemDebuggerBreak => "System Debugger Break",
            GenericDesktop::ApplicationBreak => "Application Break",
            GenericDesktop::ApplicationDebuggerBreak => "Application Debugger Break",
            GenericDesktop::SystemSpeakerMute => "System Speaker Mute",
            GenericDesktop::SystemHibernate => "System Hibernate",
            GenericDesktop::SystemMicrophoneMute => "System Microphone Mute",
            GenericDesktop::SystemAccessibilityBinding => "System Accessibility Binding",
            GenericDesktop::SystemDisplayInvert => "System Display Invert",
            GenericDesktop::SystemDisplayInternal => "System Display Internal",
            GenericDesktop::SystemDisplayExternal => "System Display External",
            GenericDesktop::SystemDisplayBoth => "System Display Both",
            GenericDesktop::SystemDisplayDual => "System Display Dual",
            GenericDesktop::SystemDisplayToggleIntExtMode => "System Display Toggle Int/Ext Mode",
            GenericDesktop::SystemDisplaySwapPrimarySecondary => {
                "System Display Swap Primary/Secondary"
            }
            GenericDesktop::SystemDisplayToggleLCDAutoscale => {
                "System Display Toggle LCD Autoscale"
            }
            GenericDesktop::SensorZone => "Sensor Zone",
            GenericDesktop::RPM => "RPM",
            GenericDesktop::CoolantLevel => "Coolant Level",
            GenericDesktop::CoolantCriticalLevel => "Coolant Critical Level",
            GenericDesktop::CoolantPump => "Coolant Pump",
            GenericDesktop::ChassisEnclosure => "Chassis Enclosure",
            GenericDesktop::WirelessRadioButton => "Wireless Radio Button",
            GenericDesktop::WirelessRadioLED => "Wireless Radio LED",
            GenericDesktop::WirelessRadioSliderSwitch => "Wireless Radio Slider Switch",
            GenericDesktop::SystemDisplayRotationLockButton => {
                "System Display Rotation Lock Button"
            }
            GenericDesktop::SystemDisplayRotationLockSliderSwitch => {
                "System Display Rotation Lock Slider Switch"
            }
            GenericDesktop::ControlEnable => "Control Enable",
            GenericDesktop::DockableDeviceUniqueID => "Dockable Device Unique ID",
            GenericDesktop::DockableDeviceVendorID => "Dockable Device Vendor ID",
            GenericDesktop::DockableDevicePrimaryUsagePage => "Dockable Device Primary Usage Page",
            GenericDesktop::DockableDevicePrimaryUsageID => "Dockable Device Primary Usage ID",
            GenericDesktop::DockableDeviceDockingState => "Dockable Device Docking State",
            GenericDesktop::DockableDeviceDisplayOcclusion => "Dockable Device Display Occlusion",
            GenericDesktop::DockableDeviceObjectType => "Dockable Device Object Type",
            GenericDesktop::CallActiveLED => "Call Active LED",
            GenericDesktop::CallMuteToggle => "Call Mute Toggle",
            GenericDesktop::CallMuteLED => "Call Mute LED",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for GenericDesktop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for GenericDesktop {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::GenericDesktop(self)](Usage::GenericDesktop)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for GenericDesktop {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x1` for [GenericDesktop]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::GenericDesktop]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&GenericDesktop> for u16 {
    fn from(genericdesktop: &GenericDesktop) -> u16 {
        *genericdesktop as u16
    }
}

impl From<GenericDesktop> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [GenericDesktop::usage_page_value()].
    fn from(genericdesktop: GenericDesktop) -> u16 {
        u16::from(&genericdesktop)
    }
}

impl From<&GenericDesktop> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [GenericDesktop::usage_value()].
    fn from(genericdesktop: &GenericDesktop) -> u32 {
        let up = UsagePage::from(genericdesktop);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(genericdesktop) as u32;
        up | id
    }
}

impl From<&GenericDesktop> for UsagePage {
    /// Always returns [UsagePage::GenericDesktop] and is
    /// identical to [GenericDesktop::usage_page()].
    fn from(_: &GenericDesktop) -> UsagePage {
        UsagePage::GenericDesktop
    }
}

impl From<GenericDesktop> for UsagePage {
    /// Always returns [UsagePage::GenericDesktop] and is
    /// identical to [GenericDesktop::usage_page()].
    fn from(_: GenericDesktop) -> UsagePage {
        UsagePage::GenericDesktop
    }
}

impl From<&GenericDesktop> for Usage {
    fn from(genericdesktop: &GenericDesktop) -> Usage {
        Usage::try_from(u32::from(genericdesktop)).unwrap()
    }
}

impl From<GenericDesktop> for Usage {
    fn from(genericdesktop: GenericDesktop) -> Usage {
        Usage::from(&genericdesktop)
    }
}

impl TryFrom<u16> for GenericDesktop {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<GenericDesktop> {
        match usage_id {
            1 => Ok(GenericDesktop::Pointer),
            2 => Ok(GenericDesktop::Mouse),
            4 => Ok(GenericDesktop::Joystick),
            5 => Ok(GenericDesktop::Gamepad),
            6 => Ok(GenericDesktop::Keyboard),
            7 => Ok(GenericDesktop::Keypad),
            8 => Ok(GenericDesktop::MultiaxisController),
            9 => Ok(GenericDesktop::TabletPCSystemControls),
            10 => Ok(GenericDesktop::WaterCoolingDevice),
            11 => Ok(GenericDesktop::ComputerChassisDevice),
            12 => Ok(GenericDesktop::WirelessRadioControls),
            13 => Ok(GenericDesktop::PortableDeviceControl),
            14 => Ok(GenericDesktop::SystemMultiAxisController),
            15 => Ok(GenericDesktop::SpatialController),
            16 => Ok(GenericDesktop::AssistiveControl),
            17 => Ok(GenericDesktop::DeviceDock),
            18 => Ok(GenericDesktop::DockableDevice),
            19 => Ok(GenericDesktop::CallStateManagementControl),
            48 => Ok(GenericDesktop::X),
            49 => Ok(GenericDesktop::Y),
            50 => Ok(GenericDesktop::Z),
            51 => Ok(GenericDesktop::Rx),
            52 => Ok(GenericDesktop::Ry),
            53 => Ok(GenericDesktop::Rz),
            54 => Ok(GenericDesktop::Slider),
            55 => Ok(GenericDesktop::Dial),
            56 => Ok(GenericDesktop::Wheel),
            57 => Ok(GenericDesktop::HatSwitch),
            58 => Ok(GenericDesktop::CountedBuffer),
            59 => Ok(GenericDesktop::ByteCount),
            60 => Ok(GenericDesktop::MotionWakeup),
            61 => Ok(GenericDesktop::Start),
            62 => Ok(GenericDesktop::Select),
            64 => Ok(GenericDesktop::Vx),
            65 => Ok(GenericDesktop::Vy),
            66 => Ok(GenericDesktop::Vz),
            67 => Ok(GenericDesktop::Vbrx),
            68 => Ok(GenericDesktop::Vbry),
            69 => Ok(GenericDesktop::Vbrz),
            70 => Ok(GenericDesktop::Vno),
            71 => Ok(GenericDesktop::FeatureNotification),
            72 => Ok(GenericDesktop::ResolutionMultiplier),
            73 => Ok(GenericDesktop::Qx),
            74 => Ok(GenericDesktop::Qy),
            75 => Ok(GenericDesktop::Qz),
            76 => Ok(GenericDesktop::Qw),
            128 => Ok(GenericDesktop::SystemControl),
            129 => Ok(GenericDesktop::SystemPowerDown),
            130 => Ok(GenericDesktop::SystemSleep),
            131 => Ok(GenericDesktop::SystemWakeUp),
            132 => Ok(GenericDesktop::SystemContextMenu),
            133 => Ok(GenericDesktop::SystemMainMenu),
            134 => Ok(GenericDesktop::SystemAppMenu),
            135 => Ok(GenericDesktop::SystemMenuHelp),
            136 => Ok(GenericDesktop::SystemMenuExit),
            137 => Ok(GenericDesktop::SystemMenuSelect),
            138 => Ok(GenericDesktop::SystemMenuRight),
            139 => Ok(GenericDesktop::SystemMenuLeft),
            140 => Ok(GenericDesktop::SystemMenuUp),
            141 => Ok(GenericDesktop::SystemMenuDown),
            142 => Ok(GenericDesktop::SystemColdRestart),
            143 => Ok(GenericDesktop::SystemWarmRestart),
            144 => Ok(GenericDesktop::DpadUp),
            145 => Ok(GenericDesktop::DpadDown),
            146 => Ok(GenericDesktop::DpadRight),
            147 => Ok(GenericDesktop::DpadLeft),
            148 => Ok(GenericDesktop::IndexTrigger),
            149 => Ok(GenericDesktop::PalmTrigger),
            150 => Ok(GenericDesktop::Thumbstick),
            151 => Ok(GenericDesktop::SystemFunctionShift),
            152 => Ok(GenericDesktop::SystemFunctionShiftLock),
            153 => Ok(GenericDesktop::SystemFunctionShiftLockIndicator),
            154 => Ok(GenericDesktop::SystemDismissNotification),
            155 => Ok(GenericDesktop::SystemDoNotDisturb),
            160 => Ok(GenericDesktop::SystemDock),
            161 => Ok(GenericDesktop::SystemUndock),
            162 => Ok(GenericDesktop::SystemSetup),
            163 => Ok(GenericDesktop::SystemBreak),
            164 => Ok(GenericDesktop::SystemDebuggerBreak),
            165 => Ok(GenericDesktop::ApplicationBreak),
            166 => Ok(GenericDesktop::ApplicationDebuggerBreak),
            167 => Ok(GenericDesktop::SystemSpeakerMute),
            168 => Ok(GenericDesktop::SystemHibernate),
            169 => Ok(GenericDesktop::SystemMicrophoneMute),
            170 => Ok(GenericDesktop::SystemAccessibilityBinding),
            176 => Ok(GenericDesktop::SystemDisplayInvert),
            177 => Ok(GenericDesktop::SystemDisplayInternal),
            178 => Ok(GenericDesktop::SystemDisplayExternal),
            179 => Ok(GenericDesktop::SystemDisplayBoth),
            180 => Ok(GenericDesktop::SystemDisplayDual),
            181 => Ok(GenericDesktop::SystemDisplayToggleIntExtMode),
            182 => Ok(GenericDesktop::SystemDisplaySwapPrimarySecondary),
            183 => Ok(GenericDesktop::SystemDisplayToggleLCDAutoscale),
            192 => Ok(GenericDesktop::SensorZone),
            193 => Ok(GenericDesktop::RPM),
            194 => Ok(GenericDesktop::CoolantLevel),
            195 => Ok(GenericDesktop::CoolantCriticalLevel),
            196 => Ok(GenericDesktop::CoolantPump),
            197 => Ok(GenericDesktop::ChassisEnclosure),
            198 => Ok(GenericDesktop::WirelessRadioButton),
            199 => Ok(GenericDesktop::WirelessRadioLED),
            200 => Ok(GenericDesktop::WirelessRadioSliderSwitch),
            201 => Ok(GenericDesktop::SystemDisplayRotationLockButton),
            202 => Ok(GenericDesktop::SystemDisplayRotationLockSliderSwitch),
            203 => Ok(GenericDesktop::ControlEnable),
            208 => Ok(GenericDesktop::DockableDeviceUniqueID),
            209 => Ok(GenericDesktop::DockableDeviceVendorID),
            210 => Ok(GenericDesktop::DockableDevicePrimaryUsagePage),
            211 => Ok(GenericDesktop::DockableDevicePrimaryUsageID),
            212 => Ok(GenericDesktop::DockableDeviceDockingState),
            213 => Ok(GenericDesktop::DockableDeviceDisplayOcclusion),
            214 => Ok(GenericDesktop::DockableDeviceObjectType),
            224 => Ok(GenericDesktop::CallActiveLED),
            225 => Ok(GenericDesktop::CallMuteToggle),
            226 => Ok(GenericDesktop::CallMuteLED),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for GenericDesktop {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x2`: "Simulation Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::SimulationControls(SimulationControls::AutomobileSimulationDevice);
/// let u2 = Usage::new_from_page_and_id(0x2, 0x2).unwrap();
/// let u3 = Usage::from(SimulationControls::AutomobileSimulationDevice);
/// let u4: Usage = SimulationControls::AutomobileSimulationDevice.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::SimulationControls));
/// assert_eq!(0x2, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x2 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Automobile Simulation Device", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum SimulationControls {
    /// Usage ID `0x1`: "Flight Simulation Device"
    FlightSimulationDevice = 0x1,
    /// Usage ID `0x2`: "Automobile Simulation Device"
    AutomobileSimulationDevice = 0x2,
    /// Usage ID `0x3`: "Tank Simulation Device"
    TankSimulationDevice = 0x3,
    /// Usage ID `0x4`: "Spaceship Simulation Device"
    SpaceshipSimulationDevice = 0x4,
    /// Usage ID `0x5`: "Submarine Simulation Device"
    SubmarineSimulationDevice = 0x5,
    /// Usage ID `0x6`: "Sailing Simulation Device"
    SailingSimulationDevice = 0x6,
    /// Usage ID `0x7`: "Motorcycle Simulation Device"
    MotorcycleSimulationDevice = 0x7,
    /// Usage ID `0x8`: "Sports Simulation Device"
    SportsSimulationDevice = 0x8,
    /// Usage ID `0x9`: "Airplane Simulation Device"
    AirplaneSimulationDevice = 0x9,
    /// Usage ID `0xA`: "Helicopter Simulation Device"
    HelicopterSimulationDevice = 0xA,
    /// Usage ID `0xB`: "Magic Carpet Simulation Device"
    MagicCarpetSimulationDevice = 0xB,
    /// Usage ID `0xC`: "Bicycle Simulation Device"
    BicycleSimulationDevice = 0xC,
    /// Usage ID `0x20`: "Flight Control Stick"
    FlightControlStick = 0x20,
    /// Usage ID `0x21`: "Flight Stick"
    FlightStick = 0x21,
    /// Usage ID `0x22`: "Cyclic Control"
    CyclicControl = 0x22,
    /// Usage ID `0x23`: "Cyclic Trim"
    CyclicTrim = 0x23,
    /// Usage ID `0x24`: "Flight Yoke"
    FlightYoke = 0x24,
    /// Usage ID `0x25`: "Track Control"
    TrackControl = 0x25,
    /// Usage ID `0xB0`: "Aileron"
    Aileron = 0xB0,
    /// Usage ID `0xB1`: "Aileron Trim"
    AileronTrim = 0xB1,
    /// Usage ID `0xB2`: "Anti-Torque Control"
    AntiTorqueControl = 0xB2,
    /// Usage ID `0xB3`: "Autopilot Enable"
    AutopilotEnable = 0xB3,
    /// Usage ID `0xB4`: "Chaff Release"
    ChaffRelease = 0xB4,
    /// Usage ID `0xB5`: "Collective Control"
    CollectiveControl = 0xB5,
    /// Usage ID `0xB6`: "Dive Brake"
    DiveBrake = 0xB6,
    /// Usage ID `0xB7`: "Electronic Countermeasures"
    ElectronicCountermeasures = 0xB7,
    /// Usage ID `0xB8`: "Elevator"
    Elevator = 0xB8,
    /// Usage ID `0xB9`: "Elevator Trim"
    ElevatorTrim = 0xB9,
    /// Usage ID `0xBA`: "Rudder"
    Rudder = 0xBA,
    /// Usage ID `0xBB`: "Throttle"
    Throttle = 0xBB,
    /// Usage ID `0xBC`: "Flight Communications"
    FlightCommunications = 0xBC,
    /// Usage ID `0xBD`: "Flare Release"
    FlareRelease = 0xBD,
    /// Usage ID `0xBE`: "Landing Gear"
    LandingGear = 0xBE,
    /// Usage ID `0xBF`: "Toe Brake"
    ToeBrake = 0xBF,
    /// Usage ID `0xC0`: "Trigger"
    Trigger = 0xC0,
    /// Usage ID `0xC1`: "Weapons Arm"
    WeaponsArm = 0xC1,
    /// Usage ID `0xC2`: "Weapons Select"
    WeaponsSelect = 0xC2,
    /// Usage ID `0xC3`: "Wing Flaps"
    WingFlaps = 0xC3,
    /// Usage ID `0xC4`: "Accelerator"
    Accelerator = 0xC4,
    /// Usage ID `0xC5`: "Brake"
    Brake = 0xC5,
    /// Usage ID `0xC6`: "Clutch"
    Clutch = 0xC6,
    /// Usage ID `0xC7`: "Shifter"
    Shifter = 0xC7,
    /// Usage ID `0xC8`: "Steering"
    Steering = 0xC8,
    /// Usage ID `0xC9`: "Turret Direction"
    TurretDirection = 0xC9,
    /// Usage ID `0xCA`: "Barrel Elevation"
    BarrelElevation = 0xCA,
    /// Usage ID `0xCB`: "Dive Plane"
    DivePlane = 0xCB,
    /// Usage ID `0xCC`: "Ballast"
    Ballast = 0xCC,
    /// Usage ID `0xCD`: "Bicycle Crank"
    BicycleCrank = 0xCD,
    /// Usage ID `0xCE`: "Handle Bars"
    HandleBars = 0xCE,
    /// Usage ID `0xCF`: "Front Brake"
    FrontBrake = 0xCF,
    /// Usage ID `0xD0`: "Rear Brake"
    RearBrake = 0xD0,
}

impl SimulationControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            SimulationControls::FlightSimulationDevice => "Flight Simulation Device",
            SimulationControls::AutomobileSimulationDevice => "Automobile Simulation Device",
            SimulationControls::TankSimulationDevice => "Tank Simulation Device",
            SimulationControls::SpaceshipSimulationDevice => "Spaceship Simulation Device",
            SimulationControls::SubmarineSimulationDevice => "Submarine Simulation Device",
            SimulationControls::SailingSimulationDevice => "Sailing Simulation Device",
            SimulationControls::MotorcycleSimulationDevice => "Motorcycle Simulation Device",
            SimulationControls::SportsSimulationDevice => "Sports Simulation Device",
            SimulationControls::AirplaneSimulationDevice => "Airplane Simulation Device",
            SimulationControls::HelicopterSimulationDevice => "Helicopter Simulation Device",
            SimulationControls::MagicCarpetSimulationDevice => "Magic Carpet Simulation Device",
            SimulationControls::BicycleSimulationDevice => "Bicycle Simulation Device",
            SimulationControls::FlightControlStick => "Flight Control Stick",
            SimulationControls::FlightStick => "Flight Stick",
            SimulationControls::CyclicControl => "Cyclic Control",
            SimulationControls::CyclicTrim => "Cyclic Trim",
            SimulationControls::FlightYoke => "Flight Yoke",
            SimulationControls::TrackControl => "Track Control",
            SimulationControls::Aileron => "Aileron",
            SimulationControls::AileronTrim => "Aileron Trim",
            SimulationControls::AntiTorqueControl => "Anti-Torque Control",
            SimulationControls::AutopilotEnable => "Autopilot Enable",
            SimulationControls::ChaffRelease => "Chaff Release",
            SimulationControls::CollectiveControl => "Collective Control",
            SimulationControls::DiveBrake => "Dive Brake",
            SimulationControls::ElectronicCountermeasures => "Electronic Countermeasures",
            SimulationControls::Elevator => "Elevator",
            SimulationControls::ElevatorTrim => "Elevator Trim",
            SimulationControls::Rudder => "Rudder",
            SimulationControls::Throttle => "Throttle",
            SimulationControls::FlightCommunications => "Flight Communications",
            SimulationControls::FlareRelease => "Flare Release",
            SimulationControls::LandingGear => "Landing Gear",
            SimulationControls::ToeBrake => "Toe Brake",
            SimulationControls::Trigger => "Trigger",
            SimulationControls::WeaponsArm => "Weapons Arm",
            SimulationControls::WeaponsSelect => "Weapons Select",
            SimulationControls::WingFlaps => "Wing Flaps",
            SimulationControls::Accelerator => "Accelerator",
            SimulationControls::Brake => "Brake",
            SimulationControls::Clutch => "Clutch",
            SimulationControls::Shifter => "Shifter",
            SimulationControls::Steering => "Steering",
            SimulationControls::TurretDirection => "Turret Direction",
            SimulationControls::BarrelElevation => "Barrel Elevation",
            SimulationControls::DivePlane => "Dive Plane",
            SimulationControls::Ballast => "Ballast",
            SimulationControls::BicycleCrank => "Bicycle Crank",
            SimulationControls::HandleBars => "Handle Bars",
            SimulationControls::FrontBrake => "Front Brake",
            SimulationControls::RearBrake => "Rear Brake",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SimulationControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for SimulationControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::SimulationControls(self)](Usage::SimulationControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for SimulationControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x2` for [SimulationControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::SimulationControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&SimulationControls> for u16 {
    fn from(simulationcontrols: &SimulationControls) -> u16 {
        *simulationcontrols as u16
    }
}

impl From<SimulationControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [SimulationControls::usage_page_value()].
    fn from(simulationcontrols: SimulationControls) -> u16 {
        u16::from(&simulationcontrols)
    }
}

impl From<&SimulationControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [SimulationControls::usage_value()].
    fn from(simulationcontrols: &SimulationControls) -> u32 {
        let up = UsagePage::from(simulationcontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(simulationcontrols) as u32;
        up | id
    }
}

impl From<&SimulationControls> for UsagePage {
    /// Always returns [UsagePage::SimulationControls] and is
    /// identical to [SimulationControls::usage_page()].
    fn from(_: &SimulationControls) -> UsagePage {
        UsagePage::SimulationControls
    }
}

impl From<SimulationControls> for UsagePage {
    /// Always returns [UsagePage::SimulationControls] and is
    /// identical to [SimulationControls::usage_page()].
    fn from(_: SimulationControls) -> UsagePage {
        UsagePage::SimulationControls
    }
}

impl From<&SimulationControls> for Usage {
    fn from(simulationcontrols: &SimulationControls) -> Usage {
        Usage::try_from(u32::from(simulationcontrols)).unwrap()
    }
}

impl From<SimulationControls> for Usage {
    fn from(simulationcontrols: SimulationControls) -> Usage {
        Usage::from(&simulationcontrols)
    }
}

impl TryFrom<u16> for SimulationControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<SimulationControls> {
        match usage_id {
            1 => Ok(SimulationControls::FlightSimulationDevice),
            2 => Ok(SimulationControls::AutomobileSimulationDevice),
            3 => Ok(SimulationControls::TankSimulationDevice),
            4 => Ok(SimulationControls::SpaceshipSimulationDevice),
            5 => Ok(SimulationControls::SubmarineSimulationDevice),
            6 => Ok(SimulationControls::SailingSimulationDevice),
            7 => Ok(SimulationControls::MotorcycleSimulationDevice),
            8 => Ok(SimulationControls::SportsSimulationDevice),
            9 => Ok(SimulationControls::AirplaneSimulationDevice),
            10 => Ok(SimulationControls::HelicopterSimulationDevice),
            11 => Ok(SimulationControls::MagicCarpetSimulationDevice),
            12 => Ok(SimulationControls::BicycleSimulationDevice),
            32 => Ok(SimulationControls::FlightControlStick),
            33 => Ok(SimulationControls::FlightStick),
            34 => Ok(SimulationControls::CyclicControl),
            35 => Ok(SimulationControls::CyclicTrim),
            36 => Ok(SimulationControls::FlightYoke),
            37 => Ok(SimulationControls::TrackControl),
            176 => Ok(SimulationControls::Aileron),
            177 => Ok(SimulationControls::AileronTrim),
            178 => Ok(SimulationControls::AntiTorqueControl),
            179 => Ok(SimulationControls::AutopilotEnable),
            180 => Ok(SimulationControls::ChaffRelease),
            181 => Ok(SimulationControls::CollectiveControl),
            182 => Ok(SimulationControls::DiveBrake),
            183 => Ok(SimulationControls::ElectronicCountermeasures),
            184 => Ok(SimulationControls::Elevator),
            185 => Ok(SimulationControls::ElevatorTrim),
            186 => Ok(SimulationControls::Rudder),
            187 => Ok(SimulationControls::Throttle),
            188 => Ok(SimulationControls::FlightCommunications),
            189 => Ok(SimulationControls::FlareRelease),
            190 => Ok(SimulationControls::LandingGear),
            191 => Ok(SimulationControls::ToeBrake),
            192 => Ok(SimulationControls::Trigger),
            193 => Ok(SimulationControls::WeaponsArm),
            194 => Ok(SimulationControls::WeaponsSelect),
            195 => Ok(SimulationControls::WingFlaps),
            196 => Ok(SimulationControls::Accelerator),
            197 => Ok(SimulationControls::Brake),
            198 => Ok(SimulationControls::Clutch),
            199 => Ok(SimulationControls::Shifter),
            200 => Ok(SimulationControls::Steering),
            201 => Ok(SimulationControls::TurretDirection),
            202 => Ok(SimulationControls::BarrelElevation),
            203 => Ok(SimulationControls::DivePlane),
            204 => Ok(SimulationControls::Ballast),
            205 => Ok(SimulationControls::BicycleCrank),
            206 => Ok(SimulationControls::HandleBars),
            207 => Ok(SimulationControls::FrontBrake),
            208 => Ok(SimulationControls::RearBrake),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for SimulationControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x3`: "VR Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::VRControls(VRControls::BodySuit);
/// let u2 = Usage::new_from_page_and_id(0x3, 0x2).unwrap();
/// let u3 = Usage::from(VRControls::BodySuit);
/// let u4: Usage = VRControls::BodySuit.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::VRControls));
/// assert_eq!(0x3, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x3 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Body Suit", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum VRControls {
    /// Usage ID `0x1`: "Belt"
    Belt = 0x1,
    /// Usage ID `0x2`: "Body Suit"
    BodySuit = 0x2,
    /// Usage ID `0x3`: "Flexor"
    Flexor = 0x3,
    /// Usage ID `0x4`: "Glove"
    Glove = 0x4,
    /// Usage ID `0x5`: "Head Tracker"
    HeadTracker = 0x5,
    /// Usage ID `0x6`: "Head Mounted Display"
    HeadMountedDisplay = 0x6,
    /// Usage ID `0x7`: "Hand Tracker"
    HandTracker = 0x7,
    /// Usage ID `0x8`: "Oculometer"
    Oculometer = 0x8,
    /// Usage ID `0x9`: "Vest"
    Vest = 0x9,
    /// Usage ID `0xA`: "Animatronic Device"
    AnimatronicDevice = 0xA,
    /// Usage ID `0x20`: "Stereo Enable"
    StereoEnable = 0x20,
    /// Usage ID `0x21`: "Display Enable"
    DisplayEnable = 0x21,
}

impl VRControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            VRControls::Belt => "Belt",
            VRControls::BodySuit => "Body Suit",
            VRControls::Flexor => "Flexor",
            VRControls::Glove => "Glove",
            VRControls::HeadTracker => "Head Tracker",
            VRControls::HeadMountedDisplay => "Head Mounted Display",
            VRControls::HandTracker => "Hand Tracker",
            VRControls::Oculometer => "Oculometer",
            VRControls::Vest => "Vest",
            VRControls::AnimatronicDevice => "Animatronic Device",
            VRControls::StereoEnable => "Stereo Enable",
            VRControls::DisplayEnable => "Display Enable",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for VRControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for VRControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::VRControls(self)](Usage::VRControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for VRControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x3` for [VRControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::VRControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&VRControls> for u16 {
    fn from(vrcontrols: &VRControls) -> u16 {
        *vrcontrols as u16
    }
}

impl From<VRControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [VRControls::usage_page_value()].
    fn from(vrcontrols: VRControls) -> u16 {
        u16::from(&vrcontrols)
    }
}

impl From<&VRControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [VRControls::usage_value()].
    fn from(vrcontrols: &VRControls) -> u32 {
        let up = UsagePage::from(vrcontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(vrcontrols) as u32;
        up | id
    }
}

impl From<&VRControls> for UsagePage {
    /// Always returns [UsagePage::VRControls] and is
    /// identical to [VRControls::usage_page()].
    fn from(_: &VRControls) -> UsagePage {
        UsagePage::VRControls
    }
}

impl From<VRControls> for UsagePage {
    /// Always returns [UsagePage::VRControls] and is
    /// identical to [VRControls::usage_page()].
    fn from(_: VRControls) -> UsagePage {
        UsagePage::VRControls
    }
}

impl From<&VRControls> for Usage {
    fn from(vrcontrols: &VRControls) -> Usage {
        Usage::try_from(u32::from(vrcontrols)).unwrap()
    }
}

impl From<VRControls> for Usage {
    fn from(vrcontrols: VRControls) -> Usage {
        Usage::from(&vrcontrols)
    }
}

impl TryFrom<u16> for VRControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<VRControls> {
        match usage_id {
            1 => Ok(VRControls::Belt),
            2 => Ok(VRControls::BodySuit),
            3 => Ok(VRControls::Flexor),
            4 => Ok(VRControls::Glove),
            5 => Ok(VRControls::HeadTracker),
            6 => Ok(VRControls::HeadMountedDisplay),
            7 => Ok(VRControls::HandTracker),
            8 => Ok(VRControls::Oculometer),
            9 => Ok(VRControls::Vest),
            10 => Ok(VRControls::AnimatronicDevice),
            32 => Ok(VRControls::StereoEnable),
            33 => Ok(VRControls::DisplayEnable),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for VRControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x4`: "Sport Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::SportControls(SportControls::GolfClub);
/// let u2 = Usage::new_from_page_and_id(0x4, 0x2).unwrap();
/// let u3 = Usage::from(SportControls::GolfClub);
/// let u4: Usage = SportControls::GolfClub.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::SportControls));
/// assert_eq!(0x4, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x4 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Golf Club", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum SportControls {
    /// Usage ID `0x1`: "Baseball Bat"
    BaseballBat = 0x1,
    /// Usage ID `0x2`: "Golf Club"
    GolfClub = 0x2,
    /// Usage ID `0x3`: "Rowing Machine"
    RowingMachine = 0x3,
    /// Usage ID `0x4`: "Treadmill"
    Treadmill = 0x4,
    /// Usage ID `0x30`: "Oar"
    Oar = 0x30,
    /// Usage ID `0x31`: "Slope"
    Slope = 0x31,
    /// Usage ID `0x32`: "Rate"
    Rate = 0x32,
    /// Usage ID `0x33`: "Stick Speed"
    StickSpeed = 0x33,
    /// Usage ID `0x34`: "Stick Face Angle"
    StickFaceAngle = 0x34,
    /// Usage ID `0x35`: "Stick Heel/Toe"
    StickHeelToe = 0x35,
    /// Usage ID `0x36`: "Stick Follow Through"
    StickFollowThrough = 0x36,
    /// Usage ID `0x37`: "Stick Tempo"
    StickTempo = 0x37,
    /// Usage ID `0x38`: "Stick Type"
    StickType = 0x38,
    /// Usage ID `0x39`: "Stick Height"
    StickHeight = 0x39,
    /// Usage ID `0x50`: "Putter"
    Putter = 0x50,
    /// Usage ID `0x51`: "1 Iron"
    OneIron = 0x51,
    /// Usage ID `0x52`: "2 Iron"
    TwoIron = 0x52,
    /// Usage ID `0x53`: "3 Iron"
    ThreeIron = 0x53,
    /// Usage ID `0x54`: "4 Iron"
    FourIron = 0x54,
    /// Usage ID `0x55`: "5 Iron"
    FiveIron = 0x55,
    /// Usage ID `0x56`: "6 Iron"
    SixIron = 0x56,
    /// Usage ID `0x57`: "7 Iron"
    SevenIron = 0x57,
    /// Usage ID `0x58`: "8 Iron"
    EightIron = 0x58,
    /// Usage ID `0x59`: "9 Iron"
    NineIron = 0x59,
    /// Usage ID `0x5A`: "10 Iron"
    One0Iron = 0x5A,
    /// Usage ID `0x5B`: "11 Iron"
    One1Iron = 0x5B,
    /// Usage ID `0x5C`: "Sand Wedge"
    SandWedge = 0x5C,
    /// Usage ID `0x5D`: "Loft Wedge"
    LoftWedge = 0x5D,
    /// Usage ID `0x5E`: "Power Wedge"
    PowerWedge = 0x5E,
    /// Usage ID `0x5F`: "1 Wood"
    OneWood = 0x5F,
    /// Usage ID `0x60`: "3 Wood"
    ThreeWood = 0x60,
    /// Usage ID `0x61`: "5 Wood"
    FiveWood = 0x61,
    /// Usage ID `0x62`: "7 Wood"
    SevenWood = 0x62,
    /// Usage ID `0x63`: "9 Wood"
    NineWood = 0x63,
}

impl SportControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            SportControls::BaseballBat => "Baseball Bat",
            SportControls::GolfClub => "Golf Club",
            SportControls::RowingMachine => "Rowing Machine",
            SportControls::Treadmill => "Treadmill",
            SportControls::Oar => "Oar",
            SportControls::Slope => "Slope",
            SportControls::Rate => "Rate",
            SportControls::StickSpeed => "Stick Speed",
            SportControls::StickFaceAngle => "Stick Face Angle",
            SportControls::StickHeelToe => "Stick Heel/Toe",
            SportControls::StickFollowThrough => "Stick Follow Through",
            SportControls::StickTempo => "Stick Tempo",
            SportControls::StickType => "Stick Type",
            SportControls::StickHeight => "Stick Height",
            SportControls::Putter => "Putter",
            SportControls::OneIron => "1 Iron",
            SportControls::TwoIron => "2 Iron",
            SportControls::ThreeIron => "3 Iron",
            SportControls::FourIron => "4 Iron",
            SportControls::FiveIron => "5 Iron",
            SportControls::SixIron => "6 Iron",
            SportControls::SevenIron => "7 Iron",
            SportControls::EightIron => "8 Iron",
            SportControls::NineIron => "9 Iron",
            SportControls::One0Iron => "10 Iron",
            SportControls::One1Iron => "11 Iron",
            SportControls::SandWedge => "Sand Wedge",
            SportControls::LoftWedge => "Loft Wedge",
            SportControls::PowerWedge => "Power Wedge",
            SportControls::OneWood => "1 Wood",
            SportControls::ThreeWood => "3 Wood",
            SportControls::FiveWood => "5 Wood",
            SportControls::SevenWood => "7 Wood",
            SportControls::NineWood => "9 Wood",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SportControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for SportControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::SportControls(self)](Usage::SportControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for SportControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x4` for [SportControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::SportControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&SportControls> for u16 {
    fn from(sportcontrols: &SportControls) -> u16 {
        *sportcontrols as u16
    }
}

impl From<SportControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [SportControls::usage_page_value()].
    fn from(sportcontrols: SportControls) -> u16 {
        u16::from(&sportcontrols)
    }
}

impl From<&SportControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [SportControls::usage_value()].
    fn from(sportcontrols: &SportControls) -> u32 {
        let up = UsagePage::from(sportcontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(sportcontrols) as u32;
        up | id
    }
}

impl From<&SportControls> for UsagePage {
    /// Always returns [UsagePage::SportControls] and is
    /// identical to [SportControls::usage_page()].
    fn from(_: &SportControls) -> UsagePage {
        UsagePage::SportControls
    }
}

impl From<SportControls> for UsagePage {
    /// Always returns [UsagePage::SportControls] and is
    /// identical to [SportControls::usage_page()].
    fn from(_: SportControls) -> UsagePage {
        UsagePage::SportControls
    }
}

impl From<&SportControls> for Usage {
    fn from(sportcontrols: &SportControls) -> Usage {
        Usage::try_from(u32::from(sportcontrols)).unwrap()
    }
}

impl From<SportControls> for Usage {
    fn from(sportcontrols: SportControls) -> Usage {
        Usage::from(&sportcontrols)
    }
}

impl TryFrom<u16> for SportControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<SportControls> {
        match usage_id {
            1 => Ok(SportControls::BaseballBat),
            2 => Ok(SportControls::GolfClub),
            3 => Ok(SportControls::RowingMachine),
            4 => Ok(SportControls::Treadmill),
            48 => Ok(SportControls::Oar),
            49 => Ok(SportControls::Slope),
            50 => Ok(SportControls::Rate),
            51 => Ok(SportControls::StickSpeed),
            52 => Ok(SportControls::StickFaceAngle),
            53 => Ok(SportControls::StickHeelToe),
            54 => Ok(SportControls::StickFollowThrough),
            55 => Ok(SportControls::StickTempo),
            56 => Ok(SportControls::StickType),
            57 => Ok(SportControls::StickHeight),
            80 => Ok(SportControls::Putter),
            81 => Ok(SportControls::OneIron),
            82 => Ok(SportControls::TwoIron),
            83 => Ok(SportControls::ThreeIron),
            84 => Ok(SportControls::FourIron),
            85 => Ok(SportControls::FiveIron),
            86 => Ok(SportControls::SixIron),
            87 => Ok(SportControls::SevenIron),
            88 => Ok(SportControls::EightIron),
            89 => Ok(SportControls::NineIron),
            90 => Ok(SportControls::One0Iron),
            91 => Ok(SportControls::One1Iron),
            92 => Ok(SportControls::SandWedge),
            93 => Ok(SportControls::LoftWedge),
            94 => Ok(SportControls::PowerWedge),
            95 => Ok(SportControls::OneWood),
            96 => Ok(SportControls::ThreeWood),
            97 => Ok(SportControls::FiveWood),
            98 => Ok(SportControls::SevenWood),
            99 => Ok(SportControls::NineWood),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for SportControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x5`: "Game Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::GameControls(GameControls::PinballDevice);
/// let u2 = Usage::new_from_page_and_id(0x5, 0x2).unwrap();
/// let u3 = Usage::from(GameControls::PinballDevice);
/// let u4: Usage = GameControls::PinballDevice.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::GameControls));
/// assert_eq!(0x5, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x5 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Pinball Device", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum GameControls {
    /// Usage ID `0x1`: "3D Game Controller"
    ThreeDGameController = 0x1,
    /// Usage ID `0x2`: "Pinball Device"
    PinballDevice = 0x2,
    /// Usage ID `0x3`: "Gun Device"
    GunDevice = 0x3,
    /// Usage ID `0x20`: "Point of View"
    PointofView = 0x20,
    /// Usage ID `0x21`: "Turn Right/Left"
    TurnRightLeft = 0x21,
    /// Usage ID `0x22`: "Pitch Forward/Backward"
    PitchForwardBackward = 0x22,
    /// Usage ID `0x23`: "Roll Right/Left"
    RollRightLeft = 0x23,
    /// Usage ID `0x24`: "Move Right/Left"
    MoveRightLeft = 0x24,
    /// Usage ID `0x25`: "Move Forward/Backward"
    MoveForwardBackward = 0x25,
    /// Usage ID `0x26`: "Move Up/Down"
    MoveUpDown = 0x26,
    /// Usage ID `0x27`: "Lean Right/Left"
    LeanRightLeft = 0x27,
    /// Usage ID `0x28`: "Lean Forward/Backward"
    LeanForwardBackward = 0x28,
    /// Usage ID `0x29`: "Height of POV"
    HeightofPOV = 0x29,
    /// Usage ID `0x2A`: "Flipper"
    Flipper = 0x2A,
    /// Usage ID `0x2B`: "Secondary Flipper"
    SecondaryFlipper = 0x2B,
    /// Usage ID `0x2C`: "Bump"
    Bump = 0x2C,
    /// Usage ID `0x2D`: "New Game"
    NewGame = 0x2D,
    /// Usage ID `0x2E`: "Shoot Ball"
    ShootBall = 0x2E,
    /// Usage ID `0x2F`: "Player"
    Player = 0x2F,
    /// Usage ID `0x30`: "Gun Bolt"
    GunBolt = 0x30,
    /// Usage ID `0x31`: "Gun Clip"
    GunClip = 0x31,
    /// Usage ID `0x32`: "Gun Selector"
    GunSelector = 0x32,
    /// Usage ID `0x33`: "Gun Single Shot"
    GunSingleShot = 0x33,
    /// Usage ID `0x34`: "Gun Burst"
    GunBurst = 0x34,
    /// Usage ID `0x35`: "Gun Automatic"
    GunAutomatic = 0x35,
    /// Usage ID `0x36`: "Gun Safety"
    GunSafety = 0x36,
    /// Usage ID `0x37`: "Gamepad Fire/Jump"
    GamepadFireJump = 0x37,
    /// Usage ID `0x39`: "Gamepad Trigger"
    GamepadTrigger = 0x39,
    /// Usage ID `0x3A`: "Form-fitting Gamepad"
    FormfittingGamepad = 0x3A,
}

impl GameControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            GameControls::ThreeDGameController => "3D Game Controller",
            GameControls::PinballDevice => "Pinball Device",
            GameControls::GunDevice => "Gun Device",
            GameControls::PointofView => "Point of View",
            GameControls::TurnRightLeft => "Turn Right/Left",
            GameControls::PitchForwardBackward => "Pitch Forward/Backward",
            GameControls::RollRightLeft => "Roll Right/Left",
            GameControls::MoveRightLeft => "Move Right/Left",
            GameControls::MoveForwardBackward => "Move Forward/Backward",
            GameControls::MoveUpDown => "Move Up/Down",
            GameControls::LeanRightLeft => "Lean Right/Left",
            GameControls::LeanForwardBackward => "Lean Forward/Backward",
            GameControls::HeightofPOV => "Height of POV",
            GameControls::Flipper => "Flipper",
            GameControls::SecondaryFlipper => "Secondary Flipper",
            GameControls::Bump => "Bump",
            GameControls::NewGame => "New Game",
            GameControls::ShootBall => "Shoot Ball",
            GameControls::Player => "Player",
            GameControls::GunBolt => "Gun Bolt",
            GameControls::GunClip => "Gun Clip",
            GameControls::GunSelector => "Gun Selector",
            GameControls::GunSingleShot => "Gun Single Shot",
            GameControls::GunBurst => "Gun Burst",
            GameControls::GunAutomatic => "Gun Automatic",
            GameControls::GunSafety => "Gun Safety",
            GameControls::GamepadFireJump => "Gamepad Fire/Jump",
            GameControls::GamepadTrigger => "Gamepad Trigger",
            GameControls::FormfittingGamepad => "Form-fitting Gamepad",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for GameControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for GameControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::GameControls(self)](Usage::GameControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for GameControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x5` for [GameControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::GameControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&GameControls> for u16 {
    fn from(gamecontrols: &GameControls) -> u16 {
        *gamecontrols as u16
    }
}

impl From<GameControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [GameControls::usage_page_value()].
    fn from(gamecontrols: GameControls) -> u16 {
        u16::from(&gamecontrols)
    }
}

impl From<&GameControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [GameControls::usage_value()].
    fn from(gamecontrols: &GameControls) -> u32 {
        let up = UsagePage::from(gamecontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(gamecontrols) as u32;
        up | id
    }
}

impl From<&GameControls> for UsagePage {
    /// Always returns [UsagePage::GameControls] and is
    /// identical to [GameControls::usage_page()].
    fn from(_: &GameControls) -> UsagePage {
        UsagePage::GameControls
    }
}

impl From<GameControls> for UsagePage {
    /// Always returns [UsagePage::GameControls] and is
    /// identical to [GameControls::usage_page()].
    fn from(_: GameControls) -> UsagePage {
        UsagePage::GameControls
    }
}

impl From<&GameControls> for Usage {
    fn from(gamecontrols: &GameControls) -> Usage {
        Usage::try_from(u32::from(gamecontrols)).unwrap()
    }
}

impl From<GameControls> for Usage {
    fn from(gamecontrols: GameControls) -> Usage {
        Usage::from(&gamecontrols)
    }
}

impl TryFrom<u16> for GameControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<GameControls> {
        match usage_id {
            1 => Ok(GameControls::ThreeDGameController),
            2 => Ok(GameControls::PinballDevice),
            3 => Ok(GameControls::GunDevice),
            32 => Ok(GameControls::PointofView),
            33 => Ok(GameControls::TurnRightLeft),
            34 => Ok(GameControls::PitchForwardBackward),
            35 => Ok(GameControls::RollRightLeft),
            36 => Ok(GameControls::MoveRightLeft),
            37 => Ok(GameControls::MoveForwardBackward),
            38 => Ok(GameControls::MoveUpDown),
            39 => Ok(GameControls::LeanRightLeft),
            40 => Ok(GameControls::LeanForwardBackward),
            41 => Ok(GameControls::HeightofPOV),
            42 => Ok(GameControls::Flipper),
            43 => Ok(GameControls::SecondaryFlipper),
            44 => Ok(GameControls::Bump),
            45 => Ok(GameControls::NewGame),
            46 => Ok(GameControls::ShootBall),
            47 => Ok(GameControls::Player),
            48 => Ok(GameControls::GunBolt),
            49 => Ok(GameControls::GunClip),
            50 => Ok(GameControls::GunSelector),
            51 => Ok(GameControls::GunSingleShot),
            52 => Ok(GameControls::GunBurst),
            53 => Ok(GameControls::GunAutomatic),
            54 => Ok(GameControls::GunSafety),
            55 => Ok(GameControls::GamepadFireJump),
            57 => Ok(GameControls::GamepadTrigger),
            58 => Ok(GameControls::FormfittingGamepad),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for GameControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x6`: "Generic Device Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::GenericDeviceControls(GenericDeviceControls::BatteryStrength);
/// let u2 = Usage::new_from_page_and_id(0x6, 0x20).unwrap();
/// let u3 = Usage::from(GenericDeviceControls::BatteryStrength);
/// let u4: Usage = GenericDeviceControls::BatteryStrength.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::GenericDeviceControls));
/// assert_eq!(0x6, u1.usage_page_value());
/// assert_eq!(0x20, u1.usage_id_value());
/// assert_eq!((0x6 << 16) | 0x20, u1.usage_value());
/// assert_eq!("Battery Strength", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum GenericDeviceControls {
    /// Usage ID `0x1`: "Background/Nonuser Controls"
    BackgroundNonuserControls = 0x1,
    /// Usage ID `0x20`: "Battery Strength"
    BatteryStrength = 0x20,
    /// Usage ID `0x21`: "Wireless Channel"
    WirelessChannel = 0x21,
    /// Usage ID `0x22`: "Wireless ID"
    WirelessID = 0x22,
    /// Usage ID `0x23`: "Discover Wireless Control"
    DiscoverWirelessControl = 0x23,
    /// Usage ID `0x24`: "Security Code Character Entered"
    SecurityCodeCharacterEntered = 0x24,
    /// Usage ID `0x25`: "Security Code Character Erased"
    SecurityCodeCharacterErased = 0x25,
    /// Usage ID `0x26`: "Security Code Cleared"
    SecurityCodeCleared = 0x26,
    /// Usage ID `0x27`: "Sequence ID"
    SequenceID = 0x27,
    /// Usage ID `0x28`: "Sequence ID Reset"
    SequenceIDReset = 0x28,
    /// Usage ID `0x29`: "RF Signal Strength"
    RFSignalStrength = 0x29,
    /// Usage ID `0x2A`: "Software Version"
    SoftwareVersion = 0x2A,
    /// Usage ID `0x2B`: "Protocol Version"
    ProtocolVersion = 0x2B,
    /// Usage ID `0x2C`: "Hardware Version"
    HardwareVersion = 0x2C,
    /// Usage ID `0x2D`: "Major"
    Major = 0x2D,
    /// Usage ID `0x2E`: "Minor"
    Minor = 0x2E,
    /// Usage ID `0x2F`: "Revision"
    Revision = 0x2F,
    /// Usage ID `0x30`: "Handedness"
    Handedness = 0x30,
    /// Usage ID `0x31`: "Either Hand"
    EitherHand = 0x31,
    /// Usage ID `0x32`: "Left Hand"
    LeftHand = 0x32,
    /// Usage ID `0x33`: "Right Hand"
    RightHand = 0x33,
    /// Usage ID `0x34`: "Both Hands"
    BothHands = 0x34,
    /// Usage ID `0x40`: "Grip Pose Offset"
    GripPoseOffset = 0x40,
    /// Usage ID `0x41`: "Pointer Pose Offset"
    PointerPoseOffset = 0x41,
}

impl GenericDeviceControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            GenericDeviceControls::BackgroundNonuserControls => "Background/Nonuser Controls",
            GenericDeviceControls::BatteryStrength => "Battery Strength",
            GenericDeviceControls::WirelessChannel => "Wireless Channel",
            GenericDeviceControls::WirelessID => "Wireless ID",
            GenericDeviceControls::DiscoverWirelessControl => "Discover Wireless Control",
            GenericDeviceControls::SecurityCodeCharacterEntered => {
                "Security Code Character Entered"
            }
            GenericDeviceControls::SecurityCodeCharacterErased => "Security Code Character Erased",
            GenericDeviceControls::SecurityCodeCleared => "Security Code Cleared",
            GenericDeviceControls::SequenceID => "Sequence ID",
            GenericDeviceControls::SequenceIDReset => "Sequence ID Reset",
            GenericDeviceControls::RFSignalStrength => "RF Signal Strength",
            GenericDeviceControls::SoftwareVersion => "Software Version",
            GenericDeviceControls::ProtocolVersion => "Protocol Version",
            GenericDeviceControls::HardwareVersion => "Hardware Version",
            GenericDeviceControls::Major => "Major",
            GenericDeviceControls::Minor => "Minor",
            GenericDeviceControls::Revision => "Revision",
            GenericDeviceControls::Handedness => "Handedness",
            GenericDeviceControls::EitherHand => "Either Hand",
            GenericDeviceControls::LeftHand => "Left Hand",
            GenericDeviceControls::RightHand => "Right Hand",
            GenericDeviceControls::BothHands => "Both Hands",
            GenericDeviceControls::GripPoseOffset => "Grip Pose Offset",
            GenericDeviceControls::PointerPoseOffset => "Pointer Pose Offset",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for GenericDeviceControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for GenericDeviceControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::GenericDeviceControls(self)](Usage::GenericDeviceControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for GenericDeviceControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x6` for [GenericDeviceControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::GenericDeviceControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&GenericDeviceControls> for u16 {
    fn from(genericdevicecontrols: &GenericDeviceControls) -> u16 {
        *genericdevicecontrols as u16
    }
}

impl From<GenericDeviceControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [GenericDeviceControls::usage_page_value()].
    fn from(genericdevicecontrols: GenericDeviceControls) -> u16 {
        u16::from(&genericdevicecontrols)
    }
}

impl From<&GenericDeviceControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [GenericDeviceControls::usage_value()].
    fn from(genericdevicecontrols: &GenericDeviceControls) -> u32 {
        let up = UsagePage::from(genericdevicecontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(genericdevicecontrols) as u32;
        up | id
    }
}

impl From<&GenericDeviceControls> for UsagePage {
    /// Always returns [UsagePage::GenericDeviceControls] and is
    /// identical to [GenericDeviceControls::usage_page()].
    fn from(_: &GenericDeviceControls) -> UsagePage {
        UsagePage::GenericDeviceControls
    }
}

impl From<GenericDeviceControls> for UsagePage {
    /// Always returns [UsagePage::GenericDeviceControls] and is
    /// identical to [GenericDeviceControls::usage_page()].
    fn from(_: GenericDeviceControls) -> UsagePage {
        UsagePage::GenericDeviceControls
    }
}

impl From<&GenericDeviceControls> for Usage {
    fn from(genericdevicecontrols: &GenericDeviceControls) -> Usage {
        Usage::try_from(u32::from(genericdevicecontrols)).unwrap()
    }
}

impl From<GenericDeviceControls> for Usage {
    fn from(genericdevicecontrols: GenericDeviceControls) -> Usage {
        Usage::from(&genericdevicecontrols)
    }
}

impl TryFrom<u16> for GenericDeviceControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<GenericDeviceControls> {
        match usage_id {
            1 => Ok(GenericDeviceControls::BackgroundNonuserControls),
            32 => Ok(GenericDeviceControls::BatteryStrength),
            33 => Ok(GenericDeviceControls::WirelessChannel),
            34 => Ok(GenericDeviceControls::WirelessID),
            35 => Ok(GenericDeviceControls::DiscoverWirelessControl),
            36 => Ok(GenericDeviceControls::SecurityCodeCharacterEntered),
            37 => Ok(GenericDeviceControls::SecurityCodeCharacterErased),
            38 => Ok(GenericDeviceControls::SecurityCodeCleared),
            39 => Ok(GenericDeviceControls::SequenceID),
            40 => Ok(GenericDeviceControls::SequenceIDReset),
            41 => Ok(GenericDeviceControls::RFSignalStrength),
            42 => Ok(GenericDeviceControls::SoftwareVersion),
            43 => Ok(GenericDeviceControls::ProtocolVersion),
            44 => Ok(GenericDeviceControls::HardwareVersion),
            45 => Ok(GenericDeviceControls::Major),
            46 => Ok(GenericDeviceControls::Minor),
            47 => Ok(GenericDeviceControls::Revision),
            48 => Ok(GenericDeviceControls::Handedness),
            49 => Ok(GenericDeviceControls::EitherHand),
            50 => Ok(GenericDeviceControls::LeftHand),
            51 => Ok(GenericDeviceControls::RightHand),
            52 => Ok(GenericDeviceControls::BothHands),
            64 => Ok(GenericDeviceControls::GripPoseOffset),
            65 => Ok(GenericDeviceControls::PointerPoseOffset),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for GenericDeviceControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x7`: "Keyboard/Keypad"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::KeyboardKeypad(KeyboardKeypad::POSTFail);
/// let u2 = Usage::new_from_page_and_id(0x7, 0x2).unwrap();
/// let u3 = Usage::from(KeyboardKeypad::POSTFail);
/// let u4: Usage = KeyboardKeypad::POSTFail.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::KeyboardKeypad));
/// assert_eq!(0x7, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x7 << 16) | 0x2, u1.usage_value());
/// assert_eq!("POSTFail", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum KeyboardKeypad {
    /// Usage ID `0x1`: "ErrorRollOver"
    ErrorRollOver = 0x1,
    /// Usage ID `0x2`: "POSTFail"
    POSTFail = 0x2,
    /// Usage ID `0x3`: "ErrorUndefined"
    ErrorUndefined = 0x3,
    /// Usage ID `0x4`: "Keyboard A"
    KeyboardA = 0x4,
    /// Usage ID `0x5`: "Keyboard B"
    KeyboardB = 0x5,
    /// Usage ID `0x6`: "Keyboard C"
    KeyboardC = 0x6,
    /// Usage ID `0x7`: "Keyboard D"
    KeyboardD = 0x7,
    /// Usage ID `0x8`: "Keyboard E"
    KeyboardE = 0x8,
    /// Usage ID `0x9`: "Keyboard F"
    KeyboardF = 0x9,
    /// Usage ID `0xA`: "Keyboard G"
    KeyboardG = 0xA,
    /// Usage ID `0xB`: "Keyboard H"
    KeyboardH = 0xB,
    /// Usage ID `0xC`: "Keyboard I"
    KeyboardI = 0xC,
    /// Usage ID `0xD`: "Keyboard J"
    KeyboardJ = 0xD,
    /// Usage ID `0xE`: "Keyboard K"
    KeyboardK = 0xE,
    /// Usage ID `0xF`: "Keyboard L"
    KeyboardL = 0xF,
    /// Usage ID `0x10`: "Keyboard M"
    KeyboardM = 0x10,
    /// Usage ID `0x11`: "Keyboard N"
    KeyboardN = 0x11,
    /// Usage ID `0x12`: "Keyboard O"
    KeyboardO = 0x12,
    /// Usage ID `0x13`: "Keyboard P"
    KeyboardP = 0x13,
    /// Usage ID `0x14`: "Keyboard Q"
    KeyboardQ = 0x14,
    /// Usage ID `0x15`: "Keyboard R"
    KeyboardR = 0x15,
    /// Usage ID `0x16`: "Keyboard S"
    KeyboardS = 0x16,
    /// Usage ID `0x17`: "Keyboard T"
    KeyboardT = 0x17,
    /// Usage ID `0x18`: "Keyboard U"
    KeyboardU = 0x18,
    /// Usage ID `0x19`: "Keyboard V"
    KeyboardV = 0x19,
    /// Usage ID `0x1A`: "Keyboard W"
    KeyboardW = 0x1A,
    /// Usage ID `0x1B`: "Keyboard X"
    KeyboardX = 0x1B,
    /// Usage ID `0x1C`: "Keyboard Y"
    KeyboardY = 0x1C,
    /// Usage ID `0x1D`: "Keyboard Z"
    KeyboardZ = 0x1D,
    /// Usage ID `0x1E`: "Keyboard 1 and Bang"
    Keyboard1andBang = 0x1E,
    /// Usage ID `0x1F`: "Keyboard 2 and At"
    Keyboard2andAt = 0x1F,
    /// Usage ID `0x20`: "Keyboard 3 and Hash"
    Keyboard3andHash = 0x20,
    /// Usage ID `0x21`: "Keyboard 4 and Dollar"
    Keyboard4andDollar = 0x21,
    /// Usage ID `0x22`: "Keyboard 5 and Percent"
    Keyboard5andPercent = 0x22,
    /// Usage ID `0x23`: "Keyboard 6 and Caret"
    Keyboard6andCaret = 0x23,
    /// Usage ID `0x24`: "Keyboard 7 and Ampersand"
    Keyboard7andAmpersand = 0x24,
    /// Usage ID `0x25`: "Keyboard 8 and Star"
    Keyboard8andStar = 0x25,
    /// Usage ID `0x26`: "Keyboard 9 and Left Bracket"
    Keyboard9andLeftBracket = 0x26,
    /// Usage ID `0x27`: "Keyboard 0 and Right Bracket"
    Keyboard0andRightBracket = 0x27,
    /// Usage ID `0x28`: "Keyboard Return Enter"
    KeyboardReturnEnter = 0x28,
    /// Usage ID `0x29`: "Keyboard Escape"
    KeyboardEscape = 0x29,
    /// Usage ID `0x2A`: "Keyboard Delete"
    KeyboardDelete = 0x2A,
    /// Usage ID `0x2B`: "Keyboard Tab"
    KeyboardTab = 0x2B,
    /// Usage ID `0x2C`: "Keyboard Spacebar"
    KeyboardSpacebar = 0x2C,
    /// Usage ID `0x2D`: "Keyboard Dash and Underscore"
    KeyboardDashandUnderscore = 0x2D,
    /// Usage ID `0x2E`: "Keyboard Equals and Plus"
    KeyboardEqualsandPlus = 0x2E,
    /// Usage ID `0x2F`: "Keyboard Left Brace"
    KeyboardLeftBrace = 0x2F,
    /// Usage ID `0x30`: "Keyboard Right Brace"
    KeyboardRightBrace = 0x30,
    /// Usage ID `0x31`: "Keyboard Backslash and Pipe"
    KeyboardBackslashandPipe = 0x31,
    /// Usage ID `0x32`: "Keyboard Non-US Hash and Tilde"
    KeyboardNonUSHashandTilde = 0x32,
    /// Usage ID `0x33`: "Keyboard SemiColon and Colon"
    KeyboardSemiColonandColon = 0x33,
    /// Usage ID `0x34`: "Keyboard Left Apos and Double"
    KeyboardLeftAposandDouble = 0x34,
    /// Usage ID `0x35`: "Keyboard Grave Accent and Tilde"
    KeyboardGraveAccentandTilde = 0x35,
    /// Usage ID `0x36`: "Keyboard Comma and LessThan"
    KeyboardCommaandLessThan = 0x36,
    /// Usage ID `0x37`: "Keyboard Period and GreaterThan"
    KeyboardPeriodandGreaterThan = 0x37,
    /// Usage ID `0x38`: "Keyboard ForwardSlash and QuestionMark"
    KeyboardForwardSlashandQuestionMark = 0x38,
    /// Usage ID `0x39`: "Keyboard Caps Lock"
    KeyboardCapsLock = 0x39,
    /// Usage ID `0x3A`: "Keyboard F1"
    KeyboardF1 = 0x3A,
    /// Usage ID `0x3B`: "Keyboard F2"
    KeyboardF2 = 0x3B,
    /// Usage ID `0x3C`: "Keyboard F3"
    KeyboardF3 = 0x3C,
    /// Usage ID `0x3D`: "Keyboard F4"
    KeyboardF4 = 0x3D,
    /// Usage ID `0x3E`: "Keyboard F5"
    KeyboardF5 = 0x3E,
    /// Usage ID `0x3F`: "Keyboard F6"
    KeyboardF6 = 0x3F,
    /// Usage ID `0x40`: "Keyboard F7"
    KeyboardF7 = 0x40,
    /// Usage ID `0x41`: "Keyboard F8"
    KeyboardF8 = 0x41,
    /// Usage ID `0x42`: "Keyboard F9"
    KeyboardF9 = 0x42,
    /// Usage ID `0x43`: "Keyboard F10"
    KeyboardF10 = 0x43,
    /// Usage ID `0x44`: "Keyboard F11"
    KeyboardF11 = 0x44,
    /// Usage ID `0x45`: "Keyboard F12"
    KeyboardF12 = 0x45,
    /// Usage ID `0x46`: "Keyboard PrintScreen"
    KeyboardPrintScreen = 0x46,
    /// Usage ID `0x47`: "Keyboard Scroll Lock"
    KeyboardScrollLock = 0x47,
    /// Usage ID `0x48`: "Keyboard Pause"
    KeyboardPause = 0x48,
    /// Usage ID `0x49`: "Keyboard Insert"
    KeyboardInsert = 0x49,
    /// Usage ID `0x4A`: "Keyboard Home"
    KeyboardHome = 0x4A,
    /// Usage ID `0x4B`: "Keyboard PageUp"
    KeyboardPageUp = 0x4B,
    /// Usage ID `0x4C`: "Keyboard Delete Forward"
    KeyboardDeleteForward = 0x4C,
    /// Usage ID `0x4D`: "Keyboard End"
    KeyboardEnd = 0x4D,
    /// Usage ID `0x4E`: "Keyboard PageDown"
    KeyboardPageDown = 0x4E,
    /// Usage ID `0x4F`: "Keyboard RightArrow"
    KeyboardRightArrow = 0x4F,
    /// Usage ID `0x50`: "Keyboard LeftArrow"
    KeyboardLeftArrow = 0x50,
    /// Usage ID `0x51`: "Keyboard DownArrow"
    KeyboardDownArrow = 0x51,
    /// Usage ID `0x52`: "Keyboard UpArrow"
    KeyboardUpArrow = 0x52,
    /// Usage ID `0x53`: "Keypad Num Lock and Clear"
    KeypadNumLockandClear = 0x53,
    /// Usage ID `0x54`: "Keypad ForwardSlash"
    KeypadForwardSlash = 0x54,
    /// Usage ID `0x55`: "Keypad Star"
    KeypadStar = 0x55,
    /// Usage ID `0x56`: "Keypad Dash"
    KeypadDash = 0x56,
    /// Usage ID `0x57`: "Keypad Plus"
    KeypadPlus = 0x57,
    /// Usage ID `0x58`: "Keypad ENTER"
    KeypadENTER = 0x58,
    /// Usage ID `0x59`: "Keypad 1 and End"
    Keypad1andEnd = 0x59,
    /// Usage ID `0x5A`: "Keypad 2 and Down Arrow"
    Keypad2andDownArrow = 0x5A,
    /// Usage ID `0x5B`: "Keypad 3 and PageDn"
    Keypad3andPageDn = 0x5B,
    /// Usage ID `0x5C`: "Keypad 4 and Left Arrow"
    Keypad4andLeftArrow = 0x5C,
    /// Usage ID `0x5D`: "Keypad 5"
    Keypad5 = 0x5D,
    /// Usage ID `0x5E`: "Keypad 6 and Right Arrow"
    Keypad6andRightArrow = 0x5E,
    /// Usage ID `0x5F`: "Keypad 7 and Home"
    Keypad7andHome = 0x5F,
    /// Usage ID `0x60`: "Keypad 8 and Up Arrow"
    Keypad8andUpArrow = 0x60,
    /// Usage ID `0x61`: "Keypad 9 and PageUp"
    Keypad9andPageUp = 0x61,
    /// Usage ID `0x62`: "Keypad 0 and Insert"
    Keypad0andInsert = 0x62,
    /// Usage ID `0x63`: "Keypad Period and Delete"
    KeypadPeriodandDelete = 0x63,
    /// Usage ID `0x64`: "Keyboard Non-US Backslash and Pipe"
    KeyboardNonUSBackslashandPipe = 0x64,
    /// Usage ID `0x65`: "Keyboard Application"
    KeyboardApplication = 0x65,
    /// Usage ID `0x66`: "Keyboard Power"
    KeyboardPower = 0x66,
    /// Usage ID `0x67`: "Keypad Equals"
    KeypadEquals = 0x67,
    /// Usage ID `0x68`: "Keyboard F13"
    KeyboardF13 = 0x68,
    /// Usage ID `0x69`: "Keyboard F14"
    KeyboardF14 = 0x69,
    /// Usage ID `0x6A`: "Keyboard F15"
    KeyboardF15 = 0x6A,
    /// Usage ID `0x6B`: "Keyboard F16"
    KeyboardF16 = 0x6B,
    /// Usage ID `0x6C`: "Keyboard F17"
    KeyboardF17 = 0x6C,
    /// Usage ID `0x6D`: "Keyboard F18"
    KeyboardF18 = 0x6D,
    /// Usage ID `0x6E`: "Keyboard F19"
    KeyboardF19 = 0x6E,
    /// Usage ID `0x6F`: "Keyboard F20"
    KeyboardF20 = 0x6F,
    /// Usage ID `0x70`: "Keyboard F21"
    KeyboardF21 = 0x70,
    /// Usage ID `0x71`: "Keyboard F22"
    KeyboardF22 = 0x71,
    /// Usage ID `0x72`: "Keyboard F23"
    KeyboardF23 = 0x72,
    /// Usage ID `0x73`: "Keyboard F24"
    KeyboardF24 = 0x73,
    /// Usage ID `0x74`: "Keyboard Execute"
    KeyboardExecute = 0x74,
    /// Usage ID `0x75`: "Keyboard Help"
    KeyboardHelp = 0x75,
    /// Usage ID `0x76`: "Keyboard Menu"
    KeyboardMenu = 0x76,
    /// Usage ID `0x77`: "Keyboard Select"
    KeyboardSelect = 0x77,
    /// Usage ID `0x78`: "Keyboard Stop"
    KeyboardStop = 0x78,
    /// Usage ID `0x79`: "Keyboard Again"
    KeyboardAgain = 0x79,
    /// Usage ID `0x7A`: "Keyboard Undo"
    KeyboardUndo = 0x7A,
    /// Usage ID `0x7B`: "Keyboard Cut"
    KeyboardCut = 0x7B,
    /// Usage ID `0x7C`: "Keyboard Copy"
    KeyboardCopy = 0x7C,
    /// Usage ID `0x7D`: "Keyboard Paste"
    KeyboardPaste = 0x7D,
    /// Usage ID `0x7E`: "Keyboard Find"
    KeyboardFind = 0x7E,
    /// Usage ID `0x7F`: "Keyboard Mute"
    KeyboardMute = 0x7F,
    /// Usage ID `0x80`: "Keyboard Volume Up"
    KeyboardVolumeUp = 0x80,
    /// Usage ID `0x81`: "Keyboard Volume Down"
    KeyboardVolumeDown = 0x81,
    /// Usage ID `0x82`: "Keyboard Locking Caps Lock"
    KeyboardLockingCapsLock = 0x82,
    /// Usage ID `0x83`: "Keyboard Locking Num Lock"
    KeyboardLockingNumLock = 0x83,
    /// Usage ID `0x84`: "Keyboard Locking Scroll Lock"
    KeyboardLockingScrollLock = 0x84,
    /// Usage ID `0x85`: "Keypad Comma"
    KeypadComma = 0x85,
    /// Usage ID `0x86`: "Keypad Equal Sign"
    KeypadEqualSign = 0x86,
    /// Usage ID `0x87`: "Keyboard International1"
    KeyboardInternational1 = 0x87,
    /// Usage ID `0x88`: "Keyboard International2"
    KeyboardInternational2 = 0x88,
    /// Usage ID `0x89`: "Keyboard International3"
    KeyboardInternational3 = 0x89,
    /// Usage ID `0x8A`: "Keyboard International4"
    KeyboardInternational4 = 0x8A,
    /// Usage ID `0x8B`: "Keyboard International5"
    KeyboardInternational5 = 0x8B,
    /// Usage ID `0x8C`: "Keyboard International6"
    KeyboardInternational6 = 0x8C,
    /// Usage ID `0x8D`: "Keyboard International7"
    KeyboardInternational7 = 0x8D,
    /// Usage ID `0x8E`: "Keyboard International8"
    KeyboardInternational8 = 0x8E,
    /// Usage ID `0x8F`: "Keyboard International9"
    KeyboardInternational9 = 0x8F,
    /// Usage ID `0x90`: "Keyboard LANG1"
    KeyboardLANG1 = 0x90,
    /// Usage ID `0x91`: "Keyboard LANG2"
    KeyboardLANG2 = 0x91,
    /// Usage ID `0x92`: "Keyboard LANG3"
    KeyboardLANG3 = 0x92,
    /// Usage ID `0x93`: "Keyboard LANG4"
    KeyboardLANG4 = 0x93,
    /// Usage ID `0x94`: "Keyboard LANG5"
    KeyboardLANG5 = 0x94,
    /// Usage ID `0x95`: "Keyboard LANG6"
    KeyboardLANG6 = 0x95,
    /// Usage ID `0x96`: "Keyboard LANG7"
    KeyboardLANG7 = 0x96,
    /// Usage ID `0x97`: "Keyboard LANG8"
    KeyboardLANG8 = 0x97,
    /// Usage ID `0x98`: "Keyboard LANG9"
    KeyboardLANG9 = 0x98,
    /// Usage ID `0x99`: "Keyboard Alternate Erase"
    KeyboardAlternateErase = 0x99,
    /// Usage ID `0x9A`: "Keyboard SysReq Attention"
    KeyboardSysReqAttention = 0x9A,
    /// Usage ID `0x9B`: "Keyboard Cancel"
    KeyboardCancel = 0x9B,
    /// Usage ID `0x9C`: "Keyboard Clear"
    KeyboardClear = 0x9C,
    /// Usage ID `0x9D`: "Keyboard Prior"
    KeyboardPrior = 0x9D,
    /// Usage ID `0x9E`: "Keyboard Return"
    KeyboardReturn = 0x9E,
    /// Usage ID `0x9F`: "Keyboard Separator"
    KeyboardSeparator = 0x9F,
    /// Usage ID `0xA0`: "Keyboard Out"
    KeyboardOut = 0xA0,
    /// Usage ID `0xA1`: "Keyboard Oper"
    KeyboardOper = 0xA1,
    /// Usage ID `0xA2`: "Keyboard Clear Again"
    KeyboardClearAgain = 0xA2,
    /// Usage ID `0xA3`: "Keyboard CrSel Props"
    KeyboardCrSelProps = 0xA3,
    /// Usage ID `0xA4`: "Keyboard ExSel"
    KeyboardExSel = 0xA4,
    /// Usage ID `0xB0`: "Keypad Double 0"
    KeypadDouble0 = 0xB0,
    /// Usage ID `0xB1`: "Keypad Triple 0"
    KeypadTriple0 = 0xB1,
    /// Usage ID `0xB2`: "Thousands Separator"
    ThousandsSeparator = 0xB2,
    /// Usage ID `0xB3`: "Decimal Separator"
    DecimalSeparator = 0xB3,
    /// Usage ID `0xB4`: "Currency Unit"
    CurrencyUnit = 0xB4,
    /// Usage ID `0xB5`: "Currency Sub-unit"
    CurrencySubunit = 0xB5,
    /// Usage ID `0xB6`: "Keypad Left Bracket"
    KeypadLeftBracket = 0xB6,
    /// Usage ID `0xB7`: "Keypad Right Bracket"
    KeypadRightBracket = 0xB7,
    /// Usage ID `0xB8`: "Keypad Left Brace"
    KeypadLeftBrace = 0xB8,
    /// Usage ID `0xB9`: "Keypad Right Brace"
    KeypadRightBrace = 0xB9,
    /// Usage ID `0xBA`: "Keypad Tab"
    KeypadTab = 0xBA,
    /// Usage ID `0xBB`: "Keypad Backspace"
    KeypadBackspace = 0xBB,
    /// Usage ID `0xBC`: "Keypad A"
    KeypadA = 0xBC,
    /// Usage ID `0xBD`: "Keypad B"
    KeypadB = 0xBD,
    /// Usage ID `0xBE`: "Keypad C"
    KeypadC = 0xBE,
    /// Usage ID `0xBF`: "Keypad D"
    KeypadD = 0xBF,
    /// Usage ID `0xC0`: "Keypad E"
    KeypadE = 0xC0,
    /// Usage ID `0xC1`: "Keypad F"
    KeypadF = 0xC1,
    /// Usage ID `0xC2`: "Keypad XOR"
    KeypadXOR = 0xC2,
    /// Usage ID `0xC3`: "Keypad Caret"
    KeypadCaret = 0xC3,
    /// Usage ID `0xC4`: "Keypad Percentage"
    KeypadPercentage = 0xC4,
    /// Usage ID `0xC5`: "Keypad Less"
    KeypadLess = 0xC5,
    /// Usage ID `0xC6`: "Keypad Greater"
    KeypadGreater = 0xC6,
    /// Usage ID `0xC7`: "Keypad Ampersand"
    KeypadAmpersand = 0xC7,
    /// Usage ID `0xC8`: "Keypad Double Ampersand"
    KeypadDoubleAmpersand = 0xC8,
    /// Usage ID `0xC9`: "Keypad Bar"
    KeypadBar = 0xC9,
    /// Usage ID `0xCA`: "Keypad Double Bar"
    KeypadDoubleBar = 0xCA,
    /// Usage ID `0xCB`: "Keypad Colon"
    KeypadColon = 0xCB,
    /// Usage ID `0xCC`: "Keypad Hash"
    KeypadHash = 0xCC,
    /// Usage ID `0xCD`: "Keypad Space"
    KeypadSpace = 0xCD,
    /// Usage ID `0xCE`: "Keypad At"
    KeypadAt = 0xCE,
    /// Usage ID `0xCF`: "Keypad Bang"
    KeypadBang = 0xCF,
    /// Usage ID `0xD0`: "Keypad Memory Store"
    KeypadMemoryStore = 0xD0,
    /// Usage ID `0xD1`: "Keypad Memory Recall"
    KeypadMemoryRecall = 0xD1,
    /// Usage ID `0xD2`: "Keypad Memory Clear"
    KeypadMemoryClear = 0xD2,
    /// Usage ID `0xD3`: "Keypad Memory Add"
    KeypadMemoryAdd = 0xD3,
    /// Usage ID `0xD4`: "Keypad Memory Subtract"
    KeypadMemorySubtract = 0xD4,
    /// Usage ID `0xD5`: "Keypad Memory Multiply"
    KeypadMemoryMultiply = 0xD5,
    /// Usage ID `0xD6`: "Keypad Memory Divide"
    KeypadMemoryDivide = 0xD6,
    /// Usage ID `0xD7`: "Keypad Plus Minus"
    KeypadPlusMinus = 0xD7,
    /// Usage ID `0xD8`: "Keypad Clear"
    KeypadClear = 0xD8,
    /// Usage ID `0xD9`: "Keypad Clear Entry"
    KeypadClearEntry = 0xD9,
    /// Usage ID `0xDA`: "Keypad Binary"
    KeypadBinary = 0xDA,
    /// Usage ID `0xDB`: "Keypad Octal"
    KeypadOctal = 0xDB,
    /// Usage ID `0xDC`: "Keypad Decimal"
    KeypadDecimal = 0xDC,
    /// Usage ID `0xDD`: "Keypad Hexadecimal"
    KeypadHexadecimal = 0xDD,
    /// Usage ID `0xE0`: "Keyboard LeftControl"
    KeyboardLeftControl = 0xE0,
    /// Usage ID `0xE1`: "Keyboard LeftShift"
    KeyboardLeftShift = 0xE1,
    /// Usage ID `0xE2`: "Keyboard LeftAlt"
    KeyboardLeftAlt = 0xE2,
    /// Usage ID `0xE3`: "Keyboard Left GUI"
    KeyboardLeftGUI = 0xE3,
    /// Usage ID `0xE4`: "Keyboard RightControl"
    KeyboardRightControl = 0xE4,
    /// Usage ID `0xE5`: "Keyboard RightShift"
    KeyboardRightShift = 0xE5,
    /// Usage ID `0xE6`: "Keyboard RightAlt"
    KeyboardRightAlt = 0xE6,
    /// Usage ID `0xE7`: "Keyboard Right GUI"
    KeyboardRightGUI = 0xE7,
}

impl KeyboardKeypad {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            KeyboardKeypad::ErrorRollOver => "ErrorRollOver",
            KeyboardKeypad::POSTFail => "POSTFail",
            KeyboardKeypad::ErrorUndefined => "ErrorUndefined",
            KeyboardKeypad::KeyboardA => "Keyboard A",
            KeyboardKeypad::KeyboardB => "Keyboard B",
            KeyboardKeypad::KeyboardC => "Keyboard C",
            KeyboardKeypad::KeyboardD => "Keyboard D",
            KeyboardKeypad::KeyboardE => "Keyboard E",
            KeyboardKeypad::KeyboardF => "Keyboard F",
            KeyboardKeypad::KeyboardG => "Keyboard G",
            KeyboardKeypad::KeyboardH => "Keyboard H",
            KeyboardKeypad::KeyboardI => "Keyboard I",
            KeyboardKeypad::KeyboardJ => "Keyboard J",
            KeyboardKeypad::KeyboardK => "Keyboard K",
            KeyboardKeypad::KeyboardL => "Keyboard L",
            KeyboardKeypad::KeyboardM => "Keyboard M",
            KeyboardKeypad::KeyboardN => "Keyboard N",
            KeyboardKeypad::KeyboardO => "Keyboard O",
            KeyboardKeypad::KeyboardP => "Keyboard P",
            KeyboardKeypad::KeyboardQ => "Keyboard Q",
            KeyboardKeypad::KeyboardR => "Keyboard R",
            KeyboardKeypad::KeyboardS => "Keyboard S",
            KeyboardKeypad::KeyboardT => "Keyboard T",
            KeyboardKeypad::KeyboardU => "Keyboard U",
            KeyboardKeypad::KeyboardV => "Keyboard V",
            KeyboardKeypad::KeyboardW => "Keyboard W",
            KeyboardKeypad::KeyboardX => "Keyboard X",
            KeyboardKeypad::KeyboardY => "Keyboard Y",
            KeyboardKeypad::KeyboardZ => "Keyboard Z",
            KeyboardKeypad::Keyboard1andBang => "Keyboard 1 and Bang",
            KeyboardKeypad::Keyboard2andAt => "Keyboard 2 and At",
            KeyboardKeypad::Keyboard3andHash => "Keyboard 3 and Hash",
            KeyboardKeypad::Keyboard4andDollar => "Keyboard 4 and Dollar",
            KeyboardKeypad::Keyboard5andPercent => "Keyboard 5 and Percent",
            KeyboardKeypad::Keyboard6andCaret => "Keyboard 6 and Caret",
            KeyboardKeypad::Keyboard7andAmpersand => "Keyboard 7 and Ampersand",
            KeyboardKeypad::Keyboard8andStar => "Keyboard 8 and Star",
            KeyboardKeypad::Keyboard9andLeftBracket => "Keyboard 9 and Left Bracket",
            KeyboardKeypad::Keyboard0andRightBracket => "Keyboard 0 and Right Bracket",
            KeyboardKeypad::KeyboardReturnEnter => "Keyboard Return Enter",
            KeyboardKeypad::KeyboardEscape => "Keyboard Escape",
            KeyboardKeypad::KeyboardDelete => "Keyboard Delete",
            KeyboardKeypad::KeyboardTab => "Keyboard Tab",
            KeyboardKeypad::KeyboardSpacebar => "Keyboard Spacebar",
            KeyboardKeypad::KeyboardDashandUnderscore => "Keyboard Dash and Underscore",
            KeyboardKeypad::KeyboardEqualsandPlus => "Keyboard Equals and Plus",
            KeyboardKeypad::KeyboardLeftBrace => "Keyboard Left Brace",
            KeyboardKeypad::KeyboardRightBrace => "Keyboard Right Brace",
            KeyboardKeypad::KeyboardBackslashandPipe => "Keyboard Backslash and Pipe",
            KeyboardKeypad::KeyboardNonUSHashandTilde => "Keyboard Non-US Hash and Tilde",
            KeyboardKeypad::KeyboardSemiColonandColon => "Keyboard SemiColon and Colon",
            KeyboardKeypad::KeyboardLeftAposandDouble => "Keyboard Left Apos and Double",
            KeyboardKeypad::KeyboardGraveAccentandTilde => "Keyboard Grave Accent and Tilde",
            KeyboardKeypad::KeyboardCommaandLessThan => "Keyboard Comma and LessThan",
            KeyboardKeypad::KeyboardPeriodandGreaterThan => "Keyboard Period and GreaterThan",
            KeyboardKeypad::KeyboardForwardSlashandQuestionMark => {
                "Keyboard ForwardSlash and QuestionMark"
            }
            KeyboardKeypad::KeyboardCapsLock => "Keyboard Caps Lock",
            KeyboardKeypad::KeyboardF1 => "Keyboard F1",
            KeyboardKeypad::KeyboardF2 => "Keyboard F2",
            KeyboardKeypad::KeyboardF3 => "Keyboard F3",
            KeyboardKeypad::KeyboardF4 => "Keyboard F4",
            KeyboardKeypad::KeyboardF5 => "Keyboard F5",
            KeyboardKeypad::KeyboardF6 => "Keyboard F6",
            KeyboardKeypad::KeyboardF7 => "Keyboard F7",
            KeyboardKeypad::KeyboardF8 => "Keyboard F8",
            KeyboardKeypad::KeyboardF9 => "Keyboard F9",
            KeyboardKeypad::KeyboardF10 => "Keyboard F10",
            KeyboardKeypad::KeyboardF11 => "Keyboard F11",
            KeyboardKeypad::KeyboardF12 => "Keyboard F12",
            KeyboardKeypad::KeyboardPrintScreen => "Keyboard PrintScreen",
            KeyboardKeypad::KeyboardScrollLock => "Keyboard Scroll Lock",
            KeyboardKeypad::KeyboardPause => "Keyboard Pause",
            KeyboardKeypad::KeyboardInsert => "Keyboard Insert",
            KeyboardKeypad::KeyboardHome => "Keyboard Home",
            KeyboardKeypad::KeyboardPageUp => "Keyboard PageUp",
            KeyboardKeypad::KeyboardDeleteForward => "Keyboard Delete Forward",
            KeyboardKeypad::KeyboardEnd => "Keyboard End",
            KeyboardKeypad::KeyboardPageDown => "Keyboard PageDown",
            KeyboardKeypad::KeyboardRightArrow => "Keyboard RightArrow",
            KeyboardKeypad::KeyboardLeftArrow => "Keyboard LeftArrow",
            KeyboardKeypad::KeyboardDownArrow => "Keyboard DownArrow",
            KeyboardKeypad::KeyboardUpArrow => "Keyboard UpArrow",
            KeyboardKeypad::KeypadNumLockandClear => "Keypad Num Lock and Clear",
            KeyboardKeypad::KeypadForwardSlash => "Keypad ForwardSlash",
            KeyboardKeypad::KeypadStar => "Keypad Star",
            KeyboardKeypad::KeypadDash => "Keypad Dash",
            KeyboardKeypad::KeypadPlus => "Keypad Plus",
            KeyboardKeypad::KeypadENTER => "Keypad ENTER",
            KeyboardKeypad::Keypad1andEnd => "Keypad 1 and End",
            KeyboardKeypad::Keypad2andDownArrow => "Keypad 2 and Down Arrow",
            KeyboardKeypad::Keypad3andPageDn => "Keypad 3 and PageDn",
            KeyboardKeypad::Keypad4andLeftArrow => "Keypad 4 and Left Arrow",
            KeyboardKeypad::Keypad5 => "Keypad 5",
            KeyboardKeypad::Keypad6andRightArrow => "Keypad 6 and Right Arrow",
            KeyboardKeypad::Keypad7andHome => "Keypad 7 and Home",
            KeyboardKeypad::Keypad8andUpArrow => "Keypad 8 and Up Arrow",
            KeyboardKeypad::Keypad9andPageUp => "Keypad 9 and PageUp",
            KeyboardKeypad::Keypad0andInsert => "Keypad 0 and Insert",
            KeyboardKeypad::KeypadPeriodandDelete => "Keypad Period and Delete",
            KeyboardKeypad::KeyboardNonUSBackslashandPipe => "Keyboard Non-US Backslash and Pipe",
            KeyboardKeypad::KeyboardApplication => "Keyboard Application",
            KeyboardKeypad::KeyboardPower => "Keyboard Power",
            KeyboardKeypad::KeypadEquals => "Keypad Equals",
            KeyboardKeypad::KeyboardF13 => "Keyboard F13",
            KeyboardKeypad::KeyboardF14 => "Keyboard F14",
            KeyboardKeypad::KeyboardF15 => "Keyboard F15",
            KeyboardKeypad::KeyboardF16 => "Keyboard F16",
            KeyboardKeypad::KeyboardF17 => "Keyboard F17",
            KeyboardKeypad::KeyboardF18 => "Keyboard F18",
            KeyboardKeypad::KeyboardF19 => "Keyboard F19",
            KeyboardKeypad::KeyboardF20 => "Keyboard F20",
            KeyboardKeypad::KeyboardF21 => "Keyboard F21",
            KeyboardKeypad::KeyboardF22 => "Keyboard F22",
            KeyboardKeypad::KeyboardF23 => "Keyboard F23",
            KeyboardKeypad::KeyboardF24 => "Keyboard F24",
            KeyboardKeypad::KeyboardExecute => "Keyboard Execute",
            KeyboardKeypad::KeyboardHelp => "Keyboard Help",
            KeyboardKeypad::KeyboardMenu => "Keyboard Menu",
            KeyboardKeypad::KeyboardSelect => "Keyboard Select",
            KeyboardKeypad::KeyboardStop => "Keyboard Stop",
            KeyboardKeypad::KeyboardAgain => "Keyboard Again",
            KeyboardKeypad::KeyboardUndo => "Keyboard Undo",
            KeyboardKeypad::KeyboardCut => "Keyboard Cut",
            KeyboardKeypad::KeyboardCopy => "Keyboard Copy",
            KeyboardKeypad::KeyboardPaste => "Keyboard Paste",
            KeyboardKeypad::KeyboardFind => "Keyboard Find",
            KeyboardKeypad::KeyboardMute => "Keyboard Mute",
            KeyboardKeypad::KeyboardVolumeUp => "Keyboard Volume Up",
            KeyboardKeypad::KeyboardVolumeDown => "Keyboard Volume Down",
            KeyboardKeypad::KeyboardLockingCapsLock => "Keyboard Locking Caps Lock",
            KeyboardKeypad::KeyboardLockingNumLock => "Keyboard Locking Num Lock",
            KeyboardKeypad::KeyboardLockingScrollLock => "Keyboard Locking Scroll Lock",
            KeyboardKeypad::KeypadComma => "Keypad Comma",
            KeyboardKeypad::KeypadEqualSign => "Keypad Equal Sign",
            KeyboardKeypad::KeyboardInternational1 => "Keyboard International1",
            KeyboardKeypad::KeyboardInternational2 => "Keyboard International2",
            KeyboardKeypad::KeyboardInternational3 => "Keyboard International3",
            KeyboardKeypad::KeyboardInternational4 => "Keyboard International4",
            KeyboardKeypad::KeyboardInternational5 => "Keyboard International5",
            KeyboardKeypad::KeyboardInternational6 => "Keyboard International6",
            KeyboardKeypad::KeyboardInternational7 => "Keyboard International7",
            KeyboardKeypad::KeyboardInternational8 => "Keyboard International8",
            KeyboardKeypad::KeyboardInternational9 => "Keyboard International9",
            KeyboardKeypad::KeyboardLANG1 => "Keyboard LANG1",
            KeyboardKeypad::KeyboardLANG2 => "Keyboard LANG2",
            KeyboardKeypad::KeyboardLANG3 => "Keyboard LANG3",
            KeyboardKeypad::KeyboardLANG4 => "Keyboard LANG4",
            KeyboardKeypad::KeyboardLANG5 => "Keyboard LANG5",
            KeyboardKeypad::KeyboardLANG6 => "Keyboard LANG6",
            KeyboardKeypad::KeyboardLANG7 => "Keyboard LANG7",
            KeyboardKeypad::KeyboardLANG8 => "Keyboard LANG8",
            KeyboardKeypad::KeyboardLANG9 => "Keyboard LANG9",
            KeyboardKeypad::KeyboardAlternateErase => "Keyboard Alternate Erase",
            KeyboardKeypad::KeyboardSysReqAttention => "Keyboard SysReq Attention",
            KeyboardKeypad::KeyboardCancel => "Keyboard Cancel",
            KeyboardKeypad::KeyboardClear => "Keyboard Clear",
            KeyboardKeypad::KeyboardPrior => "Keyboard Prior",
            KeyboardKeypad::KeyboardReturn => "Keyboard Return",
            KeyboardKeypad::KeyboardSeparator => "Keyboard Separator",
            KeyboardKeypad::KeyboardOut => "Keyboard Out",
            KeyboardKeypad::KeyboardOper => "Keyboard Oper",
            KeyboardKeypad::KeyboardClearAgain => "Keyboard Clear Again",
            KeyboardKeypad::KeyboardCrSelProps => "Keyboard CrSel Props",
            KeyboardKeypad::KeyboardExSel => "Keyboard ExSel",
            KeyboardKeypad::KeypadDouble0 => "Keypad Double 0",
            KeyboardKeypad::KeypadTriple0 => "Keypad Triple 0",
            KeyboardKeypad::ThousandsSeparator => "Thousands Separator",
            KeyboardKeypad::DecimalSeparator => "Decimal Separator",
            KeyboardKeypad::CurrencyUnit => "Currency Unit",
            KeyboardKeypad::CurrencySubunit => "Currency Sub-unit",
            KeyboardKeypad::KeypadLeftBracket => "Keypad Left Bracket",
            KeyboardKeypad::KeypadRightBracket => "Keypad Right Bracket",
            KeyboardKeypad::KeypadLeftBrace => "Keypad Left Brace",
            KeyboardKeypad::KeypadRightBrace => "Keypad Right Brace",
            KeyboardKeypad::KeypadTab => "Keypad Tab",
            KeyboardKeypad::KeypadBackspace => "Keypad Backspace",
            KeyboardKeypad::KeypadA => "Keypad A",
            KeyboardKeypad::KeypadB => "Keypad B",
            KeyboardKeypad::KeypadC => "Keypad C",
            KeyboardKeypad::KeypadD => "Keypad D",
            KeyboardKeypad::KeypadE => "Keypad E",
            KeyboardKeypad::KeypadF => "Keypad F",
            KeyboardKeypad::KeypadXOR => "Keypad XOR",
            KeyboardKeypad::KeypadCaret => "Keypad Caret",
            KeyboardKeypad::KeypadPercentage => "Keypad Percentage",
            KeyboardKeypad::KeypadLess => "Keypad Less",
            KeyboardKeypad::KeypadGreater => "Keypad Greater",
            KeyboardKeypad::KeypadAmpersand => "Keypad Ampersand",
            KeyboardKeypad::KeypadDoubleAmpersand => "Keypad Double Ampersand",
            KeyboardKeypad::KeypadBar => "Keypad Bar",
            KeyboardKeypad::KeypadDoubleBar => "Keypad Double Bar",
            KeyboardKeypad::KeypadColon => "Keypad Colon",
            KeyboardKeypad::KeypadHash => "Keypad Hash",
            KeyboardKeypad::KeypadSpace => "Keypad Space",
            KeyboardKeypad::KeypadAt => "Keypad At",
            KeyboardKeypad::KeypadBang => "Keypad Bang",
            KeyboardKeypad::KeypadMemoryStore => "Keypad Memory Store",
            KeyboardKeypad::KeypadMemoryRecall => "Keypad Memory Recall",
            KeyboardKeypad::KeypadMemoryClear => "Keypad Memory Clear",
            KeyboardKeypad::KeypadMemoryAdd => "Keypad Memory Add",
            KeyboardKeypad::KeypadMemorySubtract => "Keypad Memory Subtract",
            KeyboardKeypad::KeypadMemoryMultiply => "Keypad Memory Multiply",
            KeyboardKeypad::KeypadMemoryDivide => "Keypad Memory Divide",
            KeyboardKeypad::KeypadPlusMinus => "Keypad Plus Minus",
            KeyboardKeypad::KeypadClear => "Keypad Clear",
            KeyboardKeypad::KeypadClearEntry => "Keypad Clear Entry",
            KeyboardKeypad::KeypadBinary => "Keypad Binary",
            KeyboardKeypad::KeypadOctal => "Keypad Octal",
            KeyboardKeypad::KeypadDecimal => "Keypad Decimal",
            KeyboardKeypad::KeypadHexadecimal => "Keypad Hexadecimal",
            KeyboardKeypad::KeyboardLeftControl => "Keyboard LeftControl",
            KeyboardKeypad::KeyboardLeftShift => "Keyboard LeftShift",
            KeyboardKeypad::KeyboardLeftAlt => "Keyboard LeftAlt",
            KeyboardKeypad::KeyboardLeftGUI => "Keyboard Left GUI",
            KeyboardKeypad::KeyboardRightControl => "Keyboard RightControl",
            KeyboardKeypad::KeyboardRightShift => "Keyboard RightShift",
            KeyboardKeypad::KeyboardRightAlt => "Keyboard RightAlt",
            KeyboardKeypad::KeyboardRightGUI => "Keyboard Right GUI",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for KeyboardKeypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for KeyboardKeypad {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::KeyboardKeypad(self)](Usage::KeyboardKeypad)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for KeyboardKeypad {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x7` for [KeyboardKeypad]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::KeyboardKeypad]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&KeyboardKeypad> for u16 {
    fn from(keyboardkeypad: &KeyboardKeypad) -> u16 {
        *keyboardkeypad as u16
    }
}

impl From<KeyboardKeypad> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [KeyboardKeypad::usage_page_value()].
    fn from(keyboardkeypad: KeyboardKeypad) -> u16 {
        u16::from(&keyboardkeypad)
    }
}

impl From<&KeyboardKeypad> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [KeyboardKeypad::usage_value()].
    fn from(keyboardkeypad: &KeyboardKeypad) -> u32 {
        let up = UsagePage::from(keyboardkeypad);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(keyboardkeypad) as u32;
        up | id
    }
}

impl From<&KeyboardKeypad> for UsagePage {
    /// Always returns [UsagePage::KeyboardKeypad] and is
    /// identical to [KeyboardKeypad::usage_page()].
    fn from(_: &KeyboardKeypad) -> UsagePage {
        UsagePage::KeyboardKeypad
    }
}

impl From<KeyboardKeypad> for UsagePage {
    /// Always returns [UsagePage::KeyboardKeypad] and is
    /// identical to [KeyboardKeypad::usage_page()].
    fn from(_: KeyboardKeypad) -> UsagePage {
        UsagePage::KeyboardKeypad
    }
}

impl From<&KeyboardKeypad> for Usage {
    fn from(keyboardkeypad: &KeyboardKeypad) -> Usage {
        Usage::try_from(u32::from(keyboardkeypad)).unwrap()
    }
}

impl From<KeyboardKeypad> for Usage {
    fn from(keyboardkeypad: KeyboardKeypad) -> Usage {
        Usage::from(&keyboardkeypad)
    }
}

impl TryFrom<u16> for KeyboardKeypad {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<KeyboardKeypad> {
        match usage_id {
            1 => Ok(KeyboardKeypad::ErrorRollOver),
            2 => Ok(KeyboardKeypad::POSTFail),
            3 => Ok(KeyboardKeypad::ErrorUndefined),
            4 => Ok(KeyboardKeypad::KeyboardA),
            5 => Ok(KeyboardKeypad::KeyboardB),
            6 => Ok(KeyboardKeypad::KeyboardC),
            7 => Ok(KeyboardKeypad::KeyboardD),
            8 => Ok(KeyboardKeypad::KeyboardE),
            9 => Ok(KeyboardKeypad::KeyboardF),
            10 => Ok(KeyboardKeypad::KeyboardG),
            11 => Ok(KeyboardKeypad::KeyboardH),
            12 => Ok(KeyboardKeypad::KeyboardI),
            13 => Ok(KeyboardKeypad::KeyboardJ),
            14 => Ok(KeyboardKeypad::KeyboardK),
            15 => Ok(KeyboardKeypad::KeyboardL),
            16 => Ok(KeyboardKeypad::KeyboardM),
            17 => Ok(KeyboardKeypad::KeyboardN),
            18 => Ok(KeyboardKeypad::KeyboardO),
            19 => Ok(KeyboardKeypad::KeyboardP),
            20 => Ok(KeyboardKeypad::KeyboardQ),
            21 => Ok(KeyboardKeypad::KeyboardR),
            22 => Ok(KeyboardKeypad::KeyboardS),
            23 => Ok(KeyboardKeypad::KeyboardT),
            24 => Ok(KeyboardKeypad::KeyboardU),
            25 => Ok(KeyboardKeypad::KeyboardV),
            26 => Ok(KeyboardKeypad::KeyboardW),
            27 => Ok(KeyboardKeypad::KeyboardX),
            28 => Ok(KeyboardKeypad::KeyboardY),
            29 => Ok(KeyboardKeypad::KeyboardZ),
            30 => Ok(KeyboardKeypad::Keyboard1andBang),
            31 => Ok(KeyboardKeypad::Keyboard2andAt),
            32 => Ok(KeyboardKeypad::Keyboard3andHash),
            33 => Ok(KeyboardKeypad::Keyboard4andDollar),
            34 => Ok(KeyboardKeypad::Keyboard5andPercent),
            35 => Ok(KeyboardKeypad::Keyboard6andCaret),
            36 => Ok(KeyboardKeypad::Keyboard7andAmpersand),
            37 => Ok(KeyboardKeypad::Keyboard8andStar),
            38 => Ok(KeyboardKeypad::Keyboard9andLeftBracket),
            39 => Ok(KeyboardKeypad::Keyboard0andRightBracket),
            40 => Ok(KeyboardKeypad::KeyboardReturnEnter),
            41 => Ok(KeyboardKeypad::KeyboardEscape),
            42 => Ok(KeyboardKeypad::KeyboardDelete),
            43 => Ok(KeyboardKeypad::KeyboardTab),
            44 => Ok(KeyboardKeypad::KeyboardSpacebar),
            45 => Ok(KeyboardKeypad::KeyboardDashandUnderscore),
            46 => Ok(KeyboardKeypad::KeyboardEqualsandPlus),
            47 => Ok(KeyboardKeypad::KeyboardLeftBrace),
            48 => Ok(KeyboardKeypad::KeyboardRightBrace),
            49 => Ok(KeyboardKeypad::KeyboardBackslashandPipe),
            50 => Ok(KeyboardKeypad::KeyboardNonUSHashandTilde),
            51 => Ok(KeyboardKeypad::KeyboardSemiColonandColon),
            52 => Ok(KeyboardKeypad::KeyboardLeftAposandDouble),
            53 => Ok(KeyboardKeypad::KeyboardGraveAccentandTilde),
            54 => Ok(KeyboardKeypad::KeyboardCommaandLessThan),
            55 => Ok(KeyboardKeypad::KeyboardPeriodandGreaterThan),
            56 => Ok(KeyboardKeypad::KeyboardForwardSlashandQuestionMark),
            57 => Ok(KeyboardKeypad::KeyboardCapsLock),
            58 => Ok(KeyboardKeypad::KeyboardF1),
            59 => Ok(KeyboardKeypad::KeyboardF2),
            60 => Ok(KeyboardKeypad::KeyboardF3),
            61 => Ok(KeyboardKeypad::KeyboardF4),
            62 => Ok(KeyboardKeypad::KeyboardF5),
            63 => Ok(KeyboardKeypad::KeyboardF6),
            64 => Ok(KeyboardKeypad::KeyboardF7),
            65 => Ok(KeyboardKeypad::KeyboardF8),
            66 => Ok(KeyboardKeypad::KeyboardF9),
            67 => Ok(KeyboardKeypad::KeyboardF10),
            68 => Ok(KeyboardKeypad::KeyboardF11),
            69 => Ok(KeyboardKeypad::KeyboardF12),
            70 => Ok(KeyboardKeypad::KeyboardPrintScreen),
            71 => Ok(KeyboardKeypad::KeyboardScrollLock),
            72 => Ok(KeyboardKeypad::KeyboardPause),
            73 => Ok(KeyboardKeypad::KeyboardInsert),
            74 => Ok(KeyboardKeypad::KeyboardHome),
            75 => Ok(KeyboardKeypad::KeyboardPageUp),
            76 => Ok(KeyboardKeypad::KeyboardDeleteForward),
            77 => Ok(KeyboardKeypad::KeyboardEnd),
            78 => Ok(KeyboardKeypad::KeyboardPageDown),
            79 => Ok(KeyboardKeypad::KeyboardRightArrow),
            80 => Ok(KeyboardKeypad::KeyboardLeftArrow),
            81 => Ok(KeyboardKeypad::KeyboardDownArrow),
            82 => Ok(KeyboardKeypad::KeyboardUpArrow),
            83 => Ok(KeyboardKeypad::KeypadNumLockandClear),
            84 => Ok(KeyboardKeypad::KeypadForwardSlash),
            85 => Ok(KeyboardKeypad::KeypadStar),
            86 => Ok(KeyboardKeypad::KeypadDash),
            87 => Ok(KeyboardKeypad::KeypadPlus),
            88 => Ok(KeyboardKeypad::KeypadENTER),
            89 => Ok(KeyboardKeypad::Keypad1andEnd),
            90 => Ok(KeyboardKeypad::Keypad2andDownArrow),
            91 => Ok(KeyboardKeypad::Keypad3andPageDn),
            92 => Ok(KeyboardKeypad::Keypad4andLeftArrow),
            93 => Ok(KeyboardKeypad::Keypad5),
            94 => Ok(KeyboardKeypad::Keypad6andRightArrow),
            95 => Ok(KeyboardKeypad::Keypad7andHome),
            96 => Ok(KeyboardKeypad::Keypad8andUpArrow),
            97 => Ok(KeyboardKeypad::Keypad9andPageUp),
            98 => Ok(KeyboardKeypad::Keypad0andInsert),
            99 => Ok(KeyboardKeypad::KeypadPeriodandDelete),
            100 => Ok(KeyboardKeypad::KeyboardNonUSBackslashandPipe),
            101 => Ok(KeyboardKeypad::KeyboardApplication),
            102 => Ok(KeyboardKeypad::KeyboardPower),
            103 => Ok(KeyboardKeypad::KeypadEquals),
            104 => Ok(KeyboardKeypad::KeyboardF13),
            105 => Ok(KeyboardKeypad::KeyboardF14),
            106 => Ok(KeyboardKeypad::KeyboardF15),
            107 => Ok(KeyboardKeypad::KeyboardF16),
            108 => Ok(KeyboardKeypad::KeyboardF17),
            109 => Ok(KeyboardKeypad::KeyboardF18),
            110 => Ok(KeyboardKeypad::KeyboardF19),
            111 => Ok(KeyboardKeypad::KeyboardF20),
            112 => Ok(KeyboardKeypad::KeyboardF21),
            113 => Ok(KeyboardKeypad::KeyboardF22),
            114 => Ok(KeyboardKeypad::KeyboardF23),
            115 => Ok(KeyboardKeypad::KeyboardF24),
            116 => Ok(KeyboardKeypad::KeyboardExecute),
            117 => Ok(KeyboardKeypad::KeyboardHelp),
            118 => Ok(KeyboardKeypad::KeyboardMenu),
            119 => Ok(KeyboardKeypad::KeyboardSelect),
            120 => Ok(KeyboardKeypad::KeyboardStop),
            121 => Ok(KeyboardKeypad::KeyboardAgain),
            122 => Ok(KeyboardKeypad::KeyboardUndo),
            123 => Ok(KeyboardKeypad::KeyboardCut),
            124 => Ok(KeyboardKeypad::KeyboardCopy),
            125 => Ok(KeyboardKeypad::KeyboardPaste),
            126 => Ok(KeyboardKeypad::KeyboardFind),
            127 => Ok(KeyboardKeypad::KeyboardMute),
            128 => Ok(KeyboardKeypad::KeyboardVolumeUp),
            129 => Ok(KeyboardKeypad::KeyboardVolumeDown),
            130 => Ok(KeyboardKeypad::KeyboardLockingCapsLock),
            131 => Ok(KeyboardKeypad::KeyboardLockingNumLock),
            132 => Ok(KeyboardKeypad::KeyboardLockingScrollLock),
            133 => Ok(KeyboardKeypad::KeypadComma),
            134 => Ok(KeyboardKeypad::KeypadEqualSign),
            135 => Ok(KeyboardKeypad::KeyboardInternational1),
            136 => Ok(KeyboardKeypad::KeyboardInternational2),
            137 => Ok(KeyboardKeypad::KeyboardInternational3),
            138 => Ok(KeyboardKeypad::KeyboardInternational4),
            139 => Ok(KeyboardKeypad::KeyboardInternational5),
            140 => Ok(KeyboardKeypad::KeyboardInternational6),
            141 => Ok(KeyboardKeypad::KeyboardInternational7),
            142 => Ok(KeyboardKeypad::KeyboardInternational8),
            143 => Ok(KeyboardKeypad::KeyboardInternational9),
            144 => Ok(KeyboardKeypad::KeyboardLANG1),
            145 => Ok(KeyboardKeypad::KeyboardLANG2),
            146 => Ok(KeyboardKeypad::KeyboardLANG3),
            147 => Ok(KeyboardKeypad::KeyboardLANG4),
            148 => Ok(KeyboardKeypad::KeyboardLANG5),
            149 => Ok(KeyboardKeypad::KeyboardLANG6),
            150 => Ok(KeyboardKeypad::KeyboardLANG7),
            151 => Ok(KeyboardKeypad::KeyboardLANG8),
            152 => Ok(KeyboardKeypad::KeyboardLANG9),
            153 => Ok(KeyboardKeypad::KeyboardAlternateErase),
            154 => Ok(KeyboardKeypad::KeyboardSysReqAttention),
            155 => Ok(KeyboardKeypad::KeyboardCancel),
            156 => Ok(KeyboardKeypad::KeyboardClear),
            157 => Ok(KeyboardKeypad::KeyboardPrior),
            158 => Ok(KeyboardKeypad::KeyboardReturn),
            159 => Ok(KeyboardKeypad::KeyboardSeparator),
            160 => Ok(KeyboardKeypad::KeyboardOut),
            161 => Ok(KeyboardKeypad::KeyboardOper),
            162 => Ok(KeyboardKeypad::KeyboardClearAgain),
            163 => Ok(KeyboardKeypad::KeyboardCrSelProps),
            164 => Ok(KeyboardKeypad::KeyboardExSel),
            176 => Ok(KeyboardKeypad::KeypadDouble0),
            177 => Ok(KeyboardKeypad::KeypadTriple0),
            178 => Ok(KeyboardKeypad::ThousandsSeparator),
            179 => Ok(KeyboardKeypad::DecimalSeparator),
            180 => Ok(KeyboardKeypad::CurrencyUnit),
            181 => Ok(KeyboardKeypad::CurrencySubunit),
            182 => Ok(KeyboardKeypad::KeypadLeftBracket),
            183 => Ok(KeyboardKeypad::KeypadRightBracket),
            184 => Ok(KeyboardKeypad::KeypadLeftBrace),
            185 => Ok(KeyboardKeypad::KeypadRightBrace),
            186 => Ok(KeyboardKeypad::KeypadTab),
            187 => Ok(KeyboardKeypad::KeypadBackspace),
            188 => Ok(KeyboardKeypad::KeypadA),
            189 => Ok(KeyboardKeypad::KeypadB),
            190 => Ok(KeyboardKeypad::KeypadC),
            191 => Ok(KeyboardKeypad::KeypadD),
            192 => Ok(KeyboardKeypad::KeypadE),
            193 => Ok(KeyboardKeypad::KeypadF),
            194 => Ok(KeyboardKeypad::KeypadXOR),
            195 => Ok(KeyboardKeypad::KeypadCaret),
            196 => Ok(KeyboardKeypad::KeypadPercentage),
            197 => Ok(KeyboardKeypad::KeypadLess),
            198 => Ok(KeyboardKeypad::KeypadGreater),
            199 => Ok(KeyboardKeypad::KeypadAmpersand),
            200 => Ok(KeyboardKeypad::KeypadDoubleAmpersand),
            201 => Ok(KeyboardKeypad::KeypadBar),
            202 => Ok(KeyboardKeypad::KeypadDoubleBar),
            203 => Ok(KeyboardKeypad::KeypadColon),
            204 => Ok(KeyboardKeypad::KeypadHash),
            205 => Ok(KeyboardKeypad::KeypadSpace),
            206 => Ok(KeyboardKeypad::KeypadAt),
            207 => Ok(KeyboardKeypad::KeypadBang),
            208 => Ok(KeyboardKeypad::KeypadMemoryStore),
            209 => Ok(KeyboardKeypad::KeypadMemoryRecall),
            210 => Ok(KeyboardKeypad::KeypadMemoryClear),
            211 => Ok(KeyboardKeypad::KeypadMemoryAdd),
            212 => Ok(KeyboardKeypad::KeypadMemorySubtract),
            213 => Ok(KeyboardKeypad::KeypadMemoryMultiply),
            214 => Ok(KeyboardKeypad::KeypadMemoryDivide),
            215 => Ok(KeyboardKeypad::KeypadPlusMinus),
            216 => Ok(KeyboardKeypad::KeypadClear),
            217 => Ok(KeyboardKeypad::KeypadClearEntry),
            218 => Ok(KeyboardKeypad::KeypadBinary),
            219 => Ok(KeyboardKeypad::KeypadOctal),
            220 => Ok(KeyboardKeypad::KeypadDecimal),
            221 => Ok(KeyboardKeypad::KeypadHexadecimal),
            224 => Ok(KeyboardKeypad::KeyboardLeftControl),
            225 => Ok(KeyboardKeypad::KeyboardLeftShift),
            226 => Ok(KeyboardKeypad::KeyboardLeftAlt),
            227 => Ok(KeyboardKeypad::KeyboardLeftGUI),
            228 => Ok(KeyboardKeypad::KeyboardRightControl),
            229 => Ok(KeyboardKeypad::KeyboardRightShift),
            230 => Ok(KeyboardKeypad::KeyboardRightAlt),
            231 => Ok(KeyboardKeypad::KeyboardRightGUI),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for KeyboardKeypad {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x8`: "LED"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::LED(LED::CapsLock);
/// let u2 = Usage::new_from_page_and_id(0x8, 0x2).unwrap();
/// let u3 = Usage::from(LED::CapsLock);
/// let u4: Usage = LED::CapsLock.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::LED));
/// assert_eq!(0x8, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x8 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Caps Lock", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum LED {
    /// Usage ID `0x1`: "Num Lock"
    NumLock = 0x1,
    /// Usage ID `0x2`: "Caps Lock"
    CapsLock = 0x2,
    /// Usage ID `0x3`: "Scroll Lock"
    ScrollLock = 0x3,
    /// Usage ID `0x4`: "Compose"
    Compose = 0x4,
    /// Usage ID `0x5`: "Kana"
    Kana = 0x5,
    /// Usage ID `0x6`: "Power"
    Power = 0x6,
    /// Usage ID `0x7`: "Shift"
    Shift = 0x7,
    /// Usage ID `0x8`: "Do Not Disturb"
    DoNotDisturb = 0x8,
    /// Usage ID `0x9`: "Mute"
    Mute = 0x9,
    /// Usage ID `0xA`: "Tone Enable"
    ToneEnable = 0xA,
    /// Usage ID `0xB`: "High Cut Filter"
    HighCutFilter = 0xB,
    /// Usage ID `0xC`: "Low Cut Filter"
    LowCutFilter = 0xC,
    /// Usage ID `0xD`: "Equalizer Enable"
    EqualizerEnable = 0xD,
    /// Usage ID `0xE`: "Sound Field On"
    SoundFieldOn = 0xE,
    /// Usage ID `0xF`: "Surround On"
    SurroundOn = 0xF,
    /// Usage ID `0x10`: "Repeat"
    Repeat = 0x10,
    /// Usage ID `0x11`: "Stereo"
    Stereo = 0x11,
    /// Usage ID `0x12`: "Sampling Rate Detect"
    SamplingRateDetect = 0x12,
    /// Usage ID `0x13`: "Spinning"
    Spinning = 0x13,
    /// Usage ID `0x14`: "CAV"
    CAV = 0x14,
    /// Usage ID `0x15`: "CLV"
    CLV = 0x15,
    /// Usage ID `0x16`: "Recording Format Detect"
    RecordingFormatDetect = 0x16,
    /// Usage ID `0x17`: "Off-Hook"
    OffHook = 0x17,
    /// Usage ID `0x18`: "Ring"
    Ring = 0x18,
    /// Usage ID `0x19`: "Message Waiting"
    MessageWaiting = 0x19,
    /// Usage ID `0x1A`: "Data Mode"
    DataMode = 0x1A,
    /// Usage ID `0x1B`: "Battery Operation"
    BatteryOperation = 0x1B,
    /// Usage ID `0x1C`: "Battery OK"
    BatteryOK = 0x1C,
    /// Usage ID `0x1D`: "Battery Low"
    BatteryLow = 0x1D,
    /// Usage ID `0x1E`: "Speaker"
    Speaker = 0x1E,
    /// Usage ID `0x1F`: "Headset"
    Headset = 0x1F,
    /// Usage ID `0x20`: "Hold"
    Hold = 0x20,
    /// Usage ID `0x21`: "Microphone"
    Microphone = 0x21,
    /// Usage ID `0x22`: "Coverage"
    Coverage = 0x22,
    /// Usage ID `0x23`: "Night Mode"
    NightMode = 0x23,
    /// Usage ID `0x24`: "Send Calls"
    SendCalls = 0x24,
    /// Usage ID `0x25`: "Call Pickup"
    CallPickup = 0x25,
    /// Usage ID `0x26`: "Conference"
    Conference = 0x26,
    /// Usage ID `0x27`: "Stand-by"
    Standby = 0x27,
    /// Usage ID `0x28`: "Camera On"
    CameraOn = 0x28,
    /// Usage ID `0x29`: "Camera Off"
    CameraOff = 0x29,
    /// Usage ID `0x2A`: "On-Line"
    OnLine = 0x2A,
    /// Usage ID `0x2B`: "Off-Line"
    OffLine = 0x2B,
    /// Usage ID `0x2C`: "Busy"
    Busy = 0x2C,
    /// Usage ID `0x2D`: "Ready"
    Ready = 0x2D,
    /// Usage ID `0x2E`: "Paper-Out"
    PaperOut = 0x2E,
    /// Usage ID `0x2F`: "Paper-Jam"
    PaperJam = 0x2F,
    /// Usage ID `0x30`: "Remote"
    Remote = 0x30,
    /// Usage ID `0x31`: "Forward"
    Forward = 0x31,
    /// Usage ID `0x32`: "Reverse"
    Reverse = 0x32,
    /// Usage ID `0x33`: "Stop"
    Stop = 0x33,
    /// Usage ID `0x34`: "Rewind"
    Rewind = 0x34,
    /// Usage ID `0x35`: "Fast Forward"
    FastForward = 0x35,
    /// Usage ID `0x36`: "Play"
    Play = 0x36,
    /// Usage ID `0x37`: "Pause"
    Pause = 0x37,
    /// Usage ID `0x38`: "Record"
    Record = 0x38,
    /// Usage ID `0x39`: "Error"
    Error = 0x39,
    /// Usage ID `0x3A`: "Usage Selected Indicator"
    UsageSelectedIndicator = 0x3A,
    /// Usage ID `0x3B`: "Usage In Use Indicator"
    UsageInUseIndicator = 0x3B,
    /// Usage ID `0x3C`: "Usage Multi Mode Indicator"
    UsageMultiModeIndicator = 0x3C,
    /// Usage ID `0x3D`: "Indicator On"
    IndicatorOn = 0x3D,
    /// Usage ID `0x3E`: "Indicator Flash"
    IndicatorFlash = 0x3E,
    /// Usage ID `0x3F`: "Indicator Slow Blink"
    IndicatorSlowBlink = 0x3F,
    /// Usage ID `0x40`: "Indicator Fast Blink"
    IndicatorFastBlink = 0x40,
    /// Usage ID `0x41`: "Indicator Off"
    IndicatorOff = 0x41,
    /// Usage ID `0x42`: "Flash On Time"
    FlashOnTime = 0x42,
    /// Usage ID `0x43`: "Slow Blink On Time"
    SlowBlinkOnTime = 0x43,
    /// Usage ID `0x44`: "Slow Blink Off Time"
    SlowBlinkOffTime = 0x44,
    /// Usage ID `0x45`: "Fast Blink On Time"
    FastBlinkOnTime = 0x45,
    /// Usage ID `0x46`: "Fast Blink Off Time"
    FastBlinkOffTime = 0x46,
    /// Usage ID `0x47`: "Usage Indicator Color"
    UsageIndicatorColor = 0x47,
    /// Usage ID `0x48`: "Indicator Red"
    IndicatorRed = 0x48,
    /// Usage ID `0x49`: "Indicator Green"
    IndicatorGreen = 0x49,
    /// Usage ID `0x4A`: "Indicator Amber"
    IndicatorAmber = 0x4A,
    /// Usage ID `0x4B`: "Generic Indicator"
    GenericIndicator = 0x4B,
    /// Usage ID `0x4C`: "System Suspend"
    SystemSuspend = 0x4C,
    /// Usage ID `0x4D`: "External Power Connected"
    ExternalPowerConnected = 0x4D,
    /// Usage ID `0x4E`: "Indicator Blue"
    IndicatorBlue = 0x4E,
    /// Usage ID `0x4F`: "Indicator Orange"
    IndicatorOrange = 0x4F,
    /// Usage ID `0x50`: "Good Status"
    GoodStatus = 0x50,
    /// Usage ID `0x51`: "Warning Status"
    WarningStatus = 0x51,
    /// Usage ID `0x52`: "RGB LED"
    RGBLED = 0x52,
    /// Usage ID `0x53`: "Red LED Channel"
    RedLEDChannel = 0x53,
    /// Usage ID `0x54`: "Blue LED Channel"
    BlueLEDChannel = 0x54,
    /// Usage ID `0x55`: "Green LED Channel"
    GreenLEDChannel = 0x55,
    /// Usage ID `0x56`: "LED Intensity"
    LEDIntensity = 0x56,
    /// Usage ID `0x57`: "System Microphone Mute"
    SystemMicrophoneMute = 0x57,
    /// Usage ID `0x60`: "Player Indicator"
    PlayerIndicator = 0x60,
    /// Usage ID `0x61`: "Player 1"
    Player1 = 0x61,
    /// Usage ID `0x62`: "Player 2"
    Player2 = 0x62,
    /// Usage ID `0x63`: "Player 3"
    Player3 = 0x63,
    /// Usage ID `0x64`: "Player 4"
    Player4 = 0x64,
    /// Usage ID `0x65`: "Player 5"
    Player5 = 0x65,
    /// Usage ID `0x66`: "Player 6"
    Player6 = 0x66,
    /// Usage ID `0x67`: "Player 7"
    Player7 = 0x67,
    /// Usage ID `0x68`: "Player 8"
    Player8 = 0x68,
}

impl LED {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            LED::NumLock => "Num Lock",
            LED::CapsLock => "Caps Lock",
            LED::ScrollLock => "Scroll Lock",
            LED::Compose => "Compose",
            LED::Kana => "Kana",
            LED::Power => "Power",
            LED::Shift => "Shift",
            LED::DoNotDisturb => "Do Not Disturb",
            LED::Mute => "Mute",
            LED::ToneEnable => "Tone Enable",
            LED::HighCutFilter => "High Cut Filter",
            LED::LowCutFilter => "Low Cut Filter",
            LED::EqualizerEnable => "Equalizer Enable",
            LED::SoundFieldOn => "Sound Field On",
            LED::SurroundOn => "Surround On",
            LED::Repeat => "Repeat",
            LED::Stereo => "Stereo",
            LED::SamplingRateDetect => "Sampling Rate Detect",
            LED::Spinning => "Spinning",
            LED::CAV => "CAV",
            LED::CLV => "CLV",
            LED::RecordingFormatDetect => "Recording Format Detect",
            LED::OffHook => "Off-Hook",
            LED::Ring => "Ring",
            LED::MessageWaiting => "Message Waiting",
            LED::DataMode => "Data Mode",
            LED::BatteryOperation => "Battery Operation",
            LED::BatteryOK => "Battery OK",
            LED::BatteryLow => "Battery Low",
            LED::Speaker => "Speaker",
            LED::Headset => "Headset",
            LED::Hold => "Hold",
            LED::Microphone => "Microphone",
            LED::Coverage => "Coverage",
            LED::NightMode => "Night Mode",
            LED::SendCalls => "Send Calls",
            LED::CallPickup => "Call Pickup",
            LED::Conference => "Conference",
            LED::Standby => "Stand-by",
            LED::CameraOn => "Camera On",
            LED::CameraOff => "Camera Off",
            LED::OnLine => "On-Line",
            LED::OffLine => "Off-Line",
            LED::Busy => "Busy",
            LED::Ready => "Ready",
            LED::PaperOut => "Paper-Out",
            LED::PaperJam => "Paper-Jam",
            LED::Remote => "Remote",
            LED::Forward => "Forward",
            LED::Reverse => "Reverse",
            LED::Stop => "Stop",
            LED::Rewind => "Rewind",
            LED::FastForward => "Fast Forward",
            LED::Play => "Play",
            LED::Pause => "Pause",
            LED::Record => "Record",
            LED::Error => "Error",
            LED::UsageSelectedIndicator => "Usage Selected Indicator",
            LED::UsageInUseIndicator => "Usage In Use Indicator",
            LED::UsageMultiModeIndicator => "Usage Multi Mode Indicator",
            LED::IndicatorOn => "Indicator On",
            LED::IndicatorFlash => "Indicator Flash",
            LED::IndicatorSlowBlink => "Indicator Slow Blink",
            LED::IndicatorFastBlink => "Indicator Fast Blink",
            LED::IndicatorOff => "Indicator Off",
            LED::FlashOnTime => "Flash On Time",
            LED::SlowBlinkOnTime => "Slow Blink On Time",
            LED::SlowBlinkOffTime => "Slow Blink Off Time",
            LED::FastBlinkOnTime => "Fast Blink On Time",
            LED::FastBlinkOffTime => "Fast Blink Off Time",
            LED::UsageIndicatorColor => "Usage Indicator Color",
            LED::IndicatorRed => "Indicator Red",
            LED::IndicatorGreen => "Indicator Green",
            LED::IndicatorAmber => "Indicator Amber",
            LED::GenericIndicator => "Generic Indicator",
            LED::SystemSuspend => "System Suspend",
            LED::ExternalPowerConnected => "External Power Connected",
            LED::IndicatorBlue => "Indicator Blue",
            LED::IndicatorOrange => "Indicator Orange",
            LED::GoodStatus => "Good Status",
            LED::WarningStatus => "Warning Status",
            LED::RGBLED => "RGB LED",
            LED::RedLEDChannel => "Red LED Channel",
            LED::BlueLEDChannel => "Blue LED Channel",
            LED::GreenLEDChannel => "Green LED Channel",
            LED::LEDIntensity => "LED Intensity",
            LED::SystemMicrophoneMute => "System Microphone Mute",
            LED::PlayerIndicator => "Player Indicator",
            LED::Player1 => "Player 1",
            LED::Player2 => "Player 2",
            LED::Player3 => "Player 3",
            LED::Player4 => "Player 4",
            LED::Player5 => "Player 5",
            LED::Player6 => "Player 6",
            LED::Player7 => "Player 7",
            LED::Player8 => "Player 8",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for LED {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for LED {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::LED(self)](Usage::LED)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for LED {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x8` for [LED]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::LED]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&LED> for u16 {
    fn from(led: &LED) -> u16 {
        *led as u16
    }
}

impl From<LED> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [LED::usage_page_value()].
    fn from(led: LED) -> u16 {
        u16::from(&led)
    }
}

impl From<&LED> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [LED::usage_value()].
    fn from(led: &LED) -> u32 {
        let up = UsagePage::from(led);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(led) as u32;
        up | id
    }
}

impl From<&LED> for UsagePage {
    /// Always returns [UsagePage::LED] and is
    /// identical to [LED::usage_page()].
    fn from(_: &LED) -> UsagePage {
        UsagePage::LED
    }
}

impl From<LED> for UsagePage {
    /// Always returns [UsagePage::LED] and is
    /// identical to [LED::usage_page()].
    fn from(_: LED) -> UsagePage {
        UsagePage::LED
    }
}

impl From<&LED> for Usage {
    fn from(led: &LED) -> Usage {
        Usage::try_from(u32::from(led)).unwrap()
    }
}

impl From<LED> for Usage {
    fn from(led: LED) -> Usage {
        Usage::from(&led)
    }
}

impl TryFrom<u16> for LED {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<LED> {
        match usage_id {
            1 => Ok(LED::NumLock),
            2 => Ok(LED::CapsLock),
            3 => Ok(LED::ScrollLock),
            4 => Ok(LED::Compose),
            5 => Ok(LED::Kana),
            6 => Ok(LED::Power),
            7 => Ok(LED::Shift),
            8 => Ok(LED::DoNotDisturb),
            9 => Ok(LED::Mute),
            10 => Ok(LED::ToneEnable),
            11 => Ok(LED::HighCutFilter),
            12 => Ok(LED::LowCutFilter),
            13 => Ok(LED::EqualizerEnable),
            14 => Ok(LED::SoundFieldOn),
            15 => Ok(LED::SurroundOn),
            16 => Ok(LED::Repeat),
            17 => Ok(LED::Stereo),
            18 => Ok(LED::SamplingRateDetect),
            19 => Ok(LED::Spinning),
            20 => Ok(LED::CAV),
            21 => Ok(LED::CLV),
            22 => Ok(LED::RecordingFormatDetect),
            23 => Ok(LED::OffHook),
            24 => Ok(LED::Ring),
            25 => Ok(LED::MessageWaiting),
            26 => Ok(LED::DataMode),
            27 => Ok(LED::BatteryOperation),
            28 => Ok(LED::BatteryOK),
            29 => Ok(LED::BatteryLow),
            30 => Ok(LED::Speaker),
            31 => Ok(LED::Headset),
            32 => Ok(LED::Hold),
            33 => Ok(LED::Microphone),
            34 => Ok(LED::Coverage),
            35 => Ok(LED::NightMode),
            36 => Ok(LED::SendCalls),
            37 => Ok(LED::CallPickup),
            38 => Ok(LED::Conference),
            39 => Ok(LED::Standby),
            40 => Ok(LED::CameraOn),
            41 => Ok(LED::CameraOff),
            42 => Ok(LED::OnLine),
            43 => Ok(LED::OffLine),
            44 => Ok(LED::Busy),
            45 => Ok(LED::Ready),
            46 => Ok(LED::PaperOut),
            47 => Ok(LED::PaperJam),
            48 => Ok(LED::Remote),
            49 => Ok(LED::Forward),
            50 => Ok(LED::Reverse),
            51 => Ok(LED::Stop),
            52 => Ok(LED::Rewind),
            53 => Ok(LED::FastForward),
            54 => Ok(LED::Play),
            55 => Ok(LED::Pause),
            56 => Ok(LED::Record),
            57 => Ok(LED::Error),
            58 => Ok(LED::UsageSelectedIndicator),
            59 => Ok(LED::UsageInUseIndicator),
            60 => Ok(LED::UsageMultiModeIndicator),
            61 => Ok(LED::IndicatorOn),
            62 => Ok(LED::IndicatorFlash),
            63 => Ok(LED::IndicatorSlowBlink),
            64 => Ok(LED::IndicatorFastBlink),
            65 => Ok(LED::IndicatorOff),
            66 => Ok(LED::FlashOnTime),
            67 => Ok(LED::SlowBlinkOnTime),
            68 => Ok(LED::SlowBlinkOffTime),
            69 => Ok(LED::FastBlinkOnTime),
            70 => Ok(LED::FastBlinkOffTime),
            71 => Ok(LED::UsageIndicatorColor),
            72 => Ok(LED::IndicatorRed),
            73 => Ok(LED::IndicatorGreen),
            74 => Ok(LED::IndicatorAmber),
            75 => Ok(LED::GenericIndicator),
            76 => Ok(LED::SystemSuspend),
            77 => Ok(LED::ExternalPowerConnected),
            78 => Ok(LED::IndicatorBlue),
            79 => Ok(LED::IndicatorOrange),
            80 => Ok(LED::GoodStatus),
            81 => Ok(LED::WarningStatus),
            82 => Ok(LED::RGBLED),
            83 => Ok(LED::RedLEDChannel),
            84 => Ok(LED::BlueLEDChannel),
            85 => Ok(LED::GreenLEDChannel),
            86 => Ok(LED::LEDIntensity),
            87 => Ok(LED::SystemMicrophoneMute),
            96 => Ok(LED::PlayerIndicator),
            97 => Ok(LED::Player1),
            98 => Ok(LED::Player2),
            99 => Ok(LED::Player3),
            100 => Ok(LED::Player4),
            101 => Ok(LED::Player5),
            102 => Ok(LED::Player6),
            103 => Ok(LED::Player7),
            104 => Ok(LED::Player8),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for LED {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x9`: "Button"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
///
/// This Usage Page is generated, not defined, any Usage IDs in this Usage
/// Page are simply the button number.
///
/// ```
/// # use hut::*;
/// let u1 = Usage::Button(Button::Button(3));
/// let u2 = Usage::new_from_page_and_id(0x9, 3).unwrap();
/// let u3 = Usage::from(Button::Button(3));
/// let u4: Usage = Button::Button(3).into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Button));
/// assert_eq!(0x9, u1.usage_page_value());
/// assert_eq!(3, u1.usage_id_value());
/// assert_eq!((0x9 << 16) | 3, u1.usage_value());
/// assert_eq!("Button 3", u1.name());
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Button {
    Button(u16),
}

impl Button {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Button::Button(button) => format!("Button {button}"),
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Button {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Button(self)](Usage::Button)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Button {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x9` for [Button]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Button]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Button> for u16 {
    fn from(button: &Button) -> u16 {
        match *button {
            Button::Button(button) => button,
        }
    }
}

impl From<Button> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Button::usage_page_value()].
    fn from(button: Button) -> u16 {
        u16::from(&button)
    }
}

impl From<&Button> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Button::usage_value()].
    fn from(button: &Button) -> u32 {
        let up = UsagePage::from(button);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(button) as u32;
        up | id
    }
}

impl From<&Button> for UsagePage {
    /// Always returns [UsagePage::Button] and is
    /// identical to [Button::usage_page()].
    fn from(_: &Button) -> UsagePage {
        UsagePage::Button
    }
}

impl From<Button> for UsagePage {
    /// Always returns [UsagePage::Button] and is
    /// identical to [Button::usage_page()].
    fn from(_: Button) -> UsagePage {
        UsagePage::Button
    }
}

impl From<&Button> for Usage {
    fn from(button: &Button) -> Usage {
        Usage::try_from(u32::from(button)).unwrap()
    }
}

impl From<Button> for Usage {
    fn from(button: Button) -> Usage {
        Usage::from(&button)
    }
}

impl TryFrom<u16> for Button {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Button> {
        match usage_id {
            n => Ok(Button::Button(n)),
        }
    }
}

impl BitOr<u16> for Button {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xA`: "Ordinal"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
///
/// This Usage Page is generated, not defined, any Usage IDs in this Usage
/// Page are simply the instance number.
///
/// ```
/// # use hut::*;
/// let u1 = Usage::Ordinal(Ordinal::Ordinal(3));
/// let u2 = Usage::new_from_page_and_id(0xA, 3).unwrap();
/// let u3 = Usage::from(Ordinal::Ordinal(3));
/// let u4: Usage = Ordinal::Ordinal(3).into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Ordinal));
/// assert_eq!(0xA, u1.usage_page_value());
/// assert_eq!(3, u1.usage_id_value());
/// assert_eq!((0xA << 16) | 3, u1.usage_value());
/// assert_eq!("Instance 3", u1.name());
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Ordinal {
    Ordinal(u16),
}

impl Ordinal {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Ordinal::Ordinal(instance) => format!("Instance {instance}"),
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Ordinal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Ordinal {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Ordinal(self)](Usage::Ordinal)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Ordinal {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xA` for [Ordinal]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Ordinal]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Ordinal> for u16 {
    fn from(ordinal: &Ordinal) -> u16 {
        match *ordinal {
            Ordinal::Ordinal(instance) => instance,
        }
    }
}

impl From<Ordinal> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Ordinal::usage_page_value()].
    fn from(ordinal: Ordinal) -> u16 {
        u16::from(&ordinal)
    }
}

impl From<&Ordinal> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Ordinal::usage_value()].
    fn from(ordinal: &Ordinal) -> u32 {
        let up = UsagePage::from(ordinal);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(ordinal) as u32;
        up | id
    }
}

impl From<&Ordinal> for UsagePage {
    /// Always returns [UsagePage::Ordinal] and is
    /// identical to [Ordinal::usage_page()].
    fn from(_: &Ordinal) -> UsagePage {
        UsagePage::Ordinal
    }
}

impl From<Ordinal> for UsagePage {
    /// Always returns [UsagePage::Ordinal] and is
    /// identical to [Ordinal::usage_page()].
    fn from(_: Ordinal) -> UsagePage {
        UsagePage::Ordinal
    }
}

impl From<&Ordinal> for Usage {
    fn from(ordinal: &Ordinal) -> Usage {
        Usage::try_from(u32::from(ordinal)).unwrap()
    }
}

impl From<Ordinal> for Usage {
    fn from(ordinal: Ordinal) -> Usage {
        Usage::from(&ordinal)
    }
}

impl TryFrom<u16> for Ordinal {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Ordinal> {
        match usage_id {
            n => Ok(Ordinal::Ordinal(n)),
        }
    }
}

impl BitOr<u16> for Ordinal {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xB`: "Telephony Device"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::TelephonyDevice(TelephonyDevice::AnsweringMachine);
/// let u2 = Usage::new_from_page_and_id(0xB, 0x2).unwrap();
/// let u3 = Usage::from(TelephonyDevice::AnsweringMachine);
/// let u4: Usage = TelephonyDevice::AnsweringMachine.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::TelephonyDevice));
/// assert_eq!(0xB, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0xB << 16) | 0x2, u1.usage_value());
/// assert_eq!("Answering Machine", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum TelephonyDevice {
    /// Usage ID `0x1`: "Phone"
    Phone = 0x1,
    /// Usage ID `0x2`: "Answering Machine"
    AnsweringMachine = 0x2,
    /// Usage ID `0x3`: "Message Controls"
    MessageControls = 0x3,
    /// Usage ID `0x4`: "Handset"
    Handset = 0x4,
    /// Usage ID `0x5`: "Headset"
    Headset = 0x5,
    /// Usage ID `0x6`: "Telephony Key Pad"
    TelephonyKeyPad = 0x6,
    /// Usage ID `0x7`: "Programmable Button"
    ProgrammableButton = 0x7,
    /// Usage ID `0x20`: "Hook Switch"
    HookSwitch = 0x20,
    /// Usage ID `0x21`: "Flash"
    Flash = 0x21,
    /// Usage ID `0x22`: "Feature"
    Feature = 0x22,
    /// Usage ID `0x23`: "Hold"
    Hold = 0x23,
    /// Usage ID `0x24`: "Redial"
    Redial = 0x24,
    /// Usage ID `0x25`: "Transfer"
    Transfer = 0x25,
    /// Usage ID `0x26`: "Drop"
    Drop = 0x26,
    /// Usage ID `0x27`: "Park"
    Park = 0x27,
    /// Usage ID `0x28`: "Forward Calls"
    ForwardCalls = 0x28,
    /// Usage ID `0x29`: "Alternate Function"
    AlternateFunction = 0x29,
    /// Usage ID `0x2A`: "Line"
    Line = 0x2A,
    /// Usage ID `0x2B`: "Speaker Phone"
    SpeakerPhone = 0x2B,
    /// Usage ID `0x2C`: "Conference"
    Conference = 0x2C,
    /// Usage ID `0x2D`: "Ring Enable"
    RingEnable = 0x2D,
    /// Usage ID `0x2E`: "Ring Select"
    RingSelect = 0x2E,
    /// Usage ID `0x2F`: "Phone Mute"
    PhoneMute = 0x2F,
    /// Usage ID `0x30`: "Caller ID"
    CallerID = 0x30,
    /// Usage ID `0x31`: "Send"
    Send = 0x31,
    /// Usage ID `0x50`: "Speed Dial"
    SpeedDial = 0x50,
    /// Usage ID `0x51`: "Store Number"
    StoreNumber = 0x51,
    /// Usage ID `0x52`: "Recall Number"
    RecallNumber = 0x52,
    /// Usage ID `0x53`: "Phone Directory"
    PhoneDirectory = 0x53,
    /// Usage ID `0x70`: "Voice Mail"
    VoiceMail = 0x70,
    /// Usage ID `0x71`: "Screen Calls"
    ScreenCalls = 0x71,
    /// Usage ID `0x72`: "Do Not Disturb"
    DoNotDisturb = 0x72,
    /// Usage ID `0x73`: "Message"
    Message = 0x73,
    /// Usage ID `0x74`: "Answer On/Off"
    AnswerOnOff = 0x74,
    /// Usage ID `0x90`: "Inside Dial Tone"
    InsideDialTone = 0x90,
    /// Usage ID `0x91`: "Outside Dial Tone"
    OutsideDialTone = 0x91,
    /// Usage ID `0x92`: "Inside Ring Tone"
    InsideRingTone = 0x92,
    /// Usage ID `0x93`: "Outside Ring Tone"
    OutsideRingTone = 0x93,
    /// Usage ID `0x94`: "Priority Ring Tone"
    PriorityRingTone = 0x94,
    /// Usage ID `0x95`: "Inside Ringback"
    InsideRingback = 0x95,
    /// Usage ID `0x96`: "Priority Ringback"
    PriorityRingback = 0x96,
    /// Usage ID `0x97`: "Line Busy Tone"
    LineBusyTone = 0x97,
    /// Usage ID `0x98`: "Reorder Tone"
    ReorderTone = 0x98,
    /// Usage ID `0x99`: "Call Waiting Tone"
    CallWaitingTone = 0x99,
    /// Usage ID `0x9A`: "Confirmation Tone 1"
    ConfirmationTone1 = 0x9A,
    /// Usage ID `0x9B`: "Confirmation Tone 2"
    ConfirmationTone2 = 0x9B,
    /// Usage ID `0x9C`: "Tones Off"
    TonesOff = 0x9C,
    /// Usage ID `0x9D`: "Outside Ringback"
    OutsideRingback = 0x9D,
    /// Usage ID `0x9E`: "Ringer"
    Ringer = 0x9E,
    /// Usage ID `0xB0`: "Phone Key 0"
    PhoneKey0 = 0xB0,
    /// Usage ID `0xB1`: "Phone Key 1"
    PhoneKey1 = 0xB1,
    /// Usage ID `0xB2`: "Phone Key 2"
    PhoneKey2 = 0xB2,
    /// Usage ID `0xB3`: "Phone Key 3"
    PhoneKey3 = 0xB3,
    /// Usage ID `0xB4`: "Phone Key 4"
    PhoneKey4 = 0xB4,
    /// Usage ID `0xB5`: "Phone Key 5"
    PhoneKey5 = 0xB5,
    /// Usage ID `0xB6`: "Phone Key 6"
    PhoneKey6 = 0xB6,
    /// Usage ID `0xB7`: "Phone Key 7"
    PhoneKey7 = 0xB7,
    /// Usage ID `0xB8`: "Phone Key 8"
    PhoneKey8 = 0xB8,
    /// Usage ID `0xB9`: "Phone Key 9"
    PhoneKey9 = 0xB9,
    /// Usage ID `0xBA`: "Phone Key Star"
    PhoneKeyStar = 0xBA,
    /// Usage ID `0xBB`: "Phone Key Pound"
    PhoneKeyPound = 0xBB,
    /// Usage ID `0xBC`: "Phone Key A"
    PhoneKeyA = 0xBC,
    /// Usage ID `0xBD`: "Phone Key B"
    PhoneKeyB = 0xBD,
    /// Usage ID `0xBE`: "Phone Key C"
    PhoneKeyC = 0xBE,
    /// Usage ID `0xBF`: "Phone Key D"
    PhoneKeyD = 0xBF,
    /// Usage ID `0xC0`: "Phone Call History Key"
    PhoneCallHistoryKey = 0xC0,
    /// Usage ID `0xC1`: "Phone Caller ID Key"
    PhoneCallerIDKey = 0xC1,
    /// Usage ID `0xC2`: "Phone Settings Key"
    PhoneSettingsKey = 0xC2,
    /// Usage ID `0xF0`: "Host Control"
    HostControl = 0xF0,
    /// Usage ID `0xF1`: "Host Available"
    HostAvailable = 0xF1,
    /// Usage ID `0xF2`: "Host Call Active"
    HostCallActive = 0xF2,
    /// Usage ID `0xF3`: "Activate Handset Audio"
    ActivateHandsetAudio = 0xF3,
    /// Usage ID `0xF4`: "Ring Type"
    RingType = 0xF4,
    /// Usage ID `0xF5`: "Re-dialable Phone Number"
    RedialablePhoneNumber = 0xF5,
    /// Usage ID `0xF8`: "Stop Ring Tone"
    StopRingTone = 0xF8,
    /// Usage ID `0xF9`: "PSTN Ring Tone"
    PSTNRingTone = 0xF9,
    /// Usage ID `0xFA`: "Host Ring Tone"
    HostRingTone = 0xFA,
    /// Usage ID `0xFB`: "Alert Sound Error"
    AlertSoundError = 0xFB,
    /// Usage ID `0xFC`: "Alert Sound Confirm"
    AlertSoundConfirm = 0xFC,
    /// Usage ID `0xFD`: "Alert Sound Notification"
    AlertSoundNotification = 0xFD,
    /// Usage ID `0xFE`: "Silent Ring"
    SilentRing = 0xFE,
    /// Usage ID `0x108`: "Email Message Waiting"
    EmailMessageWaiting = 0x108,
    /// Usage ID `0x109`: "Voicemail Message Waiting"
    VoicemailMessageWaiting = 0x109,
    /// Usage ID `0x10A`: "Host Hold"
    HostHold = 0x10A,
    /// Usage ID `0x110`: "Incoming Call History Count"
    IncomingCallHistoryCount = 0x110,
    /// Usage ID `0x111`: "Outgoing Call History Count"
    OutgoingCallHistoryCount = 0x111,
    /// Usage ID `0x112`: "Incoming Call History"
    IncomingCallHistory = 0x112,
    /// Usage ID `0x113`: "Outgoing Call History"
    OutgoingCallHistory = 0x113,
    /// Usage ID `0x114`: "Phone Locale"
    PhoneLocale = 0x114,
    /// Usage ID `0x140`: "Phone Time Second"
    PhoneTimeSecond = 0x140,
    /// Usage ID `0x141`: "Phone Time Minute"
    PhoneTimeMinute = 0x141,
    /// Usage ID `0x142`: "Phone Time Hour"
    PhoneTimeHour = 0x142,
    /// Usage ID `0x143`: "Phone Date Day"
    PhoneDateDay = 0x143,
    /// Usage ID `0x144`: "Phone Date Month"
    PhoneDateMonth = 0x144,
    /// Usage ID `0x145`: "Phone Date Year"
    PhoneDateYear = 0x145,
    /// Usage ID `0x146`: "Handset Nickname"
    HandsetNickname = 0x146,
    /// Usage ID `0x147`: "Address Book ID"
    AddressBookID = 0x147,
    /// Usage ID `0x14A`: "Call Duration"
    CallDuration = 0x14A,
    /// Usage ID `0x14B`: "Dual Mode Phone"
    DualModePhone = 0x14B,
}

impl TelephonyDevice {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            TelephonyDevice::Phone => "Phone",
            TelephonyDevice::AnsweringMachine => "Answering Machine",
            TelephonyDevice::MessageControls => "Message Controls",
            TelephonyDevice::Handset => "Handset",
            TelephonyDevice::Headset => "Headset",
            TelephonyDevice::TelephonyKeyPad => "Telephony Key Pad",
            TelephonyDevice::ProgrammableButton => "Programmable Button",
            TelephonyDevice::HookSwitch => "Hook Switch",
            TelephonyDevice::Flash => "Flash",
            TelephonyDevice::Feature => "Feature",
            TelephonyDevice::Hold => "Hold",
            TelephonyDevice::Redial => "Redial",
            TelephonyDevice::Transfer => "Transfer",
            TelephonyDevice::Drop => "Drop",
            TelephonyDevice::Park => "Park",
            TelephonyDevice::ForwardCalls => "Forward Calls",
            TelephonyDevice::AlternateFunction => "Alternate Function",
            TelephonyDevice::Line => "Line",
            TelephonyDevice::SpeakerPhone => "Speaker Phone",
            TelephonyDevice::Conference => "Conference",
            TelephonyDevice::RingEnable => "Ring Enable",
            TelephonyDevice::RingSelect => "Ring Select",
            TelephonyDevice::PhoneMute => "Phone Mute",
            TelephonyDevice::CallerID => "Caller ID",
            TelephonyDevice::Send => "Send",
            TelephonyDevice::SpeedDial => "Speed Dial",
            TelephonyDevice::StoreNumber => "Store Number",
            TelephonyDevice::RecallNumber => "Recall Number",
            TelephonyDevice::PhoneDirectory => "Phone Directory",
            TelephonyDevice::VoiceMail => "Voice Mail",
            TelephonyDevice::ScreenCalls => "Screen Calls",
            TelephonyDevice::DoNotDisturb => "Do Not Disturb",
            TelephonyDevice::Message => "Message",
            TelephonyDevice::AnswerOnOff => "Answer On/Off",
            TelephonyDevice::InsideDialTone => "Inside Dial Tone",
            TelephonyDevice::OutsideDialTone => "Outside Dial Tone",
            TelephonyDevice::InsideRingTone => "Inside Ring Tone",
            TelephonyDevice::OutsideRingTone => "Outside Ring Tone",
            TelephonyDevice::PriorityRingTone => "Priority Ring Tone",
            TelephonyDevice::InsideRingback => "Inside Ringback",
            TelephonyDevice::PriorityRingback => "Priority Ringback",
            TelephonyDevice::LineBusyTone => "Line Busy Tone",
            TelephonyDevice::ReorderTone => "Reorder Tone",
            TelephonyDevice::CallWaitingTone => "Call Waiting Tone",
            TelephonyDevice::ConfirmationTone1 => "Confirmation Tone 1",
            TelephonyDevice::ConfirmationTone2 => "Confirmation Tone 2",
            TelephonyDevice::TonesOff => "Tones Off",
            TelephonyDevice::OutsideRingback => "Outside Ringback",
            TelephonyDevice::Ringer => "Ringer",
            TelephonyDevice::PhoneKey0 => "Phone Key 0",
            TelephonyDevice::PhoneKey1 => "Phone Key 1",
            TelephonyDevice::PhoneKey2 => "Phone Key 2",
            TelephonyDevice::PhoneKey3 => "Phone Key 3",
            TelephonyDevice::PhoneKey4 => "Phone Key 4",
            TelephonyDevice::PhoneKey5 => "Phone Key 5",
            TelephonyDevice::PhoneKey6 => "Phone Key 6",
            TelephonyDevice::PhoneKey7 => "Phone Key 7",
            TelephonyDevice::PhoneKey8 => "Phone Key 8",
            TelephonyDevice::PhoneKey9 => "Phone Key 9",
            TelephonyDevice::PhoneKeyStar => "Phone Key Star",
            TelephonyDevice::PhoneKeyPound => "Phone Key Pound",
            TelephonyDevice::PhoneKeyA => "Phone Key A",
            TelephonyDevice::PhoneKeyB => "Phone Key B",
            TelephonyDevice::PhoneKeyC => "Phone Key C",
            TelephonyDevice::PhoneKeyD => "Phone Key D",
            TelephonyDevice::PhoneCallHistoryKey => "Phone Call History Key",
            TelephonyDevice::PhoneCallerIDKey => "Phone Caller ID Key",
            TelephonyDevice::PhoneSettingsKey => "Phone Settings Key",
            TelephonyDevice::HostControl => "Host Control",
            TelephonyDevice::HostAvailable => "Host Available",
            TelephonyDevice::HostCallActive => "Host Call Active",
            TelephonyDevice::ActivateHandsetAudio => "Activate Handset Audio",
            TelephonyDevice::RingType => "Ring Type",
            TelephonyDevice::RedialablePhoneNumber => "Re-dialable Phone Number",
            TelephonyDevice::StopRingTone => "Stop Ring Tone",
            TelephonyDevice::PSTNRingTone => "PSTN Ring Tone",
            TelephonyDevice::HostRingTone => "Host Ring Tone",
            TelephonyDevice::AlertSoundError => "Alert Sound Error",
            TelephonyDevice::AlertSoundConfirm => "Alert Sound Confirm",
            TelephonyDevice::AlertSoundNotification => "Alert Sound Notification",
            TelephonyDevice::SilentRing => "Silent Ring",
            TelephonyDevice::EmailMessageWaiting => "Email Message Waiting",
            TelephonyDevice::VoicemailMessageWaiting => "Voicemail Message Waiting",
            TelephonyDevice::HostHold => "Host Hold",
            TelephonyDevice::IncomingCallHistoryCount => "Incoming Call History Count",
            TelephonyDevice::OutgoingCallHistoryCount => "Outgoing Call History Count",
            TelephonyDevice::IncomingCallHistory => "Incoming Call History",
            TelephonyDevice::OutgoingCallHistory => "Outgoing Call History",
            TelephonyDevice::PhoneLocale => "Phone Locale",
            TelephonyDevice::PhoneTimeSecond => "Phone Time Second",
            TelephonyDevice::PhoneTimeMinute => "Phone Time Minute",
            TelephonyDevice::PhoneTimeHour => "Phone Time Hour",
            TelephonyDevice::PhoneDateDay => "Phone Date Day",
            TelephonyDevice::PhoneDateMonth => "Phone Date Month",
            TelephonyDevice::PhoneDateYear => "Phone Date Year",
            TelephonyDevice::HandsetNickname => "Handset Nickname",
            TelephonyDevice::AddressBookID => "Address Book ID",
            TelephonyDevice::CallDuration => "Call Duration",
            TelephonyDevice::DualModePhone => "Dual Mode Phone",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for TelephonyDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for TelephonyDevice {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::TelephonyDevice(self)](Usage::TelephonyDevice)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for TelephonyDevice {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xB` for [TelephonyDevice]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::TelephonyDevice]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&TelephonyDevice> for u16 {
    fn from(telephonydevice: &TelephonyDevice) -> u16 {
        *telephonydevice as u16
    }
}

impl From<TelephonyDevice> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [TelephonyDevice::usage_page_value()].
    fn from(telephonydevice: TelephonyDevice) -> u16 {
        u16::from(&telephonydevice)
    }
}

impl From<&TelephonyDevice> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [TelephonyDevice::usage_value()].
    fn from(telephonydevice: &TelephonyDevice) -> u32 {
        let up = UsagePage::from(telephonydevice);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(telephonydevice) as u32;
        up | id
    }
}

impl From<&TelephonyDevice> for UsagePage {
    /// Always returns [UsagePage::TelephonyDevice] and is
    /// identical to [TelephonyDevice::usage_page()].
    fn from(_: &TelephonyDevice) -> UsagePage {
        UsagePage::TelephonyDevice
    }
}

impl From<TelephonyDevice> for UsagePage {
    /// Always returns [UsagePage::TelephonyDevice] and is
    /// identical to [TelephonyDevice::usage_page()].
    fn from(_: TelephonyDevice) -> UsagePage {
        UsagePage::TelephonyDevice
    }
}

impl From<&TelephonyDevice> for Usage {
    fn from(telephonydevice: &TelephonyDevice) -> Usage {
        Usage::try_from(u32::from(telephonydevice)).unwrap()
    }
}

impl From<TelephonyDevice> for Usage {
    fn from(telephonydevice: TelephonyDevice) -> Usage {
        Usage::from(&telephonydevice)
    }
}

impl TryFrom<u16> for TelephonyDevice {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<TelephonyDevice> {
        match usage_id {
            1 => Ok(TelephonyDevice::Phone),
            2 => Ok(TelephonyDevice::AnsweringMachine),
            3 => Ok(TelephonyDevice::MessageControls),
            4 => Ok(TelephonyDevice::Handset),
            5 => Ok(TelephonyDevice::Headset),
            6 => Ok(TelephonyDevice::TelephonyKeyPad),
            7 => Ok(TelephonyDevice::ProgrammableButton),
            32 => Ok(TelephonyDevice::HookSwitch),
            33 => Ok(TelephonyDevice::Flash),
            34 => Ok(TelephonyDevice::Feature),
            35 => Ok(TelephonyDevice::Hold),
            36 => Ok(TelephonyDevice::Redial),
            37 => Ok(TelephonyDevice::Transfer),
            38 => Ok(TelephonyDevice::Drop),
            39 => Ok(TelephonyDevice::Park),
            40 => Ok(TelephonyDevice::ForwardCalls),
            41 => Ok(TelephonyDevice::AlternateFunction),
            42 => Ok(TelephonyDevice::Line),
            43 => Ok(TelephonyDevice::SpeakerPhone),
            44 => Ok(TelephonyDevice::Conference),
            45 => Ok(TelephonyDevice::RingEnable),
            46 => Ok(TelephonyDevice::RingSelect),
            47 => Ok(TelephonyDevice::PhoneMute),
            48 => Ok(TelephonyDevice::CallerID),
            49 => Ok(TelephonyDevice::Send),
            80 => Ok(TelephonyDevice::SpeedDial),
            81 => Ok(TelephonyDevice::StoreNumber),
            82 => Ok(TelephonyDevice::RecallNumber),
            83 => Ok(TelephonyDevice::PhoneDirectory),
            112 => Ok(TelephonyDevice::VoiceMail),
            113 => Ok(TelephonyDevice::ScreenCalls),
            114 => Ok(TelephonyDevice::DoNotDisturb),
            115 => Ok(TelephonyDevice::Message),
            116 => Ok(TelephonyDevice::AnswerOnOff),
            144 => Ok(TelephonyDevice::InsideDialTone),
            145 => Ok(TelephonyDevice::OutsideDialTone),
            146 => Ok(TelephonyDevice::InsideRingTone),
            147 => Ok(TelephonyDevice::OutsideRingTone),
            148 => Ok(TelephonyDevice::PriorityRingTone),
            149 => Ok(TelephonyDevice::InsideRingback),
            150 => Ok(TelephonyDevice::PriorityRingback),
            151 => Ok(TelephonyDevice::LineBusyTone),
            152 => Ok(TelephonyDevice::ReorderTone),
            153 => Ok(TelephonyDevice::CallWaitingTone),
            154 => Ok(TelephonyDevice::ConfirmationTone1),
            155 => Ok(TelephonyDevice::ConfirmationTone2),
            156 => Ok(TelephonyDevice::TonesOff),
            157 => Ok(TelephonyDevice::OutsideRingback),
            158 => Ok(TelephonyDevice::Ringer),
            176 => Ok(TelephonyDevice::PhoneKey0),
            177 => Ok(TelephonyDevice::PhoneKey1),
            178 => Ok(TelephonyDevice::PhoneKey2),
            179 => Ok(TelephonyDevice::PhoneKey3),
            180 => Ok(TelephonyDevice::PhoneKey4),
            181 => Ok(TelephonyDevice::PhoneKey5),
            182 => Ok(TelephonyDevice::PhoneKey6),
            183 => Ok(TelephonyDevice::PhoneKey7),
            184 => Ok(TelephonyDevice::PhoneKey8),
            185 => Ok(TelephonyDevice::PhoneKey9),
            186 => Ok(TelephonyDevice::PhoneKeyStar),
            187 => Ok(TelephonyDevice::PhoneKeyPound),
            188 => Ok(TelephonyDevice::PhoneKeyA),
            189 => Ok(TelephonyDevice::PhoneKeyB),
            190 => Ok(TelephonyDevice::PhoneKeyC),
            191 => Ok(TelephonyDevice::PhoneKeyD),
            192 => Ok(TelephonyDevice::PhoneCallHistoryKey),
            193 => Ok(TelephonyDevice::PhoneCallerIDKey),
            194 => Ok(TelephonyDevice::PhoneSettingsKey),
            240 => Ok(TelephonyDevice::HostControl),
            241 => Ok(TelephonyDevice::HostAvailable),
            242 => Ok(TelephonyDevice::HostCallActive),
            243 => Ok(TelephonyDevice::ActivateHandsetAudio),
            244 => Ok(TelephonyDevice::RingType),
            245 => Ok(TelephonyDevice::RedialablePhoneNumber),
            248 => Ok(TelephonyDevice::StopRingTone),
            249 => Ok(TelephonyDevice::PSTNRingTone),
            250 => Ok(TelephonyDevice::HostRingTone),
            251 => Ok(TelephonyDevice::AlertSoundError),
            252 => Ok(TelephonyDevice::AlertSoundConfirm),
            253 => Ok(TelephonyDevice::AlertSoundNotification),
            254 => Ok(TelephonyDevice::SilentRing),
            264 => Ok(TelephonyDevice::EmailMessageWaiting),
            265 => Ok(TelephonyDevice::VoicemailMessageWaiting),
            266 => Ok(TelephonyDevice::HostHold),
            272 => Ok(TelephonyDevice::IncomingCallHistoryCount),
            273 => Ok(TelephonyDevice::OutgoingCallHistoryCount),
            274 => Ok(TelephonyDevice::IncomingCallHistory),
            275 => Ok(TelephonyDevice::OutgoingCallHistory),
            276 => Ok(TelephonyDevice::PhoneLocale),
            320 => Ok(TelephonyDevice::PhoneTimeSecond),
            321 => Ok(TelephonyDevice::PhoneTimeMinute),
            322 => Ok(TelephonyDevice::PhoneTimeHour),
            323 => Ok(TelephonyDevice::PhoneDateDay),
            324 => Ok(TelephonyDevice::PhoneDateMonth),
            325 => Ok(TelephonyDevice::PhoneDateYear),
            326 => Ok(TelephonyDevice::HandsetNickname),
            327 => Ok(TelephonyDevice::AddressBookID),
            330 => Ok(TelephonyDevice::CallDuration),
            331 => Ok(TelephonyDevice::DualModePhone),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for TelephonyDevice {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xC`: "Consumer"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Consumer(Consumer::NumericKeyPad);
/// let u2 = Usage::new_from_page_and_id(0xC, 0x2).unwrap();
/// let u3 = Usage::from(Consumer::NumericKeyPad);
/// let u4: Usage = Consumer::NumericKeyPad.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Consumer));
/// assert_eq!(0xC, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0xC << 16) | 0x2, u1.usage_value());
/// assert_eq!("Numeric Key Pad", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Consumer {
    /// Usage ID `0x1`: "Consumer Control"
    ConsumerControl = 0x1,
    /// Usage ID `0x2`: "Numeric Key Pad"
    NumericKeyPad = 0x2,
    /// Usage ID `0x3`: "Programmable Buttons"
    ProgrammableButtons = 0x3,
    /// Usage ID `0x4`: "Microphone"
    Microphone = 0x4,
    /// Usage ID `0x5`: "Headphone"
    Headphone = 0x5,
    /// Usage ID `0x6`: "Graphic Equalizer"
    GraphicEqualizer = 0x6,
    /// Usage ID `0x7`: "Keyboard Backlight"
    KeyboardBacklight = 0x7,
    /// Usage ID `0x20`: "+10"
    Plus10 = 0x20,
    /// Usage ID `0x21`: "+100"
    Plus100 = 0x21,
    /// Usage ID `0x22`: "AM/PM"
    AMPM = 0x22,
    /// Usage ID `0x30`: "Power"
    Power = 0x30,
    /// Usage ID `0x31`: "Reset"
    Reset = 0x31,
    /// Usage ID `0x32`: "Sleep"
    Sleep = 0x32,
    /// Usage ID `0x33`: "Sleep After"
    SleepAfter = 0x33,
    /// Usage ID `0x34`: "Sleep Mode"
    SleepMode = 0x34,
    /// Usage ID `0x35`: "Illumination"
    Illumination = 0x35,
    /// Usage ID `0x36`: "Function Buttons"
    FunctionButtons = 0x36,
    /// Usage ID `0x40`: "Menu"
    Menu = 0x40,
    /// Usage ID `0x41`: "Menu Pick"
    MenuPick = 0x41,
    /// Usage ID `0x42`: "Menu Up"
    MenuUp = 0x42,
    /// Usage ID `0x43`: "Menu Down"
    MenuDown = 0x43,
    /// Usage ID `0x44`: "Menu Left"
    MenuLeft = 0x44,
    /// Usage ID `0x45`: "Menu Right"
    MenuRight = 0x45,
    /// Usage ID `0x46`: "Menu Escape"
    MenuEscape = 0x46,
    /// Usage ID `0x47`: "Menu Value Increase"
    MenuValueIncrease = 0x47,
    /// Usage ID `0x48`: "Menu Value Decrease"
    MenuValueDecrease = 0x48,
    /// Usage ID `0x60`: "Data On Screen"
    DataOnScreen = 0x60,
    /// Usage ID `0x61`: "Closed Caption"
    ClosedCaption = 0x61,
    /// Usage ID `0x62`: "Closed Caption Select"
    ClosedCaptionSelect = 0x62,
    /// Usage ID `0x63`: "VCR/TV"
    VCRTV = 0x63,
    /// Usage ID `0x64`: "Broadcast Mode"
    BroadcastMode = 0x64,
    /// Usage ID `0x65`: "Snapshot"
    Snapshot = 0x65,
    /// Usage ID `0x66`: "Still"
    Still = 0x66,
    /// Usage ID `0x67`: "Picture-in-Picture Toggle"
    PictureinPictureToggle = 0x67,
    /// Usage ID `0x68`: "Picture-in-Picture Swap"
    PictureinPictureSwap = 0x68,
    /// Usage ID `0x69`: "Red Menu Button"
    RedMenuButton = 0x69,
    /// Usage ID `0x6A`: "Green Menu Button"
    GreenMenuButton = 0x6A,
    /// Usage ID `0x6B`: "Blue Menu Button"
    BlueMenuButton = 0x6B,
    /// Usage ID `0x6C`: "Yellow Menu Button"
    YellowMenuButton = 0x6C,
    /// Usage ID `0x6D`: "Aspect"
    Aspect = 0x6D,
    /// Usage ID `0x6E`: "3D Mode Select"
    ThreeDModeSelect = 0x6E,
    /// Usage ID `0x6F`: "Display Brightness Increment"
    DisplayBrightnessIncrement = 0x6F,
    /// Usage ID `0x70`: "Display Brightness Decrement"
    DisplayBrightnessDecrement = 0x70,
    /// Usage ID `0x71`: "Display Brightness"
    DisplayBrightness = 0x71,
    /// Usage ID `0x72`: "Display Backlight Toggle"
    DisplayBacklightToggle = 0x72,
    /// Usage ID `0x73`: "Display Set Brightness to Minimum"
    DisplaySetBrightnesstoMinimum = 0x73,
    /// Usage ID `0x74`: "Display Set Brightness to Maximum"
    DisplaySetBrightnesstoMaximum = 0x74,
    /// Usage ID `0x75`: "Display Set Auto Brightness"
    DisplaySetAutoBrightness = 0x75,
    /// Usage ID `0x76`: "Camera Access Enabled"
    CameraAccessEnabled = 0x76,
    /// Usage ID `0x77`: "Camera Access Disabled"
    CameraAccessDisabled = 0x77,
    /// Usage ID `0x78`: "Camera Access Toggle"
    CameraAccessToggle = 0x78,
    /// Usage ID `0x79`: "Keyboard Brightness Increment"
    KeyboardBrightnessIncrement = 0x79,
    /// Usage ID `0x7A`: "Keyboard Brightness Decrement"
    KeyboardBrightnessDecrement = 0x7A,
    /// Usage ID `0x7B`: "Keyboard Backlight Set Level"
    KeyboardBacklightSetLevel = 0x7B,
    /// Usage ID `0x7C`: "Keyboard Backlight OOC"
    KeyboardBacklightOOC = 0x7C,
    /// Usage ID `0x7D`: "Keyboard Backlight Set Minimum"
    KeyboardBacklightSetMinimum = 0x7D,
    /// Usage ID `0x7E`: "Keyboard Backlight Set Maximum"
    KeyboardBacklightSetMaximum = 0x7E,
    /// Usage ID `0x7F`: "Keyboard Backlight Auto"
    KeyboardBacklightAuto = 0x7F,
    /// Usage ID `0x80`: "Selection"
    Selection = 0x80,
    /// Usage ID `0x81`: "Assign Selection"
    AssignSelection = 0x81,
    /// Usage ID `0x82`: "Mode Step"
    ModeStep = 0x82,
    /// Usage ID `0x83`: "Recall Last"
    RecallLast = 0x83,
    /// Usage ID `0x84`: "Enter Channel"
    EnterChannel = 0x84,
    /// Usage ID `0x85`: "Order Movie"
    OrderMovie = 0x85,
    /// Usage ID `0x86`: "Channel"
    Channel = 0x86,
    /// Usage ID `0x87`: "Media Selection"
    MediaSelection = 0x87,
    /// Usage ID `0x88`: "Media Select Computer"
    MediaSelectComputer = 0x88,
    /// Usage ID `0x89`: "Media Select TV"
    MediaSelectTV = 0x89,
    /// Usage ID `0x8A`: "Media Select WWW"
    MediaSelectWWW = 0x8A,
    /// Usage ID `0x8B`: "Media Select DVD"
    MediaSelectDVD = 0x8B,
    /// Usage ID `0x8C`: "Media Select Telephone"
    MediaSelectTelephone = 0x8C,
    /// Usage ID `0x8D`: "Media Select Program Guide"
    MediaSelectProgramGuide = 0x8D,
    /// Usage ID `0x8E`: "Media Select Video Phone"
    MediaSelectVideoPhone = 0x8E,
    /// Usage ID `0x8F`: "Media Select Games"
    MediaSelectGames = 0x8F,
    /// Usage ID `0x90`: "Media Select Messages"
    MediaSelectMessages = 0x90,
    /// Usage ID `0x91`: "Media Select CD"
    MediaSelectCD = 0x91,
    /// Usage ID `0x92`: "Media Select VCR"
    MediaSelectVCR = 0x92,
    /// Usage ID `0x93`: "Media Select Tuner"
    MediaSelectTuner = 0x93,
    /// Usage ID `0x94`: "Quit"
    Quit = 0x94,
    /// Usage ID `0x95`: "Help"
    Help = 0x95,
    /// Usage ID `0x96`: "Media Select Tape"
    MediaSelectTape = 0x96,
    /// Usage ID `0x97`: "Media Select Cable"
    MediaSelectCable = 0x97,
    /// Usage ID `0x98`: "Media Select Satellite"
    MediaSelectSatellite = 0x98,
    /// Usage ID `0x99`: "Media Select Security"
    MediaSelectSecurity = 0x99,
    /// Usage ID `0x9A`: "Media Select Home"
    MediaSelectHome = 0x9A,
    /// Usage ID `0x9B`: "Media Select Call"
    MediaSelectCall = 0x9B,
    /// Usage ID `0x9C`: "Channel Increment"
    ChannelIncrement = 0x9C,
    /// Usage ID `0x9D`: "Channel Decrement"
    ChannelDecrement = 0x9D,
    /// Usage ID `0x9E`: "Media Select SAP"
    MediaSelectSAP = 0x9E,
    /// Usage ID `0xA0`: "VCR Plus"
    VCRPlus = 0xA0,
    /// Usage ID `0xA1`: "Once"
    Once = 0xA1,
    /// Usage ID `0xA2`: "Daily"
    Daily = 0xA2,
    /// Usage ID `0xA3`: "Weekly"
    Weekly = 0xA3,
    /// Usage ID `0xA4`: "Monthly"
    Monthly = 0xA4,
    /// Usage ID `0xB0`: "Play"
    Play = 0xB0,
    /// Usage ID `0xB1`: "Pause"
    Pause = 0xB1,
    /// Usage ID `0xB2`: "Record"
    Record = 0xB2,
    /// Usage ID `0xB3`: "Fast Forward"
    FastForward = 0xB3,
    /// Usage ID `0xB4`: "Rewind"
    Rewind = 0xB4,
    /// Usage ID `0xB5`: "Scan Next Track"
    ScanNextTrack = 0xB5,
    /// Usage ID `0xB6`: "Scan Previous Track"
    ScanPreviousTrack = 0xB6,
    /// Usage ID `0xB7`: "Stop"
    Stop = 0xB7,
    /// Usage ID `0xB8`: "Eject"
    Eject = 0xB8,
    /// Usage ID `0xB9`: "Random Play"
    RandomPlay = 0xB9,
    /// Usage ID `0xBA`: "Select Disc"
    SelectDisc = 0xBA,
    /// Usage ID `0xBB`: "Enter Disc"
    EnterDisc = 0xBB,
    /// Usage ID `0xBC`: "Repeat"
    Repeat = 0xBC,
    /// Usage ID `0xBD`: "Tracking"
    Tracking = 0xBD,
    /// Usage ID `0xBE`: "Track Normal"
    TrackNormal = 0xBE,
    /// Usage ID `0xBF`: "Slow Tracking"
    SlowTracking = 0xBF,
    /// Usage ID `0xC0`: "Frame Forward"
    FrameForward = 0xC0,
    /// Usage ID `0xC1`: "Frame Back"
    FrameBack = 0xC1,
    /// Usage ID `0xC2`: "Mark"
    Mark = 0xC2,
    /// Usage ID `0xC3`: "Clear Mark"
    ClearMark = 0xC3,
    /// Usage ID `0xC4`: "Repeat From Mark"
    RepeatFromMark = 0xC4,
    /// Usage ID `0xC5`: "Return To Mark"
    ReturnToMark = 0xC5,
    /// Usage ID `0xC6`: "Search Mark Forward"
    SearchMarkForward = 0xC6,
    /// Usage ID `0xC7`: "Search Mark Backwards"
    SearchMarkBackwards = 0xC7,
    /// Usage ID `0xC8`: "Counter Reset"
    CounterReset = 0xC8,
    /// Usage ID `0xC9`: "Show Counter"
    ShowCounter = 0xC9,
    /// Usage ID `0xCA`: "Tracking Increment"
    TrackingIncrement = 0xCA,
    /// Usage ID `0xCB`: "Tracking Decrement"
    TrackingDecrement = 0xCB,
    /// Usage ID `0xCC`: "Stop/Eject"
    StopEject = 0xCC,
    /// Usage ID `0xCD`: "Play/Pause"
    PlayPause = 0xCD,
    /// Usage ID `0xCE`: "Play/Skip"
    PlaySkip = 0xCE,
    /// Usage ID `0xCF`: "Voice Command"
    VoiceCommand = 0xCF,
    /// Usage ID `0xD0`: "Invoke Capture Interface"
    InvokeCaptureInterface = 0xD0,
    /// Usage ID `0xD1`: "Start or Stop Game Recording"
    StartorStopGameRecording = 0xD1,
    /// Usage ID `0xD2`: "Historical Game Capture"
    HistoricalGameCapture = 0xD2,
    /// Usage ID `0xD3`: "Capture Game Screenshot"
    CaptureGameScreenshot = 0xD3,
    /// Usage ID `0xD4`: "Show or Hide Recording Indicator"
    ShoworHideRecordingIndicator = 0xD4,
    /// Usage ID `0xD5`: "Start or Stop Microphone Capture"
    StartorStopMicrophoneCapture = 0xD5,
    /// Usage ID `0xD6`: "Start or Stop Camera Capture"
    StartorStopCameraCapture = 0xD6,
    /// Usage ID `0xD7`: "Start or Stop Game Broadcast"
    StartorStopGameBroadcast = 0xD7,
    /// Usage ID `0xD8`: "Start or Stop Voice Dictation Session"
    StartorStopVoiceDictationSession = 0xD8,
    /// Usage ID `0xD9`: "Invoke/Dismiss Emoji Picker"
    InvokeDismissEmojiPicker = 0xD9,
    /// Usage ID `0xE0`: "Volume"
    Volume = 0xE0,
    /// Usage ID `0xE1`: "Balance"
    Balance = 0xE1,
    /// Usage ID `0xE2`: "Mute"
    Mute = 0xE2,
    /// Usage ID `0xE3`: "Bass"
    Bass = 0xE3,
    /// Usage ID `0xE4`: "Treble"
    Treble = 0xE4,
    /// Usage ID `0xE5`: "Bass Boost"
    BassBoost = 0xE5,
    /// Usage ID `0xE6`: "Surround Mode"
    SurroundMode = 0xE6,
    /// Usage ID `0xE7`: "Loudness"
    Loudness = 0xE7,
    /// Usage ID `0xE8`: "MPX"
    MPX = 0xE8,
    /// Usage ID `0xE9`: "Volume Increment"
    VolumeIncrement = 0xE9,
    /// Usage ID `0xEA`: "Volume Decrement"
    VolumeDecrement = 0xEA,
    /// Usage ID `0xF0`: "Speed Select"
    SpeedSelect = 0xF0,
    /// Usage ID `0xF1`: "Playback Speed"
    PlaybackSpeed = 0xF1,
    /// Usage ID `0xF2`: "Standard Play"
    StandardPlay = 0xF2,
    /// Usage ID `0xF3`: "Long Play"
    LongPlay = 0xF3,
    /// Usage ID `0xF4`: "Extended Play"
    ExtendedPlay = 0xF4,
    /// Usage ID `0xF5`: "Slow"
    Slow = 0xF5,
    /// Usage ID `0x100`: "Fan Enable"
    FanEnable = 0x100,
    /// Usage ID `0x101`: "Fan Speed"
    FanSpeed = 0x101,
    /// Usage ID `0x102`: "Light Enable"
    LightEnable = 0x102,
    /// Usage ID `0x103`: "Light Illumination Level"
    LightIlluminationLevel = 0x103,
    /// Usage ID `0x104`: "Climate Control Enable"
    ClimateControlEnable = 0x104,
    /// Usage ID `0x105`: "Room Temperature"
    RoomTemperature = 0x105,
    /// Usage ID `0x106`: "Security Enable"
    SecurityEnable = 0x106,
    /// Usage ID `0x107`: "Fire Alarm"
    FireAlarm = 0x107,
    /// Usage ID `0x108`: "Police Alarm"
    PoliceAlarm = 0x108,
    /// Usage ID `0x109`: "Proximity"
    Proximity = 0x109,
    /// Usage ID `0x10A`: "Motion"
    Motion = 0x10A,
    /// Usage ID `0x10B`: "Duress Alarm"
    DuressAlarm = 0x10B,
    /// Usage ID `0x10C`: "Holdup Alarm"
    HoldupAlarm = 0x10C,
    /// Usage ID `0x10D`: "Medical Alarm"
    MedicalAlarm = 0x10D,
    /// Usage ID `0x150`: "Balance Right"
    BalanceRight = 0x150,
    /// Usage ID `0x151`: "Balance Left"
    BalanceLeft = 0x151,
    /// Usage ID `0x152`: "Bass Increment"
    BassIncrement = 0x152,
    /// Usage ID `0x153`: "Bass Decrement"
    BassDecrement = 0x153,
    /// Usage ID `0x154`: "Treble Increment"
    TrebleIncrement = 0x154,
    /// Usage ID `0x155`: "Treble Decrement"
    TrebleDecrement = 0x155,
    /// Usage ID `0x160`: "Speaker System"
    SpeakerSystem = 0x160,
    /// Usage ID `0x161`: "Channel Left"
    ChannelLeft = 0x161,
    /// Usage ID `0x162`: "Channel Right"
    ChannelRight = 0x162,
    /// Usage ID `0x163`: "Channel Center"
    ChannelCenter = 0x163,
    /// Usage ID `0x164`: "Channel Front"
    ChannelFront = 0x164,
    /// Usage ID `0x165`: "Channel Center Front"
    ChannelCenterFront = 0x165,
    /// Usage ID `0x166`: "Channel Side"
    ChannelSide = 0x166,
    /// Usage ID `0x167`: "Channel Surround"
    ChannelSurround = 0x167,
    /// Usage ID `0x168`: "Channel Low Frequency Enhancement"
    ChannelLowFrequencyEnhancement = 0x168,
    /// Usage ID `0x169`: "Channel Top"
    ChannelTop = 0x169,
    /// Usage ID `0x16A`: "Channel Unknown"
    ChannelUnknown = 0x16A,
    /// Usage ID `0x170`: "Sub-channel"
    Subchannel = 0x170,
    /// Usage ID `0x171`: "Sub-channel Increment"
    SubchannelIncrement = 0x171,
    /// Usage ID `0x172`: "Sub-channel Decrement"
    SubchannelDecrement = 0x172,
    /// Usage ID `0x173`: "Alternate Audio Increment"
    AlternateAudioIncrement = 0x173,
    /// Usage ID `0x174`: "Alternate Audio Decrement"
    AlternateAudioDecrement = 0x174,
    /// Usage ID `0x180`: "Application Launch Buttons"
    ApplicationLaunchButtons = 0x180,
    /// Usage ID `0x181`: "AL Launch Button Configuration Tool"
    ALLaunchButtonConfigurationTool = 0x181,
    /// Usage ID `0x182`: "AL Programmable Button Configuration"
    ALProgrammableButtonConfiguration = 0x182,
    /// Usage ID `0x183`: "AL Consumer Control Configuration"
    ALConsumerControlConfiguration = 0x183,
    /// Usage ID `0x184`: "AL Word Processor"
    ALWordProcessor = 0x184,
    /// Usage ID `0x185`: "AL Text Editor"
    ALTextEditor = 0x185,
    /// Usage ID `0x186`: "AL Spreadsheet"
    ALSpreadsheet = 0x186,
    /// Usage ID `0x187`: "AL Graphics Editor"
    ALGraphicsEditor = 0x187,
    /// Usage ID `0x188`: "AL Presentation App"
    ALPresentationApp = 0x188,
    /// Usage ID `0x189`: "AL Database App"
    ALDatabaseApp = 0x189,
    /// Usage ID `0x18A`: "AL Email Reader"
    ALEmailReader = 0x18A,
    /// Usage ID `0x18B`: "AL Newsreader"
    ALNewsreader = 0x18B,
    /// Usage ID `0x18C`: "AL Voicemail"
    ALVoicemail = 0x18C,
    /// Usage ID `0x18D`: "AL Contacts/Address Book"
    ALContactsAddressBook = 0x18D,
    /// Usage ID `0x18E`: "AL Calendar/Schedule"
    ALCalendarSchedule = 0x18E,
    /// Usage ID `0x18F`: "AL Task/Project Manager"
    ALTaskProjectManager = 0x18F,
    /// Usage ID `0x190`: "AL Log/Journal/Timecard"
    ALLogJournalTimecard = 0x190,
    /// Usage ID `0x191`: "AL Checkbook/Finance"
    ALCheckbookFinance = 0x191,
    /// Usage ID `0x192`: "AL Calculator"
    ALCalculator = 0x192,
    /// Usage ID `0x193`: "AL A/V Capture/Playback"
    ALAVCapturePlayback = 0x193,
    /// Usage ID `0x194`: "AL Local Machine Browser"
    ALLocalMachineBrowser = 0x194,
    /// Usage ID `0x195`: "AL LAN/WAN Browser"
    ALLANWANBrowser = 0x195,
    /// Usage ID `0x196`: "AL Internet Browser"
    ALInternetBrowser = 0x196,
    /// Usage ID `0x197`: "AL Remote Networking/ISP Connect"
    ALRemoteNetworkingISPConnect = 0x197,
    /// Usage ID `0x198`: "AL Network Conference"
    ALNetworkConference = 0x198,
    /// Usage ID `0x199`: "AL Network Chat"
    ALNetworkChat = 0x199,
    /// Usage ID `0x19A`: "AL Telephony/Dialer"
    ALTelephonyDialer = 0x19A,
    /// Usage ID `0x19B`: "AL Logon"
    ALLogon = 0x19B,
    /// Usage ID `0x19C`: "AL Logoff"
    ALLogoff = 0x19C,
    /// Usage ID `0x19D`: "AL Logon/Logoff"
    ALLogonLogoff = 0x19D,
    /// Usage ID `0x19E`: "AL Terminal Lock/Screensaver"
    ALTerminalLockScreensaver = 0x19E,
    /// Usage ID `0x19F`: "AL Control Panel"
    ALControlPanel = 0x19F,
    /// Usage ID `0x1A0`: "AL Command Line Processor/Run"
    ALCommandLineProcessorRun = 0x1A0,
    /// Usage ID `0x1A1`: "AL Process/Task Manager"
    ALProcessTaskManager = 0x1A1,
    /// Usage ID `0x1A2`: "AL Select Task/Application"
    ALSelectTaskApplication = 0x1A2,
    /// Usage ID `0x1A3`: "AL Next Task/Application"
    ALNextTaskApplication = 0x1A3,
    /// Usage ID `0x1A4`: "AL Previous Task/Application"
    ALPreviousTaskApplication = 0x1A4,
    /// Usage ID `0x1A5`: "AL Preemptive Halt Task/Application"
    ALPreemptiveHaltTaskApplication = 0x1A5,
    /// Usage ID `0x1A6`: "AL Integrated Help Center"
    ALIntegratedHelpCenter = 0x1A6,
    /// Usage ID `0x1A7`: "AL Documents"
    ALDocuments = 0x1A7,
    /// Usage ID `0x1A8`: "AL Thesaurus"
    ALThesaurus = 0x1A8,
    /// Usage ID `0x1A9`: "AL Dictionary"
    ALDictionary = 0x1A9,
    /// Usage ID `0x1AA`: "AL Desktop"
    ALDesktop = 0x1AA,
    /// Usage ID `0x1AB`: "AL Spell Check"
    ALSpellCheck = 0x1AB,
    /// Usage ID `0x1AC`: "AL Grammar Check"
    ALGrammarCheck = 0x1AC,
    /// Usage ID `0x1AD`: "AL Wireless Status"
    ALWirelessStatus = 0x1AD,
    /// Usage ID `0x1AE`: "AL Keyboard Layout"
    ALKeyboardLayout = 0x1AE,
    /// Usage ID `0x1AF`: "AL Virus Protection"
    ALVirusProtection = 0x1AF,
    /// Usage ID `0x1B0`: "AL Encryption"
    ALEncryption = 0x1B0,
    /// Usage ID `0x1B1`: "AL Screen Saver"
    ALScreenSaver = 0x1B1,
    /// Usage ID `0x1B2`: "AL Alarms"
    ALAlarms = 0x1B2,
    /// Usage ID `0x1B3`: "AL Clock"
    ALClock = 0x1B3,
    /// Usage ID `0x1B4`: "AL File Browser"
    ALFileBrowser = 0x1B4,
    /// Usage ID `0x1B5`: "AL Power Status"
    ALPowerStatus = 0x1B5,
    /// Usage ID `0x1B6`: "AL Image Browser"
    ALImageBrowser = 0x1B6,
    /// Usage ID `0x1B7`: "AL Audio Browser"
    ALAudioBrowser = 0x1B7,
    /// Usage ID `0x1B8`: "AL Movie Browser"
    ALMovieBrowser = 0x1B8,
    /// Usage ID `0x1B9`: "AL Digital Rights Manager"
    ALDigitalRightsManager = 0x1B9,
    /// Usage ID `0x1BA`: "AL Digital Wallet"
    ALDigitalWallet = 0x1BA,
    /// Usage ID `0x1BC`: "AL Instant Messaging"
    ALInstantMessaging = 0x1BC,
    /// Usage ID `0x1BD`: "AL OEM Features/ Tips/Tutorial Browser"
    ALOEMFeaturesTipsTutorialBrowser = 0x1BD,
    /// Usage ID `0x1BE`: "AL OEM Help"
    ALOEMHelp = 0x1BE,
    /// Usage ID `0x1BF`: "AL Online Community"
    ALOnlineCommunity = 0x1BF,
    /// Usage ID `0x1C0`: "AL Entertainment Content Browser"
    ALEntertainmentContentBrowser = 0x1C0,
    /// Usage ID `0x1C1`: "AL Online Shopping Browser"
    ALOnlineShoppingBrowser = 0x1C1,
    /// Usage ID `0x1C2`: "AL SmartCard Information/Help"
    ALSmartCardInformationHelp = 0x1C2,
    /// Usage ID `0x1C3`: "AL Market Monitor/Finance Browser"
    ALMarketMonitorFinanceBrowser = 0x1C3,
    /// Usage ID `0x1C4`: "AL Customized Corporate News Browser"
    ALCustomizedCorporateNewsBrowser = 0x1C4,
    /// Usage ID `0x1C5`: "AL Online Activity Browser"
    ALOnlineActivityBrowser = 0x1C5,
    /// Usage ID `0x1C6`: "AL Research/Search Browser"
    ALResearchSearchBrowser = 0x1C6,
    /// Usage ID `0x1C7`: "AL Audio Player"
    ALAudioPlayer = 0x1C7,
    /// Usage ID `0x1C8`: "AL Message Status"
    ALMessageStatus = 0x1C8,
    /// Usage ID `0x1C9`: "AL Contact Sync"
    ALContactSync = 0x1C9,
    /// Usage ID `0x1CA`: "AL Navigation"
    ALNavigation = 0x1CA,
    /// Usage ID `0x1CB`: "AL Context-aware Desktop Assistant"
    ALContextawareDesktopAssistant = 0x1CB,
    /// Usage ID `0x200`: "Generic GUI Application Controls"
    GenericGUIApplicationControls = 0x200,
    /// Usage ID `0x201`: "AC New"
    ACNew = 0x201,
    /// Usage ID `0x202`: "AC Open"
    ACOpen = 0x202,
    /// Usage ID `0x203`: "AC Close"
    ACClose = 0x203,
    /// Usage ID `0x204`: "AC Exit"
    ACExit = 0x204,
    /// Usage ID `0x205`: "AC Maximize"
    ACMaximize = 0x205,
    /// Usage ID `0x206`: "AC Minimize"
    ACMinimize = 0x206,
    /// Usage ID `0x207`: "AC Save"
    ACSave = 0x207,
    /// Usage ID `0x208`: "AC Print"
    ACPrint = 0x208,
    /// Usage ID `0x209`: "AC Properties"
    ACProperties = 0x209,
    /// Usage ID `0x21A`: "AC Undo"
    ACUndo = 0x21A,
    /// Usage ID `0x21B`: "AC Copy"
    ACCopy = 0x21B,
    /// Usage ID `0x21C`: "AC Cut"
    ACCut = 0x21C,
    /// Usage ID `0x21D`: "AC Paste"
    ACPaste = 0x21D,
    /// Usage ID `0x21E`: "AC Select All"
    ACSelectAll = 0x21E,
    /// Usage ID `0x21F`: "AC Find"
    ACFind = 0x21F,
    /// Usage ID `0x220`: "AC Find and Replace"
    ACFindandReplace = 0x220,
    /// Usage ID `0x221`: "AC Search"
    ACSearch = 0x221,
    /// Usage ID `0x222`: "AC Go To"
    ACGoTo = 0x222,
    /// Usage ID `0x223`: "AC Home"
    ACHome = 0x223,
    /// Usage ID `0x224`: "AC Back"
    ACBack = 0x224,
    /// Usage ID `0x225`: "AC Forward"
    ACForward = 0x225,
    /// Usage ID `0x226`: "AC Stop"
    ACStop = 0x226,
    /// Usage ID `0x227`: "AC Refresh"
    ACRefresh = 0x227,
    /// Usage ID `0x228`: "AC Previous Link"
    ACPreviousLink = 0x228,
    /// Usage ID `0x229`: "AC Next Link"
    ACNextLink = 0x229,
    /// Usage ID `0x22A`: "AC Bookmarks"
    ACBookmarks = 0x22A,
    /// Usage ID `0x22B`: "AC History"
    ACHistory = 0x22B,
    /// Usage ID `0x22C`: "AC Subscriptions"
    ACSubscriptions = 0x22C,
    /// Usage ID `0x22D`: "AC Zoom In"
    ACZoomIn = 0x22D,
    /// Usage ID `0x22E`: "AC Zoom Out"
    ACZoomOut = 0x22E,
    /// Usage ID `0x22F`: "AC Zoom"
    ACZoom = 0x22F,
    /// Usage ID `0x230`: "AC Full Screen View"
    ACFullScreenView = 0x230,
    /// Usage ID `0x231`: "AC Normal View"
    ACNormalView = 0x231,
    /// Usage ID `0x232`: "AC View Toggle"
    ACViewToggle = 0x232,
    /// Usage ID `0x233`: "AC Scroll Up"
    ACScrollUp = 0x233,
    /// Usage ID `0x234`: "AC Scroll Down"
    ACScrollDown = 0x234,
    /// Usage ID `0x235`: "AC Scroll"
    ACScroll = 0x235,
    /// Usage ID `0x236`: "AC Pan Left"
    ACPanLeft = 0x236,
    /// Usage ID `0x237`: "AC Pan Right"
    ACPanRight = 0x237,
    /// Usage ID `0x238`: "AC Pan"
    ACPan = 0x238,
    /// Usage ID `0x239`: "AC New Window"
    ACNewWindow = 0x239,
    /// Usage ID `0x23A`: "AC Tile Horizontally"
    ACTileHorizontally = 0x23A,
    /// Usage ID `0x23B`: "AC Tile Vertically"
    ACTileVertically = 0x23B,
    /// Usage ID `0x23C`: "AC Format"
    ACFormat = 0x23C,
    /// Usage ID `0x23D`: "AC Edit"
    ACEdit = 0x23D,
    /// Usage ID `0x23E`: "AC Bold"
    ACBold = 0x23E,
    /// Usage ID `0x23F`: "AC Italics"
    ACItalics = 0x23F,
    /// Usage ID `0x240`: "AC Underline"
    ACUnderline = 0x240,
    /// Usage ID `0x241`: "AC Strikethrough"
    ACStrikethrough = 0x241,
    /// Usage ID `0x242`: "AC Subscript"
    ACSubscript = 0x242,
    /// Usage ID `0x243`: "AC Superscript"
    ACSuperscript = 0x243,
    /// Usage ID `0x244`: "AC All Caps"
    ACAllCaps = 0x244,
    /// Usage ID `0x245`: "AC Rotate"
    ACRotate = 0x245,
    /// Usage ID `0x246`: "AC Resize"
    ACResize = 0x246,
    /// Usage ID `0x247`: "AC Flip Horizontal"
    ACFlipHorizontal = 0x247,
    /// Usage ID `0x248`: "AC Flip Vertical"
    ACFlipVertical = 0x248,
    /// Usage ID `0x249`: "AC Mirror Horizontal"
    ACMirrorHorizontal = 0x249,
    /// Usage ID `0x24A`: "AC Mirror Vertical"
    ACMirrorVertical = 0x24A,
    /// Usage ID `0x24B`: "AC Font Select"
    ACFontSelect = 0x24B,
    /// Usage ID `0x24C`: "AC Font Color"
    ACFontColor = 0x24C,
    /// Usage ID `0x24D`: "AC Font Size"
    ACFontSize = 0x24D,
    /// Usage ID `0x24E`: "AC Justify Left"
    ACJustifyLeft = 0x24E,
    /// Usage ID `0x24F`: "AC Justify Center H"
    ACJustifyCenterH = 0x24F,
    /// Usage ID `0x250`: "AC Justify Right"
    ACJustifyRight = 0x250,
    /// Usage ID `0x251`: "AC Justify Block H"
    ACJustifyBlockH = 0x251,
    /// Usage ID `0x252`: "AC Justify Top"
    ACJustifyTop = 0x252,
    /// Usage ID `0x253`: "AC Justify Center V"
    ACJustifyCenterV = 0x253,
    /// Usage ID `0x254`: "AC Justify Bottom"
    ACJustifyBottom = 0x254,
    /// Usage ID `0x255`: "AC Justify Block V"
    ACJustifyBlockV = 0x255,
    /// Usage ID `0x256`: "AC Indent Decrease"
    ACIndentDecrease = 0x256,
    /// Usage ID `0x257`: "AC Indent Increase"
    ACIndentIncrease = 0x257,
    /// Usage ID `0x258`: "AC Numbered List"
    ACNumberedList = 0x258,
    /// Usage ID `0x259`: "AC Restart Numbering"
    ACRestartNumbering = 0x259,
    /// Usage ID `0x25A`: "AC Bulleted List"
    ACBulletedList = 0x25A,
    /// Usage ID `0x25B`: "AC Promote"
    ACPromote = 0x25B,
    /// Usage ID `0x25C`: "AC Demote"
    ACDemote = 0x25C,
    /// Usage ID `0x25D`: "AC Yes"
    ACYes = 0x25D,
    /// Usage ID `0x25E`: "AC No"
    ACNo = 0x25E,
    /// Usage ID `0x25F`: "AC Cancel"
    ACCancel = 0x25F,
    /// Usage ID `0x260`: "AC Catalog"
    ACCatalog = 0x260,
    /// Usage ID `0x261`: "AC Buy/Checkout"
    ACBuyCheckout = 0x261,
    /// Usage ID `0x262`: "AC Add to Cart"
    ACAddtoCart = 0x262,
    /// Usage ID `0x263`: "AC Expand"
    ACExpand = 0x263,
    /// Usage ID `0x264`: "AC Expand All"
    ACExpandAll = 0x264,
    /// Usage ID `0x265`: "AC Collapse"
    ACCollapse = 0x265,
    /// Usage ID `0x266`: "AC Collapse All"
    ACCollapseAll = 0x266,
    /// Usage ID `0x267`: "AC Print Preview"
    ACPrintPreview = 0x267,
    /// Usage ID `0x268`: "AC Paste Special"
    ACPasteSpecial = 0x268,
    /// Usage ID `0x269`: "AC Insert Mode"
    ACInsertMode = 0x269,
    /// Usage ID `0x26A`: "AC Delete"
    ACDelete = 0x26A,
    /// Usage ID `0x26B`: "AC Lock"
    ACLock = 0x26B,
    /// Usage ID `0x26C`: "AC Unlock"
    ACUnlock = 0x26C,
    /// Usage ID `0x26D`: "AC Protect"
    ACProtect = 0x26D,
    /// Usage ID `0x26E`: "AC Unprotect"
    ACUnprotect = 0x26E,
    /// Usage ID `0x26F`: "AC Attach Comment"
    ACAttachComment = 0x26F,
    /// Usage ID `0x270`: "AC Delete Comment"
    ACDeleteComment = 0x270,
    /// Usage ID `0x271`: "AC View Comment"
    ACViewComment = 0x271,
    /// Usage ID `0x272`: "AC Select Word"
    ACSelectWord = 0x272,
    /// Usage ID `0x273`: "AC Select Sentence"
    ACSelectSentence = 0x273,
    /// Usage ID `0x274`: "AC Select Paragraph"
    ACSelectParagraph = 0x274,
    /// Usage ID `0x275`: "AC Select Column"
    ACSelectColumn = 0x275,
    /// Usage ID `0x276`: "AC Select Row"
    ACSelectRow = 0x276,
    /// Usage ID `0x277`: "AC Select Table"
    ACSelectTable = 0x277,
    /// Usage ID `0x278`: "AC Select Object"
    ACSelectObject = 0x278,
    /// Usage ID `0x279`: "AC Redo/Repeat"
    ACRedoRepeat = 0x279,
    /// Usage ID `0x27A`: "AC Sort"
    ACSort = 0x27A,
    /// Usage ID `0x27B`: "AC Sort Ascending"
    ACSortAscending = 0x27B,
    /// Usage ID `0x27C`: "AC Sort Descending"
    ACSortDescending = 0x27C,
    /// Usage ID `0x27D`: "AC Filter"
    ACFilter = 0x27D,
    /// Usage ID `0x27E`: "AC Set Clock"
    ACSetClock = 0x27E,
    /// Usage ID `0x27F`: "AC View Clock"
    ACViewClock = 0x27F,
    /// Usage ID `0x280`: "AC Select Time Zone"
    ACSelectTimeZone = 0x280,
    /// Usage ID `0x281`: "AC Edit Time Zones"
    ACEditTimeZones = 0x281,
    /// Usage ID `0x282`: "AC Set Alarm"
    ACSetAlarm = 0x282,
    /// Usage ID `0x283`: "AC Clear Alarm"
    ACClearAlarm = 0x283,
    /// Usage ID `0x284`: "AC Snooze Alarm"
    ACSnoozeAlarm = 0x284,
    /// Usage ID `0x285`: "AC Reset Alarm"
    ACResetAlarm = 0x285,
    /// Usage ID `0x286`: "AC Synchronize"
    ACSynchronize = 0x286,
    /// Usage ID `0x287`: "AC Send/Receive"
    ACSendReceive = 0x287,
    /// Usage ID `0x288`: "AC Send To"
    ACSendTo = 0x288,
    /// Usage ID `0x289`: "AC Reply"
    ACReply = 0x289,
    /// Usage ID `0x28A`: "AC Reply All"
    ACReplyAll = 0x28A,
    /// Usage ID `0x28B`: "AC Forward Msg"
    ACForwardMsg = 0x28B,
    /// Usage ID `0x28C`: "AC Send"
    ACSend = 0x28C,
    /// Usage ID `0x28D`: "AC Attach File"
    ACAttachFile = 0x28D,
    /// Usage ID `0x28E`: "AC Upload"
    ACUpload = 0x28E,
    /// Usage ID `0x28F`: "AC Download (Save Target As)"
    ACDownloadSaveTargetAs = 0x28F,
    /// Usage ID `0x290`: "AC Set Borders"
    ACSetBorders = 0x290,
    /// Usage ID `0x291`: "AC Insert Row"
    ACInsertRow = 0x291,
    /// Usage ID `0x292`: "AC Insert Column"
    ACInsertColumn = 0x292,
    /// Usage ID `0x293`: "AC Insert File"
    ACInsertFile = 0x293,
    /// Usage ID `0x294`: "AC Insert Picture"
    ACInsertPicture = 0x294,
    /// Usage ID `0x295`: "AC Insert Object"
    ACInsertObject = 0x295,
    /// Usage ID `0x296`: "AC Insert Symbol"
    ACInsertSymbol = 0x296,
    /// Usage ID `0x297`: "AC Save and Close"
    ACSaveandClose = 0x297,
    /// Usage ID `0x298`: "AC Rename"
    ACRename = 0x298,
    /// Usage ID `0x299`: "AC Merge"
    ACMerge = 0x299,
    /// Usage ID `0x29A`: "AC Split"
    ACSplit = 0x29A,
    /// Usage ID `0x29B`: "AC Disribute Horizontally"
    ACDisributeHorizontally = 0x29B,
    /// Usage ID `0x29C`: "AC Distribute Vertically"
    ACDistributeVertically = 0x29C,
    /// Usage ID `0x29D`: "AC Next Keyboard Layout Select"
    ACNextKeyboardLayoutSelect = 0x29D,
    /// Usage ID `0x29E`: "AC Navigation Guidance"
    ACNavigationGuidance = 0x29E,
    /// Usage ID `0x29F`: "AC Desktop Show All Windows"
    ACDesktopShowAllWindows = 0x29F,
    /// Usage ID `0x2A0`: "AC Soft Key Left"
    ACSoftKeyLeft = 0x2A0,
    /// Usage ID `0x2A1`: "AC Soft Key Right"
    ACSoftKeyRight = 0x2A1,
    /// Usage ID `0x2A2`: "AC Desktop Show All Applications"
    ACDesktopShowAllApplications = 0x2A2,
    /// Usage ID `0x2B0`: "AC Idle Keep Alive"
    ACIdleKeepAlive = 0x2B0,
    /// Usage ID `0x2C0`: "Extended Keyboard Attributes Collection"
    ExtendedKeyboardAttributesCollection = 0x2C0,
    /// Usage ID `0x2C1`: "Keyboard Form Factor"
    KeyboardFormFactor = 0x2C1,
    /// Usage ID `0x2C2`: "Keyboard Key Type"
    KeyboardKeyType = 0x2C2,
    /// Usage ID `0x2C3`: "Keyboard Physical Layout"
    KeyboardPhysicalLayout = 0x2C3,
    /// Usage ID `0x2C4`: "Vendor-Specific Keyboard Physical Layout"
    VendorSpecificKeyboardPhysicalLayout = 0x2C4,
    /// Usage ID `0x2C5`: "Keyboard IETF Language Tag Index"
    KeyboardIETFLanguageTagIndex = 0x2C5,
    /// Usage ID `0x2C6`: "Implemented Keyboard Input Assist Controls"
    ImplementedKeyboardInputAssistControls = 0x2C6,
    /// Usage ID `0x2C7`: "Keyboard Input Assist Previous"
    KeyboardInputAssistPrevious = 0x2C7,
    /// Usage ID `0x2C8`: "Keyboard Input Assist Next"
    KeyboardInputAssistNext = 0x2C8,
    /// Usage ID `0x2C9`: "Keyboard Input Assist Previous Group"
    KeyboardInputAssistPreviousGroup = 0x2C9,
    /// Usage ID `0x2CA`: "Keyboard Input Assist Next Group"
    KeyboardInputAssistNextGroup = 0x2CA,
    /// Usage ID `0x2CB`: "Keyboard Input Assist Accept"
    KeyboardInputAssistAccept = 0x2CB,
    /// Usage ID `0x2CC`: "Keyboard Input Assist Cancel"
    KeyboardInputAssistCancel = 0x2CC,
    /// Usage ID `0x2D0`: "Privacy Screen Toggle"
    PrivacyScreenToggle = 0x2D0,
    /// Usage ID `0x2D1`: "Privacy Screen Level Decrement"
    PrivacyScreenLevelDecrement = 0x2D1,
    /// Usage ID `0x2D2`: "Privacy Screen Level Increment"
    PrivacyScreenLevelIncrement = 0x2D2,
    /// Usage ID `0x2D3`: "Privacy Screen Level Minimum"
    PrivacyScreenLevelMinimum = 0x2D3,
    /// Usage ID `0x2D4`: "Privacy Screen Level Maximum"
    PrivacyScreenLevelMaximum = 0x2D4,
    /// Usage ID `0x500`: "Contact Edited"
    ContactEdited = 0x500,
    /// Usage ID `0x501`: "Contact Added"
    ContactAdded = 0x501,
    /// Usage ID `0x502`: "Contact Record Active"
    ContactRecordActive = 0x502,
    /// Usage ID `0x503`: "Contact Index"
    ContactIndex = 0x503,
    /// Usage ID `0x504`: "Contact Nickname"
    ContactNickname = 0x504,
    /// Usage ID `0x505`: "Contact First Name"
    ContactFirstName = 0x505,
    /// Usage ID `0x506`: "Contact Last Name"
    ContactLastName = 0x506,
    /// Usage ID `0x507`: "Contact Full Name"
    ContactFullName = 0x507,
    /// Usage ID `0x508`: "Contact Phone Number Personal"
    ContactPhoneNumberPersonal = 0x508,
    /// Usage ID `0x509`: "Contact Phone Number Business"
    ContactPhoneNumberBusiness = 0x509,
    /// Usage ID `0x50A`: "Contact Phone Number Mobile"
    ContactPhoneNumberMobile = 0x50A,
    /// Usage ID `0x50B`: "Contact Phone Number Pager"
    ContactPhoneNumberPager = 0x50B,
    /// Usage ID `0x50C`: "Contact Phone Number Fax"
    ContactPhoneNumberFax = 0x50C,
    /// Usage ID `0x50D`: "Contact Phone Number Other"
    ContactPhoneNumberOther = 0x50D,
    /// Usage ID `0x50E`: "Contact Email Personal"
    ContactEmailPersonal = 0x50E,
    /// Usage ID `0x50F`: "Contact Email Business"
    ContactEmailBusiness = 0x50F,
    /// Usage ID `0x510`: "Contact Email Other"
    ContactEmailOther = 0x510,
    /// Usage ID `0x511`: "Contact Email Main"
    ContactEmailMain = 0x511,
    /// Usage ID `0x512`: "Contact Speed Dial Number"
    ContactSpeedDialNumber = 0x512,
    /// Usage ID `0x513`: "Contact Status Flag"
    ContactStatusFlag = 0x513,
    /// Usage ID `0x514`: "Contact Misc."
    ContactMisc = 0x514,
    /// Usage ID `0x515`: "Keyboard Brightness Next"
    KeyboardBrightnessNext = 0x515,
    /// Usage ID `0x516`: "Keyboard Brightness Previous"
    KeyboardBrightnessPrevious = 0x516,
    /// Usage ID `0x517`: "Keyboard Backlight Level Suggestion"
    KeyboardBacklightLevelSuggestion = 0x517,
}

impl Consumer {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Consumer::ConsumerControl => "Consumer Control",
            Consumer::NumericKeyPad => "Numeric Key Pad",
            Consumer::ProgrammableButtons => "Programmable Buttons",
            Consumer::Microphone => "Microphone",
            Consumer::Headphone => "Headphone",
            Consumer::GraphicEqualizer => "Graphic Equalizer",
            Consumer::KeyboardBacklight => "Keyboard Backlight",
            Consumer::Plus10 => "+10",
            Consumer::Plus100 => "+100",
            Consumer::AMPM => "AM/PM",
            Consumer::Power => "Power",
            Consumer::Reset => "Reset",
            Consumer::Sleep => "Sleep",
            Consumer::SleepAfter => "Sleep After",
            Consumer::SleepMode => "Sleep Mode",
            Consumer::Illumination => "Illumination",
            Consumer::FunctionButtons => "Function Buttons",
            Consumer::Menu => "Menu",
            Consumer::MenuPick => "Menu Pick",
            Consumer::MenuUp => "Menu Up",
            Consumer::MenuDown => "Menu Down",
            Consumer::MenuLeft => "Menu Left",
            Consumer::MenuRight => "Menu Right",
            Consumer::MenuEscape => "Menu Escape",
            Consumer::MenuValueIncrease => "Menu Value Increase",
            Consumer::MenuValueDecrease => "Menu Value Decrease",
            Consumer::DataOnScreen => "Data On Screen",
            Consumer::ClosedCaption => "Closed Caption",
            Consumer::ClosedCaptionSelect => "Closed Caption Select",
            Consumer::VCRTV => "VCR/TV",
            Consumer::BroadcastMode => "Broadcast Mode",
            Consumer::Snapshot => "Snapshot",
            Consumer::Still => "Still",
            Consumer::PictureinPictureToggle => "Picture-in-Picture Toggle",
            Consumer::PictureinPictureSwap => "Picture-in-Picture Swap",
            Consumer::RedMenuButton => "Red Menu Button",
            Consumer::GreenMenuButton => "Green Menu Button",
            Consumer::BlueMenuButton => "Blue Menu Button",
            Consumer::YellowMenuButton => "Yellow Menu Button",
            Consumer::Aspect => "Aspect",
            Consumer::ThreeDModeSelect => "3D Mode Select",
            Consumer::DisplayBrightnessIncrement => "Display Brightness Increment",
            Consumer::DisplayBrightnessDecrement => "Display Brightness Decrement",
            Consumer::DisplayBrightness => "Display Brightness",
            Consumer::DisplayBacklightToggle => "Display Backlight Toggle",
            Consumer::DisplaySetBrightnesstoMinimum => "Display Set Brightness to Minimum",
            Consumer::DisplaySetBrightnesstoMaximum => "Display Set Brightness to Maximum",
            Consumer::DisplaySetAutoBrightness => "Display Set Auto Brightness",
            Consumer::CameraAccessEnabled => "Camera Access Enabled",
            Consumer::CameraAccessDisabled => "Camera Access Disabled",
            Consumer::CameraAccessToggle => "Camera Access Toggle",
            Consumer::KeyboardBrightnessIncrement => "Keyboard Brightness Increment",
            Consumer::KeyboardBrightnessDecrement => "Keyboard Brightness Decrement",
            Consumer::KeyboardBacklightSetLevel => "Keyboard Backlight Set Level",
            Consumer::KeyboardBacklightOOC => "Keyboard Backlight OOC",
            Consumer::KeyboardBacklightSetMinimum => "Keyboard Backlight Set Minimum",
            Consumer::KeyboardBacklightSetMaximum => "Keyboard Backlight Set Maximum",
            Consumer::KeyboardBacklightAuto => "Keyboard Backlight Auto",
            Consumer::Selection => "Selection",
            Consumer::AssignSelection => "Assign Selection",
            Consumer::ModeStep => "Mode Step",
            Consumer::RecallLast => "Recall Last",
            Consumer::EnterChannel => "Enter Channel",
            Consumer::OrderMovie => "Order Movie",
            Consumer::Channel => "Channel",
            Consumer::MediaSelection => "Media Selection",
            Consumer::MediaSelectComputer => "Media Select Computer",
            Consumer::MediaSelectTV => "Media Select TV",
            Consumer::MediaSelectWWW => "Media Select WWW",
            Consumer::MediaSelectDVD => "Media Select DVD",
            Consumer::MediaSelectTelephone => "Media Select Telephone",
            Consumer::MediaSelectProgramGuide => "Media Select Program Guide",
            Consumer::MediaSelectVideoPhone => "Media Select Video Phone",
            Consumer::MediaSelectGames => "Media Select Games",
            Consumer::MediaSelectMessages => "Media Select Messages",
            Consumer::MediaSelectCD => "Media Select CD",
            Consumer::MediaSelectVCR => "Media Select VCR",
            Consumer::MediaSelectTuner => "Media Select Tuner",
            Consumer::Quit => "Quit",
            Consumer::Help => "Help",
            Consumer::MediaSelectTape => "Media Select Tape",
            Consumer::MediaSelectCable => "Media Select Cable",
            Consumer::MediaSelectSatellite => "Media Select Satellite",
            Consumer::MediaSelectSecurity => "Media Select Security",
            Consumer::MediaSelectHome => "Media Select Home",
            Consumer::MediaSelectCall => "Media Select Call",
            Consumer::ChannelIncrement => "Channel Increment",
            Consumer::ChannelDecrement => "Channel Decrement",
            Consumer::MediaSelectSAP => "Media Select SAP",
            Consumer::VCRPlus => "VCR Plus",
            Consumer::Once => "Once",
            Consumer::Daily => "Daily",
            Consumer::Weekly => "Weekly",
            Consumer::Monthly => "Monthly",
            Consumer::Play => "Play",
            Consumer::Pause => "Pause",
            Consumer::Record => "Record",
            Consumer::FastForward => "Fast Forward",
            Consumer::Rewind => "Rewind",
            Consumer::ScanNextTrack => "Scan Next Track",
            Consumer::ScanPreviousTrack => "Scan Previous Track",
            Consumer::Stop => "Stop",
            Consumer::Eject => "Eject",
            Consumer::RandomPlay => "Random Play",
            Consumer::SelectDisc => "Select Disc",
            Consumer::EnterDisc => "Enter Disc",
            Consumer::Repeat => "Repeat",
            Consumer::Tracking => "Tracking",
            Consumer::TrackNormal => "Track Normal",
            Consumer::SlowTracking => "Slow Tracking",
            Consumer::FrameForward => "Frame Forward",
            Consumer::FrameBack => "Frame Back",
            Consumer::Mark => "Mark",
            Consumer::ClearMark => "Clear Mark",
            Consumer::RepeatFromMark => "Repeat From Mark",
            Consumer::ReturnToMark => "Return To Mark",
            Consumer::SearchMarkForward => "Search Mark Forward",
            Consumer::SearchMarkBackwards => "Search Mark Backwards",
            Consumer::CounterReset => "Counter Reset",
            Consumer::ShowCounter => "Show Counter",
            Consumer::TrackingIncrement => "Tracking Increment",
            Consumer::TrackingDecrement => "Tracking Decrement",
            Consumer::StopEject => "Stop/Eject",
            Consumer::PlayPause => "Play/Pause",
            Consumer::PlaySkip => "Play/Skip",
            Consumer::VoiceCommand => "Voice Command",
            Consumer::InvokeCaptureInterface => "Invoke Capture Interface",
            Consumer::StartorStopGameRecording => "Start or Stop Game Recording",
            Consumer::HistoricalGameCapture => "Historical Game Capture",
            Consumer::CaptureGameScreenshot => "Capture Game Screenshot",
            Consumer::ShoworHideRecordingIndicator => "Show or Hide Recording Indicator",
            Consumer::StartorStopMicrophoneCapture => "Start or Stop Microphone Capture",
            Consumer::StartorStopCameraCapture => "Start or Stop Camera Capture",
            Consumer::StartorStopGameBroadcast => "Start or Stop Game Broadcast",
            Consumer::StartorStopVoiceDictationSession => "Start or Stop Voice Dictation Session",
            Consumer::InvokeDismissEmojiPicker => "Invoke/Dismiss Emoji Picker",
            Consumer::Volume => "Volume",
            Consumer::Balance => "Balance",
            Consumer::Mute => "Mute",
            Consumer::Bass => "Bass",
            Consumer::Treble => "Treble",
            Consumer::BassBoost => "Bass Boost",
            Consumer::SurroundMode => "Surround Mode",
            Consumer::Loudness => "Loudness",
            Consumer::MPX => "MPX",
            Consumer::VolumeIncrement => "Volume Increment",
            Consumer::VolumeDecrement => "Volume Decrement",
            Consumer::SpeedSelect => "Speed Select",
            Consumer::PlaybackSpeed => "Playback Speed",
            Consumer::StandardPlay => "Standard Play",
            Consumer::LongPlay => "Long Play",
            Consumer::ExtendedPlay => "Extended Play",
            Consumer::Slow => "Slow",
            Consumer::FanEnable => "Fan Enable",
            Consumer::FanSpeed => "Fan Speed",
            Consumer::LightEnable => "Light Enable",
            Consumer::LightIlluminationLevel => "Light Illumination Level",
            Consumer::ClimateControlEnable => "Climate Control Enable",
            Consumer::RoomTemperature => "Room Temperature",
            Consumer::SecurityEnable => "Security Enable",
            Consumer::FireAlarm => "Fire Alarm",
            Consumer::PoliceAlarm => "Police Alarm",
            Consumer::Proximity => "Proximity",
            Consumer::Motion => "Motion",
            Consumer::DuressAlarm => "Duress Alarm",
            Consumer::HoldupAlarm => "Holdup Alarm",
            Consumer::MedicalAlarm => "Medical Alarm",
            Consumer::BalanceRight => "Balance Right",
            Consumer::BalanceLeft => "Balance Left",
            Consumer::BassIncrement => "Bass Increment",
            Consumer::BassDecrement => "Bass Decrement",
            Consumer::TrebleIncrement => "Treble Increment",
            Consumer::TrebleDecrement => "Treble Decrement",
            Consumer::SpeakerSystem => "Speaker System",
            Consumer::ChannelLeft => "Channel Left",
            Consumer::ChannelRight => "Channel Right",
            Consumer::ChannelCenter => "Channel Center",
            Consumer::ChannelFront => "Channel Front",
            Consumer::ChannelCenterFront => "Channel Center Front",
            Consumer::ChannelSide => "Channel Side",
            Consumer::ChannelSurround => "Channel Surround",
            Consumer::ChannelLowFrequencyEnhancement => "Channel Low Frequency Enhancement",
            Consumer::ChannelTop => "Channel Top",
            Consumer::ChannelUnknown => "Channel Unknown",
            Consumer::Subchannel => "Sub-channel",
            Consumer::SubchannelIncrement => "Sub-channel Increment",
            Consumer::SubchannelDecrement => "Sub-channel Decrement",
            Consumer::AlternateAudioIncrement => "Alternate Audio Increment",
            Consumer::AlternateAudioDecrement => "Alternate Audio Decrement",
            Consumer::ApplicationLaunchButtons => "Application Launch Buttons",
            Consumer::ALLaunchButtonConfigurationTool => "AL Launch Button Configuration Tool",
            Consumer::ALProgrammableButtonConfiguration => "AL Programmable Button Configuration",
            Consumer::ALConsumerControlConfiguration => "AL Consumer Control Configuration",
            Consumer::ALWordProcessor => "AL Word Processor",
            Consumer::ALTextEditor => "AL Text Editor",
            Consumer::ALSpreadsheet => "AL Spreadsheet",
            Consumer::ALGraphicsEditor => "AL Graphics Editor",
            Consumer::ALPresentationApp => "AL Presentation App",
            Consumer::ALDatabaseApp => "AL Database App",
            Consumer::ALEmailReader => "AL Email Reader",
            Consumer::ALNewsreader => "AL Newsreader",
            Consumer::ALVoicemail => "AL Voicemail",
            Consumer::ALContactsAddressBook => "AL Contacts/Address Book",
            Consumer::ALCalendarSchedule => "AL Calendar/Schedule",
            Consumer::ALTaskProjectManager => "AL Task/Project Manager",
            Consumer::ALLogJournalTimecard => "AL Log/Journal/Timecard",
            Consumer::ALCheckbookFinance => "AL Checkbook/Finance",
            Consumer::ALCalculator => "AL Calculator",
            Consumer::ALAVCapturePlayback => "AL A/V Capture/Playback",
            Consumer::ALLocalMachineBrowser => "AL Local Machine Browser",
            Consumer::ALLANWANBrowser => "AL LAN/WAN Browser",
            Consumer::ALInternetBrowser => "AL Internet Browser",
            Consumer::ALRemoteNetworkingISPConnect => "AL Remote Networking/ISP Connect",
            Consumer::ALNetworkConference => "AL Network Conference",
            Consumer::ALNetworkChat => "AL Network Chat",
            Consumer::ALTelephonyDialer => "AL Telephony/Dialer",
            Consumer::ALLogon => "AL Logon",
            Consumer::ALLogoff => "AL Logoff",
            Consumer::ALLogonLogoff => "AL Logon/Logoff",
            Consumer::ALTerminalLockScreensaver => "AL Terminal Lock/Screensaver",
            Consumer::ALControlPanel => "AL Control Panel",
            Consumer::ALCommandLineProcessorRun => "AL Command Line Processor/Run",
            Consumer::ALProcessTaskManager => "AL Process/Task Manager",
            Consumer::ALSelectTaskApplication => "AL Select Task/Application",
            Consumer::ALNextTaskApplication => "AL Next Task/Application",
            Consumer::ALPreviousTaskApplication => "AL Previous Task/Application",
            Consumer::ALPreemptiveHaltTaskApplication => "AL Preemptive Halt Task/Application",
            Consumer::ALIntegratedHelpCenter => "AL Integrated Help Center",
            Consumer::ALDocuments => "AL Documents",
            Consumer::ALThesaurus => "AL Thesaurus",
            Consumer::ALDictionary => "AL Dictionary",
            Consumer::ALDesktop => "AL Desktop",
            Consumer::ALSpellCheck => "AL Spell Check",
            Consumer::ALGrammarCheck => "AL Grammar Check",
            Consumer::ALWirelessStatus => "AL Wireless Status",
            Consumer::ALKeyboardLayout => "AL Keyboard Layout",
            Consumer::ALVirusProtection => "AL Virus Protection",
            Consumer::ALEncryption => "AL Encryption",
            Consumer::ALScreenSaver => "AL Screen Saver",
            Consumer::ALAlarms => "AL Alarms",
            Consumer::ALClock => "AL Clock",
            Consumer::ALFileBrowser => "AL File Browser",
            Consumer::ALPowerStatus => "AL Power Status",
            Consumer::ALImageBrowser => "AL Image Browser",
            Consumer::ALAudioBrowser => "AL Audio Browser",
            Consumer::ALMovieBrowser => "AL Movie Browser",
            Consumer::ALDigitalRightsManager => "AL Digital Rights Manager",
            Consumer::ALDigitalWallet => "AL Digital Wallet",
            Consumer::ALInstantMessaging => "AL Instant Messaging",
            Consumer::ALOEMFeaturesTipsTutorialBrowser => "AL OEM Features/ Tips/Tutorial Browser",
            Consumer::ALOEMHelp => "AL OEM Help",
            Consumer::ALOnlineCommunity => "AL Online Community",
            Consumer::ALEntertainmentContentBrowser => "AL Entertainment Content Browser",
            Consumer::ALOnlineShoppingBrowser => "AL Online Shopping Browser",
            Consumer::ALSmartCardInformationHelp => "AL SmartCard Information/Help",
            Consumer::ALMarketMonitorFinanceBrowser => "AL Market Monitor/Finance Browser",
            Consumer::ALCustomizedCorporateNewsBrowser => "AL Customized Corporate News Browser",
            Consumer::ALOnlineActivityBrowser => "AL Online Activity Browser",
            Consumer::ALResearchSearchBrowser => "AL Research/Search Browser",
            Consumer::ALAudioPlayer => "AL Audio Player",
            Consumer::ALMessageStatus => "AL Message Status",
            Consumer::ALContactSync => "AL Contact Sync",
            Consumer::ALNavigation => "AL Navigation",
            Consumer::ALContextawareDesktopAssistant => "AL Context-aware Desktop Assistant",
            Consumer::GenericGUIApplicationControls => "Generic GUI Application Controls",
            Consumer::ACNew => "AC New",
            Consumer::ACOpen => "AC Open",
            Consumer::ACClose => "AC Close",
            Consumer::ACExit => "AC Exit",
            Consumer::ACMaximize => "AC Maximize",
            Consumer::ACMinimize => "AC Minimize",
            Consumer::ACSave => "AC Save",
            Consumer::ACPrint => "AC Print",
            Consumer::ACProperties => "AC Properties",
            Consumer::ACUndo => "AC Undo",
            Consumer::ACCopy => "AC Copy",
            Consumer::ACCut => "AC Cut",
            Consumer::ACPaste => "AC Paste",
            Consumer::ACSelectAll => "AC Select All",
            Consumer::ACFind => "AC Find",
            Consumer::ACFindandReplace => "AC Find and Replace",
            Consumer::ACSearch => "AC Search",
            Consumer::ACGoTo => "AC Go To",
            Consumer::ACHome => "AC Home",
            Consumer::ACBack => "AC Back",
            Consumer::ACForward => "AC Forward",
            Consumer::ACStop => "AC Stop",
            Consumer::ACRefresh => "AC Refresh",
            Consumer::ACPreviousLink => "AC Previous Link",
            Consumer::ACNextLink => "AC Next Link",
            Consumer::ACBookmarks => "AC Bookmarks",
            Consumer::ACHistory => "AC History",
            Consumer::ACSubscriptions => "AC Subscriptions",
            Consumer::ACZoomIn => "AC Zoom In",
            Consumer::ACZoomOut => "AC Zoom Out",
            Consumer::ACZoom => "AC Zoom",
            Consumer::ACFullScreenView => "AC Full Screen View",
            Consumer::ACNormalView => "AC Normal View",
            Consumer::ACViewToggle => "AC View Toggle",
            Consumer::ACScrollUp => "AC Scroll Up",
            Consumer::ACScrollDown => "AC Scroll Down",
            Consumer::ACScroll => "AC Scroll",
            Consumer::ACPanLeft => "AC Pan Left",
            Consumer::ACPanRight => "AC Pan Right",
            Consumer::ACPan => "AC Pan",
            Consumer::ACNewWindow => "AC New Window",
            Consumer::ACTileHorizontally => "AC Tile Horizontally",
            Consumer::ACTileVertically => "AC Tile Vertically",
            Consumer::ACFormat => "AC Format",
            Consumer::ACEdit => "AC Edit",
            Consumer::ACBold => "AC Bold",
            Consumer::ACItalics => "AC Italics",
            Consumer::ACUnderline => "AC Underline",
            Consumer::ACStrikethrough => "AC Strikethrough",
            Consumer::ACSubscript => "AC Subscript",
            Consumer::ACSuperscript => "AC Superscript",
            Consumer::ACAllCaps => "AC All Caps",
            Consumer::ACRotate => "AC Rotate",
            Consumer::ACResize => "AC Resize",
            Consumer::ACFlipHorizontal => "AC Flip Horizontal",
            Consumer::ACFlipVertical => "AC Flip Vertical",
            Consumer::ACMirrorHorizontal => "AC Mirror Horizontal",
            Consumer::ACMirrorVertical => "AC Mirror Vertical",
            Consumer::ACFontSelect => "AC Font Select",
            Consumer::ACFontColor => "AC Font Color",
            Consumer::ACFontSize => "AC Font Size",
            Consumer::ACJustifyLeft => "AC Justify Left",
            Consumer::ACJustifyCenterH => "AC Justify Center H",
            Consumer::ACJustifyRight => "AC Justify Right",
            Consumer::ACJustifyBlockH => "AC Justify Block H",
            Consumer::ACJustifyTop => "AC Justify Top",
            Consumer::ACJustifyCenterV => "AC Justify Center V",
            Consumer::ACJustifyBottom => "AC Justify Bottom",
            Consumer::ACJustifyBlockV => "AC Justify Block V",
            Consumer::ACIndentDecrease => "AC Indent Decrease",
            Consumer::ACIndentIncrease => "AC Indent Increase",
            Consumer::ACNumberedList => "AC Numbered List",
            Consumer::ACRestartNumbering => "AC Restart Numbering",
            Consumer::ACBulletedList => "AC Bulleted List",
            Consumer::ACPromote => "AC Promote",
            Consumer::ACDemote => "AC Demote",
            Consumer::ACYes => "AC Yes",
            Consumer::ACNo => "AC No",
            Consumer::ACCancel => "AC Cancel",
            Consumer::ACCatalog => "AC Catalog",
            Consumer::ACBuyCheckout => "AC Buy/Checkout",
            Consumer::ACAddtoCart => "AC Add to Cart",
            Consumer::ACExpand => "AC Expand",
            Consumer::ACExpandAll => "AC Expand All",
            Consumer::ACCollapse => "AC Collapse",
            Consumer::ACCollapseAll => "AC Collapse All",
            Consumer::ACPrintPreview => "AC Print Preview",
            Consumer::ACPasteSpecial => "AC Paste Special",
            Consumer::ACInsertMode => "AC Insert Mode",
            Consumer::ACDelete => "AC Delete",
            Consumer::ACLock => "AC Lock",
            Consumer::ACUnlock => "AC Unlock",
            Consumer::ACProtect => "AC Protect",
            Consumer::ACUnprotect => "AC Unprotect",
            Consumer::ACAttachComment => "AC Attach Comment",
            Consumer::ACDeleteComment => "AC Delete Comment",
            Consumer::ACViewComment => "AC View Comment",
            Consumer::ACSelectWord => "AC Select Word",
            Consumer::ACSelectSentence => "AC Select Sentence",
            Consumer::ACSelectParagraph => "AC Select Paragraph",
            Consumer::ACSelectColumn => "AC Select Column",
            Consumer::ACSelectRow => "AC Select Row",
            Consumer::ACSelectTable => "AC Select Table",
            Consumer::ACSelectObject => "AC Select Object",
            Consumer::ACRedoRepeat => "AC Redo/Repeat",
            Consumer::ACSort => "AC Sort",
            Consumer::ACSortAscending => "AC Sort Ascending",
            Consumer::ACSortDescending => "AC Sort Descending",
            Consumer::ACFilter => "AC Filter",
            Consumer::ACSetClock => "AC Set Clock",
            Consumer::ACViewClock => "AC View Clock",
            Consumer::ACSelectTimeZone => "AC Select Time Zone",
            Consumer::ACEditTimeZones => "AC Edit Time Zones",
            Consumer::ACSetAlarm => "AC Set Alarm",
            Consumer::ACClearAlarm => "AC Clear Alarm",
            Consumer::ACSnoozeAlarm => "AC Snooze Alarm",
            Consumer::ACResetAlarm => "AC Reset Alarm",
            Consumer::ACSynchronize => "AC Synchronize",
            Consumer::ACSendReceive => "AC Send/Receive",
            Consumer::ACSendTo => "AC Send To",
            Consumer::ACReply => "AC Reply",
            Consumer::ACReplyAll => "AC Reply All",
            Consumer::ACForwardMsg => "AC Forward Msg",
            Consumer::ACSend => "AC Send",
            Consumer::ACAttachFile => "AC Attach File",
            Consumer::ACUpload => "AC Upload",
            Consumer::ACDownloadSaveTargetAs => "AC Download (Save Target As)",
            Consumer::ACSetBorders => "AC Set Borders",
            Consumer::ACInsertRow => "AC Insert Row",
            Consumer::ACInsertColumn => "AC Insert Column",
            Consumer::ACInsertFile => "AC Insert File",
            Consumer::ACInsertPicture => "AC Insert Picture",
            Consumer::ACInsertObject => "AC Insert Object",
            Consumer::ACInsertSymbol => "AC Insert Symbol",
            Consumer::ACSaveandClose => "AC Save and Close",
            Consumer::ACRename => "AC Rename",
            Consumer::ACMerge => "AC Merge",
            Consumer::ACSplit => "AC Split",
            Consumer::ACDisributeHorizontally => "AC Disribute Horizontally",
            Consumer::ACDistributeVertically => "AC Distribute Vertically",
            Consumer::ACNextKeyboardLayoutSelect => "AC Next Keyboard Layout Select",
            Consumer::ACNavigationGuidance => "AC Navigation Guidance",
            Consumer::ACDesktopShowAllWindows => "AC Desktop Show All Windows",
            Consumer::ACSoftKeyLeft => "AC Soft Key Left",
            Consumer::ACSoftKeyRight => "AC Soft Key Right",
            Consumer::ACDesktopShowAllApplications => "AC Desktop Show All Applications",
            Consumer::ACIdleKeepAlive => "AC Idle Keep Alive",
            Consumer::ExtendedKeyboardAttributesCollection => {
                "Extended Keyboard Attributes Collection"
            }
            Consumer::KeyboardFormFactor => "Keyboard Form Factor",
            Consumer::KeyboardKeyType => "Keyboard Key Type",
            Consumer::KeyboardPhysicalLayout => "Keyboard Physical Layout",
            Consumer::VendorSpecificKeyboardPhysicalLayout => {
                "Vendor-Specific Keyboard Physical Layout"
            }
            Consumer::KeyboardIETFLanguageTagIndex => "Keyboard IETF Language Tag Index",
            Consumer::ImplementedKeyboardInputAssistControls => {
                "Implemented Keyboard Input Assist Controls"
            }
            Consumer::KeyboardInputAssistPrevious => "Keyboard Input Assist Previous",
            Consumer::KeyboardInputAssistNext => "Keyboard Input Assist Next",
            Consumer::KeyboardInputAssistPreviousGroup => "Keyboard Input Assist Previous Group",
            Consumer::KeyboardInputAssistNextGroup => "Keyboard Input Assist Next Group",
            Consumer::KeyboardInputAssistAccept => "Keyboard Input Assist Accept",
            Consumer::KeyboardInputAssistCancel => "Keyboard Input Assist Cancel",
            Consumer::PrivacyScreenToggle => "Privacy Screen Toggle",
            Consumer::PrivacyScreenLevelDecrement => "Privacy Screen Level Decrement",
            Consumer::PrivacyScreenLevelIncrement => "Privacy Screen Level Increment",
            Consumer::PrivacyScreenLevelMinimum => "Privacy Screen Level Minimum",
            Consumer::PrivacyScreenLevelMaximum => "Privacy Screen Level Maximum",
            Consumer::ContactEdited => "Contact Edited",
            Consumer::ContactAdded => "Contact Added",
            Consumer::ContactRecordActive => "Contact Record Active",
            Consumer::ContactIndex => "Contact Index",
            Consumer::ContactNickname => "Contact Nickname",
            Consumer::ContactFirstName => "Contact First Name",
            Consumer::ContactLastName => "Contact Last Name",
            Consumer::ContactFullName => "Contact Full Name",
            Consumer::ContactPhoneNumberPersonal => "Contact Phone Number Personal",
            Consumer::ContactPhoneNumberBusiness => "Contact Phone Number Business",
            Consumer::ContactPhoneNumberMobile => "Contact Phone Number Mobile",
            Consumer::ContactPhoneNumberPager => "Contact Phone Number Pager",
            Consumer::ContactPhoneNumberFax => "Contact Phone Number Fax",
            Consumer::ContactPhoneNumberOther => "Contact Phone Number Other",
            Consumer::ContactEmailPersonal => "Contact Email Personal",
            Consumer::ContactEmailBusiness => "Contact Email Business",
            Consumer::ContactEmailOther => "Contact Email Other",
            Consumer::ContactEmailMain => "Contact Email Main",
            Consumer::ContactSpeedDialNumber => "Contact Speed Dial Number",
            Consumer::ContactStatusFlag => "Contact Status Flag",
            Consumer::ContactMisc => "Contact Misc.",
            Consumer::KeyboardBrightnessNext => "Keyboard Brightness Next",
            Consumer::KeyboardBrightnessPrevious => "Keyboard Brightness Previous",
            Consumer::KeyboardBacklightLevelSuggestion => "Keyboard Backlight Level Suggestion",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Consumer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Consumer {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Consumer(self)](Usage::Consumer)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Consumer {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xC` for [Consumer]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Consumer]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Consumer> for u16 {
    fn from(consumer: &Consumer) -> u16 {
        *consumer as u16
    }
}

impl From<Consumer> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Consumer::usage_page_value()].
    fn from(consumer: Consumer) -> u16 {
        u16::from(&consumer)
    }
}

impl From<&Consumer> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Consumer::usage_value()].
    fn from(consumer: &Consumer) -> u32 {
        let up = UsagePage::from(consumer);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(consumer) as u32;
        up | id
    }
}

impl From<&Consumer> for UsagePage {
    /// Always returns [UsagePage::Consumer] and is
    /// identical to [Consumer::usage_page()].
    fn from(_: &Consumer) -> UsagePage {
        UsagePage::Consumer
    }
}

impl From<Consumer> for UsagePage {
    /// Always returns [UsagePage::Consumer] and is
    /// identical to [Consumer::usage_page()].
    fn from(_: Consumer) -> UsagePage {
        UsagePage::Consumer
    }
}

impl From<&Consumer> for Usage {
    fn from(consumer: &Consumer) -> Usage {
        Usage::try_from(u32::from(consumer)).unwrap()
    }
}

impl From<Consumer> for Usage {
    fn from(consumer: Consumer) -> Usage {
        Usage::from(&consumer)
    }
}

impl TryFrom<u16> for Consumer {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Consumer> {
        match usage_id {
            1 => Ok(Consumer::ConsumerControl),
            2 => Ok(Consumer::NumericKeyPad),
            3 => Ok(Consumer::ProgrammableButtons),
            4 => Ok(Consumer::Microphone),
            5 => Ok(Consumer::Headphone),
            6 => Ok(Consumer::GraphicEqualizer),
            7 => Ok(Consumer::KeyboardBacklight),
            32 => Ok(Consumer::Plus10),
            33 => Ok(Consumer::Plus100),
            34 => Ok(Consumer::AMPM),
            48 => Ok(Consumer::Power),
            49 => Ok(Consumer::Reset),
            50 => Ok(Consumer::Sleep),
            51 => Ok(Consumer::SleepAfter),
            52 => Ok(Consumer::SleepMode),
            53 => Ok(Consumer::Illumination),
            54 => Ok(Consumer::FunctionButtons),
            64 => Ok(Consumer::Menu),
            65 => Ok(Consumer::MenuPick),
            66 => Ok(Consumer::MenuUp),
            67 => Ok(Consumer::MenuDown),
            68 => Ok(Consumer::MenuLeft),
            69 => Ok(Consumer::MenuRight),
            70 => Ok(Consumer::MenuEscape),
            71 => Ok(Consumer::MenuValueIncrease),
            72 => Ok(Consumer::MenuValueDecrease),
            96 => Ok(Consumer::DataOnScreen),
            97 => Ok(Consumer::ClosedCaption),
            98 => Ok(Consumer::ClosedCaptionSelect),
            99 => Ok(Consumer::VCRTV),
            100 => Ok(Consumer::BroadcastMode),
            101 => Ok(Consumer::Snapshot),
            102 => Ok(Consumer::Still),
            103 => Ok(Consumer::PictureinPictureToggle),
            104 => Ok(Consumer::PictureinPictureSwap),
            105 => Ok(Consumer::RedMenuButton),
            106 => Ok(Consumer::GreenMenuButton),
            107 => Ok(Consumer::BlueMenuButton),
            108 => Ok(Consumer::YellowMenuButton),
            109 => Ok(Consumer::Aspect),
            110 => Ok(Consumer::ThreeDModeSelect),
            111 => Ok(Consumer::DisplayBrightnessIncrement),
            112 => Ok(Consumer::DisplayBrightnessDecrement),
            113 => Ok(Consumer::DisplayBrightness),
            114 => Ok(Consumer::DisplayBacklightToggle),
            115 => Ok(Consumer::DisplaySetBrightnesstoMinimum),
            116 => Ok(Consumer::DisplaySetBrightnesstoMaximum),
            117 => Ok(Consumer::DisplaySetAutoBrightness),
            118 => Ok(Consumer::CameraAccessEnabled),
            119 => Ok(Consumer::CameraAccessDisabled),
            120 => Ok(Consumer::CameraAccessToggle),
            121 => Ok(Consumer::KeyboardBrightnessIncrement),
            122 => Ok(Consumer::KeyboardBrightnessDecrement),
            123 => Ok(Consumer::KeyboardBacklightSetLevel),
            124 => Ok(Consumer::KeyboardBacklightOOC),
            125 => Ok(Consumer::KeyboardBacklightSetMinimum),
            126 => Ok(Consumer::KeyboardBacklightSetMaximum),
            127 => Ok(Consumer::KeyboardBacklightAuto),
            128 => Ok(Consumer::Selection),
            129 => Ok(Consumer::AssignSelection),
            130 => Ok(Consumer::ModeStep),
            131 => Ok(Consumer::RecallLast),
            132 => Ok(Consumer::EnterChannel),
            133 => Ok(Consumer::OrderMovie),
            134 => Ok(Consumer::Channel),
            135 => Ok(Consumer::MediaSelection),
            136 => Ok(Consumer::MediaSelectComputer),
            137 => Ok(Consumer::MediaSelectTV),
            138 => Ok(Consumer::MediaSelectWWW),
            139 => Ok(Consumer::MediaSelectDVD),
            140 => Ok(Consumer::MediaSelectTelephone),
            141 => Ok(Consumer::MediaSelectProgramGuide),
            142 => Ok(Consumer::MediaSelectVideoPhone),
            143 => Ok(Consumer::MediaSelectGames),
            144 => Ok(Consumer::MediaSelectMessages),
            145 => Ok(Consumer::MediaSelectCD),
            146 => Ok(Consumer::MediaSelectVCR),
            147 => Ok(Consumer::MediaSelectTuner),
            148 => Ok(Consumer::Quit),
            149 => Ok(Consumer::Help),
            150 => Ok(Consumer::MediaSelectTape),
            151 => Ok(Consumer::MediaSelectCable),
            152 => Ok(Consumer::MediaSelectSatellite),
            153 => Ok(Consumer::MediaSelectSecurity),
            154 => Ok(Consumer::MediaSelectHome),
            155 => Ok(Consumer::MediaSelectCall),
            156 => Ok(Consumer::ChannelIncrement),
            157 => Ok(Consumer::ChannelDecrement),
            158 => Ok(Consumer::MediaSelectSAP),
            160 => Ok(Consumer::VCRPlus),
            161 => Ok(Consumer::Once),
            162 => Ok(Consumer::Daily),
            163 => Ok(Consumer::Weekly),
            164 => Ok(Consumer::Monthly),
            176 => Ok(Consumer::Play),
            177 => Ok(Consumer::Pause),
            178 => Ok(Consumer::Record),
            179 => Ok(Consumer::FastForward),
            180 => Ok(Consumer::Rewind),
            181 => Ok(Consumer::ScanNextTrack),
            182 => Ok(Consumer::ScanPreviousTrack),
            183 => Ok(Consumer::Stop),
            184 => Ok(Consumer::Eject),
            185 => Ok(Consumer::RandomPlay),
            186 => Ok(Consumer::SelectDisc),
            187 => Ok(Consumer::EnterDisc),
            188 => Ok(Consumer::Repeat),
            189 => Ok(Consumer::Tracking),
            190 => Ok(Consumer::TrackNormal),
            191 => Ok(Consumer::SlowTracking),
            192 => Ok(Consumer::FrameForward),
            193 => Ok(Consumer::FrameBack),
            194 => Ok(Consumer::Mark),
            195 => Ok(Consumer::ClearMark),
            196 => Ok(Consumer::RepeatFromMark),
            197 => Ok(Consumer::ReturnToMark),
            198 => Ok(Consumer::SearchMarkForward),
            199 => Ok(Consumer::SearchMarkBackwards),
            200 => Ok(Consumer::CounterReset),
            201 => Ok(Consumer::ShowCounter),
            202 => Ok(Consumer::TrackingIncrement),
            203 => Ok(Consumer::TrackingDecrement),
            204 => Ok(Consumer::StopEject),
            205 => Ok(Consumer::PlayPause),
            206 => Ok(Consumer::PlaySkip),
            207 => Ok(Consumer::VoiceCommand),
            208 => Ok(Consumer::InvokeCaptureInterface),
            209 => Ok(Consumer::StartorStopGameRecording),
            210 => Ok(Consumer::HistoricalGameCapture),
            211 => Ok(Consumer::CaptureGameScreenshot),
            212 => Ok(Consumer::ShoworHideRecordingIndicator),
            213 => Ok(Consumer::StartorStopMicrophoneCapture),
            214 => Ok(Consumer::StartorStopCameraCapture),
            215 => Ok(Consumer::StartorStopGameBroadcast),
            216 => Ok(Consumer::StartorStopVoiceDictationSession),
            217 => Ok(Consumer::InvokeDismissEmojiPicker),
            224 => Ok(Consumer::Volume),
            225 => Ok(Consumer::Balance),
            226 => Ok(Consumer::Mute),
            227 => Ok(Consumer::Bass),
            228 => Ok(Consumer::Treble),
            229 => Ok(Consumer::BassBoost),
            230 => Ok(Consumer::SurroundMode),
            231 => Ok(Consumer::Loudness),
            232 => Ok(Consumer::MPX),
            233 => Ok(Consumer::VolumeIncrement),
            234 => Ok(Consumer::VolumeDecrement),
            240 => Ok(Consumer::SpeedSelect),
            241 => Ok(Consumer::PlaybackSpeed),
            242 => Ok(Consumer::StandardPlay),
            243 => Ok(Consumer::LongPlay),
            244 => Ok(Consumer::ExtendedPlay),
            245 => Ok(Consumer::Slow),
            256 => Ok(Consumer::FanEnable),
            257 => Ok(Consumer::FanSpeed),
            258 => Ok(Consumer::LightEnable),
            259 => Ok(Consumer::LightIlluminationLevel),
            260 => Ok(Consumer::ClimateControlEnable),
            261 => Ok(Consumer::RoomTemperature),
            262 => Ok(Consumer::SecurityEnable),
            263 => Ok(Consumer::FireAlarm),
            264 => Ok(Consumer::PoliceAlarm),
            265 => Ok(Consumer::Proximity),
            266 => Ok(Consumer::Motion),
            267 => Ok(Consumer::DuressAlarm),
            268 => Ok(Consumer::HoldupAlarm),
            269 => Ok(Consumer::MedicalAlarm),
            336 => Ok(Consumer::BalanceRight),
            337 => Ok(Consumer::BalanceLeft),
            338 => Ok(Consumer::BassIncrement),
            339 => Ok(Consumer::BassDecrement),
            340 => Ok(Consumer::TrebleIncrement),
            341 => Ok(Consumer::TrebleDecrement),
            352 => Ok(Consumer::SpeakerSystem),
            353 => Ok(Consumer::ChannelLeft),
            354 => Ok(Consumer::ChannelRight),
            355 => Ok(Consumer::ChannelCenter),
            356 => Ok(Consumer::ChannelFront),
            357 => Ok(Consumer::ChannelCenterFront),
            358 => Ok(Consumer::ChannelSide),
            359 => Ok(Consumer::ChannelSurround),
            360 => Ok(Consumer::ChannelLowFrequencyEnhancement),
            361 => Ok(Consumer::ChannelTop),
            362 => Ok(Consumer::ChannelUnknown),
            368 => Ok(Consumer::Subchannel),
            369 => Ok(Consumer::SubchannelIncrement),
            370 => Ok(Consumer::SubchannelDecrement),
            371 => Ok(Consumer::AlternateAudioIncrement),
            372 => Ok(Consumer::AlternateAudioDecrement),
            384 => Ok(Consumer::ApplicationLaunchButtons),
            385 => Ok(Consumer::ALLaunchButtonConfigurationTool),
            386 => Ok(Consumer::ALProgrammableButtonConfiguration),
            387 => Ok(Consumer::ALConsumerControlConfiguration),
            388 => Ok(Consumer::ALWordProcessor),
            389 => Ok(Consumer::ALTextEditor),
            390 => Ok(Consumer::ALSpreadsheet),
            391 => Ok(Consumer::ALGraphicsEditor),
            392 => Ok(Consumer::ALPresentationApp),
            393 => Ok(Consumer::ALDatabaseApp),
            394 => Ok(Consumer::ALEmailReader),
            395 => Ok(Consumer::ALNewsreader),
            396 => Ok(Consumer::ALVoicemail),
            397 => Ok(Consumer::ALContactsAddressBook),
            398 => Ok(Consumer::ALCalendarSchedule),
            399 => Ok(Consumer::ALTaskProjectManager),
            400 => Ok(Consumer::ALLogJournalTimecard),
            401 => Ok(Consumer::ALCheckbookFinance),
            402 => Ok(Consumer::ALCalculator),
            403 => Ok(Consumer::ALAVCapturePlayback),
            404 => Ok(Consumer::ALLocalMachineBrowser),
            405 => Ok(Consumer::ALLANWANBrowser),
            406 => Ok(Consumer::ALInternetBrowser),
            407 => Ok(Consumer::ALRemoteNetworkingISPConnect),
            408 => Ok(Consumer::ALNetworkConference),
            409 => Ok(Consumer::ALNetworkChat),
            410 => Ok(Consumer::ALTelephonyDialer),
            411 => Ok(Consumer::ALLogon),
            412 => Ok(Consumer::ALLogoff),
            413 => Ok(Consumer::ALLogonLogoff),
            414 => Ok(Consumer::ALTerminalLockScreensaver),
            415 => Ok(Consumer::ALControlPanel),
            416 => Ok(Consumer::ALCommandLineProcessorRun),
            417 => Ok(Consumer::ALProcessTaskManager),
            418 => Ok(Consumer::ALSelectTaskApplication),
            419 => Ok(Consumer::ALNextTaskApplication),
            420 => Ok(Consumer::ALPreviousTaskApplication),
            421 => Ok(Consumer::ALPreemptiveHaltTaskApplication),
            422 => Ok(Consumer::ALIntegratedHelpCenter),
            423 => Ok(Consumer::ALDocuments),
            424 => Ok(Consumer::ALThesaurus),
            425 => Ok(Consumer::ALDictionary),
            426 => Ok(Consumer::ALDesktop),
            427 => Ok(Consumer::ALSpellCheck),
            428 => Ok(Consumer::ALGrammarCheck),
            429 => Ok(Consumer::ALWirelessStatus),
            430 => Ok(Consumer::ALKeyboardLayout),
            431 => Ok(Consumer::ALVirusProtection),
            432 => Ok(Consumer::ALEncryption),
            433 => Ok(Consumer::ALScreenSaver),
            434 => Ok(Consumer::ALAlarms),
            435 => Ok(Consumer::ALClock),
            436 => Ok(Consumer::ALFileBrowser),
            437 => Ok(Consumer::ALPowerStatus),
            438 => Ok(Consumer::ALImageBrowser),
            439 => Ok(Consumer::ALAudioBrowser),
            440 => Ok(Consumer::ALMovieBrowser),
            441 => Ok(Consumer::ALDigitalRightsManager),
            442 => Ok(Consumer::ALDigitalWallet),
            444 => Ok(Consumer::ALInstantMessaging),
            445 => Ok(Consumer::ALOEMFeaturesTipsTutorialBrowser),
            446 => Ok(Consumer::ALOEMHelp),
            447 => Ok(Consumer::ALOnlineCommunity),
            448 => Ok(Consumer::ALEntertainmentContentBrowser),
            449 => Ok(Consumer::ALOnlineShoppingBrowser),
            450 => Ok(Consumer::ALSmartCardInformationHelp),
            451 => Ok(Consumer::ALMarketMonitorFinanceBrowser),
            452 => Ok(Consumer::ALCustomizedCorporateNewsBrowser),
            453 => Ok(Consumer::ALOnlineActivityBrowser),
            454 => Ok(Consumer::ALResearchSearchBrowser),
            455 => Ok(Consumer::ALAudioPlayer),
            456 => Ok(Consumer::ALMessageStatus),
            457 => Ok(Consumer::ALContactSync),
            458 => Ok(Consumer::ALNavigation),
            459 => Ok(Consumer::ALContextawareDesktopAssistant),
            512 => Ok(Consumer::GenericGUIApplicationControls),
            513 => Ok(Consumer::ACNew),
            514 => Ok(Consumer::ACOpen),
            515 => Ok(Consumer::ACClose),
            516 => Ok(Consumer::ACExit),
            517 => Ok(Consumer::ACMaximize),
            518 => Ok(Consumer::ACMinimize),
            519 => Ok(Consumer::ACSave),
            520 => Ok(Consumer::ACPrint),
            521 => Ok(Consumer::ACProperties),
            538 => Ok(Consumer::ACUndo),
            539 => Ok(Consumer::ACCopy),
            540 => Ok(Consumer::ACCut),
            541 => Ok(Consumer::ACPaste),
            542 => Ok(Consumer::ACSelectAll),
            543 => Ok(Consumer::ACFind),
            544 => Ok(Consumer::ACFindandReplace),
            545 => Ok(Consumer::ACSearch),
            546 => Ok(Consumer::ACGoTo),
            547 => Ok(Consumer::ACHome),
            548 => Ok(Consumer::ACBack),
            549 => Ok(Consumer::ACForward),
            550 => Ok(Consumer::ACStop),
            551 => Ok(Consumer::ACRefresh),
            552 => Ok(Consumer::ACPreviousLink),
            553 => Ok(Consumer::ACNextLink),
            554 => Ok(Consumer::ACBookmarks),
            555 => Ok(Consumer::ACHistory),
            556 => Ok(Consumer::ACSubscriptions),
            557 => Ok(Consumer::ACZoomIn),
            558 => Ok(Consumer::ACZoomOut),
            559 => Ok(Consumer::ACZoom),
            560 => Ok(Consumer::ACFullScreenView),
            561 => Ok(Consumer::ACNormalView),
            562 => Ok(Consumer::ACViewToggle),
            563 => Ok(Consumer::ACScrollUp),
            564 => Ok(Consumer::ACScrollDown),
            565 => Ok(Consumer::ACScroll),
            566 => Ok(Consumer::ACPanLeft),
            567 => Ok(Consumer::ACPanRight),
            568 => Ok(Consumer::ACPan),
            569 => Ok(Consumer::ACNewWindow),
            570 => Ok(Consumer::ACTileHorizontally),
            571 => Ok(Consumer::ACTileVertically),
            572 => Ok(Consumer::ACFormat),
            573 => Ok(Consumer::ACEdit),
            574 => Ok(Consumer::ACBold),
            575 => Ok(Consumer::ACItalics),
            576 => Ok(Consumer::ACUnderline),
            577 => Ok(Consumer::ACStrikethrough),
            578 => Ok(Consumer::ACSubscript),
            579 => Ok(Consumer::ACSuperscript),
            580 => Ok(Consumer::ACAllCaps),
            581 => Ok(Consumer::ACRotate),
            582 => Ok(Consumer::ACResize),
            583 => Ok(Consumer::ACFlipHorizontal),
            584 => Ok(Consumer::ACFlipVertical),
            585 => Ok(Consumer::ACMirrorHorizontal),
            586 => Ok(Consumer::ACMirrorVertical),
            587 => Ok(Consumer::ACFontSelect),
            588 => Ok(Consumer::ACFontColor),
            589 => Ok(Consumer::ACFontSize),
            590 => Ok(Consumer::ACJustifyLeft),
            591 => Ok(Consumer::ACJustifyCenterH),
            592 => Ok(Consumer::ACJustifyRight),
            593 => Ok(Consumer::ACJustifyBlockH),
            594 => Ok(Consumer::ACJustifyTop),
            595 => Ok(Consumer::ACJustifyCenterV),
            596 => Ok(Consumer::ACJustifyBottom),
            597 => Ok(Consumer::ACJustifyBlockV),
            598 => Ok(Consumer::ACIndentDecrease),
            599 => Ok(Consumer::ACIndentIncrease),
            600 => Ok(Consumer::ACNumberedList),
            601 => Ok(Consumer::ACRestartNumbering),
            602 => Ok(Consumer::ACBulletedList),
            603 => Ok(Consumer::ACPromote),
            604 => Ok(Consumer::ACDemote),
            605 => Ok(Consumer::ACYes),
            606 => Ok(Consumer::ACNo),
            607 => Ok(Consumer::ACCancel),
            608 => Ok(Consumer::ACCatalog),
            609 => Ok(Consumer::ACBuyCheckout),
            610 => Ok(Consumer::ACAddtoCart),
            611 => Ok(Consumer::ACExpand),
            612 => Ok(Consumer::ACExpandAll),
            613 => Ok(Consumer::ACCollapse),
            614 => Ok(Consumer::ACCollapseAll),
            615 => Ok(Consumer::ACPrintPreview),
            616 => Ok(Consumer::ACPasteSpecial),
            617 => Ok(Consumer::ACInsertMode),
            618 => Ok(Consumer::ACDelete),
            619 => Ok(Consumer::ACLock),
            620 => Ok(Consumer::ACUnlock),
            621 => Ok(Consumer::ACProtect),
            622 => Ok(Consumer::ACUnprotect),
            623 => Ok(Consumer::ACAttachComment),
            624 => Ok(Consumer::ACDeleteComment),
            625 => Ok(Consumer::ACViewComment),
            626 => Ok(Consumer::ACSelectWord),
            627 => Ok(Consumer::ACSelectSentence),
            628 => Ok(Consumer::ACSelectParagraph),
            629 => Ok(Consumer::ACSelectColumn),
            630 => Ok(Consumer::ACSelectRow),
            631 => Ok(Consumer::ACSelectTable),
            632 => Ok(Consumer::ACSelectObject),
            633 => Ok(Consumer::ACRedoRepeat),
            634 => Ok(Consumer::ACSort),
            635 => Ok(Consumer::ACSortAscending),
            636 => Ok(Consumer::ACSortDescending),
            637 => Ok(Consumer::ACFilter),
            638 => Ok(Consumer::ACSetClock),
            639 => Ok(Consumer::ACViewClock),
            640 => Ok(Consumer::ACSelectTimeZone),
            641 => Ok(Consumer::ACEditTimeZones),
            642 => Ok(Consumer::ACSetAlarm),
            643 => Ok(Consumer::ACClearAlarm),
            644 => Ok(Consumer::ACSnoozeAlarm),
            645 => Ok(Consumer::ACResetAlarm),
            646 => Ok(Consumer::ACSynchronize),
            647 => Ok(Consumer::ACSendReceive),
            648 => Ok(Consumer::ACSendTo),
            649 => Ok(Consumer::ACReply),
            650 => Ok(Consumer::ACReplyAll),
            651 => Ok(Consumer::ACForwardMsg),
            652 => Ok(Consumer::ACSend),
            653 => Ok(Consumer::ACAttachFile),
            654 => Ok(Consumer::ACUpload),
            655 => Ok(Consumer::ACDownloadSaveTargetAs),
            656 => Ok(Consumer::ACSetBorders),
            657 => Ok(Consumer::ACInsertRow),
            658 => Ok(Consumer::ACInsertColumn),
            659 => Ok(Consumer::ACInsertFile),
            660 => Ok(Consumer::ACInsertPicture),
            661 => Ok(Consumer::ACInsertObject),
            662 => Ok(Consumer::ACInsertSymbol),
            663 => Ok(Consumer::ACSaveandClose),
            664 => Ok(Consumer::ACRename),
            665 => Ok(Consumer::ACMerge),
            666 => Ok(Consumer::ACSplit),
            667 => Ok(Consumer::ACDisributeHorizontally),
            668 => Ok(Consumer::ACDistributeVertically),
            669 => Ok(Consumer::ACNextKeyboardLayoutSelect),
            670 => Ok(Consumer::ACNavigationGuidance),
            671 => Ok(Consumer::ACDesktopShowAllWindows),
            672 => Ok(Consumer::ACSoftKeyLeft),
            673 => Ok(Consumer::ACSoftKeyRight),
            674 => Ok(Consumer::ACDesktopShowAllApplications),
            688 => Ok(Consumer::ACIdleKeepAlive),
            704 => Ok(Consumer::ExtendedKeyboardAttributesCollection),
            705 => Ok(Consumer::KeyboardFormFactor),
            706 => Ok(Consumer::KeyboardKeyType),
            707 => Ok(Consumer::KeyboardPhysicalLayout),
            708 => Ok(Consumer::VendorSpecificKeyboardPhysicalLayout),
            709 => Ok(Consumer::KeyboardIETFLanguageTagIndex),
            710 => Ok(Consumer::ImplementedKeyboardInputAssistControls),
            711 => Ok(Consumer::KeyboardInputAssistPrevious),
            712 => Ok(Consumer::KeyboardInputAssistNext),
            713 => Ok(Consumer::KeyboardInputAssistPreviousGroup),
            714 => Ok(Consumer::KeyboardInputAssistNextGroup),
            715 => Ok(Consumer::KeyboardInputAssistAccept),
            716 => Ok(Consumer::KeyboardInputAssistCancel),
            720 => Ok(Consumer::PrivacyScreenToggle),
            721 => Ok(Consumer::PrivacyScreenLevelDecrement),
            722 => Ok(Consumer::PrivacyScreenLevelIncrement),
            723 => Ok(Consumer::PrivacyScreenLevelMinimum),
            724 => Ok(Consumer::PrivacyScreenLevelMaximum),
            1280 => Ok(Consumer::ContactEdited),
            1281 => Ok(Consumer::ContactAdded),
            1282 => Ok(Consumer::ContactRecordActive),
            1283 => Ok(Consumer::ContactIndex),
            1284 => Ok(Consumer::ContactNickname),
            1285 => Ok(Consumer::ContactFirstName),
            1286 => Ok(Consumer::ContactLastName),
            1287 => Ok(Consumer::ContactFullName),
            1288 => Ok(Consumer::ContactPhoneNumberPersonal),
            1289 => Ok(Consumer::ContactPhoneNumberBusiness),
            1290 => Ok(Consumer::ContactPhoneNumberMobile),
            1291 => Ok(Consumer::ContactPhoneNumberPager),
            1292 => Ok(Consumer::ContactPhoneNumberFax),
            1293 => Ok(Consumer::ContactPhoneNumberOther),
            1294 => Ok(Consumer::ContactEmailPersonal),
            1295 => Ok(Consumer::ContactEmailBusiness),
            1296 => Ok(Consumer::ContactEmailOther),
            1297 => Ok(Consumer::ContactEmailMain),
            1298 => Ok(Consumer::ContactSpeedDialNumber),
            1299 => Ok(Consumer::ContactStatusFlag),
            1300 => Ok(Consumer::ContactMisc),
            1301 => Ok(Consumer::KeyboardBrightnessNext),
            1302 => Ok(Consumer::KeyboardBrightnessPrevious),
            1303 => Ok(Consumer::KeyboardBacklightLevelSuggestion),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Consumer {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xD`: "Digitizers"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Digitizers(Digitizers::Pen);
/// let u2 = Usage::new_from_page_and_id(0xD, 0x2).unwrap();
/// let u3 = Usage::from(Digitizers::Pen);
/// let u4: Usage = Digitizers::Pen.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Digitizers));
/// assert_eq!(0xD, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0xD << 16) | 0x2, u1.usage_value());
/// assert_eq!("Pen", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Digitizers {
    /// Usage ID `0x1`: "Digitizer"
    Digitizer = 0x1,
    /// Usage ID `0x2`: "Pen"
    Pen = 0x2,
    /// Usage ID `0x3`: "Light Pen"
    LightPen = 0x3,
    /// Usage ID `0x4`: "Touch Screen"
    TouchScreen = 0x4,
    /// Usage ID `0x5`: "Touch Pad"
    TouchPad = 0x5,
    /// Usage ID `0x6`: "Whiteboard"
    Whiteboard = 0x6,
    /// Usage ID `0x7`: "Coordinate Measuring Machine"
    CoordinateMeasuringMachine = 0x7,
    /// Usage ID `0x8`: "3D Digitizer"
    ThreeDDigitizer = 0x8,
    /// Usage ID `0x9`: "Stereo Plotter"
    StereoPlotter = 0x9,
    /// Usage ID `0xA`: "Articulated Arm"
    ArticulatedArm = 0xA,
    /// Usage ID `0xB`: "Armature"
    Armature = 0xB,
    /// Usage ID `0xC`: "Multiple Point Digitizer"
    MultiplePointDigitizer = 0xC,
    /// Usage ID `0xD`: "Free Space Wand"
    FreeSpaceWand = 0xD,
    /// Usage ID `0xE`: "Device Configuration"
    DeviceConfiguration = 0xE,
    /// Usage ID `0xF`: "Capacitive Heat Map Digitizer"
    CapacitiveHeatMapDigitizer = 0xF,
    /// Usage ID `0x20`: "Stylus"
    Stylus = 0x20,
    /// Usage ID `0x21`: "Puck"
    Puck = 0x21,
    /// Usage ID `0x22`: "Finger"
    Finger = 0x22,
    /// Usage ID `0x23`: "Device settings"
    Devicesettings = 0x23,
    /// Usage ID `0x24`: "Character Gesture"
    CharacterGesture = 0x24,
    /// Usage ID `0x30`: "Tip Pressure"
    TipPressure = 0x30,
    /// Usage ID `0x31`: "Barrel Pressure"
    BarrelPressure = 0x31,
    /// Usage ID `0x32`: "In Range"
    InRange = 0x32,
    /// Usage ID `0x33`: "Touch"
    Touch = 0x33,
    /// Usage ID `0x34`: "Untouch"
    Untouch = 0x34,
    /// Usage ID `0x35`: "Tap"
    Tap = 0x35,
    /// Usage ID `0x36`: "Quality"
    Quality = 0x36,
    /// Usage ID `0x37`: "Data Valid"
    DataValid = 0x37,
    /// Usage ID `0x38`: "Transducer Index"
    TransducerIndex = 0x38,
    /// Usage ID `0x39`: "Tablet Function Keys"
    TabletFunctionKeys = 0x39,
    /// Usage ID `0x3A`: "Program Change Keys"
    ProgramChangeKeys = 0x3A,
    /// Usage ID `0x3B`: "Battery Strength"
    BatteryStrength = 0x3B,
    /// Usage ID `0x3C`: "Invert"
    Invert = 0x3C,
    /// Usage ID `0x3D`: "X Tilt"
    XTilt = 0x3D,
    /// Usage ID `0x3E`: "Y Tilt"
    YTilt = 0x3E,
    /// Usage ID `0x3F`: "Azimuth"
    Azimuth = 0x3F,
    /// Usage ID `0x40`: "Altitude"
    Altitude = 0x40,
    /// Usage ID `0x41`: "Twist"
    Twist = 0x41,
    /// Usage ID `0x42`: "Tip Switch"
    TipSwitch = 0x42,
    /// Usage ID `0x43`: "Secondary Tip Switch"
    SecondaryTipSwitch = 0x43,
    /// Usage ID `0x44`: "Barrel Switch"
    BarrelSwitch = 0x44,
    /// Usage ID `0x45`: "Eraser"
    Eraser = 0x45,
    /// Usage ID `0x46`: "Tablet Pick"
    TabletPick = 0x46,
    /// Usage ID `0x47`: "Touch Valid"
    TouchValid = 0x47,
    /// Usage ID `0x48`: "Width"
    Width = 0x48,
    /// Usage ID `0x49`: "Height"
    Height = 0x49,
    /// Usage ID `0x51`: "Contact Identifier"
    ContactIdentifier = 0x51,
    /// Usage ID `0x52`: "Device Mode"
    DeviceMode = 0x52,
    /// Usage ID `0x53`: "Device Identifier"
    DeviceIdentifier = 0x53,
    /// Usage ID `0x54`: "Contact Count"
    ContactCount = 0x54,
    /// Usage ID `0x55`: "Contact Count Maximum"
    ContactCountMaximum = 0x55,
    /// Usage ID `0x56`: "Scan Time"
    ScanTime = 0x56,
    /// Usage ID `0x57`: "Surface Switch"
    SurfaceSwitch = 0x57,
    /// Usage ID `0x58`: "Button Switch"
    ButtonSwitch = 0x58,
    /// Usage ID `0x59`: "Pad Type"
    PadType = 0x59,
    /// Usage ID `0x5A`: "Secondary Barrel Switch"
    SecondaryBarrelSwitch = 0x5A,
    /// Usage ID `0x5B`: "Transducer Serial Number"
    TransducerSerialNumber = 0x5B,
    /// Usage ID `0x5C`: "Preferred Color"
    PreferredColor = 0x5C,
    /// Usage ID `0x5D`: "Preferred Color is Locked"
    PreferredColorisLocked = 0x5D,
    /// Usage ID `0x5E`: "Preferred Line Width"
    PreferredLineWidth = 0x5E,
    /// Usage ID `0x5F`: "Preferred Line Width is Locked"
    PreferredLineWidthisLocked = 0x5F,
    /// Usage ID `0x60`: "Latency Mode"
    LatencyMode = 0x60,
    /// Usage ID `0x61`: "Gesture Character Quality"
    GestureCharacterQuality = 0x61,
    /// Usage ID `0x62`: "Character Gesture Data Length"
    CharacterGestureDataLength = 0x62,
    /// Usage ID `0x63`: "Character Gesture Data"
    CharacterGestureData = 0x63,
    /// Usage ID `0x64`: "Gesture Character Encoding"
    GestureCharacterEncoding = 0x64,
    /// Usage ID `0x65`: "UTF8 Character Gesture Encoding"
    UTF8CharacterGestureEncoding = 0x65,
    /// Usage ID `0x66`: "UTF16 Little Endian Character Gesture Encoding"
    UTF16LittleEndianCharacterGestureEncoding = 0x66,
    /// Usage ID `0x67`: "UTF16 Big Endian Character Gesture Encoding"
    UTF16BigEndianCharacterGestureEncoding = 0x67,
    /// Usage ID `0x68`: "UTF32 Little Endian Character Gesture Encoding"
    UTF32LittleEndianCharacterGestureEncoding = 0x68,
    /// Usage ID `0x69`: "UTF32 Big Endian Character Gesture Encoding"
    UTF32BigEndianCharacterGestureEncoding = 0x69,
    /// Usage ID `0x6A`: "Capacitive Heat Map Protocol Vendor ID"
    CapacitiveHeatMapProtocolVendorID = 0x6A,
    /// Usage ID `0x6B`: "Capacitive Heat Map Protocol Version"
    CapacitiveHeatMapProtocolVersion = 0x6B,
    /// Usage ID `0x6C`: "Capacitive Heat Map Frame Data"
    CapacitiveHeatMapFrameData = 0x6C,
    /// Usage ID `0x6D`: "Gesture Character Enable"
    GestureCharacterEnable = 0x6D,
    /// Usage ID `0x6E`: "Transducer Serial Number Part 2"
    TransducerSerialNumberPart2 = 0x6E,
    /// Usage ID `0x6F`: "No Preferred Color"
    NoPreferredColor = 0x6F,
    /// Usage ID `0x70`: "Preferred Line Style"
    PreferredLineStyle = 0x70,
    /// Usage ID `0x71`: "Preferred Line Style is Locked"
    PreferredLineStyleisLocked = 0x71,
    /// Usage ID `0x72`: "Ink"
    Ink = 0x72,
    /// Usage ID `0x73`: "Pencil"
    Pencil = 0x73,
    /// Usage ID `0x74`: "Highlighter"
    Highlighter = 0x74,
    /// Usage ID `0x75`: "Chisel Marker"
    ChiselMarker = 0x75,
    /// Usage ID `0x76`: "Brush"
    Brush = 0x76,
    /// Usage ID `0x77`: "No Preference"
    NoPreference = 0x77,
    /// Usage ID `0x80`: "Digitizer Diagnostic"
    DigitizerDiagnostic = 0x80,
    /// Usage ID `0x81`: "Digitizer Error"
    DigitizerError = 0x81,
    /// Usage ID `0x82`: "Err Normal Status"
    ErrNormalStatus = 0x82,
    /// Usage ID `0x83`: "Err Transducers Exceeded"
    ErrTransducersExceeded = 0x83,
    /// Usage ID `0x84`: "Err Full Trans Features Unavailable"
    ErrFullTransFeaturesUnavailable = 0x84,
    /// Usage ID `0x85`: "Err Charge Low"
    ErrChargeLow = 0x85,
    /// Usage ID `0x90`: "Transducer Software Info"
    TransducerSoftwareInfo = 0x90,
    /// Usage ID `0x91`: "Transducer Vendor Id"
    TransducerVendorId = 0x91,
    /// Usage ID `0x92`: "Transducer Product Id"
    TransducerProductId = 0x92,
    /// Usage ID `0x93`: "Device Supported Protocols"
    DeviceSupportedProtocols = 0x93,
    /// Usage ID `0x94`: "Transducer Supported Protocols"
    TransducerSupportedProtocols = 0x94,
    /// Usage ID `0x95`: "No Protocol"
    NoProtocol = 0x95,
    /// Usage ID `0x96`: "Wacom AES Protocol"
    WacomAESProtocol = 0x96,
    /// Usage ID `0x97`: "USI Protocol"
    USIProtocol = 0x97,
    /// Usage ID `0x98`: "Microsoft Pen Protocol"
    MicrosoftPenProtocol = 0x98,
    /// Usage ID `0xA0`: "Supported Report Rates"
    SupportedReportRates = 0xA0,
    /// Usage ID `0xA1`: "Report Rate"
    ReportRate = 0xA1,
    /// Usage ID `0xA2`: "Transducer Connected"
    TransducerConnected = 0xA2,
    /// Usage ID `0xA3`: "Switch Disabled"
    SwitchDisabled = 0xA3,
    /// Usage ID `0xA4`: "Switch Unimplemented"
    SwitchUnimplemented = 0xA4,
    /// Usage ID `0xA5`: "Transducer Switches"
    TransducerSwitches = 0xA5,
    /// Usage ID `0xA6`: "Transducer Index Selector"
    TransducerIndexSelector = 0xA6,
    /// Usage ID `0xB0`: "Button Press Threshold"
    ButtonPressThreshold = 0xB0,
}

impl Digitizers {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Digitizers::Digitizer => "Digitizer",
            Digitizers::Pen => "Pen",
            Digitizers::LightPen => "Light Pen",
            Digitizers::TouchScreen => "Touch Screen",
            Digitizers::TouchPad => "Touch Pad",
            Digitizers::Whiteboard => "Whiteboard",
            Digitizers::CoordinateMeasuringMachine => "Coordinate Measuring Machine",
            Digitizers::ThreeDDigitizer => "3D Digitizer",
            Digitizers::StereoPlotter => "Stereo Plotter",
            Digitizers::ArticulatedArm => "Articulated Arm",
            Digitizers::Armature => "Armature",
            Digitizers::MultiplePointDigitizer => "Multiple Point Digitizer",
            Digitizers::FreeSpaceWand => "Free Space Wand",
            Digitizers::DeviceConfiguration => "Device Configuration",
            Digitizers::CapacitiveHeatMapDigitizer => "Capacitive Heat Map Digitizer",
            Digitizers::Stylus => "Stylus",
            Digitizers::Puck => "Puck",
            Digitizers::Finger => "Finger",
            Digitizers::Devicesettings => "Device settings",
            Digitizers::CharacterGesture => "Character Gesture",
            Digitizers::TipPressure => "Tip Pressure",
            Digitizers::BarrelPressure => "Barrel Pressure",
            Digitizers::InRange => "In Range",
            Digitizers::Touch => "Touch",
            Digitizers::Untouch => "Untouch",
            Digitizers::Tap => "Tap",
            Digitizers::Quality => "Quality",
            Digitizers::DataValid => "Data Valid",
            Digitizers::TransducerIndex => "Transducer Index",
            Digitizers::TabletFunctionKeys => "Tablet Function Keys",
            Digitizers::ProgramChangeKeys => "Program Change Keys",
            Digitizers::BatteryStrength => "Battery Strength",
            Digitizers::Invert => "Invert",
            Digitizers::XTilt => "X Tilt",
            Digitizers::YTilt => "Y Tilt",
            Digitizers::Azimuth => "Azimuth",
            Digitizers::Altitude => "Altitude",
            Digitizers::Twist => "Twist",
            Digitizers::TipSwitch => "Tip Switch",
            Digitizers::SecondaryTipSwitch => "Secondary Tip Switch",
            Digitizers::BarrelSwitch => "Barrel Switch",
            Digitizers::Eraser => "Eraser",
            Digitizers::TabletPick => "Tablet Pick",
            Digitizers::TouchValid => "Touch Valid",
            Digitizers::Width => "Width",
            Digitizers::Height => "Height",
            Digitizers::ContactIdentifier => "Contact Identifier",
            Digitizers::DeviceMode => "Device Mode",
            Digitizers::DeviceIdentifier => "Device Identifier",
            Digitizers::ContactCount => "Contact Count",
            Digitizers::ContactCountMaximum => "Contact Count Maximum",
            Digitizers::ScanTime => "Scan Time",
            Digitizers::SurfaceSwitch => "Surface Switch",
            Digitizers::ButtonSwitch => "Button Switch",
            Digitizers::PadType => "Pad Type",
            Digitizers::SecondaryBarrelSwitch => "Secondary Barrel Switch",
            Digitizers::TransducerSerialNumber => "Transducer Serial Number",
            Digitizers::PreferredColor => "Preferred Color",
            Digitizers::PreferredColorisLocked => "Preferred Color is Locked",
            Digitizers::PreferredLineWidth => "Preferred Line Width",
            Digitizers::PreferredLineWidthisLocked => "Preferred Line Width is Locked",
            Digitizers::LatencyMode => "Latency Mode",
            Digitizers::GestureCharacterQuality => "Gesture Character Quality",
            Digitizers::CharacterGestureDataLength => "Character Gesture Data Length",
            Digitizers::CharacterGestureData => "Character Gesture Data",
            Digitizers::GestureCharacterEncoding => "Gesture Character Encoding",
            Digitizers::UTF8CharacterGestureEncoding => "UTF8 Character Gesture Encoding",
            Digitizers::UTF16LittleEndianCharacterGestureEncoding => {
                "UTF16 Little Endian Character Gesture Encoding"
            }
            Digitizers::UTF16BigEndianCharacterGestureEncoding => {
                "UTF16 Big Endian Character Gesture Encoding"
            }
            Digitizers::UTF32LittleEndianCharacterGestureEncoding => {
                "UTF32 Little Endian Character Gesture Encoding"
            }
            Digitizers::UTF32BigEndianCharacterGestureEncoding => {
                "UTF32 Big Endian Character Gesture Encoding"
            }
            Digitizers::CapacitiveHeatMapProtocolVendorID => {
                "Capacitive Heat Map Protocol Vendor ID"
            }
            Digitizers::CapacitiveHeatMapProtocolVersion => "Capacitive Heat Map Protocol Version",
            Digitizers::CapacitiveHeatMapFrameData => "Capacitive Heat Map Frame Data",
            Digitizers::GestureCharacterEnable => "Gesture Character Enable",
            Digitizers::TransducerSerialNumberPart2 => "Transducer Serial Number Part 2",
            Digitizers::NoPreferredColor => "No Preferred Color",
            Digitizers::PreferredLineStyle => "Preferred Line Style",
            Digitizers::PreferredLineStyleisLocked => "Preferred Line Style is Locked",
            Digitizers::Ink => "Ink",
            Digitizers::Pencil => "Pencil",
            Digitizers::Highlighter => "Highlighter",
            Digitizers::ChiselMarker => "Chisel Marker",
            Digitizers::Brush => "Brush",
            Digitizers::NoPreference => "No Preference",
            Digitizers::DigitizerDiagnostic => "Digitizer Diagnostic",
            Digitizers::DigitizerError => "Digitizer Error",
            Digitizers::ErrNormalStatus => "Err Normal Status",
            Digitizers::ErrTransducersExceeded => "Err Transducers Exceeded",
            Digitizers::ErrFullTransFeaturesUnavailable => "Err Full Trans Features Unavailable",
            Digitizers::ErrChargeLow => "Err Charge Low",
            Digitizers::TransducerSoftwareInfo => "Transducer Software Info",
            Digitizers::TransducerVendorId => "Transducer Vendor Id",
            Digitizers::TransducerProductId => "Transducer Product Id",
            Digitizers::DeviceSupportedProtocols => "Device Supported Protocols",
            Digitizers::TransducerSupportedProtocols => "Transducer Supported Protocols",
            Digitizers::NoProtocol => "No Protocol",
            Digitizers::WacomAESProtocol => "Wacom AES Protocol",
            Digitizers::USIProtocol => "USI Protocol",
            Digitizers::MicrosoftPenProtocol => "Microsoft Pen Protocol",
            Digitizers::SupportedReportRates => "Supported Report Rates",
            Digitizers::ReportRate => "Report Rate",
            Digitizers::TransducerConnected => "Transducer Connected",
            Digitizers::SwitchDisabled => "Switch Disabled",
            Digitizers::SwitchUnimplemented => "Switch Unimplemented",
            Digitizers::TransducerSwitches => "Transducer Switches",
            Digitizers::TransducerIndexSelector => "Transducer Index Selector",
            Digitizers::ButtonPressThreshold => "Button Press Threshold",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Digitizers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Digitizers {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Digitizers(self)](Usage::Digitizers)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Digitizers {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xD` for [Digitizers]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Digitizers]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Digitizers> for u16 {
    fn from(digitizers: &Digitizers) -> u16 {
        *digitizers as u16
    }
}

impl From<Digitizers> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Digitizers::usage_page_value()].
    fn from(digitizers: Digitizers) -> u16 {
        u16::from(&digitizers)
    }
}

impl From<&Digitizers> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Digitizers::usage_value()].
    fn from(digitizers: &Digitizers) -> u32 {
        let up = UsagePage::from(digitizers);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(digitizers) as u32;
        up | id
    }
}

impl From<&Digitizers> for UsagePage {
    /// Always returns [UsagePage::Digitizers] and is
    /// identical to [Digitizers::usage_page()].
    fn from(_: &Digitizers) -> UsagePage {
        UsagePage::Digitizers
    }
}

impl From<Digitizers> for UsagePage {
    /// Always returns [UsagePage::Digitizers] and is
    /// identical to [Digitizers::usage_page()].
    fn from(_: Digitizers) -> UsagePage {
        UsagePage::Digitizers
    }
}

impl From<&Digitizers> for Usage {
    fn from(digitizers: &Digitizers) -> Usage {
        Usage::try_from(u32::from(digitizers)).unwrap()
    }
}

impl From<Digitizers> for Usage {
    fn from(digitizers: Digitizers) -> Usage {
        Usage::from(&digitizers)
    }
}

impl TryFrom<u16> for Digitizers {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Digitizers> {
        match usage_id {
            1 => Ok(Digitizers::Digitizer),
            2 => Ok(Digitizers::Pen),
            3 => Ok(Digitizers::LightPen),
            4 => Ok(Digitizers::TouchScreen),
            5 => Ok(Digitizers::TouchPad),
            6 => Ok(Digitizers::Whiteboard),
            7 => Ok(Digitizers::CoordinateMeasuringMachine),
            8 => Ok(Digitizers::ThreeDDigitizer),
            9 => Ok(Digitizers::StereoPlotter),
            10 => Ok(Digitizers::ArticulatedArm),
            11 => Ok(Digitizers::Armature),
            12 => Ok(Digitizers::MultiplePointDigitizer),
            13 => Ok(Digitizers::FreeSpaceWand),
            14 => Ok(Digitizers::DeviceConfiguration),
            15 => Ok(Digitizers::CapacitiveHeatMapDigitizer),
            32 => Ok(Digitizers::Stylus),
            33 => Ok(Digitizers::Puck),
            34 => Ok(Digitizers::Finger),
            35 => Ok(Digitizers::Devicesettings),
            36 => Ok(Digitizers::CharacterGesture),
            48 => Ok(Digitizers::TipPressure),
            49 => Ok(Digitizers::BarrelPressure),
            50 => Ok(Digitizers::InRange),
            51 => Ok(Digitizers::Touch),
            52 => Ok(Digitizers::Untouch),
            53 => Ok(Digitizers::Tap),
            54 => Ok(Digitizers::Quality),
            55 => Ok(Digitizers::DataValid),
            56 => Ok(Digitizers::TransducerIndex),
            57 => Ok(Digitizers::TabletFunctionKeys),
            58 => Ok(Digitizers::ProgramChangeKeys),
            59 => Ok(Digitizers::BatteryStrength),
            60 => Ok(Digitizers::Invert),
            61 => Ok(Digitizers::XTilt),
            62 => Ok(Digitizers::YTilt),
            63 => Ok(Digitizers::Azimuth),
            64 => Ok(Digitizers::Altitude),
            65 => Ok(Digitizers::Twist),
            66 => Ok(Digitizers::TipSwitch),
            67 => Ok(Digitizers::SecondaryTipSwitch),
            68 => Ok(Digitizers::BarrelSwitch),
            69 => Ok(Digitizers::Eraser),
            70 => Ok(Digitizers::TabletPick),
            71 => Ok(Digitizers::TouchValid),
            72 => Ok(Digitizers::Width),
            73 => Ok(Digitizers::Height),
            81 => Ok(Digitizers::ContactIdentifier),
            82 => Ok(Digitizers::DeviceMode),
            83 => Ok(Digitizers::DeviceIdentifier),
            84 => Ok(Digitizers::ContactCount),
            85 => Ok(Digitizers::ContactCountMaximum),
            86 => Ok(Digitizers::ScanTime),
            87 => Ok(Digitizers::SurfaceSwitch),
            88 => Ok(Digitizers::ButtonSwitch),
            89 => Ok(Digitizers::PadType),
            90 => Ok(Digitizers::SecondaryBarrelSwitch),
            91 => Ok(Digitizers::TransducerSerialNumber),
            92 => Ok(Digitizers::PreferredColor),
            93 => Ok(Digitizers::PreferredColorisLocked),
            94 => Ok(Digitizers::PreferredLineWidth),
            95 => Ok(Digitizers::PreferredLineWidthisLocked),
            96 => Ok(Digitizers::LatencyMode),
            97 => Ok(Digitizers::GestureCharacterQuality),
            98 => Ok(Digitizers::CharacterGestureDataLength),
            99 => Ok(Digitizers::CharacterGestureData),
            100 => Ok(Digitizers::GestureCharacterEncoding),
            101 => Ok(Digitizers::UTF8CharacterGestureEncoding),
            102 => Ok(Digitizers::UTF16LittleEndianCharacterGestureEncoding),
            103 => Ok(Digitizers::UTF16BigEndianCharacterGestureEncoding),
            104 => Ok(Digitizers::UTF32LittleEndianCharacterGestureEncoding),
            105 => Ok(Digitizers::UTF32BigEndianCharacterGestureEncoding),
            106 => Ok(Digitizers::CapacitiveHeatMapProtocolVendorID),
            107 => Ok(Digitizers::CapacitiveHeatMapProtocolVersion),
            108 => Ok(Digitizers::CapacitiveHeatMapFrameData),
            109 => Ok(Digitizers::GestureCharacterEnable),
            110 => Ok(Digitizers::TransducerSerialNumberPart2),
            111 => Ok(Digitizers::NoPreferredColor),
            112 => Ok(Digitizers::PreferredLineStyle),
            113 => Ok(Digitizers::PreferredLineStyleisLocked),
            114 => Ok(Digitizers::Ink),
            115 => Ok(Digitizers::Pencil),
            116 => Ok(Digitizers::Highlighter),
            117 => Ok(Digitizers::ChiselMarker),
            118 => Ok(Digitizers::Brush),
            119 => Ok(Digitizers::NoPreference),
            128 => Ok(Digitizers::DigitizerDiagnostic),
            129 => Ok(Digitizers::DigitizerError),
            130 => Ok(Digitizers::ErrNormalStatus),
            131 => Ok(Digitizers::ErrTransducersExceeded),
            132 => Ok(Digitizers::ErrFullTransFeaturesUnavailable),
            133 => Ok(Digitizers::ErrChargeLow),
            144 => Ok(Digitizers::TransducerSoftwareInfo),
            145 => Ok(Digitizers::TransducerVendorId),
            146 => Ok(Digitizers::TransducerProductId),
            147 => Ok(Digitizers::DeviceSupportedProtocols),
            148 => Ok(Digitizers::TransducerSupportedProtocols),
            149 => Ok(Digitizers::NoProtocol),
            150 => Ok(Digitizers::WacomAESProtocol),
            151 => Ok(Digitizers::USIProtocol),
            152 => Ok(Digitizers::MicrosoftPenProtocol),
            160 => Ok(Digitizers::SupportedReportRates),
            161 => Ok(Digitizers::ReportRate),
            162 => Ok(Digitizers::TransducerConnected),
            163 => Ok(Digitizers::SwitchDisabled),
            164 => Ok(Digitizers::SwitchUnimplemented),
            165 => Ok(Digitizers::TransducerSwitches),
            166 => Ok(Digitizers::TransducerIndexSelector),
            176 => Ok(Digitizers::ButtonPressThreshold),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Digitizers {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xE`: "Haptics"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Haptics(Haptics::WaveformList);
/// let u2 = Usage::new_from_page_and_id(0xE, 0x10).unwrap();
/// let u3 = Usage::from(Haptics::WaveformList);
/// let u4: Usage = Haptics::WaveformList.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Haptics));
/// assert_eq!(0xE, u1.usage_page_value());
/// assert_eq!(0x10, u1.usage_id_value());
/// assert_eq!((0xE << 16) | 0x10, u1.usage_value());
/// assert_eq!("Waveform List", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Haptics {
    /// Usage ID `0x1`: "Simple Haptic Controller"
    SimpleHapticController = 0x1,
    /// Usage ID `0x10`: "Waveform List"
    WaveformList = 0x10,
    /// Usage ID `0x11`: "Duration List"
    DurationList = 0x11,
    /// Usage ID `0x20`: "Auto Trigger"
    AutoTrigger = 0x20,
    /// Usage ID `0x21`: "Manual Trigger"
    ManualTrigger = 0x21,
    /// Usage ID `0x22`: "Auto Trigger Associated Control"
    AutoTriggerAssociatedControl = 0x22,
    /// Usage ID `0x23`: "Intensity"
    Intensity = 0x23,
    /// Usage ID `0x24`: "Repeat Count"
    RepeatCount = 0x24,
    /// Usage ID `0x25`: "Retrigger Period"
    RetriggerPeriod = 0x25,
    /// Usage ID `0x26`: "Waveform Vendor Page"
    WaveformVendorPage = 0x26,
    /// Usage ID `0x27`: "Waveform Vendor ID"
    WaveformVendorID = 0x27,
    /// Usage ID `0x28`: "Waveform Cutoff Time"
    WaveformCutoffTime = 0x28,
    /// Usage ID `0x1001`: "Waveform None"
    WaveformNone = 0x1001,
    /// Usage ID `0x1002`: "Waveform Stop"
    WaveformStop = 0x1002,
    /// Usage ID `0x1003`: "Waveform Click"
    WaveformClick = 0x1003,
    /// Usage ID `0x1004`: "Waveform Buzz Continuous"
    WaveformBuzzContinuous = 0x1004,
    /// Usage ID `0x1005`: "Waveform Rumble Continuous"
    WaveformRumbleContinuous = 0x1005,
    /// Usage ID `0x1006`: "Waveform Press"
    WaveformPress = 0x1006,
    /// Usage ID `0x1007`: "Waveform Release"
    WaveformRelease = 0x1007,
    /// Usage ID `0x1008`: "Waveform Hover"
    WaveformHover = 0x1008,
    /// Usage ID `0x1009`: "Waveform Success"
    WaveformSuccess = 0x1009,
    /// Usage ID `0x100A`: "Waveform Error"
    WaveformError = 0x100A,
    /// Usage ID `0x100B`: "Waveform Ink Continuous"
    WaveformInkContinuous = 0x100B,
    /// Usage ID `0x100C`: "Waveform Pencil Continuous"
    WaveformPencilContinuous = 0x100C,
    /// Usage ID `0x100D`: "Waveform Marker Continuous"
    WaveformMarkerContinuous = 0x100D,
    /// Usage ID `0x100E`: "Waveform Chisel Marker Continuous"
    WaveformChiselMarkerContinuous = 0x100E,
    /// Usage ID `0x100F`: "Waveform Brush Continuous"
    WaveformBrushContinuous = 0x100F,
    /// Usage ID `0x1010`: "Waveform Eraser Continuous"
    WaveformEraserContinuous = 0x1010,
    /// Usage ID `0x1011`: "Waveform Sparkle Continuous"
    WaveformSparkleContinuous = 0x1011,
}

impl Haptics {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Haptics::SimpleHapticController => "Simple Haptic Controller",
            Haptics::WaveformList => "Waveform List",
            Haptics::DurationList => "Duration List",
            Haptics::AutoTrigger => "Auto Trigger",
            Haptics::ManualTrigger => "Manual Trigger",
            Haptics::AutoTriggerAssociatedControl => "Auto Trigger Associated Control",
            Haptics::Intensity => "Intensity",
            Haptics::RepeatCount => "Repeat Count",
            Haptics::RetriggerPeriod => "Retrigger Period",
            Haptics::WaveformVendorPage => "Waveform Vendor Page",
            Haptics::WaveformVendorID => "Waveform Vendor ID",
            Haptics::WaveformCutoffTime => "Waveform Cutoff Time",
            Haptics::WaveformNone => "Waveform None",
            Haptics::WaveformStop => "Waveform Stop",
            Haptics::WaveformClick => "Waveform Click",
            Haptics::WaveformBuzzContinuous => "Waveform Buzz Continuous",
            Haptics::WaveformRumbleContinuous => "Waveform Rumble Continuous",
            Haptics::WaveformPress => "Waveform Press",
            Haptics::WaveformRelease => "Waveform Release",
            Haptics::WaveformHover => "Waveform Hover",
            Haptics::WaveformSuccess => "Waveform Success",
            Haptics::WaveformError => "Waveform Error",
            Haptics::WaveformInkContinuous => "Waveform Ink Continuous",
            Haptics::WaveformPencilContinuous => "Waveform Pencil Continuous",
            Haptics::WaveformMarkerContinuous => "Waveform Marker Continuous",
            Haptics::WaveformChiselMarkerContinuous => "Waveform Chisel Marker Continuous",
            Haptics::WaveformBrushContinuous => "Waveform Brush Continuous",
            Haptics::WaveformEraserContinuous => "Waveform Eraser Continuous",
            Haptics::WaveformSparkleContinuous => "Waveform Sparkle Continuous",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Haptics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Haptics {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Haptics(self)](Usage::Haptics)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Haptics {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xE` for [Haptics]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Haptics]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Haptics> for u16 {
    fn from(haptics: &Haptics) -> u16 {
        *haptics as u16
    }
}

impl From<Haptics> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Haptics::usage_page_value()].
    fn from(haptics: Haptics) -> u16 {
        u16::from(&haptics)
    }
}

impl From<&Haptics> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Haptics::usage_value()].
    fn from(haptics: &Haptics) -> u32 {
        let up = UsagePage::from(haptics);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(haptics) as u32;
        up | id
    }
}

impl From<&Haptics> for UsagePage {
    /// Always returns [UsagePage::Haptics] and is
    /// identical to [Haptics::usage_page()].
    fn from(_: &Haptics) -> UsagePage {
        UsagePage::Haptics
    }
}

impl From<Haptics> for UsagePage {
    /// Always returns [UsagePage::Haptics] and is
    /// identical to [Haptics::usage_page()].
    fn from(_: Haptics) -> UsagePage {
        UsagePage::Haptics
    }
}

impl From<&Haptics> for Usage {
    fn from(haptics: &Haptics) -> Usage {
        Usage::try_from(u32::from(haptics)).unwrap()
    }
}

impl From<Haptics> for Usage {
    fn from(haptics: Haptics) -> Usage {
        Usage::from(&haptics)
    }
}

impl TryFrom<u16> for Haptics {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Haptics> {
        match usage_id {
            1 => Ok(Haptics::SimpleHapticController),
            16 => Ok(Haptics::WaveformList),
            17 => Ok(Haptics::DurationList),
            32 => Ok(Haptics::AutoTrigger),
            33 => Ok(Haptics::ManualTrigger),
            34 => Ok(Haptics::AutoTriggerAssociatedControl),
            35 => Ok(Haptics::Intensity),
            36 => Ok(Haptics::RepeatCount),
            37 => Ok(Haptics::RetriggerPeriod),
            38 => Ok(Haptics::WaveformVendorPage),
            39 => Ok(Haptics::WaveformVendorID),
            40 => Ok(Haptics::WaveformCutoffTime),
            4097 => Ok(Haptics::WaveformNone),
            4098 => Ok(Haptics::WaveformStop),
            4099 => Ok(Haptics::WaveformClick),
            4100 => Ok(Haptics::WaveformBuzzContinuous),
            4101 => Ok(Haptics::WaveformRumbleContinuous),
            4102 => Ok(Haptics::WaveformPress),
            4103 => Ok(Haptics::WaveformRelease),
            4104 => Ok(Haptics::WaveformHover),
            4105 => Ok(Haptics::WaveformSuccess),
            4106 => Ok(Haptics::WaveformError),
            4107 => Ok(Haptics::WaveformInkContinuous),
            4108 => Ok(Haptics::WaveformPencilContinuous),
            4109 => Ok(Haptics::WaveformMarkerContinuous),
            4110 => Ok(Haptics::WaveformChiselMarkerContinuous),
            4111 => Ok(Haptics::WaveformBrushContinuous),
            4112 => Ok(Haptics::WaveformEraserContinuous),
            4113 => Ok(Haptics::WaveformSparkleContinuous),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Haptics {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xF`: "Physical Input Device"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::PhysicalInputDevice(PhysicalInputDevice::Normal);
/// let u2 = Usage::new_from_page_and_id(0xF, 0x20).unwrap();
/// let u3 = Usage::from(PhysicalInputDevice::Normal);
/// let u4: Usage = PhysicalInputDevice::Normal.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::PhysicalInputDevice));
/// assert_eq!(0xF, u1.usage_page_value());
/// assert_eq!(0x20, u1.usage_id_value());
/// assert_eq!((0xF << 16) | 0x20, u1.usage_value());
/// assert_eq!("Normal", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum PhysicalInputDevice {
    /// Usage ID `0x1`: "Physical Input Device"
    PhysicalInputDevice = 0x1,
    /// Usage ID `0x20`: "Normal"
    Normal = 0x20,
    /// Usage ID `0x21`: "Set Effect Report"
    SetEffectReport = 0x21,
    /// Usage ID `0x22`: "Effect Parameter Block Index"
    EffectParameterBlockIndex = 0x22,
    /// Usage ID `0x23`: "Parameter Block Offset"
    ParameterBlockOffset = 0x23,
    /// Usage ID `0x24`: "ROM Flag"
    ROMFlag = 0x24,
    /// Usage ID `0x25`: "Effect Type"
    EffectType = 0x25,
    /// Usage ID `0x26`: "ET Constant-Force"
    ETConstantForce = 0x26,
    /// Usage ID `0x27`: "ET Ramp"
    ETRamp = 0x27,
    /// Usage ID `0x28`: "ET Custom-Force"
    ETCustomForce = 0x28,
    /// Usage ID `0x30`: "ET Square"
    ETSquare = 0x30,
    /// Usage ID `0x31`: "ET Sine"
    ETSine = 0x31,
    /// Usage ID `0x32`: "ET Triangle"
    ETTriangle = 0x32,
    /// Usage ID `0x33`: "ET Sawtooth Up"
    ETSawtoothUp = 0x33,
    /// Usage ID `0x34`: "ET Sawtooth Down"
    ETSawtoothDown = 0x34,
    /// Usage ID `0x40`: "ET Spring"
    ETSpring = 0x40,
    /// Usage ID `0x41`: "ET Damper"
    ETDamper = 0x41,
    /// Usage ID `0x42`: "ET Inertia"
    ETInertia = 0x42,
    /// Usage ID `0x43`: "ET Friction"
    ETFriction = 0x43,
    /// Usage ID `0x50`: "Duration"
    Duration = 0x50,
    /// Usage ID `0x51`: "Sample Period"
    SamplePeriod = 0x51,
    /// Usage ID `0x52`: "Gain"
    Gain = 0x52,
    /// Usage ID `0x53`: "Trigger Button"
    TriggerButton = 0x53,
    /// Usage ID `0x54`: "Trigger Repeat Interval"
    TriggerRepeatInterval = 0x54,
    /// Usage ID `0x55`: "Axes Enable"
    AxesEnable = 0x55,
    /// Usage ID `0x56`: "Direction Enable"
    DirectionEnable = 0x56,
    /// Usage ID `0x57`: "Direction"
    Direction = 0x57,
    /// Usage ID `0x58`: "Type Specific Block Offset"
    TypeSpecificBlockOffset = 0x58,
    /// Usage ID `0x59`: "Block Type"
    BlockType = 0x59,
    /// Usage ID `0x5A`: "Set Envelope Report"
    SetEnvelopeReport = 0x5A,
    /// Usage ID `0x5B`: "Attack Level"
    AttackLevel = 0x5B,
    /// Usage ID `0x5C`: "Attack Time"
    AttackTime = 0x5C,
    /// Usage ID `0x5D`: "Fade Level"
    FadeLevel = 0x5D,
    /// Usage ID `0x5E`: "Fade Time"
    FadeTime = 0x5E,
    /// Usage ID `0x5F`: "Set Condition Report"
    SetConditionReport = 0x5F,
    /// Usage ID `0x60`: "Center-Point Offset"
    CenterPointOffset = 0x60,
    /// Usage ID `0x61`: "Positive Coefficient"
    PositiveCoefficient = 0x61,
    /// Usage ID `0x62`: "Negative Coefficient"
    NegativeCoefficient = 0x62,
    /// Usage ID `0x63`: "Positive Saturation"
    PositiveSaturation = 0x63,
    /// Usage ID `0x64`: "Negative Saturation"
    NegativeSaturation = 0x64,
    /// Usage ID `0x65`: "Dead Band"
    DeadBand = 0x65,
    /// Usage ID `0x66`: "Download Force Sample"
    DownloadForceSample = 0x66,
    /// Usage ID `0x67`: "Isoch Custom-Force Enable"
    IsochCustomForceEnable = 0x67,
    /// Usage ID `0x68`: "Custom-Force Data Report"
    CustomForceDataReport = 0x68,
    /// Usage ID `0x69`: "Custom-Force Data"
    CustomForceData = 0x69,
    /// Usage ID `0x6A`: "Custom-Force Vendor Defined Data"
    CustomForceVendorDefinedData = 0x6A,
    /// Usage ID `0x6B`: "Set Custom-Force Report"
    SetCustomForceReport = 0x6B,
    /// Usage ID `0x6C`: "Custom-Force Data Offset"
    CustomForceDataOffset = 0x6C,
    /// Usage ID `0x6D`: "Sample Count"
    SampleCount = 0x6D,
    /// Usage ID `0x6E`: "Set Periodic Report"
    SetPeriodicReport = 0x6E,
    /// Usage ID `0x6F`: "Offset"
    Offset = 0x6F,
    /// Usage ID `0x70`: "Magnitude"
    Magnitude = 0x70,
    /// Usage ID `0x71`: "Phase"
    Phase = 0x71,
    /// Usage ID `0x72`: "Period"
    Period = 0x72,
    /// Usage ID `0x73`: "Set Constant-Force Report"
    SetConstantForceReport = 0x73,
    /// Usage ID `0x74`: "Set Ramp-Force Report"
    SetRampForceReport = 0x74,
    /// Usage ID `0x75`: "Ramp Start"
    RampStart = 0x75,
    /// Usage ID `0x76`: "Ramp End"
    RampEnd = 0x76,
    /// Usage ID `0x77`: "Effect Operation Report"
    EffectOperationReport = 0x77,
    /// Usage ID `0x78`: "Effect Operation"
    EffectOperation = 0x78,
    /// Usage ID `0x79`: "Op Effect Start"
    OpEffectStart = 0x79,
    /// Usage ID `0x7A`: "Op Effect Start Solo"
    OpEffectStartSolo = 0x7A,
    /// Usage ID `0x7B`: "Op Effect Stop"
    OpEffectStop = 0x7B,
    /// Usage ID `0x7C`: "Loop Count"
    LoopCount = 0x7C,
    /// Usage ID `0x7D`: "Device Gain Report"
    DeviceGainReport = 0x7D,
    /// Usage ID `0x7E`: "Device Gain"
    DeviceGain = 0x7E,
    /// Usage ID `0x7F`: "Parameter Block Pools Report"
    ParameterBlockPoolsReport = 0x7F,
    /// Usage ID `0x80`: "RAM Pool Size"
    RAMPoolSize = 0x80,
    /// Usage ID `0x81`: "ROM Pool Size"
    ROMPoolSize = 0x81,
    /// Usage ID `0x82`: "ROM Effect Block Count"
    ROMEffectBlockCount = 0x82,
    /// Usage ID `0x83`: "Simultaneous Effects Max"
    SimultaneousEffectsMax = 0x83,
    /// Usage ID `0x84`: "Pool Alignment"
    PoolAlignment = 0x84,
    /// Usage ID `0x85`: "Parameter Block Move Report"
    ParameterBlockMoveReport = 0x85,
    /// Usage ID `0x86`: "Move Source"
    MoveSource = 0x86,
    /// Usage ID `0x87`: "Move Destination"
    MoveDestination = 0x87,
    /// Usage ID `0x88`: "Move Length"
    MoveLength = 0x88,
    /// Usage ID `0x89`: "Effect Parameter Block Load Report"
    EffectParameterBlockLoadReport = 0x89,
    /// Usage ID `0x8B`: "Effect Parameter Block Load Status"
    EffectParameterBlockLoadStatus = 0x8B,
    /// Usage ID `0x8C`: "Block Load Success"
    BlockLoadSuccess = 0x8C,
    /// Usage ID `0x8D`: "Block Load Full"
    BlockLoadFull = 0x8D,
    /// Usage ID `0x8E`: "Block Load Error"
    BlockLoadError = 0x8E,
    /// Usage ID `0x8F`: "Block Handle"
    BlockHandle = 0x8F,
    /// Usage ID `0x90`: "Effect Parameter Block Free Report"
    EffectParameterBlockFreeReport = 0x90,
    /// Usage ID `0x91`: "Type Specific Block Handle"
    TypeSpecificBlockHandle = 0x91,
    /// Usage ID `0x92`: "PID State Report"
    PIDStateReport = 0x92,
    /// Usage ID `0x94`: "Effect Playing"
    EffectPlaying = 0x94,
    /// Usage ID `0x95`: "PID Device Control Report"
    PIDDeviceControlReport = 0x95,
    /// Usage ID `0x96`: "PID Device Control"
    PIDDeviceControl = 0x96,
    /// Usage ID `0x97`: "DC Enable Actuators"
    DCEnableActuators = 0x97,
    /// Usage ID `0x98`: "DC Disable Actuators"
    DCDisableActuators = 0x98,
    /// Usage ID `0x99`: "DC Stop All Effects"
    DCStopAllEffects = 0x99,
    /// Usage ID `0x9A`: "DC Reset"
    DCReset = 0x9A,
    /// Usage ID `0x9B`: "DC Pause"
    DCPause = 0x9B,
    /// Usage ID `0x9C`: "DC Continue"
    DCContinue = 0x9C,
    /// Usage ID `0x9F`: "Device Paused"
    DevicePaused = 0x9F,
    /// Usage ID `0xA0`: "Actuators Enabled"
    ActuatorsEnabled = 0xA0,
    /// Usage ID `0xA4`: "Safety Switch"
    SafetySwitch = 0xA4,
    /// Usage ID `0xA5`: "Actuator Override Switch"
    ActuatorOverrideSwitch = 0xA5,
    /// Usage ID `0xA6`: "Actuator Power"
    ActuatorPower = 0xA6,
    /// Usage ID `0xA7`: "Start Delay"
    StartDelay = 0xA7,
    /// Usage ID `0xA8`: "Parameter Block Size"
    ParameterBlockSize = 0xA8,
    /// Usage ID `0xA9`: "Device-Managed Pool"
    DeviceManagedPool = 0xA9,
    /// Usage ID `0xAA`: "Shared Parameter Blocks"
    SharedParameterBlocks = 0xAA,
    /// Usage ID `0xAB`: "Create New Effect Parameter Block Report"
    CreateNewEffectParameterBlockReport = 0xAB,
    /// Usage ID `0xAC`: "RAM Pool Available"
    RAMPoolAvailable = 0xAC,
}

impl PhysicalInputDevice {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            PhysicalInputDevice::PhysicalInputDevice => "Physical Input Device",
            PhysicalInputDevice::Normal => "Normal",
            PhysicalInputDevice::SetEffectReport => "Set Effect Report",
            PhysicalInputDevice::EffectParameterBlockIndex => "Effect Parameter Block Index",
            PhysicalInputDevice::ParameterBlockOffset => "Parameter Block Offset",
            PhysicalInputDevice::ROMFlag => "ROM Flag",
            PhysicalInputDevice::EffectType => "Effect Type",
            PhysicalInputDevice::ETConstantForce => "ET Constant-Force",
            PhysicalInputDevice::ETRamp => "ET Ramp",
            PhysicalInputDevice::ETCustomForce => "ET Custom-Force",
            PhysicalInputDevice::ETSquare => "ET Square",
            PhysicalInputDevice::ETSine => "ET Sine",
            PhysicalInputDevice::ETTriangle => "ET Triangle",
            PhysicalInputDevice::ETSawtoothUp => "ET Sawtooth Up",
            PhysicalInputDevice::ETSawtoothDown => "ET Sawtooth Down",
            PhysicalInputDevice::ETSpring => "ET Spring",
            PhysicalInputDevice::ETDamper => "ET Damper",
            PhysicalInputDevice::ETInertia => "ET Inertia",
            PhysicalInputDevice::ETFriction => "ET Friction",
            PhysicalInputDevice::Duration => "Duration",
            PhysicalInputDevice::SamplePeriod => "Sample Period",
            PhysicalInputDevice::Gain => "Gain",
            PhysicalInputDevice::TriggerButton => "Trigger Button",
            PhysicalInputDevice::TriggerRepeatInterval => "Trigger Repeat Interval",
            PhysicalInputDevice::AxesEnable => "Axes Enable",
            PhysicalInputDevice::DirectionEnable => "Direction Enable",
            PhysicalInputDevice::Direction => "Direction",
            PhysicalInputDevice::TypeSpecificBlockOffset => "Type Specific Block Offset",
            PhysicalInputDevice::BlockType => "Block Type",
            PhysicalInputDevice::SetEnvelopeReport => "Set Envelope Report",
            PhysicalInputDevice::AttackLevel => "Attack Level",
            PhysicalInputDevice::AttackTime => "Attack Time",
            PhysicalInputDevice::FadeLevel => "Fade Level",
            PhysicalInputDevice::FadeTime => "Fade Time",
            PhysicalInputDevice::SetConditionReport => "Set Condition Report",
            PhysicalInputDevice::CenterPointOffset => "Center-Point Offset",
            PhysicalInputDevice::PositiveCoefficient => "Positive Coefficient",
            PhysicalInputDevice::NegativeCoefficient => "Negative Coefficient",
            PhysicalInputDevice::PositiveSaturation => "Positive Saturation",
            PhysicalInputDevice::NegativeSaturation => "Negative Saturation",
            PhysicalInputDevice::DeadBand => "Dead Band",
            PhysicalInputDevice::DownloadForceSample => "Download Force Sample",
            PhysicalInputDevice::IsochCustomForceEnable => "Isoch Custom-Force Enable",
            PhysicalInputDevice::CustomForceDataReport => "Custom-Force Data Report",
            PhysicalInputDevice::CustomForceData => "Custom-Force Data",
            PhysicalInputDevice::CustomForceVendorDefinedData => "Custom-Force Vendor Defined Data",
            PhysicalInputDevice::SetCustomForceReport => "Set Custom-Force Report",
            PhysicalInputDevice::CustomForceDataOffset => "Custom-Force Data Offset",
            PhysicalInputDevice::SampleCount => "Sample Count",
            PhysicalInputDevice::SetPeriodicReport => "Set Periodic Report",
            PhysicalInputDevice::Offset => "Offset",
            PhysicalInputDevice::Magnitude => "Magnitude",
            PhysicalInputDevice::Phase => "Phase",
            PhysicalInputDevice::Period => "Period",
            PhysicalInputDevice::SetConstantForceReport => "Set Constant-Force Report",
            PhysicalInputDevice::SetRampForceReport => "Set Ramp-Force Report",
            PhysicalInputDevice::RampStart => "Ramp Start",
            PhysicalInputDevice::RampEnd => "Ramp End",
            PhysicalInputDevice::EffectOperationReport => "Effect Operation Report",
            PhysicalInputDevice::EffectOperation => "Effect Operation",
            PhysicalInputDevice::OpEffectStart => "Op Effect Start",
            PhysicalInputDevice::OpEffectStartSolo => "Op Effect Start Solo",
            PhysicalInputDevice::OpEffectStop => "Op Effect Stop",
            PhysicalInputDevice::LoopCount => "Loop Count",
            PhysicalInputDevice::DeviceGainReport => "Device Gain Report",
            PhysicalInputDevice::DeviceGain => "Device Gain",
            PhysicalInputDevice::ParameterBlockPoolsReport => "Parameter Block Pools Report",
            PhysicalInputDevice::RAMPoolSize => "RAM Pool Size",
            PhysicalInputDevice::ROMPoolSize => "ROM Pool Size",
            PhysicalInputDevice::ROMEffectBlockCount => "ROM Effect Block Count",
            PhysicalInputDevice::SimultaneousEffectsMax => "Simultaneous Effects Max",
            PhysicalInputDevice::PoolAlignment => "Pool Alignment",
            PhysicalInputDevice::ParameterBlockMoveReport => "Parameter Block Move Report",
            PhysicalInputDevice::MoveSource => "Move Source",
            PhysicalInputDevice::MoveDestination => "Move Destination",
            PhysicalInputDevice::MoveLength => "Move Length",
            PhysicalInputDevice::EffectParameterBlockLoadReport => {
                "Effect Parameter Block Load Report"
            }
            PhysicalInputDevice::EffectParameterBlockLoadStatus => {
                "Effect Parameter Block Load Status"
            }
            PhysicalInputDevice::BlockLoadSuccess => "Block Load Success",
            PhysicalInputDevice::BlockLoadFull => "Block Load Full",
            PhysicalInputDevice::BlockLoadError => "Block Load Error",
            PhysicalInputDevice::BlockHandle => "Block Handle",
            PhysicalInputDevice::EffectParameterBlockFreeReport => {
                "Effect Parameter Block Free Report"
            }
            PhysicalInputDevice::TypeSpecificBlockHandle => "Type Specific Block Handle",
            PhysicalInputDevice::PIDStateReport => "PID State Report",
            PhysicalInputDevice::EffectPlaying => "Effect Playing",
            PhysicalInputDevice::PIDDeviceControlReport => "PID Device Control Report",
            PhysicalInputDevice::PIDDeviceControl => "PID Device Control",
            PhysicalInputDevice::DCEnableActuators => "DC Enable Actuators",
            PhysicalInputDevice::DCDisableActuators => "DC Disable Actuators",
            PhysicalInputDevice::DCStopAllEffects => "DC Stop All Effects",
            PhysicalInputDevice::DCReset => "DC Reset",
            PhysicalInputDevice::DCPause => "DC Pause",
            PhysicalInputDevice::DCContinue => "DC Continue",
            PhysicalInputDevice::DevicePaused => "Device Paused",
            PhysicalInputDevice::ActuatorsEnabled => "Actuators Enabled",
            PhysicalInputDevice::SafetySwitch => "Safety Switch",
            PhysicalInputDevice::ActuatorOverrideSwitch => "Actuator Override Switch",
            PhysicalInputDevice::ActuatorPower => "Actuator Power",
            PhysicalInputDevice::StartDelay => "Start Delay",
            PhysicalInputDevice::ParameterBlockSize => "Parameter Block Size",
            PhysicalInputDevice::DeviceManagedPool => "Device-Managed Pool",
            PhysicalInputDevice::SharedParameterBlocks => "Shared Parameter Blocks",
            PhysicalInputDevice::CreateNewEffectParameterBlockReport => {
                "Create New Effect Parameter Block Report"
            }
            PhysicalInputDevice::RAMPoolAvailable => "RAM Pool Available",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for PhysicalInputDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for PhysicalInputDevice {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::PhysicalInputDevice(self)](Usage::PhysicalInputDevice)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for PhysicalInputDevice {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xF` for [PhysicalInputDevice]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::PhysicalInputDevice]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&PhysicalInputDevice> for u16 {
    fn from(physicalinputdevice: &PhysicalInputDevice) -> u16 {
        *physicalinputdevice as u16
    }
}

impl From<PhysicalInputDevice> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [PhysicalInputDevice::usage_page_value()].
    fn from(physicalinputdevice: PhysicalInputDevice) -> u16 {
        u16::from(&physicalinputdevice)
    }
}

impl From<&PhysicalInputDevice> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [PhysicalInputDevice::usage_value()].
    fn from(physicalinputdevice: &PhysicalInputDevice) -> u32 {
        let up = UsagePage::from(physicalinputdevice);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(physicalinputdevice) as u32;
        up | id
    }
}

impl From<&PhysicalInputDevice> for UsagePage {
    /// Always returns [UsagePage::PhysicalInputDevice] and is
    /// identical to [PhysicalInputDevice::usage_page()].
    fn from(_: &PhysicalInputDevice) -> UsagePage {
        UsagePage::PhysicalInputDevice
    }
}

impl From<PhysicalInputDevice> for UsagePage {
    /// Always returns [UsagePage::PhysicalInputDevice] and is
    /// identical to [PhysicalInputDevice::usage_page()].
    fn from(_: PhysicalInputDevice) -> UsagePage {
        UsagePage::PhysicalInputDevice
    }
}

impl From<&PhysicalInputDevice> for Usage {
    fn from(physicalinputdevice: &PhysicalInputDevice) -> Usage {
        Usage::try_from(u32::from(physicalinputdevice)).unwrap()
    }
}

impl From<PhysicalInputDevice> for Usage {
    fn from(physicalinputdevice: PhysicalInputDevice) -> Usage {
        Usage::from(&physicalinputdevice)
    }
}

impl TryFrom<u16> for PhysicalInputDevice {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<PhysicalInputDevice> {
        match usage_id {
            1 => Ok(PhysicalInputDevice::PhysicalInputDevice),
            32 => Ok(PhysicalInputDevice::Normal),
            33 => Ok(PhysicalInputDevice::SetEffectReport),
            34 => Ok(PhysicalInputDevice::EffectParameterBlockIndex),
            35 => Ok(PhysicalInputDevice::ParameterBlockOffset),
            36 => Ok(PhysicalInputDevice::ROMFlag),
            37 => Ok(PhysicalInputDevice::EffectType),
            38 => Ok(PhysicalInputDevice::ETConstantForce),
            39 => Ok(PhysicalInputDevice::ETRamp),
            40 => Ok(PhysicalInputDevice::ETCustomForce),
            48 => Ok(PhysicalInputDevice::ETSquare),
            49 => Ok(PhysicalInputDevice::ETSine),
            50 => Ok(PhysicalInputDevice::ETTriangle),
            51 => Ok(PhysicalInputDevice::ETSawtoothUp),
            52 => Ok(PhysicalInputDevice::ETSawtoothDown),
            64 => Ok(PhysicalInputDevice::ETSpring),
            65 => Ok(PhysicalInputDevice::ETDamper),
            66 => Ok(PhysicalInputDevice::ETInertia),
            67 => Ok(PhysicalInputDevice::ETFriction),
            80 => Ok(PhysicalInputDevice::Duration),
            81 => Ok(PhysicalInputDevice::SamplePeriod),
            82 => Ok(PhysicalInputDevice::Gain),
            83 => Ok(PhysicalInputDevice::TriggerButton),
            84 => Ok(PhysicalInputDevice::TriggerRepeatInterval),
            85 => Ok(PhysicalInputDevice::AxesEnable),
            86 => Ok(PhysicalInputDevice::DirectionEnable),
            87 => Ok(PhysicalInputDevice::Direction),
            88 => Ok(PhysicalInputDevice::TypeSpecificBlockOffset),
            89 => Ok(PhysicalInputDevice::BlockType),
            90 => Ok(PhysicalInputDevice::SetEnvelopeReport),
            91 => Ok(PhysicalInputDevice::AttackLevel),
            92 => Ok(PhysicalInputDevice::AttackTime),
            93 => Ok(PhysicalInputDevice::FadeLevel),
            94 => Ok(PhysicalInputDevice::FadeTime),
            95 => Ok(PhysicalInputDevice::SetConditionReport),
            96 => Ok(PhysicalInputDevice::CenterPointOffset),
            97 => Ok(PhysicalInputDevice::PositiveCoefficient),
            98 => Ok(PhysicalInputDevice::NegativeCoefficient),
            99 => Ok(PhysicalInputDevice::PositiveSaturation),
            100 => Ok(PhysicalInputDevice::NegativeSaturation),
            101 => Ok(PhysicalInputDevice::DeadBand),
            102 => Ok(PhysicalInputDevice::DownloadForceSample),
            103 => Ok(PhysicalInputDevice::IsochCustomForceEnable),
            104 => Ok(PhysicalInputDevice::CustomForceDataReport),
            105 => Ok(PhysicalInputDevice::CustomForceData),
            106 => Ok(PhysicalInputDevice::CustomForceVendorDefinedData),
            107 => Ok(PhysicalInputDevice::SetCustomForceReport),
            108 => Ok(PhysicalInputDevice::CustomForceDataOffset),
            109 => Ok(PhysicalInputDevice::SampleCount),
            110 => Ok(PhysicalInputDevice::SetPeriodicReport),
            111 => Ok(PhysicalInputDevice::Offset),
            112 => Ok(PhysicalInputDevice::Magnitude),
            113 => Ok(PhysicalInputDevice::Phase),
            114 => Ok(PhysicalInputDevice::Period),
            115 => Ok(PhysicalInputDevice::SetConstantForceReport),
            116 => Ok(PhysicalInputDevice::SetRampForceReport),
            117 => Ok(PhysicalInputDevice::RampStart),
            118 => Ok(PhysicalInputDevice::RampEnd),
            119 => Ok(PhysicalInputDevice::EffectOperationReport),
            120 => Ok(PhysicalInputDevice::EffectOperation),
            121 => Ok(PhysicalInputDevice::OpEffectStart),
            122 => Ok(PhysicalInputDevice::OpEffectStartSolo),
            123 => Ok(PhysicalInputDevice::OpEffectStop),
            124 => Ok(PhysicalInputDevice::LoopCount),
            125 => Ok(PhysicalInputDevice::DeviceGainReport),
            126 => Ok(PhysicalInputDevice::DeviceGain),
            127 => Ok(PhysicalInputDevice::ParameterBlockPoolsReport),
            128 => Ok(PhysicalInputDevice::RAMPoolSize),
            129 => Ok(PhysicalInputDevice::ROMPoolSize),
            130 => Ok(PhysicalInputDevice::ROMEffectBlockCount),
            131 => Ok(PhysicalInputDevice::SimultaneousEffectsMax),
            132 => Ok(PhysicalInputDevice::PoolAlignment),
            133 => Ok(PhysicalInputDevice::ParameterBlockMoveReport),
            134 => Ok(PhysicalInputDevice::MoveSource),
            135 => Ok(PhysicalInputDevice::MoveDestination),
            136 => Ok(PhysicalInputDevice::MoveLength),
            137 => Ok(PhysicalInputDevice::EffectParameterBlockLoadReport),
            139 => Ok(PhysicalInputDevice::EffectParameterBlockLoadStatus),
            140 => Ok(PhysicalInputDevice::BlockLoadSuccess),
            141 => Ok(PhysicalInputDevice::BlockLoadFull),
            142 => Ok(PhysicalInputDevice::BlockLoadError),
            143 => Ok(PhysicalInputDevice::BlockHandle),
            144 => Ok(PhysicalInputDevice::EffectParameterBlockFreeReport),
            145 => Ok(PhysicalInputDevice::TypeSpecificBlockHandle),
            146 => Ok(PhysicalInputDevice::PIDStateReport),
            148 => Ok(PhysicalInputDevice::EffectPlaying),
            149 => Ok(PhysicalInputDevice::PIDDeviceControlReport),
            150 => Ok(PhysicalInputDevice::PIDDeviceControl),
            151 => Ok(PhysicalInputDevice::DCEnableActuators),
            152 => Ok(PhysicalInputDevice::DCDisableActuators),
            153 => Ok(PhysicalInputDevice::DCStopAllEffects),
            154 => Ok(PhysicalInputDevice::DCReset),
            155 => Ok(PhysicalInputDevice::DCPause),
            156 => Ok(PhysicalInputDevice::DCContinue),
            159 => Ok(PhysicalInputDevice::DevicePaused),
            160 => Ok(PhysicalInputDevice::ActuatorsEnabled),
            164 => Ok(PhysicalInputDevice::SafetySwitch),
            165 => Ok(PhysicalInputDevice::ActuatorOverrideSwitch),
            166 => Ok(PhysicalInputDevice::ActuatorPower),
            167 => Ok(PhysicalInputDevice::StartDelay),
            168 => Ok(PhysicalInputDevice::ParameterBlockSize),
            169 => Ok(PhysicalInputDevice::DeviceManagedPool),
            170 => Ok(PhysicalInputDevice::SharedParameterBlocks),
            171 => Ok(PhysicalInputDevice::CreateNewEffectParameterBlockReport),
            172 => Ok(PhysicalInputDevice::RAMPoolAvailable),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for PhysicalInputDevice {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x10`: "Unicode"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
///
/// This Usage Page is generated, not defined, any Usage IDs in this Usage
/// Page are simply the codepoint number.
///
/// ```
/// # use hut::*;
/// let u1 = Usage::Unicode(Unicode::Unicode(3));
/// let u2 = Usage::new_from_page_and_id(0x10, 3).unwrap();
/// let u3 = Usage::from(Unicode::Unicode(3));
/// let u4: Usage = Unicode::Unicode(3).into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Unicode));
/// assert_eq!(0x10, u1.usage_page_value());
/// assert_eq!(3, u1.usage_id_value());
/// assert_eq!((0x10 << 16) | 3, u1.usage_value());
/// assert_eq!("codepoint 3", u1.name());
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Unicode {
    Unicode(u16),
}

impl Unicode {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Unicode::Unicode(codepoint) => format!("codepoint {codepoint}"),
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Unicode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Unicode {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Unicode(self)](Usage::Unicode)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Unicode {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x10` for [Unicode]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Unicode]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Unicode> for u16 {
    fn from(unicode: &Unicode) -> u16 {
        match *unicode {
            Unicode::Unicode(codepoint) => codepoint,
        }
    }
}

impl From<Unicode> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Unicode::usage_page_value()].
    fn from(unicode: Unicode) -> u16 {
        u16::from(&unicode)
    }
}

impl From<&Unicode> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Unicode::usage_value()].
    fn from(unicode: &Unicode) -> u32 {
        let up = UsagePage::from(unicode);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(unicode) as u32;
        up | id
    }
}

impl From<&Unicode> for UsagePage {
    /// Always returns [UsagePage::Unicode] and is
    /// identical to [Unicode::usage_page()].
    fn from(_: &Unicode) -> UsagePage {
        UsagePage::Unicode
    }
}

impl From<Unicode> for UsagePage {
    /// Always returns [UsagePage::Unicode] and is
    /// identical to [Unicode::usage_page()].
    fn from(_: Unicode) -> UsagePage {
        UsagePage::Unicode
    }
}

impl From<&Unicode> for Usage {
    fn from(unicode: &Unicode) -> Usage {
        Usage::try_from(u32::from(unicode)).unwrap()
    }
}

impl From<Unicode> for Usage {
    fn from(unicode: Unicode) -> Usage {
        Usage::from(&unicode)
    }
}

impl TryFrom<u16> for Unicode {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Unicode> {
        match usage_id {
            n => Ok(Unicode::Unicode(n)),
        }
    }
}

impl BitOr<u16> for Unicode {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x11`: "SoC"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::SoC(SoC::FirmwareTransfer);
/// let u2 = Usage::new_from_page_and_id(0x11, 0x2).unwrap();
/// let u3 = Usage::from(SoC::FirmwareTransfer);
/// let u4: Usage = SoC::FirmwareTransfer.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::SoC));
/// assert_eq!(0x11, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x11 << 16) | 0x2, u1.usage_value());
/// assert_eq!("FirmwareTransfer", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum SoC {
    /// Usage ID `0x1`: "SocControl"
    SocControl = 0x1,
    /// Usage ID `0x2`: "FirmwareTransfer"
    FirmwareTransfer = 0x2,
    /// Usage ID `0x3`: "FirmwareFileId"
    FirmwareFileId = 0x3,
    /// Usage ID `0x4`: "FileOffsetInBytes"
    FileOffsetInBytes = 0x4,
    /// Usage ID `0x5`: "FileTransferSizeMaxInBytes"
    FileTransferSizeMaxInBytes = 0x5,
    /// Usage ID `0x6`: "FilePayload"
    FilePayload = 0x6,
    /// Usage ID `0x7`: "FilePayloadSizeInBytes"
    FilePayloadSizeInBytes = 0x7,
    /// Usage ID `0x8`: "FilePayloadContainsLastBytes"
    FilePayloadContainsLastBytes = 0x8,
    /// Usage ID `0x9`: "FileTransferStop"
    FileTransferStop = 0x9,
    /// Usage ID `0xA`: "FileTransferTillEnd"
    FileTransferTillEnd = 0xA,
}

impl SoC {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            SoC::SocControl => "SocControl",
            SoC::FirmwareTransfer => "FirmwareTransfer",
            SoC::FirmwareFileId => "FirmwareFileId",
            SoC::FileOffsetInBytes => "FileOffsetInBytes",
            SoC::FileTransferSizeMaxInBytes => "FileTransferSizeMaxInBytes",
            SoC::FilePayload => "FilePayload",
            SoC::FilePayloadSizeInBytes => "FilePayloadSizeInBytes",
            SoC::FilePayloadContainsLastBytes => "FilePayloadContainsLastBytes",
            SoC::FileTransferStop => "FileTransferStop",
            SoC::FileTransferTillEnd => "FileTransferTillEnd",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for SoC {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for SoC {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::SoC(self)](Usage::SoC)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for SoC {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x11` for [SoC]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::SoC]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&SoC> for u16 {
    fn from(soc: &SoC) -> u16 {
        *soc as u16
    }
}

impl From<SoC> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [SoC::usage_page_value()].
    fn from(soc: SoC) -> u16 {
        u16::from(&soc)
    }
}

impl From<&SoC> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [SoC::usage_value()].
    fn from(soc: &SoC) -> u32 {
        let up = UsagePage::from(soc);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(soc) as u32;
        up | id
    }
}

impl From<&SoC> for UsagePage {
    /// Always returns [UsagePage::SoC] and is
    /// identical to [SoC::usage_page()].
    fn from(_: &SoC) -> UsagePage {
        UsagePage::SoC
    }
}

impl From<SoC> for UsagePage {
    /// Always returns [UsagePage::SoC] and is
    /// identical to [SoC::usage_page()].
    fn from(_: SoC) -> UsagePage {
        UsagePage::SoC
    }
}

impl From<&SoC> for Usage {
    fn from(soc: &SoC) -> Usage {
        Usage::try_from(u32::from(soc)).unwrap()
    }
}

impl From<SoC> for Usage {
    fn from(soc: SoC) -> Usage {
        Usage::from(&soc)
    }
}

impl TryFrom<u16> for SoC {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<SoC> {
        match usage_id {
            1 => Ok(SoC::SocControl),
            2 => Ok(SoC::FirmwareTransfer),
            3 => Ok(SoC::FirmwareFileId),
            4 => Ok(SoC::FileOffsetInBytes),
            5 => Ok(SoC::FileTransferSizeMaxInBytes),
            6 => Ok(SoC::FilePayload),
            7 => Ok(SoC::FilePayloadSizeInBytes),
            8 => Ok(SoC::FilePayloadContainsLastBytes),
            9 => Ok(SoC::FileTransferStop),
            10 => Ok(SoC::FileTransferTillEnd),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for SoC {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x12`: "Eye and Head Trackers"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::EyeandHeadTrackers(EyeandHeadTrackers::HeadTracker);
/// let u2 = Usage::new_from_page_and_id(0x12, 0x2).unwrap();
/// let u3 = Usage::from(EyeandHeadTrackers::HeadTracker);
/// let u4: Usage = EyeandHeadTrackers::HeadTracker.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::EyeandHeadTrackers));
/// assert_eq!(0x12, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x12 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Head Tracker", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum EyeandHeadTrackers {
    /// Usage ID `0x1`: "Eye Tracker"
    EyeTracker = 0x1,
    /// Usage ID `0x2`: "Head Tracker"
    HeadTracker = 0x2,
    /// Usage ID `0x10`: "Tracking Data"
    TrackingData = 0x10,
    /// Usage ID `0x11`: "Capabilities"
    Capabilities = 0x11,
    /// Usage ID `0x12`: "Configuration"
    Configuration = 0x12,
    /// Usage ID `0x13`: "Status"
    Status = 0x13,
    /// Usage ID `0x14`: "Control"
    Control = 0x14,
    /// Usage ID `0x20`: "Sensor Timestamp"
    SensorTimestamp = 0x20,
    /// Usage ID `0x21`: "Position X"
    PositionX = 0x21,
    /// Usage ID `0x22`: "Position Y"
    PositionY = 0x22,
    /// Usage ID `0x23`: "Position Z"
    PositionZ = 0x23,
    /// Usage ID `0x24`: "Gaze Point"
    GazePoint = 0x24,
    /// Usage ID `0x25`: "Left Eye Position"
    LeftEyePosition = 0x25,
    /// Usage ID `0x26`: "Right Eye Position"
    RightEyePosition = 0x26,
    /// Usage ID `0x27`: "Head Position"
    HeadPosition = 0x27,
    /// Usage ID `0x28`: "Head Direction Point"
    HeadDirectionPoint = 0x28,
    /// Usage ID `0x29`: "Rotation about X axis"
    RotationaboutXaxis = 0x29,
    /// Usage ID `0x2A`: "Rotation about Y axis"
    RotationaboutYaxis = 0x2A,
    /// Usage ID `0x2B`: "Rotation about Z axis"
    RotationaboutZaxis = 0x2B,
    /// Usage ID `0x100`: "Tracker Quality"
    TrackerQuality = 0x100,
    /// Usage ID `0x101`: "Minimum Tracking Distance"
    MinimumTrackingDistance = 0x101,
    /// Usage ID `0x102`: "Optimum Tracking Distance"
    OptimumTrackingDistance = 0x102,
    /// Usage ID `0x103`: "Maximum Tracking Distance"
    MaximumTrackingDistance = 0x103,
    /// Usage ID `0x104`: "Maximum Screen Plane Width"
    MaximumScreenPlaneWidth = 0x104,
    /// Usage ID `0x105`: "Maximum Screen Plane Height"
    MaximumScreenPlaneHeight = 0x105,
    /// Usage ID `0x200`: "Display Manufacturer ID"
    DisplayManufacturerID = 0x200,
    /// Usage ID `0x201`: "Display Product ID"
    DisplayProductID = 0x201,
    /// Usage ID `0x202`: "Display Serial Number"
    DisplaySerialNumber = 0x202,
    /// Usage ID `0x203`: "Display Manufacturer Date"
    DisplayManufacturerDate = 0x203,
    /// Usage ID `0x204`: "Calibrated Screen Width"
    CalibratedScreenWidth = 0x204,
    /// Usage ID `0x205`: "Calibrated Screen Height"
    CalibratedScreenHeight = 0x205,
    /// Usage ID `0x300`: "Sampling Frequency"
    SamplingFrequency = 0x300,
    /// Usage ID `0x301`: "Configuration Status"
    ConfigurationStatus = 0x301,
    /// Usage ID `0x400`: "Device Mode Request"
    DeviceModeRequest = 0x400,
}

impl EyeandHeadTrackers {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            EyeandHeadTrackers::EyeTracker => "Eye Tracker",
            EyeandHeadTrackers::HeadTracker => "Head Tracker",
            EyeandHeadTrackers::TrackingData => "Tracking Data",
            EyeandHeadTrackers::Capabilities => "Capabilities",
            EyeandHeadTrackers::Configuration => "Configuration",
            EyeandHeadTrackers::Status => "Status",
            EyeandHeadTrackers::Control => "Control",
            EyeandHeadTrackers::SensorTimestamp => "Sensor Timestamp",
            EyeandHeadTrackers::PositionX => "Position X",
            EyeandHeadTrackers::PositionY => "Position Y",
            EyeandHeadTrackers::PositionZ => "Position Z",
            EyeandHeadTrackers::GazePoint => "Gaze Point",
            EyeandHeadTrackers::LeftEyePosition => "Left Eye Position",
            EyeandHeadTrackers::RightEyePosition => "Right Eye Position",
            EyeandHeadTrackers::HeadPosition => "Head Position",
            EyeandHeadTrackers::HeadDirectionPoint => "Head Direction Point",
            EyeandHeadTrackers::RotationaboutXaxis => "Rotation about X axis",
            EyeandHeadTrackers::RotationaboutYaxis => "Rotation about Y axis",
            EyeandHeadTrackers::RotationaboutZaxis => "Rotation about Z axis",
            EyeandHeadTrackers::TrackerQuality => "Tracker Quality",
            EyeandHeadTrackers::MinimumTrackingDistance => "Minimum Tracking Distance",
            EyeandHeadTrackers::OptimumTrackingDistance => "Optimum Tracking Distance",
            EyeandHeadTrackers::MaximumTrackingDistance => "Maximum Tracking Distance",
            EyeandHeadTrackers::MaximumScreenPlaneWidth => "Maximum Screen Plane Width",
            EyeandHeadTrackers::MaximumScreenPlaneHeight => "Maximum Screen Plane Height",
            EyeandHeadTrackers::DisplayManufacturerID => "Display Manufacturer ID",
            EyeandHeadTrackers::DisplayProductID => "Display Product ID",
            EyeandHeadTrackers::DisplaySerialNumber => "Display Serial Number",
            EyeandHeadTrackers::DisplayManufacturerDate => "Display Manufacturer Date",
            EyeandHeadTrackers::CalibratedScreenWidth => "Calibrated Screen Width",
            EyeandHeadTrackers::CalibratedScreenHeight => "Calibrated Screen Height",
            EyeandHeadTrackers::SamplingFrequency => "Sampling Frequency",
            EyeandHeadTrackers::ConfigurationStatus => "Configuration Status",
            EyeandHeadTrackers::DeviceModeRequest => "Device Mode Request",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for EyeandHeadTrackers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for EyeandHeadTrackers {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::EyeandHeadTrackers(self)](Usage::EyeandHeadTrackers)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for EyeandHeadTrackers {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x12` for [EyeandHeadTrackers]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::EyeandHeadTrackers]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&EyeandHeadTrackers> for u16 {
    fn from(eyeandheadtrackers: &EyeandHeadTrackers) -> u16 {
        *eyeandheadtrackers as u16
    }
}

impl From<EyeandHeadTrackers> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [EyeandHeadTrackers::usage_page_value()].
    fn from(eyeandheadtrackers: EyeandHeadTrackers) -> u16 {
        u16::from(&eyeandheadtrackers)
    }
}

impl From<&EyeandHeadTrackers> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [EyeandHeadTrackers::usage_value()].
    fn from(eyeandheadtrackers: &EyeandHeadTrackers) -> u32 {
        let up = UsagePage::from(eyeandheadtrackers);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(eyeandheadtrackers) as u32;
        up | id
    }
}

impl From<&EyeandHeadTrackers> for UsagePage {
    /// Always returns [UsagePage::EyeandHeadTrackers] and is
    /// identical to [EyeandHeadTrackers::usage_page()].
    fn from(_: &EyeandHeadTrackers) -> UsagePage {
        UsagePage::EyeandHeadTrackers
    }
}

impl From<EyeandHeadTrackers> for UsagePage {
    /// Always returns [UsagePage::EyeandHeadTrackers] and is
    /// identical to [EyeandHeadTrackers::usage_page()].
    fn from(_: EyeandHeadTrackers) -> UsagePage {
        UsagePage::EyeandHeadTrackers
    }
}

impl From<&EyeandHeadTrackers> for Usage {
    fn from(eyeandheadtrackers: &EyeandHeadTrackers) -> Usage {
        Usage::try_from(u32::from(eyeandheadtrackers)).unwrap()
    }
}

impl From<EyeandHeadTrackers> for Usage {
    fn from(eyeandheadtrackers: EyeandHeadTrackers) -> Usage {
        Usage::from(&eyeandheadtrackers)
    }
}

impl TryFrom<u16> for EyeandHeadTrackers {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<EyeandHeadTrackers> {
        match usage_id {
            1 => Ok(EyeandHeadTrackers::EyeTracker),
            2 => Ok(EyeandHeadTrackers::HeadTracker),
            16 => Ok(EyeandHeadTrackers::TrackingData),
            17 => Ok(EyeandHeadTrackers::Capabilities),
            18 => Ok(EyeandHeadTrackers::Configuration),
            19 => Ok(EyeandHeadTrackers::Status),
            20 => Ok(EyeandHeadTrackers::Control),
            32 => Ok(EyeandHeadTrackers::SensorTimestamp),
            33 => Ok(EyeandHeadTrackers::PositionX),
            34 => Ok(EyeandHeadTrackers::PositionY),
            35 => Ok(EyeandHeadTrackers::PositionZ),
            36 => Ok(EyeandHeadTrackers::GazePoint),
            37 => Ok(EyeandHeadTrackers::LeftEyePosition),
            38 => Ok(EyeandHeadTrackers::RightEyePosition),
            39 => Ok(EyeandHeadTrackers::HeadPosition),
            40 => Ok(EyeandHeadTrackers::HeadDirectionPoint),
            41 => Ok(EyeandHeadTrackers::RotationaboutXaxis),
            42 => Ok(EyeandHeadTrackers::RotationaboutYaxis),
            43 => Ok(EyeandHeadTrackers::RotationaboutZaxis),
            256 => Ok(EyeandHeadTrackers::TrackerQuality),
            257 => Ok(EyeandHeadTrackers::MinimumTrackingDistance),
            258 => Ok(EyeandHeadTrackers::OptimumTrackingDistance),
            259 => Ok(EyeandHeadTrackers::MaximumTrackingDistance),
            260 => Ok(EyeandHeadTrackers::MaximumScreenPlaneWidth),
            261 => Ok(EyeandHeadTrackers::MaximumScreenPlaneHeight),
            512 => Ok(EyeandHeadTrackers::DisplayManufacturerID),
            513 => Ok(EyeandHeadTrackers::DisplayProductID),
            514 => Ok(EyeandHeadTrackers::DisplaySerialNumber),
            515 => Ok(EyeandHeadTrackers::DisplayManufacturerDate),
            516 => Ok(EyeandHeadTrackers::CalibratedScreenWidth),
            517 => Ok(EyeandHeadTrackers::CalibratedScreenHeight),
            768 => Ok(EyeandHeadTrackers::SamplingFrequency),
            769 => Ok(EyeandHeadTrackers::ConfigurationStatus),
            1024 => Ok(EyeandHeadTrackers::DeviceModeRequest),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for EyeandHeadTrackers {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x14`: "Auxiliary Display"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::AuxiliaryDisplay(AuxiliaryDisplay::AuxiliaryDisplay);
/// let u2 = Usage::new_from_page_and_id(0x14, 0x2).unwrap();
/// let u3 = Usage::from(AuxiliaryDisplay::AuxiliaryDisplay);
/// let u4: Usage = AuxiliaryDisplay::AuxiliaryDisplay.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::AuxiliaryDisplay));
/// assert_eq!(0x14, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x14 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Auxiliary Display", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum AuxiliaryDisplay {
    /// Usage ID `0x1`: "Alphanumeric Display"
    AlphanumericDisplay = 0x1,
    /// Usage ID `0x2`: "Auxiliary Display"
    AuxiliaryDisplay = 0x2,
    /// Usage ID `0x20`: "Display Attributes Report"
    DisplayAttributesReport = 0x20,
    /// Usage ID `0x21`: "ASCII Character Set"
    ASCIICharacterSet = 0x21,
    /// Usage ID `0x22`: "Data Read Back"
    DataReadBack = 0x22,
    /// Usage ID `0x23`: "Font Read Back"
    FontReadBack = 0x23,
    /// Usage ID `0x24`: "Display Control Report"
    DisplayControlReport = 0x24,
    /// Usage ID `0x25`: "Clear Display"
    ClearDisplay = 0x25,
    /// Usage ID `0x26`: "Display Enable"
    DisplayEnable = 0x26,
    /// Usage ID `0x27`: "Screen Saver Delay"
    ScreenSaverDelay = 0x27,
    /// Usage ID `0x28`: "Screen Saver Enable"
    ScreenSaverEnable = 0x28,
    /// Usage ID `0x29`: "Vertical Scroll"
    VerticalScroll = 0x29,
    /// Usage ID `0x2A`: "Horizontal Scroll"
    HorizontalScroll = 0x2A,
    /// Usage ID `0x2B`: "Character Report"
    CharacterReport = 0x2B,
    /// Usage ID `0x2C`: "Display Data"
    DisplayData = 0x2C,
    /// Usage ID `0x2D`: "Display Status"
    DisplayStatus = 0x2D,
    /// Usage ID `0x2E`: "Stat Not Ready"
    StatNotReady = 0x2E,
    /// Usage ID `0x2F`: "Stat Ready"
    StatReady = 0x2F,
    /// Usage ID `0x30`: "Err Not a loadable character"
    ErrNotaloadablecharacter = 0x30,
    /// Usage ID `0x31`: "Err Font data cannot be read"
    ErrFontdatacannotberead = 0x31,
    /// Usage ID `0x32`: "Cursor Position Report"
    CursorPositionReport = 0x32,
    /// Usage ID `0x33`: "Row"
    Row = 0x33,
    /// Usage ID `0x34`: "Column"
    Column = 0x34,
    /// Usage ID `0x35`: "Rows"
    Rows = 0x35,
    /// Usage ID `0x36`: "Columns"
    Columns = 0x36,
    /// Usage ID `0x37`: "Cursor Pixel Positioning"
    CursorPixelPositioning = 0x37,
    /// Usage ID `0x38`: "Cursor Mode"
    CursorMode = 0x38,
    /// Usage ID `0x39`: "Cursor Enable"
    CursorEnable = 0x39,
    /// Usage ID `0x3A`: "Cursor Blink"
    CursorBlink = 0x3A,
    /// Usage ID `0x3B`: "Font Report"
    FontReport = 0x3B,
    /// Usage ID `0x3C`: "Font Data"
    FontData = 0x3C,
    /// Usage ID `0x3D`: "Character Width"
    CharacterWidth = 0x3D,
    /// Usage ID `0x3E`: "Character Height"
    CharacterHeight = 0x3E,
    /// Usage ID `0x3F`: "Character Spacing Horizontal"
    CharacterSpacingHorizontal = 0x3F,
    /// Usage ID `0x40`: "Character Spacing Vertical"
    CharacterSpacingVertical = 0x40,
    /// Usage ID `0x41`: "Unicode Character Set"
    UnicodeCharacterSet = 0x41,
    /// Usage ID `0x42`: "Font 7-Segment"
    Font7Segment = 0x42,
    /// Usage ID `0x43`: "7-Segment Direct Map"
    SevenSegmentDirectMap = 0x43,
    /// Usage ID `0x44`: "Font 14-Segment"
    Font14Segment = 0x44,
    /// Usage ID `0x45`: "14-Segment Direct Map"
    One4SegmentDirectMap = 0x45,
    /// Usage ID `0x46`: "Display Brightness"
    DisplayBrightness = 0x46,
    /// Usage ID `0x47`: "Display Contrast"
    DisplayContrast = 0x47,
    /// Usage ID `0x48`: "Character Attribute"
    CharacterAttribute = 0x48,
    /// Usage ID `0x49`: "Attribute Readback"
    AttributeReadback = 0x49,
    /// Usage ID `0x4A`: "Attribute Data"
    AttributeData = 0x4A,
    /// Usage ID `0x4B`: "Char Attr Enhance"
    CharAttrEnhance = 0x4B,
    /// Usage ID `0x4C`: "Char Attr Underline"
    CharAttrUnderline = 0x4C,
    /// Usage ID `0x4D`: "Char Attr Blink"
    CharAttrBlink = 0x4D,
    /// Usage ID `0x80`: "Bitmap Size X"
    BitmapSizeX = 0x80,
    /// Usage ID `0x81`: "Bitmap Size Y"
    BitmapSizeY = 0x81,
    /// Usage ID `0x82`: "Max Blit Size"
    MaxBlitSize = 0x82,
    /// Usage ID `0x83`: "Bit Depth Format"
    BitDepthFormat = 0x83,
    /// Usage ID `0x84`: "Display Orientation"
    DisplayOrientation = 0x84,
    /// Usage ID `0x85`: "Palette Report"
    PaletteReport = 0x85,
    /// Usage ID `0x86`: "Palette Data Size"
    PaletteDataSize = 0x86,
    /// Usage ID `0x87`: "Palette Data Offset"
    PaletteDataOffset = 0x87,
    /// Usage ID `0x88`: "Palette Data"
    PaletteData = 0x88,
    /// Usage ID `0x8A`: "Blit Report"
    BlitReport = 0x8A,
    /// Usage ID `0x8B`: "Blit Rectangle X1"
    BlitRectangleX1 = 0x8B,
    /// Usage ID `0x8C`: "Blit Rectangle Y1"
    BlitRectangleY1 = 0x8C,
    /// Usage ID `0x8D`: "Blit Rectangle X2"
    BlitRectangleX2 = 0x8D,
    /// Usage ID `0x8E`: "Blit Rectangle Y2"
    BlitRectangleY2 = 0x8E,
    /// Usage ID `0x8F`: "Blit Data"
    BlitData = 0x8F,
    /// Usage ID `0x90`: "Soft Button"
    SoftButton = 0x90,
    /// Usage ID `0x91`: "Soft Button ID"
    SoftButtonID = 0x91,
    /// Usage ID `0x92`: "Soft Button Side"
    SoftButtonSide = 0x92,
    /// Usage ID `0x93`: "Soft Button Offset 1"
    SoftButtonOffset1 = 0x93,
    /// Usage ID `0x94`: "Soft Button Offset 2"
    SoftButtonOffset2 = 0x94,
    /// Usage ID `0x95`: "Soft Button Report"
    SoftButtonReport = 0x95,
    /// Usage ID `0xC2`: "Soft Keys"
    SoftKeys = 0xC2,
    /// Usage ID `0xCC`: "Display Data Extensions"
    DisplayDataExtensions = 0xCC,
    /// Usage ID `0xCF`: "Character Mapping"
    CharacterMapping = 0xCF,
    /// Usage ID `0xDD`: "Unicode Equivalent"
    UnicodeEquivalent = 0xDD,
    /// Usage ID `0xDF`: "Character Page Mapping"
    CharacterPageMapping = 0xDF,
    /// Usage ID `0xFF`: "Request Report"
    RequestReport = 0xFF,
}

impl AuxiliaryDisplay {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            AuxiliaryDisplay::AlphanumericDisplay => "Alphanumeric Display",
            AuxiliaryDisplay::AuxiliaryDisplay => "Auxiliary Display",
            AuxiliaryDisplay::DisplayAttributesReport => "Display Attributes Report",
            AuxiliaryDisplay::ASCIICharacterSet => "ASCII Character Set",
            AuxiliaryDisplay::DataReadBack => "Data Read Back",
            AuxiliaryDisplay::FontReadBack => "Font Read Back",
            AuxiliaryDisplay::DisplayControlReport => "Display Control Report",
            AuxiliaryDisplay::ClearDisplay => "Clear Display",
            AuxiliaryDisplay::DisplayEnable => "Display Enable",
            AuxiliaryDisplay::ScreenSaverDelay => "Screen Saver Delay",
            AuxiliaryDisplay::ScreenSaverEnable => "Screen Saver Enable",
            AuxiliaryDisplay::VerticalScroll => "Vertical Scroll",
            AuxiliaryDisplay::HorizontalScroll => "Horizontal Scroll",
            AuxiliaryDisplay::CharacterReport => "Character Report",
            AuxiliaryDisplay::DisplayData => "Display Data",
            AuxiliaryDisplay::DisplayStatus => "Display Status",
            AuxiliaryDisplay::StatNotReady => "Stat Not Ready",
            AuxiliaryDisplay::StatReady => "Stat Ready",
            AuxiliaryDisplay::ErrNotaloadablecharacter => "Err Not a loadable character",
            AuxiliaryDisplay::ErrFontdatacannotberead => "Err Font data cannot be read",
            AuxiliaryDisplay::CursorPositionReport => "Cursor Position Report",
            AuxiliaryDisplay::Row => "Row",
            AuxiliaryDisplay::Column => "Column",
            AuxiliaryDisplay::Rows => "Rows",
            AuxiliaryDisplay::Columns => "Columns",
            AuxiliaryDisplay::CursorPixelPositioning => "Cursor Pixel Positioning",
            AuxiliaryDisplay::CursorMode => "Cursor Mode",
            AuxiliaryDisplay::CursorEnable => "Cursor Enable",
            AuxiliaryDisplay::CursorBlink => "Cursor Blink",
            AuxiliaryDisplay::FontReport => "Font Report",
            AuxiliaryDisplay::FontData => "Font Data",
            AuxiliaryDisplay::CharacterWidth => "Character Width",
            AuxiliaryDisplay::CharacterHeight => "Character Height",
            AuxiliaryDisplay::CharacterSpacingHorizontal => "Character Spacing Horizontal",
            AuxiliaryDisplay::CharacterSpacingVertical => "Character Spacing Vertical",
            AuxiliaryDisplay::UnicodeCharacterSet => "Unicode Character Set",
            AuxiliaryDisplay::Font7Segment => "Font 7-Segment",
            AuxiliaryDisplay::SevenSegmentDirectMap => "7-Segment Direct Map",
            AuxiliaryDisplay::Font14Segment => "Font 14-Segment",
            AuxiliaryDisplay::One4SegmentDirectMap => "14-Segment Direct Map",
            AuxiliaryDisplay::DisplayBrightness => "Display Brightness",
            AuxiliaryDisplay::DisplayContrast => "Display Contrast",
            AuxiliaryDisplay::CharacterAttribute => "Character Attribute",
            AuxiliaryDisplay::AttributeReadback => "Attribute Readback",
            AuxiliaryDisplay::AttributeData => "Attribute Data",
            AuxiliaryDisplay::CharAttrEnhance => "Char Attr Enhance",
            AuxiliaryDisplay::CharAttrUnderline => "Char Attr Underline",
            AuxiliaryDisplay::CharAttrBlink => "Char Attr Blink",
            AuxiliaryDisplay::BitmapSizeX => "Bitmap Size X",
            AuxiliaryDisplay::BitmapSizeY => "Bitmap Size Y",
            AuxiliaryDisplay::MaxBlitSize => "Max Blit Size",
            AuxiliaryDisplay::BitDepthFormat => "Bit Depth Format",
            AuxiliaryDisplay::DisplayOrientation => "Display Orientation",
            AuxiliaryDisplay::PaletteReport => "Palette Report",
            AuxiliaryDisplay::PaletteDataSize => "Palette Data Size",
            AuxiliaryDisplay::PaletteDataOffset => "Palette Data Offset",
            AuxiliaryDisplay::PaletteData => "Palette Data",
            AuxiliaryDisplay::BlitReport => "Blit Report",
            AuxiliaryDisplay::BlitRectangleX1 => "Blit Rectangle X1",
            AuxiliaryDisplay::BlitRectangleY1 => "Blit Rectangle Y1",
            AuxiliaryDisplay::BlitRectangleX2 => "Blit Rectangle X2",
            AuxiliaryDisplay::BlitRectangleY2 => "Blit Rectangle Y2",
            AuxiliaryDisplay::BlitData => "Blit Data",
            AuxiliaryDisplay::SoftButton => "Soft Button",
            AuxiliaryDisplay::SoftButtonID => "Soft Button ID",
            AuxiliaryDisplay::SoftButtonSide => "Soft Button Side",
            AuxiliaryDisplay::SoftButtonOffset1 => "Soft Button Offset 1",
            AuxiliaryDisplay::SoftButtonOffset2 => "Soft Button Offset 2",
            AuxiliaryDisplay::SoftButtonReport => "Soft Button Report",
            AuxiliaryDisplay::SoftKeys => "Soft Keys",
            AuxiliaryDisplay::DisplayDataExtensions => "Display Data Extensions",
            AuxiliaryDisplay::CharacterMapping => "Character Mapping",
            AuxiliaryDisplay::UnicodeEquivalent => "Unicode Equivalent",
            AuxiliaryDisplay::CharacterPageMapping => "Character Page Mapping",
            AuxiliaryDisplay::RequestReport => "Request Report",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for AuxiliaryDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for AuxiliaryDisplay {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::AuxiliaryDisplay(self)](Usage::AuxiliaryDisplay)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for AuxiliaryDisplay {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x14` for [AuxiliaryDisplay]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::AuxiliaryDisplay]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&AuxiliaryDisplay> for u16 {
    fn from(auxiliarydisplay: &AuxiliaryDisplay) -> u16 {
        *auxiliarydisplay as u16
    }
}

impl From<AuxiliaryDisplay> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [AuxiliaryDisplay::usage_page_value()].
    fn from(auxiliarydisplay: AuxiliaryDisplay) -> u16 {
        u16::from(&auxiliarydisplay)
    }
}

impl From<&AuxiliaryDisplay> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [AuxiliaryDisplay::usage_value()].
    fn from(auxiliarydisplay: &AuxiliaryDisplay) -> u32 {
        let up = UsagePage::from(auxiliarydisplay);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(auxiliarydisplay) as u32;
        up | id
    }
}

impl From<&AuxiliaryDisplay> for UsagePage {
    /// Always returns [UsagePage::AuxiliaryDisplay] and is
    /// identical to [AuxiliaryDisplay::usage_page()].
    fn from(_: &AuxiliaryDisplay) -> UsagePage {
        UsagePage::AuxiliaryDisplay
    }
}

impl From<AuxiliaryDisplay> for UsagePage {
    /// Always returns [UsagePage::AuxiliaryDisplay] and is
    /// identical to [AuxiliaryDisplay::usage_page()].
    fn from(_: AuxiliaryDisplay) -> UsagePage {
        UsagePage::AuxiliaryDisplay
    }
}

impl From<&AuxiliaryDisplay> for Usage {
    fn from(auxiliarydisplay: &AuxiliaryDisplay) -> Usage {
        Usage::try_from(u32::from(auxiliarydisplay)).unwrap()
    }
}

impl From<AuxiliaryDisplay> for Usage {
    fn from(auxiliarydisplay: AuxiliaryDisplay) -> Usage {
        Usage::from(&auxiliarydisplay)
    }
}

impl TryFrom<u16> for AuxiliaryDisplay {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<AuxiliaryDisplay> {
        match usage_id {
            1 => Ok(AuxiliaryDisplay::AlphanumericDisplay),
            2 => Ok(AuxiliaryDisplay::AuxiliaryDisplay),
            32 => Ok(AuxiliaryDisplay::DisplayAttributesReport),
            33 => Ok(AuxiliaryDisplay::ASCIICharacterSet),
            34 => Ok(AuxiliaryDisplay::DataReadBack),
            35 => Ok(AuxiliaryDisplay::FontReadBack),
            36 => Ok(AuxiliaryDisplay::DisplayControlReport),
            37 => Ok(AuxiliaryDisplay::ClearDisplay),
            38 => Ok(AuxiliaryDisplay::DisplayEnable),
            39 => Ok(AuxiliaryDisplay::ScreenSaverDelay),
            40 => Ok(AuxiliaryDisplay::ScreenSaverEnable),
            41 => Ok(AuxiliaryDisplay::VerticalScroll),
            42 => Ok(AuxiliaryDisplay::HorizontalScroll),
            43 => Ok(AuxiliaryDisplay::CharacterReport),
            44 => Ok(AuxiliaryDisplay::DisplayData),
            45 => Ok(AuxiliaryDisplay::DisplayStatus),
            46 => Ok(AuxiliaryDisplay::StatNotReady),
            47 => Ok(AuxiliaryDisplay::StatReady),
            48 => Ok(AuxiliaryDisplay::ErrNotaloadablecharacter),
            49 => Ok(AuxiliaryDisplay::ErrFontdatacannotberead),
            50 => Ok(AuxiliaryDisplay::CursorPositionReport),
            51 => Ok(AuxiliaryDisplay::Row),
            52 => Ok(AuxiliaryDisplay::Column),
            53 => Ok(AuxiliaryDisplay::Rows),
            54 => Ok(AuxiliaryDisplay::Columns),
            55 => Ok(AuxiliaryDisplay::CursorPixelPositioning),
            56 => Ok(AuxiliaryDisplay::CursorMode),
            57 => Ok(AuxiliaryDisplay::CursorEnable),
            58 => Ok(AuxiliaryDisplay::CursorBlink),
            59 => Ok(AuxiliaryDisplay::FontReport),
            60 => Ok(AuxiliaryDisplay::FontData),
            61 => Ok(AuxiliaryDisplay::CharacterWidth),
            62 => Ok(AuxiliaryDisplay::CharacterHeight),
            63 => Ok(AuxiliaryDisplay::CharacterSpacingHorizontal),
            64 => Ok(AuxiliaryDisplay::CharacterSpacingVertical),
            65 => Ok(AuxiliaryDisplay::UnicodeCharacterSet),
            66 => Ok(AuxiliaryDisplay::Font7Segment),
            67 => Ok(AuxiliaryDisplay::SevenSegmentDirectMap),
            68 => Ok(AuxiliaryDisplay::Font14Segment),
            69 => Ok(AuxiliaryDisplay::One4SegmentDirectMap),
            70 => Ok(AuxiliaryDisplay::DisplayBrightness),
            71 => Ok(AuxiliaryDisplay::DisplayContrast),
            72 => Ok(AuxiliaryDisplay::CharacterAttribute),
            73 => Ok(AuxiliaryDisplay::AttributeReadback),
            74 => Ok(AuxiliaryDisplay::AttributeData),
            75 => Ok(AuxiliaryDisplay::CharAttrEnhance),
            76 => Ok(AuxiliaryDisplay::CharAttrUnderline),
            77 => Ok(AuxiliaryDisplay::CharAttrBlink),
            128 => Ok(AuxiliaryDisplay::BitmapSizeX),
            129 => Ok(AuxiliaryDisplay::BitmapSizeY),
            130 => Ok(AuxiliaryDisplay::MaxBlitSize),
            131 => Ok(AuxiliaryDisplay::BitDepthFormat),
            132 => Ok(AuxiliaryDisplay::DisplayOrientation),
            133 => Ok(AuxiliaryDisplay::PaletteReport),
            134 => Ok(AuxiliaryDisplay::PaletteDataSize),
            135 => Ok(AuxiliaryDisplay::PaletteDataOffset),
            136 => Ok(AuxiliaryDisplay::PaletteData),
            138 => Ok(AuxiliaryDisplay::BlitReport),
            139 => Ok(AuxiliaryDisplay::BlitRectangleX1),
            140 => Ok(AuxiliaryDisplay::BlitRectangleY1),
            141 => Ok(AuxiliaryDisplay::BlitRectangleX2),
            142 => Ok(AuxiliaryDisplay::BlitRectangleY2),
            143 => Ok(AuxiliaryDisplay::BlitData),
            144 => Ok(AuxiliaryDisplay::SoftButton),
            145 => Ok(AuxiliaryDisplay::SoftButtonID),
            146 => Ok(AuxiliaryDisplay::SoftButtonSide),
            147 => Ok(AuxiliaryDisplay::SoftButtonOffset1),
            148 => Ok(AuxiliaryDisplay::SoftButtonOffset2),
            149 => Ok(AuxiliaryDisplay::SoftButtonReport),
            194 => Ok(AuxiliaryDisplay::SoftKeys),
            204 => Ok(AuxiliaryDisplay::DisplayDataExtensions),
            207 => Ok(AuxiliaryDisplay::CharacterMapping),
            221 => Ok(AuxiliaryDisplay::UnicodeEquivalent),
            223 => Ok(AuxiliaryDisplay::CharacterPageMapping),
            255 => Ok(AuxiliaryDisplay::RequestReport),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for AuxiliaryDisplay {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x20`: "Sensors"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Sensors(Sensors::Biometric);
/// let u2 = Usage::new_from_page_and_id(0x20, 0x10).unwrap();
/// let u3 = Usage::from(Sensors::Biometric);
/// let u4: Usage = Sensors::Biometric.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Sensors));
/// assert_eq!(0x20, u1.usage_page_value());
/// assert_eq!(0x10, u1.usage_id_value());
/// assert_eq!((0x20 << 16) | 0x10, u1.usage_value());
/// assert_eq!("Biometric", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Sensors {
    /// Usage ID `0x1`: "Sensor"
    Sensor = 0x1,
    /// Usage ID `0x10`: "Biometric"
    Biometric = 0x10,
    /// Usage ID `0x11`: "Biometric: Human Presence"
    BiometricHumanPresence = 0x11,
    /// Usage ID `0x12`: "Biometric: Human Proximity"
    BiometricHumanProximity = 0x12,
    /// Usage ID `0x13`: "Biometric: Human Touch"
    BiometricHumanTouch = 0x13,
    /// Usage ID `0x14`: "Biometric: Blood Pressure"
    BiometricBloodPressure = 0x14,
    /// Usage ID `0x15`: "Biometric: Body Temperature"
    BiometricBodyTemperature = 0x15,
    /// Usage ID `0x16`: "Biometric: Heart Rate"
    BiometricHeartRate = 0x16,
    /// Usage ID `0x17`: "Biometric: Heart Rate Variability"
    BiometricHeartRateVariability = 0x17,
    /// Usage ID `0x18`: "Biometric: Peripheral Oxygen Saturation"
    BiometricPeripheralOxygenSaturation = 0x18,
    /// Usage ID `0x19`: "Biometric: Respiratory Rate"
    BiometricRespiratoryRate = 0x19,
    /// Usage ID `0x20`: "Electrical"
    Electrical = 0x20,
    /// Usage ID `0x21`: "Electrical: Capacitance"
    ElectricalCapacitance = 0x21,
    /// Usage ID `0x22`: "Electrical: Current"
    ElectricalCurrent = 0x22,
    /// Usage ID `0x23`: "Electrical: Power"
    ElectricalPower = 0x23,
    /// Usage ID `0x24`: "Electrical: Inductance"
    ElectricalInductance = 0x24,
    /// Usage ID `0x25`: "Electrical: Resistance"
    ElectricalResistance = 0x25,
    /// Usage ID `0x26`: "Electrical: Voltage"
    ElectricalVoltage = 0x26,
    /// Usage ID `0x27`: "Electrical: Potentiometer"
    ElectricalPotentiometer = 0x27,
    /// Usage ID `0x28`: "Electrical: Frequency"
    ElectricalFrequency = 0x28,
    /// Usage ID `0x29`: "Electrical: Period"
    ElectricalPeriod = 0x29,
    /// Usage ID `0x30`: "Environmental"
    Environmental = 0x30,
    /// Usage ID `0x31`: "Environmental: Atmospheric Pressure"
    EnvironmentalAtmosphericPressure = 0x31,
    /// Usage ID `0x32`: "Environmental: Humidity"
    EnvironmentalHumidity = 0x32,
    /// Usage ID `0x33`: "Environmental: Temperature"
    EnvironmentalTemperature = 0x33,
    /// Usage ID `0x34`: "Environmental: Wind Direction"
    EnvironmentalWindDirection = 0x34,
    /// Usage ID `0x35`: "Environmental: Wind Speed"
    EnvironmentalWindSpeed = 0x35,
    /// Usage ID `0x36`: "Environmental: Air Quality"
    EnvironmentalAirQuality = 0x36,
    /// Usage ID `0x37`: "Environmental: Heat Index"
    EnvironmentalHeatIndex = 0x37,
    /// Usage ID `0x38`: "Environmental: Surface Temperature"
    EnvironmentalSurfaceTemperature = 0x38,
    /// Usage ID `0x39`: "Environmental: Volatile Organic Compounds"
    EnvironmentalVolatileOrganicCompounds = 0x39,
    /// Usage ID `0x3A`: "Environmental: Object Presence"
    EnvironmentalObjectPresence = 0x3A,
    /// Usage ID `0x3B`: "Environmental: Object Proximity"
    EnvironmentalObjectProximity = 0x3B,
    /// Usage ID `0x40`: "Light"
    Light = 0x40,
    /// Usage ID `0x41`: "Light: Ambient Light"
    LightAmbientLight = 0x41,
    /// Usage ID `0x42`: "Light: Consumer Infrared"
    LightConsumerInfrared = 0x42,
    /// Usage ID `0x43`: "Light: Infrared Light"
    LightInfraredLight = 0x43,
    /// Usage ID `0x44`: "Light: Visible Light"
    LightVisibleLight = 0x44,
    /// Usage ID `0x45`: "Light: Ultraviolet Light"
    LightUltravioletLight = 0x45,
    /// Usage ID `0x50`: "Location"
    Location = 0x50,
    /// Usage ID `0x51`: "Location: Broadcast"
    LocationBroadcast = 0x51,
    /// Usage ID `0x52`: "Location: Dead Reckoning"
    LocationDeadReckoning = 0x52,
    /// Usage ID `0x53`: "Location: GPS (Global Positioning System)"
    LocationGPSGlobalPositioningSystem = 0x53,
    /// Usage ID `0x54`: "Location: Lookup"
    LocationLookup = 0x54,
    /// Usage ID `0x55`: "Location: Other"
    LocationOther = 0x55,
    /// Usage ID `0x56`: "Location: Static"
    LocationStatic = 0x56,
    /// Usage ID `0x57`: "Location: Triangulation"
    LocationTriangulation = 0x57,
    /// Usage ID `0x60`: "Mechanical"
    Mechanical = 0x60,
    /// Usage ID `0x61`: "Mechanical: Boolean Switch"
    MechanicalBooleanSwitch = 0x61,
    /// Usage ID `0x62`: "Mechanical: Boolean Switch Array"
    MechanicalBooleanSwitchArray = 0x62,
    /// Usage ID `0x63`: "Mechanical: Multivalue Switch"
    MechanicalMultivalueSwitch = 0x63,
    /// Usage ID `0x64`: "Mechanical: Force"
    MechanicalForce = 0x64,
    /// Usage ID `0x65`: "Mechanical: Pressure"
    MechanicalPressure = 0x65,
    /// Usage ID `0x66`: "Mechanical: Strain"
    MechanicalStrain = 0x66,
    /// Usage ID `0x67`: "Mechanical: Weight"
    MechanicalWeight = 0x67,
    /// Usage ID `0x68`: "Mechanical: Haptic Vibrator"
    MechanicalHapticVibrator = 0x68,
    /// Usage ID `0x69`: "Mechanical: Hall Effect Switch"
    MechanicalHallEffectSwitch = 0x69,
    /// Usage ID `0x70`: "Motion"
    Motion = 0x70,
    /// Usage ID `0x71`: "Motion: Accelerometer 1D"
    MotionAccelerometer1D = 0x71,
    /// Usage ID `0x72`: "Motion: Accelerometer 2D"
    MotionAccelerometer2D = 0x72,
    /// Usage ID `0x73`: "Motion: Accelerometer 3D"
    MotionAccelerometer3D = 0x73,
    /// Usage ID `0x74`: "Motion: Gyrometer 1D"
    MotionGyrometer1D = 0x74,
    /// Usage ID `0x75`: "Motion: Gyrometer 2D"
    MotionGyrometer2D = 0x75,
    /// Usage ID `0x76`: "Motion: Gyrometer 3D"
    MotionGyrometer3D = 0x76,
    /// Usage ID `0x77`: "Motion: Motion Detector"
    MotionMotionDetector = 0x77,
    /// Usage ID `0x78`: "Motion: Speedometer"
    MotionSpeedometer = 0x78,
    /// Usage ID `0x79`: "Motion: Accelerometer"
    MotionAccelerometer = 0x79,
    /// Usage ID `0x7A`: "Motion: Gyrometer"
    MotionGyrometer = 0x7A,
    /// Usage ID `0x7B`: "Motion: Gravity Vector"
    MotionGravityVector = 0x7B,
    /// Usage ID `0x7C`: "Motion: Linear Accelerometer"
    MotionLinearAccelerometer = 0x7C,
    /// Usage ID `0x80`: "Orientation"
    Orientation = 0x80,
    /// Usage ID `0x81`: "Orientation: Compass 1D"
    OrientationCompass1D = 0x81,
    /// Usage ID `0x82`: "Orientation: Compass 2D"
    OrientationCompass2D = 0x82,
    /// Usage ID `0x83`: "Orientation: Compass 3D"
    OrientationCompass3D = 0x83,
    /// Usage ID `0x84`: "Orientation: Inclinometer 1D"
    OrientationInclinometer1D = 0x84,
    /// Usage ID `0x85`: "Orientation: Inclinometer 2D"
    OrientationInclinometer2D = 0x85,
    /// Usage ID `0x86`: "Orientation: Inclinometer 3D"
    OrientationInclinometer3D = 0x86,
    /// Usage ID `0x87`: "Orientation: Distance 1D"
    OrientationDistance1D = 0x87,
    /// Usage ID `0x88`: "Orientation: Distance 2D"
    OrientationDistance2D = 0x88,
    /// Usage ID `0x89`: "Orientation: Distance 3D"
    OrientationDistance3D = 0x89,
    /// Usage ID `0x8A`: "Orientation: Device Orientation"
    OrientationDeviceOrientation = 0x8A,
    /// Usage ID `0x8B`: "Orientation: Compass"
    OrientationCompass = 0x8B,
    /// Usage ID `0x8C`: "Orientation: Inclinometer"
    OrientationInclinometer = 0x8C,
    /// Usage ID `0x8D`: "Orientation: Distance"
    OrientationDistance = 0x8D,
    /// Usage ID `0x8E`: "Orientation: Relative Orientation"
    OrientationRelativeOrientation = 0x8E,
    /// Usage ID `0x8F`: "Orientation: Simple Orientation"
    OrientationSimpleOrientation = 0x8F,
    /// Usage ID `0x90`: "Scanner"
    Scanner = 0x90,
    /// Usage ID `0x91`: "Scanner: Barcode"
    ScannerBarcode = 0x91,
    /// Usage ID `0x92`: "Scanner: RFID"
    ScannerRFID = 0x92,
    /// Usage ID `0x93`: "Scanner: NFC"
    ScannerNFC = 0x93,
    /// Usage ID `0xA0`: "Time"
    Time = 0xA0,
    /// Usage ID `0xA1`: "Time: Alarm Timer"
    TimeAlarmTimer = 0xA1,
    /// Usage ID `0xA2`: "Time: Real Time Clock"
    TimeRealTimeClock = 0xA2,
    /// Usage ID `0xB0`: "Personal Activity"
    PersonalActivity = 0xB0,
    /// Usage ID `0xB1`: "Personal Activity: Activity Detection"
    PersonalActivityActivityDetection = 0xB1,
    /// Usage ID `0xB2`: "Personal Activity: Device Position"
    PersonalActivityDevicePosition = 0xB2,
    /// Usage ID `0xB3`: "Personal Activity: Floor Tracker"
    PersonalActivityFloorTracker = 0xB3,
    /// Usage ID `0xB4`: "Personal Activity: Pedometer"
    PersonalActivityPedometer = 0xB4,
    /// Usage ID `0xB5`: "Personal Activity: Step Detection"
    PersonalActivityStepDetection = 0xB5,
    /// Usage ID `0xC0`: "Orientation Extended"
    OrientationExtended = 0xC0,
    /// Usage ID `0xC1`: "Orientation Extended: Geomagnetic Orientation"
    OrientationExtendedGeomagneticOrientation = 0xC1,
    /// Usage ID `0xC2`: "Orientation Extended: Magnetometer"
    OrientationExtendedMagnetometer = 0xC2,
    /// Usage ID `0xD0`: "Gesture"
    Gesture = 0xD0,
    /// Usage ID `0xD1`: "Gesture: Chassis Flip Gesture"
    GestureChassisFlipGesture = 0xD1,
    /// Usage ID `0xD2`: "Gesture: Hinge Fold Gesture"
    GestureHingeFoldGesture = 0xD2,
    /// Usage ID `0xE0`: "Other"
    Other = 0xE0,
    /// Usage ID `0xE1`: "Other: Custom"
    OtherCustom = 0xE1,
    /// Usage ID `0xE2`: "Other: Generic"
    OtherGeneric = 0xE2,
    /// Usage ID `0xE3`: "Other: Generic Enumerator"
    OtherGenericEnumerator = 0xE3,
    /// Usage ID `0xE4`: "Other: Hinge Angle"
    OtherHingeAngle = 0xE4,
    /// Usage ID `0xF0`: "Vendor Reserved 1"
    VendorReserved1 = 0xF0,
    /// Usage ID `0xF1`: "Vendor Reserved 2"
    VendorReserved2 = 0xF1,
    /// Usage ID `0xF2`: "Vendor Reserved 3"
    VendorReserved3 = 0xF2,
    /// Usage ID `0xF3`: "Vendor Reserved 4"
    VendorReserved4 = 0xF3,
    /// Usage ID `0xF4`: "Vendor Reserved 5"
    VendorReserved5 = 0xF4,
    /// Usage ID `0xF5`: "Vendor Reserved 6"
    VendorReserved6 = 0xF5,
    /// Usage ID `0xF6`: "Vendor Reserved 7"
    VendorReserved7 = 0xF6,
    /// Usage ID `0xF7`: "Vendor Reserved 8"
    VendorReserved8 = 0xF7,
    /// Usage ID `0xF8`: "Vendor Reserved 9"
    VendorReserved9 = 0xF8,
    /// Usage ID `0xF9`: "Vendor Reserved 10"
    VendorReserved10 = 0xF9,
    /// Usage ID `0xFA`: "Vendor Reserved 11"
    VendorReserved11 = 0xFA,
    /// Usage ID `0xFB`: "Vendor Reserved 12"
    VendorReserved12 = 0xFB,
    /// Usage ID `0xFC`: "Vendor Reserved 13"
    VendorReserved13 = 0xFC,
    /// Usage ID `0xFD`: "Vendor Reserved 14"
    VendorReserved14 = 0xFD,
    /// Usage ID `0xFE`: "Vendor Reserved 15"
    VendorReserved15 = 0xFE,
    /// Usage ID `0xFF`: "Vendor Reserved 16"
    VendorReserved16 = 0xFF,
    /// Usage ID `0x200`: "Event"
    Event = 0x200,
    /// Usage ID `0x201`: "Event: Sensor State"
    EventSensorState = 0x201,
    /// Usage ID `0x202`: "Event: Sensor Event"
    EventSensorEvent = 0x202,
    /// Usage ID `0x300`: "Property"
    Property = 0x300,
    /// Usage ID `0x301`: "Property: Friendly Name"
    PropertyFriendlyName = 0x301,
    /// Usage ID `0x302`: "Property: Persistent Unique ID"
    PropertyPersistentUniqueID = 0x302,
    /// Usage ID `0x303`: "Property: Sensor Status"
    PropertySensorStatus = 0x303,
    /// Usage ID `0x304`: "Property: Minimum Report Interval"
    PropertyMinimumReportInterval = 0x304,
    /// Usage ID `0x305`: "Property: Sensor Manufacturer"
    PropertySensorManufacturer = 0x305,
    /// Usage ID `0x306`: "Property: Sensor Model"
    PropertySensorModel = 0x306,
    /// Usage ID `0x307`: "Property: Sensor Serial Number"
    PropertySensorSerialNumber = 0x307,
    /// Usage ID `0x308`: "Property: Sensor Description"
    PropertySensorDescription = 0x308,
    /// Usage ID `0x309`: "Property: Sensor Connection Type"
    PropertySensorConnectionType = 0x309,
    /// Usage ID `0x30A`: "Property: Sensor Device Path"
    PropertySensorDevicePath = 0x30A,
    /// Usage ID `0x30B`: "Property: Hardware Revision"
    PropertyHardwareRevision = 0x30B,
    /// Usage ID `0x30C`: "Property: Firmware Version"
    PropertyFirmwareVersion = 0x30C,
    /// Usage ID `0x30D`: "Property: Release Date"
    PropertyReleaseDate = 0x30D,
    /// Usage ID `0x30E`: "Property: Report Interval"
    PropertyReportInterval = 0x30E,
    /// Usage ID `0x30F`: "Property: Change Sensitivity Absolute"
    PropertyChangeSensitivityAbsolute = 0x30F,
    /// Usage ID `0x310`: "Property: Change Sensitivity Percent of Range"
    PropertyChangeSensitivityPercentofRange = 0x310,
    /// Usage ID `0x311`: "Property: Change Sensitivity Percent Relative"
    PropertyChangeSensitivityPercentRelative = 0x311,
    /// Usage ID `0x312`: "Property: Accuracy"
    PropertyAccuracy = 0x312,
    /// Usage ID `0x313`: "Property: Resolution"
    PropertyResolution = 0x313,
    /// Usage ID `0x314`: "Property: Maximum"
    PropertyMaximum = 0x314,
    /// Usage ID `0x315`: "Property: Minimum"
    PropertyMinimum = 0x315,
    /// Usage ID `0x316`: "Property: Reporting State"
    PropertyReportingState = 0x316,
    /// Usage ID `0x317`: "Property: Sampling Rate"
    PropertySamplingRate = 0x317,
    /// Usage ID `0x318`: "Property: Response Curve"
    PropertyResponseCurve = 0x318,
    /// Usage ID `0x319`: "Property: Power State"
    PropertyPowerState = 0x319,
    /// Usage ID `0x31A`: "Property: Maximum FIFO Events"
    PropertyMaximumFIFOEvents = 0x31A,
    /// Usage ID `0x31B`: "Property: Report Latency"
    PropertyReportLatency = 0x31B,
    /// Usage ID `0x31C`: "Property: Flush FIFO Events"
    PropertyFlushFIFOEvents = 0x31C,
    /// Usage ID `0x31D`: "Property: Maximum Power Consumption"
    PropertyMaximumPowerConsumption = 0x31D,
    /// Usage ID `0x31E`: "Property: Is Primary"
    PropertyIsPrimary = 0x31E,
    /// Usage ID `0x31F`: "Property: Human Presence Detection Type"
    PropertyHumanPresenceDetectionType = 0x31F,
    /// Usage ID `0x400`: "Data Field: Location"
    DataFieldLocation = 0x400,
    /// Usage ID `0x402`: "Data Field: Altitude Antenna Sea Level"
    DataFieldAltitudeAntennaSeaLevel = 0x402,
    /// Usage ID `0x403`: "Data Field: Differential Reference Station ID"
    DataFieldDifferentialReferenceStationID = 0x403,
    /// Usage ID `0x404`: "Data Field: Altitude Ellipsoid Error"
    DataFieldAltitudeEllipsoidError = 0x404,
    /// Usage ID `0x405`: "Data Field: Altitude Ellipsoid"
    DataFieldAltitudeEllipsoid = 0x405,
    /// Usage ID `0x406`: "Data Field: Altitude Sea Level Error"
    DataFieldAltitudeSeaLevelError = 0x406,
    /// Usage ID `0x407`: "Data Field: Altitude Sea Level"
    DataFieldAltitudeSeaLevel = 0x407,
    /// Usage ID `0x408`: "Data Field: Differential GPS Data Age"
    DataFieldDifferentialGPSDataAge = 0x408,
    /// Usage ID `0x409`: "Data Field: Error Radius"
    DataFieldErrorRadius = 0x409,
    /// Usage ID `0x40A`: "Data Field: Fix Quality"
    DataFieldFixQuality = 0x40A,
    /// Usage ID `0x40B`: "Data Field: Fix Type"
    DataFieldFixType = 0x40B,
    /// Usage ID `0x40C`: "Data Field: Geoidal Separation"
    DataFieldGeoidalSeparation = 0x40C,
    /// Usage ID `0x40D`: "Data Field: GPS Operation Mode"
    DataFieldGPSOperationMode = 0x40D,
    /// Usage ID `0x40E`: "Data Field: GPS Selection Mode"
    DataFieldGPSSelectionMode = 0x40E,
    /// Usage ID `0x40F`: "Data Field: GPS Status"
    DataFieldGPSStatus = 0x40F,
    /// Usage ID `0x410`: "Data Field: Position Dilution of Precision"
    DataFieldPositionDilutionofPrecision = 0x410,
    /// Usage ID `0x411`: "Data Field: Horizontal Dilution of Precision"
    DataFieldHorizontalDilutionofPrecision = 0x411,
    /// Usage ID `0x412`: "Data Field: Vertical Dilution of Precision"
    DataFieldVerticalDilutionofPrecision = 0x412,
    /// Usage ID `0x413`: "Data Field: Latitude"
    DataFieldLatitude = 0x413,
    /// Usage ID `0x414`: "Data Field: Longitude"
    DataFieldLongitude = 0x414,
    /// Usage ID `0x415`: "Data Field: True Heading"
    DataFieldTrueHeading = 0x415,
    /// Usage ID `0x416`: "Data Field: Magnetic Heading"
    DataFieldMagneticHeading = 0x416,
    /// Usage ID `0x417`: "Data Field: Magnetic Variation"
    DataFieldMagneticVariation = 0x417,
    /// Usage ID `0x418`: "Data Field: Speed"
    DataFieldSpeed = 0x418,
    /// Usage ID `0x419`: "Data Field: Satellites in View"
    DataFieldSatellitesinView = 0x419,
    /// Usage ID `0x41A`: "Data Field: Satellites in View Azimuth"
    DataFieldSatellitesinViewAzimuth = 0x41A,
    /// Usage ID `0x41B`: "Data Field: Satellites in View Elevation"
    DataFieldSatellitesinViewElevation = 0x41B,
    /// Usage ID `0x41C`: "Data Field: Satellites in View IDs"
    DataFieldSatellitesinViewIDs = 0x41C,
    /// Usage ID `0x41D`: "Data Field: Satellites in View PRNs"
    DataFieldSatellitesinViewPRNs = 0x41D,
    /// Usage ID `0x41E`: "Data Field: Satellites in View S/N Ratios"
    DataFieldSatellitesinViewSNRatios = 0x41E,
    /// Usage ID `0x41F`: "Data Field: Satellites Used Count"
    DataFieldSatellitesUsedCount = 0x41F,
    /// Usage ID `0x420`: "Data Field: Satellites Used PRNs"
    DataFieldSatellitesUsedPRNs = 0x420,
    /// Usage ID `0x421`: "Data Field: NMEA Sentence"
    DataFieldNMEASentence = 0x421,
    /// Usage ID `0x422`: "Data Field: Address Line 1"
    DataFieldAddressLine1 = 0x422,
    /// Usage ID `0x423`: "Data Field: Address Line 2"
    DataFieldAddressLine2 = 0x423,
    /// Usage ID `0x424`: "Data Field: City"
    DataFieldCity = 0x424,
    /// Usage ID `0x425`: "Data Field: State or Province"
    DataFieldStateorProvince = 0x425,
    /// Usage ID `0x426`: "Data Field: Country or Region"
    DataFieldCountryorRegion = 0x426,
    /// Usage ID `0x427`: "Data Field: Postal Code"
    DataFieldPostalCode = 0x427,
    /// Usage ID `0x42A`: "Property: Location"
    PropertyLocation = 0x42A,
    /// Usage ID `0x42B`: "Property: Location Desired Accuracy"
    PropertyLocationDesiredAccuracy = 0x42B,
    /// Usage ID `0x430`: "Data Field: Environmental"
    DataFieldEnvironmental = 0x430,
    /// Usage ID `0x431`: "Data Field: Atmospheric Pressure"
    DataFieldAtmosphericPressure = 0x431,
    /// Usage ID `0x433`: "Data Field: Relative Humidity"
    DataFieldRelativeHumidity = 0x433,
    /// Usage ID `0x434`: "Data Field: Temperature"
    DataFieldTemperature = 0x434,
    /// Usage ID `0x435`: "Data Field: Wind Direction"
    DataFieldWindDirection = 0x435,
    /// Usage ID `0x436`: "Data Field: Wind Speed"
    DataFieldWindSpeed = 0x436,
    /// Usage ID `0x437`: "Data Field: Air Quality Index"
    DataFieldAirQualityIndex = 0x437,
    /// Usage ID `0x438`: "Data Field: Equivalent CO2"
    DataFieldEquivalentCO2 = 0x438,
    /// Usage ID `0x439`: "Data Field: Volatile Organic Compound Concentration"
    DataFieldVolatileOrganicCompoundConcentration = 0x439,
    /// Usage ID `0x43A`: "Data Field: Object Presence"
    DataFieldObjectPresence = 0x43A,
    /// Usage ID `0x43B`: "Data Field: Object Proximity Range"
    DataFieldObjectProximityRange = 0x43B,
    /// Usage ID `0x43C`: "Data Field: Object Proximity Out of Range"
    DataFieldObjectProximityOutofRange = 0x43C,
    /// Usage ID `0x440`: "Property: Environmental"
    PropertyEnvironmental = 0x440,
    /// Usage ID `0x441`: "Property: Reference Pressure"
    PropertyReferencePressure = 0x441,
    /// Usage ID `0x450`: "Data Field: Motion"
    DataFieldMotion = 0x450,
    /// Usage ID `0x451`: "Data Field: Motion State"
    DataFieldMotionState = 0x451,
    /// Usage ID `0x452`: "Data Field: Acceleration"
    DataFieldAcceleration = 0x452,
    /// Usage ID `0x453`: "Data Field: Acceleration Axis X"
    DataFieldAccelerationAxisX = 0x453,
    /// Usage ID `0x454`: "Data Field: Acceleration Axis Y"
    DataFieldAccelerationAxisY = 0x454,
    /// Usage ID `0x455`: "Data Field: Acceleration Axis Z"
    DataFieldAccelerationAxisZ = 0x455,
    /// Usage ID `0x456`: "Data Field: Angular Velocity"
    DataFieldAngularVelocity = 0x456,
    /// Usage ID `0x457`: "Data Field: Angular Velocity about X Axis"
    DataFieldAngularVelocityaboutXAxis = 0x457,
    /// Usage ID `0x458`: "Data Field: Angular Velocity about Y Axis"
    DataFieldAngularVelocityaboutYAxis = 0x458,
    /// Usage ID `0x459`: "Data Field: Angular Velocity about Z Axis"
    DataFieldAngularVelocityaboutZAxis = 0x459,
    /// Usage ID `0x45A`: "Data Field: Angular Position"
    DataFieldAngularPosition = 0x45A,
    /// Usage ID `0x45B`: "Data Field: Angular Position about X Axis"
    DataFieldAngularPositionaboutXAxis = 0x45B,
    /// Usage ID `0x45C`: "Data Field: Angular Position about Y Axis"
    DataFieldAngularPositionaboutYAxis = 0x45C,
    /// Usage ID `0x45D`: "Data Field: Angular Position about Z Axis"
    DataFieldAngularPositionaboutZAxis = 0x45D,
    /// Usage ID `0x45E`: "Data Field: Motion Speed"
    DataFieldMotionSpeed = 0x45E,
    /// Usage ID `0x45F`: "Data Field: Motion Intensity"
    DataFieldMotionIntensity = 0x45F,
    /// Usage ID `0x470`: "Data Field: Orientation"
    DataFieldOrientation = 0x470,
    /// Usage ID `0x471`: "Data Field: Heading"
    DataFieldHeading = 0x471,
    /// Usage ID `0x472`: "Data Field: Heading X Axis"
    DataFieldHeadingXAxis = 0x472,
    /// Usage ID `0x473`: "Data Field: Heading Y Axis"
    DataFieldHeadingYAxis = 0x473,
    /// Usage ID `0x474`: "Data Field: Heading Z Axis"
    DataFieldHeadingZAxis = 0x474,
    /// Usage ID `0x475`: "Data Field: Heading Compensated Magnetic North"
    DataFieldHeadingCompensatedMagneticNorth = 0x475,
    /// Usage ID `0x476`: "Data Field: Heading Compensated True North"
    DataFieldHeadingCompensatedTrueNorth = 0x476,
    /// Usage ID `0x477`: "Data Field: Heading Magnetic North"
    DataFieldHeadingMagneticNorth = 0x477,
    /// Usage ID `0x478`: "Data Field: Heading True North"
    DataFieldHeadingTrueNorth = 0x478,
    /// Usage ID `0x479`: "Data Field: Distance"
    DataFieldDistance = 0x479,
    /// Usage ID `0x47A`: "Data Field: Distance X Axis"
    DataFieldDistanceXAxis = 0x47A,
    /// Usage ID `0x47B`: "Data Field: Distance Y Axis"
    DataFieldDistanceYAxis = 0x47B,
    /// Usage ID `0x47C`: "Data Field: Distance Z Axis"
    DataFieldDistanceZAxis = 0x47C,
    /// Usage ID `0x47D`: "Data Field: Distance Out-of-Range"
    DataFieldDistanceOutofRange = 0x47D,
    /// Usage ID `0x47E`: "Data Field: Tilt"
    DataFieldTilt = 0x47E,
    /// Usage ID `0x47F`: "Data Field: Tilt X Axis"
    DataFieldTiltXAxis = 0x47F,
    /// Usage ID `0x480`: "Data Field: Tilt Y Axis"
    DataFieldTiltYAxis = 0x480,
    /// Usage ID `0x481`: "Data Field: Tilt Z Axis"
    DataFieldTiltZAxis = 0x481,
    /// Usage ID `0x482`: "Data Field: Rotation Matrix"
    DataFieldRotationMatrix = 0x482,
    /// Usage ID `0x483`: "Data Field: Quaternion"
    DataFieldQuaternion = 0x483,
    /// Usage ID `0x484`: "Data Field: Magnetic Flux"
    DataFieldMagneticFlux = 0x484,
    /// Usage ID `0x485`: "Data Field: Magnetic Flux X Axis"
    DataFieldMagneticFluxXAxis = 0x485,
    /// Usage ID `0x486`: "Data Field: Magnetic Flux Y Axis"
    DataFieldMagneticFluxYAxis = 0x486,
    /// Usage ID `0x487`: "Data Field: Magnetic Flux Z Axis"
    DataFieldMagneticFluxZAxis = 0x487,
    /// Usage ID `0x488`: "Data Field: Magnetometer Accuracy"
    DataFieldMagnetometerAccuracy = 0x488,
    /// Usage ID `0x489`: "Data Field: Simple Orientation Direction"
    DataFieldSimpleOrientationDirection = 0x489,
    /// Usage ID `0x490`: "Data Field: Mechanical"
    DataFieldMechanical = 0x490,
    /// Usage ID `0x491`: "Data Field: Boolean Switch State"
    DataFieldBooleanSwitchState = 0x491,
    /// Usage ID `0x492`: "Data Field: Boolean Switch Array States"
    DataFieldBooleanSwitchArrayStates = 0x492,
    /// Usage ID `0x493`: "Data Field: Multivalue Switch Value"
    DataFieldMultivalueSwitchValue = 0x493,
    /// Usage ID `0x494`: "Data Field: Force"
    DataFieldForce = 0x494,
    /// Usage ID `0x495`: "Data Field: Absolute Pressure"
    DataFieldAbsolutePressure = 0x495,
    /// Usage ID `0x496`: "Data Field: Gauge Pressure"
    DataFieldGaugePressure = 0x496,
    /// Usage ID `0x497`: "Data Field: Strain"
    DataFieldStrain = 0x497,
    /// Usage ID `0x498`: "Data Field: Weight"
    DataFieldWeight = 0x498,
    /// Usage ID `0x4A0`: "Property: Mechanical"
    PropertyMechanical = 0x4A0,
    /// Usage ID `0x4A1`: "Property: Vibration State"
    PropertyVibrationState = 0x4A1,
    /// Usage ID `0x4A2`: "Property: Forward Vibration Speed"
    PropertyForwardVibrationSpeed = 0x4A2,
    /// Usage ID `0x4A3`: "Property: Backward Vibration Speed"
    PropertyBackwardVibrationSpeed = 0x4A3,
    /// Usage ID `0x4B0`: "Data Field: Biometric"
    DataFieldBiometric = 0x4B0,
    /// Usage ID `0x4B1`: "Data Field: Human Presence"
    DataFieldHumanPresence = 0x4B1,
    /// Usage ID `0x4B2`: "Data Field: Human Proximity Range"
    DataFieldHumanProximityRange = 0x4B2,
    /// Usage ID `0x4B3`: "Data Field: Human Proximity Out of Range"
    DataFieldHumanProximityOutofRange = 0x4B3,
    /// Usage ID `0x4B4`: "Data Field: Human Touch State"
    DataFieldHumanTouchState = 0x4B4,
    /// Usage ID `0x4B5`: "Data Field: Blood Pressure"
    DataFieldBloodPressure = 0x4B5,
    /// Usage ID `0x4B6`: "Data Field: Blood Pressure Diastolic"
    DataFieldBloodPressureDiastolic = 0x4B6,
    /// Usage ID `0x4B7`: "Data Field: Blood Pressure Systolic"
    DataFieldBloodPressureSystolic = 0x4B7,
    /// Usage ID `0x4B8`: "Data Field: Heart Rate"
    DataFieldHeartRate = 0x4B8,
    /// Usage ID `0x4B9`: "Data Field: Resting Heart Rate"
    DataFieldRestingHeartRate = 0x4B9,
    /// Usage ID `0x4BA`: "Data Field: Heartbeat Interval"
    DataFieldHeartbeatInterval = 0x4BA,
    /// Usage ID `0x4BB`: "Data Field: Respiratory Rate"
    DataFieldRespiratoryRate = 0x4BB,
    /// Usage ID `0x4BC`: "Data Field: SpO2"
    DataFieldSpO2 = 0x4BC,
    /// Usage ID `0x4BD`: "Data Field: Human Attention Detected"
    DataFieldHumanAttentionDetected = 0x4BD,
    /// Usage ID `0x4BE`: "Data Field: Human Head Azimuth"
    DataFieldHumanHeadAzimuth = 0x4BE,
    /// Usage ID `0x4BF`: "Data Field: Human Head Altitude"
    DataFieldHumanHeadAltitude = 0x4BF,
    /// Usage ID `0x4C0`: "Data Field: Human Head Roll"
    DataFieldHumanHeadRoll = 0x4C0,
    /// Usage ID `0x4C1`: "Data Field: Human Head Pitch"
    DataFieldHumanHeadPitch = 0x4C1,
    /// Usage ID `0x4C2`: "Data Field: Human Head Yaw"
    DataFieldHumanHeadYaw = 0x4C2,
    /// Usage ID `0x4C3`: "Data Field: Human Correlation Id"
    DataFieldHumanCorrelationId = 0x4C3,
    /// Usage ID `0x4D0`: "Data Field: Light"
    DataFieldLight = 0x4D0,
    /// Usage ID `0x4D1`: "Data Field: Illuminance"
    DataFieldIlluminance = 0x4D1,
    /// Usage ID `0x4D2`: "Data Field: Color Temperature"
    DataFieldColorTemperature = 0x4D2,
    /// Usage ID `0x4D3`: "Data Field: Chromaticity"
    DataFieldChromaticity = 0x4D3,
    /// Usage ID `0x4D4`: "Data Field: Chromaticity X"
    DataFieldChromaticityX = 0x4D4,
    /// Usage ID `0x4D5`: "Data Field: Chromaticity Y"
    DataFieldChromaticityY = 0x4D5,
    /// Usage ID `0x4D6`: "Data Field: Consumer IR Sentence Receive"
    DataFieldConsumerIRSentenceReceive = 0x4D6,
    /// Usage ID `0x4D7`: "Data Field: Infrared Light"
    DataFieldInfraredLight = 0x4D7,
    /// Usage ID `0x4D8`: "Data Field: Red Light"
    DataFieldRedLight = 0x4D8,
    /// Usage ID `0x4D9`: "Data Field: Green Light"
    DataFieldGreenLight = 0x4D9,
    /// Usage ID `0x4DA`: "Data Field: Blue Light"
    DataFieldBlueLight = 0x4DA,
    /// Usage ID `0x4DB`: "Data Field: Ultraviolet A Light"
    DataFieldUltravioletALight = 0x4DB,
    /// Usage ID `0x4DC`: "Data Field: Ultraviolet B Light"
    DataFieldUltravioletBLight = 0x4DC,
    /// Usage ID `0x4DD`: "Data Field: Ultraviolet Index"
    DataFieldUltravioletIndex = 0x4DD,
    /// Usage ID `0x4DE`: "Data Field: Near Infrared Light"
    DataFieldNearInfraredLight = 0x4DE,
    /// Usage ID `0x4DF`: "Property: Light"
    PropertyLight = 0x4DF,
    /// Usage ID `0x4E0`: "Property: Consumer IR Sentence Send"
    PropertyConsumerIRSentenceSend = 0x4E0,
    /// Usage ID `0x4E2`: "Property: Auto Brightness Preferred"
    PropertyAutoBrightnessPreferred = 0x4E2,
    /// Usage ID `0x4E3`: "Property: Auto Color Preferred"
    PropertyAutoColorPreferred = 0x4E3,
    /// Usage ID `0x4F0`: "Data Field: Scanner"
    DataFieldScanner = 0x4F0,
    /// Usage ID `0x4F1`: "Data Field: RFID Tag 40 Bit"
    DataFieldRFIDTag40Bit = 0x4F1,
    /// Usage ID `0x4F2`: "Data Field: NFC Sentence Receive"
    DataFieldNFCSentenceReceive = 0x4F2,
    /// Usage ID `0x4F8`: "Property: Scanner"
    PropertyScanner = 0x4F8,
    /// Usage ID `0x4F9`: "Property: NFC Sentence Send"
    PropertyNFCSentenceSend = 0x4F9,
    /// Usage ID `0x500`: "Data Field: Electrical"
    DataFieldElectrical = 0x500,
    /// Usage ID `0x501`: "Data Field: Capacitance"
    DataFieldCapacitance = 0x501,
    /// Usage ID `0x502`: "Data Field: Current"
    DataFieldCurrent = 0x502,
    /// Usage ID `0x503`: "Data Field: Electrical Power"
    DataFieldElectricalPower = 0x503,
    /// Usage ID `0x504`: "Data Field: Inductance"
    DataFieldInductance = 0x504,
    /// Usage ID `0x505`: "Data Field: Resistance"
    DataFieldResistance = 0x505,
    /// Usage ID `0x506`: "Data Field: Voltage"
    DataFieldVoltage = 0x506,
    /// Usage ID `0x507`: "Data Field: Frequency"
    DataFieldFrequency = 0x507,
    /// Usage ID `0x508`: "Data Field: Period"
    DataFieldPeriod = 0x508,
    /// Usage ID `0x509`: "Data Field: Percent of Range"
    DataFieldPercentofRange = 0x509,
    /// Usage ID `0x520`: "Data Field: Time"
    DataFieldTime = 0x520,
    /// Usage ID `0x521`: "Data Field: Year"
    DataFieldYear = 0x521,
    /// Usage ID `0x522`: "Data Field: Month"
    DataFieldMonth = 0x522,
    /// Usage ID `0x523`: "Data Field: Day"
    DataFieldDay = 0x523,
    /// Usage ID `0x524`: "Data Field: Day of Week"
    DataFieldDayofWeek = 0x524,
    /// Usage ID `0x525`: "Data Field: Hour"
    DataFieldHour = 0x525,
    /// Usage ID `0x526`: "Data Field: Minute"
    DataFieldMinute = 0x526,
    /// Usage ID `0x527`: "Data Field: Second"
    DataFieldSecond = 0x527,
    /// Usage ID `0x528`: "Data Field: Millisecond"
    DataFieldMillisecond = 0x528,
    /// Usage ID `0x529`: "Data Field: Timestamp"
    DataFieldTimestamp = 0x529,
    /// Usage ID `0x52A`: "Data Field: Julian Day of Year"
    DataFieldJulianDayofYear = 0x52A,
    /// Usage ID `0x52B`: "Data Field: Time Since System Boot"
    DataFieldTimeSinceSystemBoot = 0x52B,
    /// Usage ID `0x530`: "Property: Time"
    PropertyTime = 0x530,
    /// Usage ID `0x531`: "Property: Time Zone Offset from UTC"
    PropertyTimeZoneOffsetfromUTC = 0x531,
    /// Usage ID `0x532`: "Property: Time Zone Name"
    PropertyTimeZoneName = 0x532,
    /// Usage ID `0x533`: "Property: Daylight Savings Time Observed"
    PropertyDaylightSavingsTimeObserved = 0x533,
    /// Usage ID `0x534`: "Property: Time Trim Adjustment"
    PropertyTimeTrimAdjustment = 0x534,
    /// Usage ID `0x535`: "Property: Arm Alarm"
    PropertyArmAlarm = 0x535,
    /// Usage ID `0x540`: "Data Field: Custom"
    DataFieldCustom = 0x540,
    /// Usage ID `0x541`: "Data Field: Custom Usage"
    DataFieldCustomUsage = 0x541,
    /// Usage ID `0x542`: "Data Field: Custom Boolean Array"
    DataFieldCustomBooleanArray = 0x542,
    /// Usage ID `0x543`: "Data Field: Custom Value"
    DataFieldCustomValue = 0x543,
    /// Usage ID `0x544`: "Data Field: Custom Value 1"
    DataFieldCustomValue1 = 0x544,
    /// Usage ID `0x545`: "Data Field: Custom Value 2"
    DataFieldCustomValue2 = 0x545,
    /// Usage ID `0x546`: "Data Field: Custom Value 3"
    DataFieldCustomValue3 = 0x546,
    /// Usage ID `0x547`: "Data Field: Custom Value 4"
    DataFieldCustomValue4 = 0x547,
    /// Usage ID `0x548`: "Data Field: Custom Value 5"
    DataFieldCustomValue5 = 0x548,
    /// Usage ID `0x549`: "Data Field: Custom Value 6"
    DataFieldCustomValue6 = 0x549,
    /// Usage ID `0x54A`: "Data Field: Custom Value 7"
    DataFieldCustomValue7 = 0x54A,
    /// Usage ID `0x54B`: "Data Field: Custom Value 8"
    DataFieldCustomValue8 = 0x54B,
    /// Usage ID `0x54C`: "Data Field: Custom Value 9"
    DataFieldCustomValue9 = 0x54C,
    /// Usage ID `0x54D`: "Data Field: Custom Value 10"
    DataFieldCustomValue10 = 0x54D,
    /// Usage ID `0x54E`: "Data Field: Custom Value 11"
    DataFieldCustomValue11 = 0x54E,
    /// Usage ID `0x54F`: "Data Field: Custom Value 12"
    DataFieldCustomValue12 = 0x54F,
    /// Usage ID `0x550`: "Data Field: Custom Value 13"
    DataFieldCustomValue13 = 0x550,
    /// Usage ID `0x551`: "Data Field: Custom Value 14"
    DataFieldCustomValue14 = 0x551,
    /// Usage ID `0x552`: "Data Field: Custom Value 15"
    DataFieldCustomValue15 = 0x552,
    /// Usage ID `0x553`: "Data Field: Custom Value 16"
    DataFieldCustomValue16 = 0x553,
    /// Usage ID `0x554`: "Data Field: Custom Value 17"
    DataFieldCustomValue17 = 0x554,
    /// Usage ID `0x555`: "Data Field: Custom Value 18"
    DataFieldCustomValue18 = 0x555,
    /// Usage ID `0x556`: "Data Field: Custom Value 19"
    DataFieldCustomValue19 = 0x556,
    /// Usage ID `0x557`: "Data Field: Custom Value 20"
    DataFieldCustomValue20 = 0x557,
    /// Usage ID `0x558`: "Data Field: Custom Value 21"
    DataFieldCustomValue21 = 0x558,
    /// Usage ID `0x559`: "Data Field: Custom Value 22"
    DataFieldCustomValue22 = 0x559,
    /// Usage ID `0x55A`: "Data Field: Custom Value 23"
    DataFieldCustomValue23 = 0x55A,
    /// Usage ID `0x55B`: "Data Field: Custom Value 24"
    DataFieldCustomValue24 = 0x55B,
    /// Usage ID `0x55C`: "Data Field: Custom Value 25"
    DataFieldCustomValue25 = 0x55C,
    /// Usage ID `0x55D`: "Data Field: Custom Value 26"
    DataFieldCustomValue26 = 0x55D,
    /// Usage ID `0x55E`: "Data Field: Custom Value 27"
    DataFieldCustomValue27 = 0x55E,
    /// Usage ID `0x55F`: "Data Field: Custom Value 28"
    DataFieldCustomValue28 = 0x55F,
    /// Usage ID `0x560`: "Data Field: Generic"
    DataFieldGeneric = 0x560,
    /// Usage ID `0x561`: "Data Field: Generic GUID or PROPERTYKEY"
    DataFieldGenericGUIDorPROPERTYKEY = 0x561,
    /// Usage ID `0x562`: "Data Field: Generic Category GUID"
    DataFieldGenericCategoryGUID = 0x562,
    /// Usage ID `0x563`: "Data Field: Generic Type GUID"
    DataFieldGenericTypeGUID = 0x563,
    /// Usage ID `0x564`: "Data Field: Generic Event PROPERTYKEY"
    DataFieldGenericEventPROPERTYKEY = 0x564,
    /// Usage ID `0x565`: "Data Field: Generic Property PROPERTYKEY"
    DataFieldGenericPropertyPROPERTYKEY = 0x565,
    /// Usage ID `0x566`: "Data Field: Generic Data Field PROPERTYKEY"
    DataFieldGenericDataFieldPROPERTYKEY = 0x566,
    /// Usage ID `0x567`: "Data Field: Generic Event"
    DataFieldGenericEvent = 0x567,
    /// Usage ID `0x568`: "Data Field: Generic Property"
    DataFieldGenericProperty = 0x568,
    /// Usage ID `0x569`: "Data Field: Generic Data Field"
    DataFieldGenericDataField = 0x569,
    /// Usage ID `0x56A`: "Data Field: Enumerator Table Row Index"
    DataFieldEnumeratorTableRowIndex = 0x56A,
    /// Usage ID `0x56B`: "Data Field: Enumerator Table Row Count"
    DataFieldEnumeratorTableRowCount = 0x56B,
    /// Usage ID `0x56C`: "Data Field: Generic GUID or PROPERTYKEY kind"
    DataFieldGenericGUIDorPROPERTYKEYkind = 0x56C,
    /// Usage ID `0x56D`: "Data Field: Generic GUID"
    DataFieldGenericGUID = 0x56D,
    /// Usage ID `0x56E`: "Data Field: Generic PROPERTYKEY"
    DataFieldGenericPROPERTYKEY = 0x56E,
    /// Usage ID `0x56F`: "Data Field: Generic Top Level Collection ID"
    DataFieldGenericTopLevelCollectionID = 0x56F,
    /// Usage ID `0x570`: "Data Field: Generic Report ID"
    DataFieldGenericReportID = 0x570,
    /// Usage ID `0x571`: "Data Field: Generic Report Item Position Index"
    DataFieldGenericReportItemPositionIndex = 0x571,
    /// Usage ID `0x572`: "Data Field: Generic Firmware VARTYPE"
    DataFieldGenericFirmwareVARTYPE = 0x572,
    /// Usage ID `0x573`: "Data Field: Generic Unit of Measure"
    DataFieldGenericUnitofMeasure = 0x573,
    /// Usage ID `0x574`: "Data Field: Generic Unit Exponent"
    DataFieldGenericUnitExponent = 0x574,
    /// Usage ID `0x575`: "Data Field: Generic Report Size"
    DataFieldGenericReportSize = 0x575,
    /// Usage ID `0x576`: "Data Field: Generic Report Count"
    DataFieldGenericReportCount = 0x576,
    /// Usage ID `0x580`: "Property: Generic"
    PropertyGeneric = 0x580,
    /// Usage ID `0x581`: "Property: Enumerator Table Row Index"
    PropertyEnumeratorTableRowIndex = 0x581,
    /// Usage ID `0x582`: "Property: Enumerator Table Row Count"
    PropertyEnumeratorTableRowCount = 0x582,
    /// Usage ID `0x590`: "Data Field: Personal Activity"
    DataFieldPersonalActivity = 0x590,
    /// Usage ID `0x591`: "Data Field: Activity Type"
    DataFieldActivityType = 0x591,
    /// Usage ID `0x592`: "Data Field: Activity State"
    DataFieldActivityState = 0x592,
    /// Usage ID `0x593`: "Data Field: Device Position"
    DataFieldDevicePosition = 0x593,
    /// Usage ID `0x594`: "Data Field: Step Count"
    DataFieldStepCount = 0x594,
    /// Usage ID `0x595`: "Data Field: Step Count Reset"
    DataFieldStepCountReset = 0x595,
    /// Usage ID `0x596`: "Data Field: Step Duration"
    DataFieldStepDuration = 0x596,
    /// Usage ID `0x597`: "Data Field: Step Type"
    DataFieldStepType = 0x597,
    /// Usage ID `0x5A0`: "Property: Minimum Activity Detection Interval"
    PropertyMinimumActivityDetectionInterval = 0x5A0,
    /// Usage ID `0x5A1`: "Property: Supported Activity Types"
    PropertySupportedActivityTypes = 0x5A1,
    /// Usage ID `0x5A2`: "Property: Subscribed Activity Types"
    PropertySubscribedActivityTypes = 0x5A2,
    /// Usage ID `0x5A3`: "Property: Supported Step Types"
    PropertySupportedStepTypes = 0x5A3,
    /// Usage ID `0x5A4`: "Property: Subscribed Step Types"
    PropertySubscribedStepTypes = 0x5A4,
    /// Usage ID `0x5A5`: "Property: Floor Height"
    PropertyFloorHeight = 0x5A5,
    /// Usage ID `0x5B0`: "Data Field: Custom Type ID"
    DataFieldCustomTypeID = 0x5B0,
    /// Usage ID `0x5C0`: "Property: Custom"
    PropertyCustom = 0x5C0,
    /// Usage ID `0x5C1`: "Property: Custom Value 1"
    PropertyCustomValue1 = 0x5C1,
    /// Usage ID `0x5C2`: "Property: Custom Value 2"
    PropertyCustomValue2 = 0x5C2,
    /// Usage ID `0x5C3`: "Property: Custom Value 3"
    PropertyCustomValue3 = 0x5C3,
    /// Usage ID `0x5C4`: "Property: Custom Value 4"
    PropertyCustomValue4 = 0x5C4,
    /// Usage ID `0x5C5`: "Property: Custom Value 5"
    PropertyCustomValue5 = 0x5C5,
    /// Usage ID `0x5C6`: "Property: Custom Value 6"
    PropertyCustomValue6 = 0x5C6,
    /// Usage ID `0x5C7`: "Property: Custom Value 7"
    PropertyCustomValue7 = 0x5C7,
    /// Usage ID `0x5C8`: "Property: Custom Value 8"
    PropertyCustomValue8 = 0x5C8,
    /// Usage ID `0x5C9`: "Property: Custom Value 9"
    PropertyCustomValue9 = 0x5C9,
    /// Usage ID `0x5CA`: "Property: Custom Value 10"
    PropertyCustomValue10 = 0x5CA,
    /// Usage ID `0x5CB`: "Property: Custom Value 11"
    PropertyCustomValue11 = 0x5CB,
    /// Usage ID `0x5CC`: "Property: Custom Value 12"
    PropertyCustomValue12 = 0x5CC,
    /// Usage ID `0x5CD`: "Property: Custom Value 13"
    PropertyCustomValue13 = 0x5CD,
    /// Usage ID `0x5CE`: "Property: Custom Value 14"
    PropertyCustomValue14 = 0x5CE,
    /// Usage ID `0x5CF`: "Property: Custom Value 15"
    PropertyCustomValue15 = 0x5CF,
    /// Usage ID `0x5D0`: "Property: Custom Value 16"
    PropertyCustomValue16 = 0x5D0,
    /// Usage ID `0x5E0`: "Data Field: Hinge"
    DataFieldHinge = 0x5E0,
    /// Usage ID `0x5E1`: "Data Field: Hinge Angle"
    DataFieldHingeAngle = 0x5E1,
    /// Usage ID `0x5F0`: "Data Field: Gesture Sensor"
    DataFieldGestureSensor = 0x5F0,
    /// Usage ID `0x5F1`: "Data Field: Gesture State"
    DataFieldGestureState = 0x5F1,
    /// Usage ID `0x5F2`: "Data Field: Hinge Fold Initial Angle"
    DataFieldHingeFoldInitialAngle = 0x5F2,
    /// Usage ID `0x5F3`: "Data Field: Hinge Fold Final Angle"
    DataFieldHingeFoldFinalAngle = 0x5F3,
    /// Usage ID `0x5F4`: "Data Field: Hinge Fold Contributing Panel"
    DataFieldHingeFoldContributingPanel = 0x5F4,
    /// Usage ID `0x5F5`: "Data Field: Hinge Fold Type"
    DataFieldHingeFoldType = 0x5F5,
    /// Usage ID `0x800`: "Sensor State: Undefined"
    SensorStateUndefined = 0x800,
    /// Usage ID `0x801`: "Sensor State: Ready"
    SensorStateReady = 0x801,
    /// Usage ID `0x802`: "Sensor State: Not Available"
    SensorStateNotAvailable = 0x802,
    /// Usage ID `0x803`: "Sensor State: No Data"
    SensorStateNoData = 0x803,
    /// Usage ID `0x804`: "Sensor State: Initializing"
    SensorStateInitializing = 0x804,
    /// Usage ID `0x805`: "Sensor State: Access Denied"
    SensorStateAccessDenied = 0x805,
    /// Usage ID `0x806`: "Sensor State: Error"
    SensorStateError = 0x806,
    /// Usage ID `0x810`: "Sensor Event: Unknown"
    SensorEventUnknown = 0x810,
    /// Usage ID `0x811`: "Sensor Event: State Changed"
    SensorEventStateChanged = 0x811,
    /// Usage ID `0x812`: "Sensor Event: Property Changed"
    SensorEventPropertyChanged = 0x812,
    /// Usage ID `0x813`: "Sensor Event: Data Updated"
    SensorEventDataUpdated = 0x813,
    /// Usage ID `0x814`: "Sensor Event: Poll Response"
    SensorEventPollResponse = 0x814,
    /// Usage ID `0x815`: "Sensor Event: Change Sensitivity"
    SensorEventChangeSensitivity = 0x815,
    /// Usage ID `0x816`: "Sensor Event: Range Maximum Reached"
    SensorEventRangeMaximumReached = 0x816,
    /// Usage ID `0x817`: "Sensor Event: Range Minimum Reached"
    SensorEventRangeMinimumReached = 0x817,
    /// Usage ID `0x818`: "Sensor Event: High Threshold Cross Upward"
    SensorEventHighThresholdCrossUpward = 0x818,
    /// Usage ID `0x819`: "Sensor Event: High Threshold Cross Downward"
    SensorEventHighThresholdCrossDownward = 0x819,
    /// Usage ID `0x81A`: "Sensor Event: Low Threshold Cross Upward"
    SensorEventLowThresholdCrossUpward = 0x81A,
    /// Usage ID `0x81B`: "Sensor Event: Low Threshold Cross Downward"
    SensorEventLowThresholdCrossDownward = 0x81B,
    /// Usage ID `0x81C`: "Sensor Event: Zero Threshold Cross Upward"
    SensorEventZeroThresholdCrossUpward = 0x81C,
    /// Usage ID `0x81D`: "Sensor Event: Zero Threshold Cross Downward"
    SensorEventZeroThresholdCrossDownward = 0x81D,
    /// Usage ID `0x81E`: "Sensor Event: Period Exceeded"
    SensorEventPeriodExceeded = 0x81E,
    /// Usage ID `0x81F`: "Sensor Event: Frequency Exceeded"
    SensorEventFrequencyExceeded = 0x81F,
    /// Usage ID `0x820`: "Sensor Event: Complex Trigger"
    SensorEventComplexTrigger = 0x820,
    /// Usage ID `0x830`: "Connection Type: PC Integrated"
    ConnectionTypePCIntegrated = 0x830,
    /// Usage ID `0x831`: "Connection Type: PC Attached"
    ConnectionTypePCAttached = 0x831,
    /// Usage ID `0x832`: "Connection Type: PC External"
    ConnectionTypePCExternal = 0x832,
    /// Usage ID `0x840`: "Reporting State: Report No Events"
    ReportingStateReportNoEvents = 0x840,
    /// Usage ID `0x841`: "Reporting State: Report All Events"
    ReportingStateReportAllEvents = 0x841,
    /// Usage ID `0x842`: "Reporting State: Report Threshold Events"
    ReportingStateReportThresholdEvents = 0x842,
    /// Usage ID `0x843`: "Reporting State: Wake On No Events"
    ReportingStateWakeOnNoEvents = 0x843,
    /// Usage ID `0x844`: "Reporting State: Wake On All Events"
    ReportingStateWakeOnAllEvents = 0x844,
    /// Usage ID `0x845`: "Reporting State: Wake On Threshold Events"
    ReportingStateWakeOnThresholdEvents = 0x845,
    /// Usage ID `0x846`: "Reporting State: Anytime"
    ReportingStateAnytime = 0x846,
    /// Usage ID `0x850`: "Power State: Undefined"
    PowerStateUndefined = 0x850,
    /// Usage ID `0x851`: "Power State: D0 Full Power"
    PowerStateD0FullPower = 0x851,
    /// Usage ID `0x852`: "Power State: D1 Low Power"
    PowerStateD1LowPower = 0x852,
    /// Usage ID `0x853`: "Power State: D2 Standby Power with Wakeup"
    PowerStateD2StandbyPowerwithWakeup = 0x853,
    /// Usage ID `0x854`: "Power State: D3 Sleep with Wakeup"
    PowerStateD3SleepwithWakeup = 0x854,
    /// Usage ID `0x855`: "Power State: D4 Power Off"
    PowerStateD4PowerOff = 0x855,
    /// Usage ID `0x860`: "Accuracy: Default"
    AccuracyDefault = 0x860,
    /// Usage ID `0x861`: "Accuracy: High"
    AccuracyHigh = 0x861,
    /// Usage ID `0x862`: "Accuracy: Medium"
    AccuracyMedium = 0x862,
    /// Usage ID `0x863`: "Accuracy: Low"
    AccuracyLow = 0x863,
    /// Usage ID `0x870`: "Fix Quality: No Fix"
    FixQualityNoFix = 0x870,
    /// Usage ID `0x871`: "Fix Quality: GPS"
    FixQualityGPS = 0x871,
    /// Usage ID `0x872`: "Fix Quality: DGPS"
    FixQualityDGPS = 0x872,
    /// Usage ID `0x880`: "Fix Type: No Fix"
    FixTypeNoFix = 0x880,
    /// Usage ID `0x881`: "Fix Type: GPS SPS Mode, Fix Valid"
    FixTypeGPSSPSModeFixValid = 0x881,
    /// Usage ID `0x882`: "Fix Type: DGPS SPS Mode, Fix Valid"
    FixTypeDGPSSPSModeFixValid = 0x882,
    /// Usage ID `0x883`: "Fix Type: GPS PPS Mode, Fix Valid"
    FixTypeGPSPPSModeFixValid = 0x883,
    /// Usage ID `0x884`: "Fix Type: Real Time Kinematic"
    FixTypeRealTimeKinematic = 0x884,
    /// Usage ID `0x885`: "Fix Type: Float RTK"
    FixTypeFloatRTK = 0x885,
    /// Usage ID `0x886`: "Fix Type: Estimated (dead reckoned)"
    FixTypeEstimateddeadreckoned = 0x886,
    /// Usage ID `0x887`: "Fix Type: Manual Input Mode"
    FixTypeManualInputMode = 0x887,
    /// Usage ID `0x888`: "Fix Type: Simulator Mode"
    FixTypeSimulatorMode = 0x888,
    /// Usage ID `0x890`: "GPS Operation Mode: Manual"
    GPSOperationModeManual = 0x890,
    /// Usage ID `0x891`: "GPS Operation Mode: Automatic"
    GPSOperationModeAutomatic = 0x891,
    /// Usage ID `0x8A0`: "GPS Selection Mode: Autonomous"
    GPSSelectionModeAutonomous = 0x8A0,
    /// Usage ID `0x8A1`: "GPS Selection Mode: DGPS"
    GPSSelectionModeDGPS = 0x8A1,
    /// Usage ID `0x8A2`: "GPS Selection Mode: Estimated (dead reckoned)"
    GPSSelectionModeEstimateddeadreckoned = 0x8A2,
    /// Usage ID `0x8A3`: "GPS Selection Mode: Manual Input"
    GPSSelectionModeManualInput = 0x8A3,
    /// Usage ID `0x8A4`: "GPS Selection Mode: Simulator"
    GPSSelectionModeSimulator = 0x8A4,
    /// Usage ID `0x8A5`: "GPS Selection Mode: Data Not Valid"
    GPSSelectionModeDataNotValid = 0x8A5,
    /// Usage ID `0x8B0`: "GPS Status Data: Valid"
    GPSStatusDataValid = 0x8B0,
    /// Usage ID `0x8B1`: "GPS Status Data: Not Valid"
    GPSStatusDataNotValid = 0x8B1,
    /// Usage ID `0x8C0`: "Day of Week: Sunday"
    DayofWeekSunday = 0x8C0,
    /// Usage ID `0x8C1`: "Day of Week: Monday"
    DayofWeekMonday = 0x8C1,
    /// Usage ID `0x8C2`: "Day of Week: Tuesday"
    DayofWeekTuesday = 0x8C2,
    /// Usage ID `0x8C3`: "Day of Week: Wednesday"
    DayofWeekWednesday = 0x8C3,
    /// Usage ID `0x8C4`: "Day of Week: Thursday"
    DayofWeekThursday = 0x8C4,
    /// Usage ID `0x8C5`: "Day of Week: Friday"
    DayofWeekFriday = 0x8C5,
    /// Usage ID `0x8C6`: "Day of Week: Saturday"
    DayofWeekSaturday = 0x8C6,
    /// Usage ID `0x8D0`: "Kind: Category"
    KindCategory = 0x8D0,
    /// Usage ID `0x8D1`: "Kind: Type"
    KindType = 0x8D1,
    /// Usage ID `0x8D2`: "Kind: Event"
    KindEvent = 0x8D2,
    /// Usage ID `0x8D3`: "Kind: Property"
    KindProperty = 0x8D3,
    /// Usage ID `0x8D4`: "Kind: Data Field"
    KindDataField = 0x8D4,
    /// Usage ID `0x8E0`: "Magnetometer Accuracy: Low"
    MagnetometerAccuracyLow = 0x8E0,
    /// Usage ID `0x8E1`: "Magnetometer Accuracy: Medium"
    MagnetometerAccuracyMedium = 0x8E1,
    /// Usage ID `0x8E2`: "Magnetometer Accuracy: High"
    MagnetometerAccuracyHigh = 0x8E2,
    /// Usage ID `0x8F0`: "Simple Orientation Direction: Not Rotated"
    SimpleOrientationDirectionNotRotated = 0x8F0,
    /// Usage ID `0x8F1`: "Simple Orientation Direction: Rotated 90 Degrees CCW"
    SimpleOrientationDirectionRotated90DegreesCCW = 0x8F1,
    /// Usage ID `0x8F2`: "Simple Orientation Direction: Rotated 180 Degrees CCW"
    SimpleOrientationDirectionRotated180DegreesCCW = 0x8F2,
    /// Usage ID `0x8F3`: "Simple Orientation Direction: Rotated 270 Degrees CCW"
    SimpleOrientationDirectionRotated270DegreesCCW = 0x8F3,
    /// Usage ID `0x8F4`: "Simple Orientation Direction: Face Up"
    SimpleOrientationDirectionFaceUp = 0x8F4,
    /// Usage ID `0x8F5`: "Simple Orientation Direction: Face Down"
    SimpleOrientationDirectionFaceDown = 0x8F5,
    /// Usage ID `0x900`: "VT_NULL"
    VT_NULL = 0x900,
    /// Usage ID `0x901`: "VT_BOOL"
    VT_BOOL = 0x901,
    /// Usage ID `0x902`: "VT_UI1"
    VT_UI1 = 0x902,
    /// Usage ID `0x903`: "VT_I1"
    VT_I1 = 0x903,
    /// Usage ID `0x904`: "VT_UI2"
    VT_UI2 = 0x904,
    /// Usage ID `0x905`: "VT_I2"
    VT_I2 = 0x905,
    /// Usage ID `0x906`: "VT_UI4"
    VT_UI4 = 0x906,
    /// Usage ID `0x907`: "VT_I4"
    VT_I4 = 0x907,
    /// Usage ID `0x908`: "VT_UI8"
    VT_UI8 = 0x908,
    /// Usage ID `0x909`: "VT_I8"
    VT_I8 = 0x909,
    /// Usage ID `0x90A`: "VT_R4"
    VT_R4 = 0x90A,
    /// Usage ID `0x90B`: "VT_R8"
    VT_R8 = 0x90B,
    /// Usage ID `0x90C`: "VT_WSTR"
    VT_WSTR = 0x90C,
    /// Usage ID `0x90D`: "VT_STR"
    VT_STR = 0x90D,
    /// Usage ID `0x90E`: "VT_CLSID"
    VT_CLSID = 0x90E,
    /// Usage ID `0x90F`: "VT_VECTOR VT_UI1"
    VT_VECTORVT_UI1 = 0x90F,
    /// Usage ID `0x910`: "VT_F16E0"
    VT_F16E0 = 0x910,
    /// Usage ID `0x911`: "VT_F16E1"
    VT_F16E1 = 0x911,
    /// Usage ID `0x912`: "VT_F16E2"
    VT_F16E2 = 0x912,
    /// Usage ID `0x913`: "VT_F16E3"
    VT_F16E3 = 0x913,
    /// Usage ID `0x914`: "VT_F16E4"
    VT_F16E4 = 0x914,
    /// Usage ID `0x915`: "VT_F16E5"
    VT_F16E5 = 0x915,
    /// Usage ID `0x916`: "VT_F16E6"
    VT_F16E6 = 0x916,
    /// Usage ID `0x917`: "VT_F16E7"
    VT_F16E7 = 0x917,
    /// Usage ID `0x918`: "VT_F16E8"
    VT_F16E8 = 0x918,
    /// Usage ID `0x919`: "VT_F16E9"
    VT_F16E9 = 0x919,
    /// Usage ID `0x91A`: "VT_F16EA"
    VT_F16EA = 0x91A,
    /// Usage ID `0x91B`: "VT_F16EB"
    VT_F16EB = 0x91B,
    /// Usage ID `0x91C`: "VT_F16EC"
    VT_F16EC = 0x91C,
    /// Usage ID `0x91D`: "VT_F16ED"
    VT_F16ED = 0x91D,
    /// Usage ID `0x91E`: "VT_F16EE"
    VT_F16EE = 0x91E,
    /// Usage ID `0x91F`: "VT_F16EF"
    VT_F16EF = 0x91F,
    /// Usage ID `0x920`: "VT_F32E0"
    VT_F32E0 = 0x920,
    /// Usage ID `0x921`: "VT_F32E1"
    VT_F32E1 = 0x921,
    /// Usage ID `0x922`: "VT_F32E2"
    VT_F32E2 = 0x922,
    /// Usage ID `0x923`: "VT_F32E3"
    VT_F32E3 = 0x923,
    /// Usage ID `0x924`: "VT_F32E4"
    VT_F32E4 = 0x924,
    /// Usage ID `0x925`: "VT_F32E5"
    VT_F32E5 = 0x925,
    /// Usage ID `0x926`: "VT_F32E6"
    VT_F32E6 = 0x926,
    /// Usage ID `0x927`: "VT_F32E7"
    VT_F32E7 = 0x927,
    /// Usage ID `0x928`: "VT_F32E8"
    VT_F32E8 = 0x928,
    /// Usage ID `0x929`: "VT_F32E9"
    VT_F32E9 = 0x929,
    /// Usage ID `0x92A`: "VT_F32EA"
    VT_F32EA = 0x92A,
    /// Usage ID `0x92B`: "VT_F32EB"
    VT_F32EB = 0x92B,
    /// Usage ID `0x92C`: "VT_F32EC"
    VT_F32EC = 0x92C,
    /// Usage ID `0x92D`: "VT_F32ED"
    VT_F32ED = 0x92D,
    /// Usage ID `0x92E`: "VT_F32EE"
    VT_F32EE = 0x92E,
    /// Usage ID `0x92F`: "VT_F32EF"
    VT_F32EF = 0x92F,
    /// Usage ID `0x930`: "Activity Type: Unknown"
    ActivityTypeUnknown = 0x930,
    /// Usage ID `0x931`: "Activity Type: Stationary"
    ActivityTypeStationary = 0x931,
    /// Usage ID `0x932`: "Activity Type: Fidgeting"
    ActivityTypeFidgeting = 0x932,
    /// Usage ID `0x933`: "Activity Type: Walking"
    ActivityTypeWalking = 0x933,
    /// Usage ID `0x934`: "Activity Type: Running"
    ActivityTypeRunning = 0x934,
    /// Usage ID `0x935`: "Activity Type: In Vehicle"
    ActivityTypeInVehicle = 0x935,
    /// Usage ID `0x936`: "Activity Type: Biking"
    ActivityTypeBiking = 0x936,
    /// Usage ID `0x937`: "Activity Type: Idle"
    ActivityTypeIdle = 0x937,
    /// Usage ID `0x940`: "Unit: Not Specified"
    UnitNotSpecified = 0x940,
    /// Usage ID `0x941`: "Unit: Lux"
    UnitLux = 0x941,
    /// Usage ID `0x942`: "Unit: Degrees Kelvin"
    UnitDegreesKelvin = 0x942,
    /// Usage ID `0x943`: "Unit: Degrees Celsius"
    UnitDegreesCelsius = 0x943,
    /// Usage ID `0x944`: "Unit: Pascal"
    UnitPascal = 0x944,
    /// Usage ID `0x945`: "Unit: Newton"
    UnitNewton = 0x945,
    /// Usage ID `0x946`: "Unit: Meters/Second"
    UnitMetersSecond = 0x946,
    /// Usage ID `0x947`: "Unit: Kilogram"
    UnitKilogram = 0x947,
    /// Usage ID `0x948`: "Unit: Meter"
    UnitMeter = 0x948,
    /// Usage ID `0x949`: "Unit: Meters/Second/Second"
    UnitMetersSecondSecond = 0x949,
    /// Usage ID `0x94A`: "Unit: Farad"
    UnitFarad = 0x94A,
    /// Usage ID `0x94B`: "Unit: Ampere"
    UnitAmpere = 0x94B,
    /// Usage ID `0x94C`: "Unit: Watt"
    UnitWatt = 0x94C,
    /// Usage ID `0x94D`: "Unit: Henry"
    UnitHenry = 0x94D,
    /// Usage ID `0x94E`: "Unit: Ohm"
    UnitOhm = 0x94E,
    /// Usage ID `0x94F`: "Unit: Volt"
    UnitVolt = 0x94F,
    /// Usage ID `0x950`: "Unit: Hertz"
    UnitHertz = 0x950,
    /// Usage ID `0x951`: "Unit: Bar"
    UnitBar = 0x951,
    /// Usage ID `0x952`: "Unit: Degrees Anti-clockwise"
    UnitDegreesAnticlockwise = 0x952,
    /// Usage ID `0x953`: "Unit: Degrees Clockwise"
    UnitDegreesClockwise = 0x953,
    /// Usage ID `0x954`: "Unit: Degrees"
    UnitDegrees = 0x954,
    /// Usage ID `0x955`: "Unit: Degrees/Second"
    UnitDegreesSecond = 0x955,
    /// Usage ID `0x956`: "Unit: Degrees/Second/Second"
    UnitDegreesSecondSecond = 0x956,
    /// Usage ID `0x957`: "Unit: Knot"
    UnitKnot = 0x957,
    /// Usage ID `0x958`: "Unit: Percent"
    UnitPercent = 0x958,
    /// Usage ID `0x959`: "Unit: Second"
    UnitSecond = 0x959,
    /// Usage ID `0x95A`: "Unit: Millisecond"
    UnitMillisecond = 0x95A,
    /// Usage ID `0x95B`: "Unit: G"
    UnitG = 0x95B,
    /// Usage ID `0x95C`: "Unit: Bytes"
    UnitBytes = 0x95C,
    /// Usage ID `0x95D`: "Unit: Milligauss"
    UnitMilligauss = 0x95D,
    /// Usage ID `0x95E`: "Unit: Bits"
    UnitBits = 0x95E,
    /// Usage ID `0x960`: "Activity State: No State Change"
    ActivityStateNoStateChange = 0x960,
    /// Usage ID `0x961`: "Activity State: Start Activity"
    ActivityStateStartActivity = 0x961,
    /// Usage ID `0x962`: "Activity State: End Activity"
    ActivityStateEndActivity = 0x962,
    /// Usage ID `0x970`: "Exponent 0"
    Exponent0 = 0x970,
    /// Usage ID `0x971`: "Exponent 1"
    Exponent1 = 0x971,
    /// Usage ID `0x972`: "Exponent 2"
    Exponent2 = 0x972,
    /// Usage ID `0x973`: "Exponent 3"
    Exponent3 = 0x973,
    /// Usage ID `0x974`: "Exponent 4"
    Exponent4 = 0x974,
    /// Usage ID `0x975`: "Exponent 5"
    Exponent5 = 0x975,
    /// Usage ID `0x976`: "Exponent 6"
    Exponent6 = 0x976,
    /// Usage ID `0x977`: "Exponent 7"
    Exponent7 = 0x977,
    /// Usage ID `0x978`: "Exponent 8"
    Exponent8 = 0x978,
    /// Usage ID `0x979`: "Exponent 9"
    Exponent9 = 0x979,
    /// Usage ID `0x97A`: "Exponent A"
    ExponentA = 0x97A,
    /// Usage ID `0x97B`: "Exponent B"
    ExponentB = 0x97B,
    /// Usage ID `0x97C`: "Exponent C"
    ExponentC = 0x97C,
    /// Usage ID `0x97D`: "Exponent D"
    ExponentD = 0x97D,
    /// Usage ID `0x97E`: "Exponent E"
    ExponentE = 0x97E,
    /// Usage ID `0x97F`: "Exponent F"
    ExponentF = 0x97F,
    /// Usage ID `0x980`: "Device Position: Unknown"
    DevicePositionUnknown = 0x980,
    /// Usage ID `0x981`: "Device Position: Unchanged"
    DevicePositionUnchanged = 0x981,
    /// Usage ID `0x982`: "Device Position: On Desk"
    DevicePositionOnDesk = 0x982,
    /// Usage ID `0x983`: "Device Position: In Hand"
    DevicePositionInHand = 0x983,
    /// Usage ID `0x984`: "Device Position: Moving in Bag"
    DevicePositionMovinginBag = 0x984,
    /// Usage ID `0x985`: "Device Position: Stationary in Bag"
    DevicePositionStationaryinBag = 0x985,
    /// Usage ID `0x990`: "Step Type: Unknown"
    StepTypeUnknown = 0x990,
    /// Usage ID `0x991`: "Step Type: Walking"
    StepTypeWalking = 0x991,
    /// Usage ID `0x992`: "Step Type: Running"
    StepTypeRunning = 0x992,
    /// Usage ID `0x9A0`: "Gesture State: Unknown"
    GestureStateUnknown = 0x9A0,
    /// Usage ID `0x9A1`: "Gesture State: Started"
    GestureStateStarted = 0x9A1,
    /// Usage ID `0x9A2`: "Gesture State: Completed"
    GestureStateCompleted = 0x9A2,
    /// Usage ID `0x9A3`: "Gesture State: Cancelled"
    GestureStateCancelled = 0x9A3,
    /// Usage ID `0x9B0`: "Hinge Fold Contributing Panel: Unknown"
    HingeFoldContributingPanelUnknown = 0x9B0,
    /// Usage ID `0x9B1`: "Hinge Fold Contributing Panel: Panel 1"
    HingeFoldContributingPanelPanel1 = 0x9B1,
    /// Usage ID `0x9B2`: "Hinge Fold Contributing Panel: Panel 2"
    HingeFoldContributingPanelPanel2 = 0x9B2,
    /// Usage ID `0x9B3`: "Hinge Fold Contributing Panel: Both"
    HingeFoldContributingPanelBoth = 0x9B3,
    /// Usage ID `0x9B4`: "Hinge Fold Type: Unknown"
    HingeFoldTypeUnknown = 0x9B4,
    /// Usage ID `0x9B5`: "Hinge Fold Type: Increasing"
    HingeFoldTypeIncreasing = 0x9B5,
    /// Usage ID `0x9B6`: "Hinge Fold Type: Decreasing"
    HingeFoldTypeDecreasing = 0x9B6,
    /// Usage ID `0x9C0`: "Human Presence Detection Type: Vendor-Defined Non-Biometric"
    HumanPresenceDetectionTypeVendorDefinedNonBiometric = 0x9C0,
    /// Usage ID `0x9C1`: "Human Presence Detection Type: Vendor-Defined Biometric"
    HumanPresenceDetectionTypeVendorDefinedBiometric = 0x9C1,
    /// Usage ID `0x9C2`: "Human Presence Detection Type: Facial Biometric"
    HumanPresenceDetectionTypeFacialBiometric = 0x9C2,
    /// Usage ID `0x9C3`: "Human Presence Detection Type: Audio Biometric"
    HumanPresenceDetectionTypeAudioBiometric = 0x9C3,
    /// Usage ID `0x1000`: "Modifier: Change Sensitivity Absolute"
    ModifierChangeSensitivityAbsolute = 0x1000,
    /// Usage ID `0x2000`: "Modifier: Maximum"
    ModifierMaximum = 0x2000,
    /// Usage ID `0x3000`: "Modifier: Minimum"
    ModifierMinimum = 0x3000,
    /// Usage ID `0x4000`: "Modifier: Accuracy"
    ModifierAccuracy = 0x4000,
    /// Usage ID `0x5000`: "Modifier: Resolution"
    ModifierResolution = 0x5000,
    /// Usage ID `0x6000`: "Modifier: Threshold High"
    ModifierThresholdHigh = 0x6000,
    /// Usage ID `0x7000`: "Modifier: Threshold Low"
    ModifierThresholdLow = 0x7000,
    /// Usage ID `0x8000`: "Modifier: Calibration Offset"
    ModifierCalibrationOffset = 0x8000,
    /// Usage ID `0x9000`: "Modifier: Calibration Multiplier"
    ModifierCalibrationMultiplier = 0x9000,
    /// Usage ID `0xA000`: "Modifier: Report Interval"
    ModifierReportInterval = 0xA000,
    /// Usage ID `0xB000`: "Modifier: Frequency Max"
    ModifierFrequencyMax = 0xB000,
    /// Usage ID `0xC000`: "Modifier: Period Max"
    ModifierPeriodMax = 0xC000,
    /// Usage ID `0xD000`: "Modifier: Change Sensitivity Percent of Range"
    ModifierChangeSensitivityPercentofRange = 0xD000,
    /// Usage ID `0xE000`: "Modifier: Change Sensitivity Percent Relative"
    ModifierChangeSensitivityPercentRelative = 0xE000,
    /// Usage ID `0xF000`: "Modifier: Vendor Reserved"
    ModifierVendorReserved = 0xF000,
}

impl Sensors {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Sensors::Sensor => "Sensor",
            Sensors::Biometric => "Biometric",
            Sensors::BiometricHumanPresence => "Biometric: Human Presence",
            Sensors::BiometricHumanProximity => "Biometric: Human Proximity",
            Sensors::BiometricHumanTouch => "Biometric: Human Touch",
            Sensors::BiometricBloodPressure => "Biometric: Blood Pressure",
            Sensors::BiometricBodyTemperature => "Biometric: Body Temperature",
            Sensors::BiometricHeartRate => "Biometric: Heart Rate",
            Sensors::BiometricHeartRateVariability => "Biometric: Heart Rate Variability",
            Sensors::BiometricPeripheralOxygenSaturation => {
                "Biometric: Peripheral Oxygen Saturation"
            }
            Sensors::BiometricRespiratoryRate => "Biometric: Respiratory Rate",
            Sensors::Electrical => "Electrical",
            Sensors::ElectricalCapacitance => "Electrical: Capacitance",
            Sensors::ElectricalCurrent => "Electrical: Current",
            Sensors::ElectricalPower => "Electrical: Power",
            Sensors::ElectricalInductance => "Electrical: Inductance",
            Sensors::ElectricalResistance => "Electrical: Resistance",
            Sensors::ElectricalVoltage => "Electrical: Voltage",
            Sensors::ElectricalPotentiometer => "Electrical: Potentiometer",
            Sensors::ElectricalFrequency => "Electrical: Frequency",
            Sensors::ElectricalPeriod => "Electrical: Period",
            Sensors::Environmental => "Environmental",
            Sensors::EnvironmentalAtmosphericPressure => "Environmental: Atmospheric Pressure",
            Sensors::EnvironmentalHumidity => "Environmental: Humidity",
            Sensors::EnvironmentalTemperature => "Environmental: Temperature",
            Sensors::EnvironmentalWindDirection => "Environmental: Wind Direction",
            Sensors::EnvironmentalWindSpeed => "Environmental: Wind Speed",
            Sensors::EnvironmentalAirQuality => "Environmental: Air Quality",
            Sensors::EnvironmentalHeatIndex => "Environmental: Heat Index",
            Sensors::EnvironmentalSurfaceTemperature => "Environmental: Surface Temperature",
            Sensors::EnvironmentalVolatileOrganicCompounds => {
                "Environmental: Volatile Organic Compounds"
            }
            Sensors::EnvironmentalObjectPresence => "Environmental: Object Presence",
            Sensors::EnvironmentalObjectProximity => "Environmental: Object Proximity",
            Sensors::Light => "Light",
            Sensors::LightAmbientLight => "Light: Ambient Light",
            Sensors::LightConsumerInfrared => "Light: Consumer Infrared",
            Sensors::LightInfraredLight => "Light: Infrared Light",
            Sensors::LightVisibleLight => "Light: Visible Light",
            Sensors::LightUltravioletLight => "Light: Ultraviolet Light",
            Sensors::Location => "Location",
            Sensors::LocationBroadcast => "Location: Broadcast",
            Sensors::LocationDeadReckoning => "Location: Dead Reckoning",
            Sensors::LocationGPSGlobalPositioningSystem => {
                "Location: GPS (Global Positioning System)"
            }
            Sensors::LocationLookup => "Location: Lookup",
            Sensors::LocationOther => "Location: Other",
            Sensors::LocationStatic => "Location: Static",
            Sensors::LocationTriangulation => "Location: Triangulation",
            Sensors::Mechanical => "Mechanical",
            Sensors::MechanicalBooleanSwitch => "Mechanical: Boolean Switch",
            Sensors::MechanicalBooleanSwitchArray => "Mechanical: Boolean Switch Array",
            Sensors::MechanicalMultivalueSwitch => "Mechanical: Multivalue Switch",
            Sensors::MechanicalForce => "Mechanical: Force",
            Sensors::MechanicalPressure => "Mechanical: Pressure",
            Sensors::MechanicalStrain => "Mechanical: Strain",
            Sensors::MechanicalWeight => "Mechanical: Weight",
            Sensors::MechanicalHapticVibrator => "Mechanical: Haptic Vibrator",
            Sensors::MechanicalHallEffectSwitch => "Mechanical: Hall Effect Switch",
            Sensors::Motion => "Motion",
            Sensors::MotionAccelerometer1D => "Motion: Accelerometer 1D",
            Sensors::MotionAccelerometer2D => "Motion: Accelerometer 2D",
            Sensors::MotionAccelerometer3D => "Motion: Accelerometer 3D",
            Sensors::MotionGyrometer1D => "Motion: Gyrometer 1D",
            Sensors::MotionGyrometer2D => "Motion: Gyrometer 2D",
            Sensors::MotionGyrometer3D => "Motion: Gyrometer 3D",
            Sensors::MotionMotionDetector => "Motion: Motion Detector",
            Sensors::MotionSpeedometer => "Motion: Speedometer",
            Sensors::MotionAccelerometer => "Motion: Accelerometer",
            Sensors::MotionGyrometer => "Motion: Gyrometer",
            Sensors::MotionGravityVector => "Motion: Gravity Vector",
            Sensors::MotionLinearAccelerometer => "Motion: Linear Accelerometer",
            Sensors::Orientation => "Orientation",
            Sensors::OrientationCompass1D => "Orientation: Compass 1D",
            Sensors::OrientationCompass2D => "Orientation: Compass 2D",
            Sensors::OrientationCompass3D => "Orientation: Compass 3D",
            Sensors::OrientationInclinometer1D => "Orientation: Inclinometer 1D",
            Sensors::OrientationInclinometer2D => "Orientation: Inclinometer 2D",
            Sensors::OrientationInclinometer3D => "Orientation: Inclinometer 3D",
            Sensors::OrientationDistance1D => "Orientation: Distance 1D",
            Sensors::OrientationDistance2D => "Orientation: Distance 2D",
            Sensors::OrientationDistance3D => "Orientation: Distance 3D",
            Sensors::OrientationDeviceOrientation => "Orientation: Device Orientation",
            Sensors::OrientationCompass => "Orientation: Compass",
            Sensors::OrientationInclinometer => "Orientation: Inclinometer",
            Sensors::OrientationDistance => "Orientation: Distance",
            Sensors::OrientationRelativeOrientation => "Orientation: Relative Orientation",
            Sensors::OrientationSimpleOrientation => "Orientation: Simple Orientation",
            Sensors::Scanner => "Scanner",
            Sensors::ScannerBarcode => "Scanner: Barcode",
            Sensors::ScannerRFID => "Scanner: RFID",
            Sensors::ScannerNFC => "Scanner: NFC",
            Sensors::Time => "Time",
            Sensors::TimeAlarmTimer => "Time: Alarm Timer",
            Sensors::TimeRealTimeClock => "Time: Real Time Clock",
            Sensors::PersonalActivity => "Personal Activity",
            Sensors::PersonalActivityActivityDetection => "Personal Activity: Activity Detection",
            Sensors::PersonalActivityDevicePosition => "Personal Activity: Device Position",
            Sensors::PersonalActivityFloorTracker => "Personal Activity: Floor Tracker",
            Sensors::PersonalActivityPedometer => "Personal Activity: Pedometer",
            Sensors::PersonalActivityStepDetection => "Personal Activity: Step Detection",
            Sensors::OrientationExtended => "Orientation Extended",
            Sensors::OrientationExtendedGeomagneticOrientation => {
                "Orientation Extended: Geomagnetic Orientation"
            }
            Sensors::OrientationExtendedMagnetometer => "Orientation Extended: Magnetometer",
            Sensors::Gesture => "Gesture",
            Sensors::GestureChassisFlipGesture => "Gesture: Chassis Flip Gesture",
            Sensors::GestureHingeFoldGesture => "Gesture: Hinge Fold Gesture",
            Sensors::Other => "Other",
            Sensors::OtherCustom => "Other: Custom",
            Sensors::OtherGeneric => "Other: Generic",
            Sensors::OtherGenericEnumerator => "Other: Generic Enumerator",
            Sensors::OtherHingeAngle => "Other: Hinge Angle",
            Sensors::VendorReserved1 => "Vendor Reserved 1",
            Sensors::VendorReserved2 => "Vendor Reserved 2",
            Sensors::VendorReserved3 => "Vendor Reserved 3",
            Sensors::VendorReserved4 => "Vendor Reserved 4",
            Sensors::VendorReserved5 => "Vendor Reserved 5",
            Sensors::VendorReserved6 => "Vendor Reserved 6",
            Sensors::VendorReserved7 => "Vendor Reserved 7",
            Sensors::VendorReserved8 => "Vendor Reserved 8",
            Sensors::VendorReserved9 => "Vendor Reserved 9",
            Sensors::VendorReserved10 => "Vendor Reserved 10",
            Sensors::VendorReserved11 => "Vendor Reserved 11",
            Sensors::VendorReserved12 => "Vendor Reserved 12",
            Sensors::VendorReserved13 => "Vendor Reserved 13",
            Sensors::VendorReserved14 => "Vendor Reserved 14",
            Sensors::VendorReserved15 => "Vendor Reserved 15",
            Sensors::VendorReserved16 => "Vendor Reserved 16",
            Sensors::Event => "Event",
            Sensors::EventSensorState => "Event: Sensor State",
            Sensors::EventSensorEvent => "Event: Sensor Event",
            Sensors::Property => "Property",
            Sensors::PropertyFriendlyName => "Property: Friendly Name",
            Sensors::PropertyPersistentUniqueID => "Property: Persistent Unique ID",
            Sensors::PropertySensorStatus => "Property: Sensor Status",
            Sensors::PropertyMinimumReportInterval => "Property: Minimum Report Interval",
            Sensors::PropertySensorManufacturer => "Property: Sensor Manufacturer",
            Sensors::PropertySensorModel => "Property: Sensor Model",
            Sensors::PropertySensorSerialNumber => "Property: Sensor Serial Number",
            Sensors::PropertySensorDescription => "Property: Sensor Description",
            Sensors::PropertySensorConnectionType => "Property: Sensor Connection Type",
            Sensors::PropertySensorDevicePath => "Property: Sensor Device Path",
            Sensors::PropertyHardwareRevision => "Property: Hardware Revision",
            Sensors::PropertyFirmwareVersion => "Property: Firmware Version",
            Sensors::PropertyReleaseDate => "Property: Release Date",
            Sensors::PropertyReportInterval => "Property: Report Interval",
            Sensors::PropertyChangeSensitivityAbsolute => "Property: Change Sensitivity Absolute",
            Sensors::PropertyChangeSensitivityPercentofRange => {
                "Property: Change Sensitivity Percent of Range"
            }
            Sensors::PropertyChangeSensitivityPercentRelative => {
                "Property: Change Sensitivity Percent Relative"
            }
            Sensors::PropertyAccuracy => "Property: Accuracy",
            Sensors::PropertyResolution => "Property: Resolution",
            Sensors::PropertyMaximum => "Property: Maximum",
            Sensors::PropertyMinimum => "Property: Minimum",
            Sensors::PropertyReportingState => "Property: Reporting State",
            Sensors::PropertySamplingRate => "Property: Sampling Rate",
            Sensors::PropertyResponseCurve => "Property: Response Curve",
            Sensors::PropertyPowerState => "Property: Power State",
            Sensors::PropertyMaximumFIFOEvents => "Property: Maximum FIFO Events",
            Sensors::PropertyReportLatency => "Property: Report Latency",
            Sensors::PropertyFlushFIFOEvents => "Property: Flush FIFO Events",
            Sensors::PropertyMaximumPowerConsumption => "Property: Maximum Power Consumption",
            Sensors::PropertyIsPrimary => "Property: Is Primary",
            Sensors::PropertyHumanPresenceDetectionType => {
                "Property: Human Presence Detection Type"
            }
            Sensors::DataFieldLocation => "Data Field: Location",
            Sensors::DataFieldAltitudeAntennaSeaLevel => "Data Field: Altitude Antenna Sea Level",
            Sensors::DataFieldDifferentialReferenceStationID => {
                "Data Field: Differential Reference Station ID"
            }
            Sensors::DataFieldAltitudeEllipsoidError => "Data Field: Altitude Ellipsoid Error",
            Sensors::DataFieldAltitudeEllipsoid => "Data Field: Altitude Ellipsoid",
            Sensors::DataFieldAltitudeSeaLevelError => "Data Field: Altitude Sea Level Error",
            Sensors::DataFieldAltitudeSeaLevel => "Data Field: Altitude Sea Level",
            Sensors::DataFieldDifferentialGPSDataAge => "Data Field: Differential GPS Data Age",
            Sensors::DataFieldErrorRadius => "Data Field: Error Radius",
            Sensors::DataFieldFixQuality => "Data Field: Fix Quality",
            Sensors::DataFieldFixType => "Data Field: Fix Type",
            Sensors::DataFieldGeoidalSeparation => "Data Field: Geoidal Separation",
            Sensors::DataFieldGPSOperationMode => "Data Field: GPS Operation Mode",
            Sensors::DataFieldGPSSelectionMode => "Data Field: GPS Selection Mode",
            Sensors::DataFieldGPSStatus => "Data Field: GPS Status",
            Sensors::DataFieldPositionDilutionofPrecision => {
                "Data Field: Position Dilution of Precision"
            }
            Sensors::DataFieldHorizontalDilutionofPrecision => {
                "Data Field: Horizontal Dilution of Precision"
            }
            Sensors::DataFieldVerticalDilutionofPrecision => {
                "Data Field: Vertical Dilution of Precision"
            }
            Sensors::DataFieldLatitude => "Data Field: Latitude",
            Sensors::DataFieldLongitude => "Data Field: Longitude",
            Sensors::DataFieldTrueHeading => "Data Field: True Heading",
            Sensors::DataFieldMagneticHeading => "Data Field: Magnetic Heading",
            Sensors::DataFieldMagneticVariation => "Data Field: Magnetic Variation",
            Sensors::DataFieldSpeed => "Data Field: Speed",
            Sensors::DataFieldSatellitesinView => "Data Field: Satellites in View",
            Sensors::DataFieldSatellitesinViewAzimuth => "Data Field: Satellites in View Azimuth",
            Sensors::DataFieldSatellitesinViewElevation => {
                "Data Field: Satellites in View Elevation"
            }
            Sensors::DataFieldSatellitesinViewIDs => "Data Field: Satellites in View IDs",
            Sensors::DataFieldSatellitesinViewPRNs => "Data Field: Satellites in View PRNs",
            Sensors::DataFieldSatellitesinViewSNRatios => {
                "Data Field: Satellites in View S/N Ratios"
            }
            Sensors::DataFieldSatellitesUsedCount => "Data Field: Satellites Used Count",
            Sensors::DataFieldSatellitesUsedPRNs => "Data Field: Satellites Used PRNs",
            Sensors::DataFieldNMEASentence => "Data Field: NMEA Sentence",
            Sensors::DataFieldAddressLine1 => "Data Field: Address Line 1",
            Sensors::DataFieldAddressLine2 => "Data Field: Address Line 2",
            Sensors::DataFieldCity => "Data Field: City",
            Sensors::DataFieldStateorProvince => "Data Field: State or Province",
            Sensors::DataFieldCountryorRegion => "Data Field: Country or Region",
            Sensors::DataFieldPostalCode => "Data Field: Postal Code",
            Sensors::PropertyLocation => "Property: Location",
            Sensors::PropertyLocationDesiredAccuracy => "Property: Location Desired Accuracy",
            Sensors::DataFieldEnvironmental => "Data Field: Environmental",
            Sensors::DataFieldAtmosphericPressure => "Data Field: Atmospheric Pressure",
            Sensors::DataFieldRelativeHumidity => "Data Field: Relative Humidity",
            Sensors::DataFieldTemperature => "Data Field: Temperature",
            Sensors::DataFieldWindDirection => "Data Field: Wind Direction",
            Sensors::DataFieldWindSpeed => "Data Field: Wind Speed",
            Sensors::DataFieldAirQualityIndex => "Data Field: Air Quality Index",
            Sensors::DataFieldEquivalentCO2 => "Data Field: Equivalent CO2",
            Sensors::DataFieldVolatileOrganicCompoundConcentration => {
                "Data Field: Volatile Organic Compound Concentration"
            }
            Sensors::DataFieldObjectPresence => "Data Field: Object Presence",
            Sensors::DataFieldObjectProximityRange => "Data Field: Object Proximity Range",
            Sensors::DataFieldObjectProximityOutofRange => {
                "Data Field: Object Proximity Out of Range"
            }
            Sensors::PropertyEnvironmental => "Property: Environmental",
            Sensors::PropertyReferencePressure => "Property: Reference Pressure",
            Sensors::DataFieldMotion => "Data Field: Motion",
            Sensors::DataFieldMotionState => "Data Field: Motion State",
            Sensors::DataFieldAcceleration => "Data Field: Acceleration",
            Sensors::DataFieldAccelerationAxisX => "Data Field: Acceleration Axis X",
            Sensors::DataFieldAccelerationAxisY => "Data Field: Acceleration Axis Y",
            Sensors::DataFieldAccelerationAxisZ => "Data Field: Acceleration Axis Z",
            Sensors::DataFieldAngularVelocity => "Data Field: Angular Velocity",
            Sensors::DataFieldAngularVelocityaboutXAxis => {
                "Data Field: Angular Velocity about X Axis"
            }
            Sensors::DataFieldAngularVelocityaboutYAxis => {
                "Data Field: Angular Velocity about Y Axis"
            }
            Sensors::DataFieldAngularVelocityaboutZAxis => {
                "Data Field: Angular Velocity about Z Axis"
            }
            Sensors::DataFieldAngularPosition => "Data Field: Angular Position",
            Sensors::DataFieldAngularPositionaboutXAxis => {
                "Data Field: Angular Position about X Axis"
            }
            Sensors::DataFieldAngularPositionaboutYAxis => {
                "Data Field: Angular Position about Y Axis"
            }
            Sensors::DataFieldAngularPositionaboutZAxis => {
                "Data Field: Angular Position about Z Axis"
            }
            Sensors::DataFieldMotionSpeed => "Data Field: Motion Speed",
            Sensors::DataFieldMotionIntensity => "Data Field: Motion Intensity",
            Sensors::DataFieldOrientation => "Data Field: Orientation",
            Sensors::DataFieldHeading => "Data Field: Heading",
            Sensors::DataFieldHeadingXAxis => "Data Field: Heading X Axis",
            Sensors::DataFieldHeadingYAxis => "Data Field: Heading Y Axis",
            Sensors::DataFieldHeadingZAxis => "Data Field: Heading Z Axis",
            Sensors::DataFieldHeadingCompensatedMagneticNorth => {
                "Data Field: Heading Compensated Magnetic North"
            }
            Sensors::DataFieldHeadingCompensatedTrueNorth => {
                "Data Field: Heading Compensated True North"
            }
            Sensors::DataFieldHeadingMagneticNorth => "Data Field: Heading Magnetic North",
            Sensors::DataFieldHeadingTrueNorth => "Data Field: Heading True North",
            Sensors::DataFieldDistance => "Data Field: Distance",
            Sensors::DataFieldDistanceXAxis => "Data Field: Distance X Axis",
            Sensors::DataFieldDistanceYAxis => "Data Field: Distance Y Axis",
            Sensors::DataFieldDistanceZAxis => "Data Field: Distance Z Axis",
            Sensors::DataFieldDistanceOutofRange => "Data Field: Distance Out-of-Range",
            Sensors::DataFieldTilt => "Data Field: Tilt",
            Sensors::DataFieldTiltXAxis => "Data Field: Tilt X Axis",
            Sensors::DataFieldTiltYAxis => "Data Field: Tilt Y Axis",
            Sensors::DataFieldTiltZAxis => "Data Field: Tilt Z Axis",
            Sensors::DataFieldRotationMatrix => "Data Field: Rotation Matrix",
            Sensors::DataFieldQuaternion => "Data Field: Quaternion",
            Sensors::DataFieldMagneticFlux => "Data Field: Magnetic Flux",
            Sensors::DataFieldMagneticFluxXAxis => "Data Field: Magnetic Flux X Axis",
            Sensors::DataFieldMagneticFluxYAxis => "Data Field: Magnetic Flux Y Axis",
            Sensors::DataFieldMagneticFluxZAxis => "Data Field: Magnetic Flux Z Axis",
            Sensors::DataFieldMagnetometerAccuracy => "Data Field: Magnetometer Accuracy",
            Sensors::DataFieldSimpleOrientationDirection => {
                "Data Field: Simple Orientation Direction"
            }
            Sensors::DataFieldMechanical => "Data Field: Mechanical",
            Sensors::DataFieldBooleanSwitchState => "Data Field: Boolean Switch State",
            Sensors::DataFieldBooleanSwitchArrayStates => "Data Field: Boolean Switch Array States",
            Sensors::DataFieldMultivalueSwitchValue => "Data Field: Multivalue Switch Value",
            Sensors::DataFieldForce => "Data Field: Force",
            Sensors::DataFieldAbsolutePressure => "Data Field: Absolute Pressure",
            Sensors::DataFieldGaugePressure => "Data Field: Gauge Pressure",
            Sensors::DataFieldStrain => "Data Field: Strain",
            Sensors::DataFieldWeight => "Data Field: Weight",
            Sensors::PropertyMechanical => "Property: Mechanical",
            Sensors::PropertyVibrationState => "Property: Vibration State",
            Sensors::PropertyForwardVibrationSpeed => "Property: Forward Vibration Speed",
            Sensors::PropertyBackwardVibrationSpeed => "Property: Backward Vibration Speed",
            Sensors::DataFieldBiometric => "Data Field: Biometric",
            Sensors::DataFieldHumanPresence => "Data Field: Human Presence",
            Sensors::DataFieldHumanProximityRange => "Data Field: Human Proximity Range",
            Sensors::DataFieldHumanProximityOutofRange => {
                "Data Field: Human Proximity Out of Range"
            }
            Sensors::DataFieldHumanTouchState => "Data Field: Human Touch State",
            Sensors::DataFieldBloodPressure => "Data Field: Blood Pressure",
            Sensors::DataFieldBloodPressureDiastolic => "Data Field: Blood Pressure Diastolic",
            Sensors::DataFieldBloodPressureSystolic => "Data Field: Blood Pressure Systolic",
            Sensors::DataFieldHeartRate => "Data Field: Heart Rate",
            Sensors::DataFieldRestingHeartRate => "Data Field: Resting Heart Rate",
            Sensors::DataFieldHeartbeatInterval => "Data Field: Heartbeat Interval",
            Sensors::DataFieldRespiratoryRate => "Data Field: Respiratory Rate",
            Sensors::DataFieldSpO2 => "Data Field: SpO2",
            Sensors::DataFieldHumanAttentionDetected => "Data Field: Human Attention Detected",
            Sensors::DataFieldHumanHeadAzimuth => "Data Field: Human Head Azimuth",
            Sensors::DataFieldHumanHeadAltitude => "Data Field: Human Head Altitude",
            Sensors::DataFieldHumanHeadRoll => "Data Field: Human Head Roll",
            Sensors::DataFieldHumanHeadPitch => "Data Field: Human Head Pitch",
            Sensors::DataFieldHumanHeadYaw => "Data Field: Human Head Yaw",
            Sensors::DataFieldHumanCorrelationId => "Data Field: Human Correlation Id",
            Sensors::DataFieldLight => "Data Field: Light",
            Sensors::DataFieldIlluminance => "Data Field: Illuminance",
            Sensors::DataFieldColorTemperature => "Data Field: Color Temperature",
            Sensors::DataFieldChromaticity => "Data Field: Chromaticity",
            Sensors::DataFieldChromaticityX => "Data Field: Chromaticity X",
            Sensors::DataFieldChromaticityY => "Data Field: Chromaticity Y",
            Sensors::DataFieldConsumerIRSentenceReceive => {
                "Data Field: Consumer IR Sentence Receive"
            }
            Sensors::DataFieldInfraredLight => "Data Field: Infrared Light",
            Sensors::DataFieldRedLight => "Data Field: Red Light",
            Sensors::DataFieldGreenLight => "Data Field: Green Light",
            Sensors::DataFieldBlueLight => "Data Field: Blue Light",
            Sensors::DataFieldUltravioletALight => "Data Field: Ultraviolet A Light",
            Sensors::DataFieldUltravioletBLight => "Data Field: Ultraviolet B Light",
            Sensors::DataFieldUltravioletIndex => "Data Field: Ultraviolet Index",
            Sensors::DataFieldNearInfraredLight => "Data Field: Near Infrared Light",
            Sensors::PropertyLight => "Property: Light",
            Sensors::PropertyConsumerIRSentenceSend => "Property: Consumer IR Sentence Send",
            Sensors::PropertyAutoBrightnessPreferred => "Property: Auto Brightness Preferred",
            Sensors::PropertyAutoColorPreferred => "Property: Auto Color Preferred",
            Sensors::DataFieldScanner => "Data Field: Scanner",
            Sensors::DataFieldRFIDTag40Bit => "Data Field: RFID Tag 40 Bit",
            Sensors::DataFieldNFCSentenceReceive => "Data Field: NFC Sentence Receive",
            Sensors::PropertyScanner => "Property: Scanner",
            Sensors::PropertyNFCSentenceSend => "Property: NFC Sentence Send",
            Sensors::DataFieldElectrical => "Data Field: Electrical",
            Sensors::DataFieldCapacitance => "Data Field: Capacitance",
            Sensors::DataFieldCurrent => "Data Field: Current",
            Sensors::DataFieldElectricalPower => "Data Field: Electrical Power",
            Sensors::DataFieldInductance => "Data Field: Inductance",
            Sensors::DataFieldResistance => "Data Field: Resistance",
            Sensors::DataFieldVoltage => "Data Field: Voltage",
            Sensors::DataFieldFrequency => "Data Field: Frequency",
            Sensors::DataFieldPeriod => "Data Field: Period",
            Sensors::DataFieldPercentofRange => "Data Field: Percent of Range",
            Sensors::DataFieldTime => "Data Field: Time",
            Sensors::DataFieldYear => "Data Field: Year",
            Sensors::DataFieldMonth => "Data Field: Month",
            Sensors::DataFieldDay => "Data Field: Day",
            Sensors::DataFieldDayofWeek => "Data Field: Day of Week",
            Sensors::DataFieldHour => "Data Field: Hour",
            Sensors::DataFieldMinute => "Data Field: Minute",
            Sensors::DataFieldSecond => "Data Field: Second",
            Sensors::DataFieldMillisecond => "Data Field: Millisecond",
            Sensors::DataFieldTimestamp => "Data Field: Timestamp",
            Sensors::DataFieldJulianDayofYear => "Data Field: Julian Day of Year",
            Sensors::DataFieldTimeSinceSystemBoot => "Data Field: Time Since System Boot",
            Sensors::PropertyTime => "Property: Time",
            Sensors::PropertyTimeZoneOffsetfromUTC => "Property: Time Zone Offset from UTC",
            Sensors::PropertyTimeZoneName => "Property: Time Zone Name",
            Sensors::PropertyDaylightSavingsTimeObserved => {
                "Property: Daylight Savings Time Observed"
            }
            Sensors::PropertyTimeTrimAdjustment => "Property: Time Trim Adjustment",
            Sensors::PropertyArmAlarm => "Property: Arm Alarm",
            Sensors::DataFieldCustom => "Data Field: Custom",
            Sensors::DataFieldCustomUsage => "Data Field: Custom Usage",
            Sensors::DataFieldCustomBooleanArray => "Data Field: Custom Boolean Array",
            Sensors::DataFieldCustomValue => "Data Field: Custom Value",
            Sensors::DataFieldCustomValue1 => "Data Field: Custom Value 1",
            Sensors::DataFieldCustomValue2 => "Data Field: Custom Value 2",
            Sensors::DataFieldCustomValue3 => "Data Field: Custom Value 3",
            Sensors::DataFieldCustomValue4 => "Data Field: Custom Value 4",
            Sensors::DataFieldCustomValue5 => "Data Field: Custom Value 5",
            Sensors::DataFieldCustomValue6 => "Data Field: Custom Value 6",
            Sensors::DataFieldCustomValue7 => "Data Field: Custom Value 7",
            Sensors::DataFieldCustomValue8 => "Data Field: Custom Value 8",
            Sensors::DataFieldCustomValue9 => "Data Field: Custom Value 9",
            Sensors::DataFieldCustomValue10 => "Data Field: Custom Value 10",
            Sensors::DataFieldCustomValue11 => "Data Field: Custom Value 11",
            Sensors::DataFieldCustomValue12 => "Data Field: Custom Value 12",
            Sensors::DataFieldCustomValue13 => "Data Field: Custom Value 13",
            Sensors::DataFieldCustomValue14 => "Data Field: Custom Value 14",
            Sensors::DataFieldCustomValue15 => "Data Field: Custom Value 15",
            Sensors::DataFieldCustomValue16 => "Data Field: Custom Value 16",
            Sensors::DataFieldCustomValue17 => "Data Field: Custom Value 17",
            Sensors::DataFieldCustomValue18 => "Data Field: Custom Value 18",
            Sensors::DataFieldCustomValue19 => "Data Field: Custom Value 19",
            Sensors::DataFieldCustomValue20 => "Data Field: Custom Value 20",
            Sensors::DataFieldCustomValue21 => "Data Field: Custom Value 21",
            Sensors::DataFieldCustomValue22 => "Data Field: Custom Value 22",
            Sensors::DataFieldCustomValue23 => "Data Field: Custom Value 23",
            Sensors::DataFieldCustomValue24 => "Data Field: Custom Value 24",
            Sensors::DataFieldCustomValue25 => "Data Field: Custom Value 25",
            Sensors::DataFieldCustomValue26 => "Data Field: Custom Value 26",
            Sensors::DataFieldCustomValue27 => "Data Field: Custom Value 27",
            Sensors::DataFieldCustomValue28 => "Data Field: Custom Value 28",
            Sensors::DataFieldGeneric => "Data Field: Generic",
            Sensors::DataFieldGenericGUIDorPROPERTYKEY => "Data Field: Generic GUID or PROPERTYKEY",
            Sensors::DataFieldGenericCategoryGUID => "Data Field: Generic Category GUID",
            Sensors::DataFieldGenericTypeGUID => "Data Field: Generic Type GUID",
            Sensors::DataFieldGenericEventPROPERTYKEY => "Data Field: Generic Event PROPERTYKEY",
            Sensors::DataFieldGenericPropertyPROPERTYKEY => {
                "Data Field: Generic Property PROPERTYKEY"
            }
            Sensors::DataFieldGenericDataFieldPROPERTYKEY => {
                "Data Field: Generic Data Field PROPERTYKEY"
            }
            Sensors::DataFieldGenericEvent => "Data Field: Generic Event",
            Sensors::DataFieldGenericProperty => "Data Field: Generic Property",
            Sensors::DataFieldGenericDataField => "Data Field: Generic Data Field",
            Sensors::DataFieldEnumeratorTableRowIndex => "Data Field: Enumerator Table Row Index",
            Sensors::DataFieldEnumeratorTableRowCount => "Data Field: Enumerator Table Row Count",
            Sensors::DataFieldGenericGUIDorPROPERTYKEYkind => {
                "Data Field: Generic GUID or PROPERTYKEY kind"
            }
            Sensors::DataFieldGenericGUID => "Data Field: Generic GUID",
            Sensors::DataFieldGenericPROPERTYKEY => "Data Field: Generic PROPERTYKEY",
            Sensors::DataFieldGenericTopLevelCollectionID => {
                "Data Field: Generic Top Level Collection ID"
            }
            Sensors::DataFieldGenericReportID => "Data Field: Generic Report ID",
            Sensors::DataFieldGenericReportItemPositionIndex => {
                "Data Field: Generic Report Item Position Index"
            }
            Sensors::DataFieldGenericFirmwareVARTYPE => "Data Field: Generic Firmware VARTYPE",
            Sensors::DataFieldGenericUnitofMeasure => "Data Field: Generic Unit of Measure",
            Sensors::DataFieldGenericUnitExponent => "Data Field: Generic Unit Exponent",
            Sensors::DataFieldGenericReportSize => "Data Field: Generic Report Size",
            Sensors::DataFieldGenericReportCount => "Data Field: Generic Report Count",
            Sensors::PropertyGeneric => "Property: Generic",
            Sensors::PropertyEnumeratorTableRowIndex => "Property: Enumerator Table Row Index",
            Sensors::PropertyEnumeratorTableRowCount => "Property: Enumerator Table Row Count",
            Sensors::DataFieldPersonalActivity => "Data Field: Personal Activity",
            Sensors::DataFieldActivityType => "Data Field: Activity Type",
            Sensors::DataFieldActivityState => "Data Field: Activity State",
            Sensors::DataFieldDevicePosition => "Data Field: Device Position",
            Sensors::DataFieldStepCount => "Data Field: Step Count",
            Sensors::DataFieldStepCountReset => "Data Field: Step Count Reset",
            Sensors::DataFieldStepDuration => "Data Field: Step Duration",
            Sensors::DataFieldStepType => "Data Field: Step Type",
            Sensors::PropertyMinimumActivityDetectionInterval => {
                "Property: Minimum Activity Detection Interval"
            }
            Sensors::PropertySupportedActivityTypes => "Property: Supported Activity Types",
            Sensors::PropertySubscribedActivityTypes => "Property: Subscribed Activity Types",
            Sensors::PropertySupportedStepTypes => "Property: Supported Step Types",
            Sensors::PropertySubscribedStepTypes => "Property: Subscribed Step Types",
            Sensors::PropertyFloorHeight => "Property: Floor Height",
            Sensors::DataFieldCustomTypeID => "Data Field: Custom Type ID",
            Sensors::PropertyCustom => "Property: Custom",
            Sensors::PropertyCustomValue1 => "Property: Custom Value 1",
            Sensors::PropertyCustomValue2 => "Property: Custom Value 2",
            Sensors::PropertyCustomValue3 => "Property: Custom Value 3",
            Sensors::PropertyCustomValue4 => "Property: Custom Value 4",
            Sensors::PropertyCustomValue5 => "Property: Custom Value 5",
            Sensors::PropertyCustomValue6 => "Property: Custom Value 6",
            Sensors::PropertyCustomValue7 => "Property: Custom Value 7",
            Sensors::PropertyCustomValue8 => "Property: Custom Value 8",
            Sensors::PropertyCustomValue9 => "Property: Custom Value 9",
            Sensors::PropertyCustomValue10 => "Property: Custom Value 10",
            Sensors::PropertyCustomValue11 => "Property: Custom Value 11",
            Sensors::PropertyCustomValue12 => "Property: Custom Value 12",
            Sensors::PropertyCustomValue13 => "Property: Custom Value 13",
            Sensors::PropertyCustomValue14 => "Property: Custom Value 14",
            Sensors::PropertyCustomValue15 => "Property: Custom Value 15",
            Sensors::PropertyCustomValue16 => "Property: Custom Value 16",
            Sensors::DataFieldHinge => "Data Field: Hinge",
            Sensors::DataFieldHingeAngle => "Data Field: Hinge Angle",
            Sensors::DataFieldGestureSensor => "Data Field: Gesture Sensor",
            Sensors::DataFieldGestureState => "Data Field: Gesture State",
            Sensors::DataFieldHingeFoldInitialAngle => "Data Field: Hinge Fold Initial Angle",
            Sensors::DataFieldHingeFoldFinalAngle => "Data Field: Hinge Fold Final Angle",
            Sensors::DataFieldHingeFoldContributingPanel => {
                "Data Field: Hinge Fold Contributing Panel"
            }
            Sensors::DataFieldHingeFoldType => "Data Field: Hinge Fold Type",
            Sensors::SensorStateUndefined => "Sensor State: Undefined",
            Sensors::SensorStateReady => "Sensor State: Ready",
            Sensors::SensorStateNotAvailable => "Sensor State: Not Available",
            Sensors::SensorStateNoData => "Sensor State: No Data",
            Sensors::SensorStateInitializing => "Sensor State: Initializing",
            Sensors::SensorStateAccessDenied => "Sensor State: Access Denied",
            Sensors::SensorStateError => "Sensor State: Error",
            Sensors::SensorEventUnknown => "Sensor Event: Unknown",
            Sensors::SensorEventStateChanged => "Sensor Event: State Changed",
            Sensors::SensorEventPropertyChanged => "Sensor Event: Property Changed",
            Sensors::SensorEventDataUpdated => "Sensor Event: Data Updated",
            Sensors::SensorEventPollResponse => "Sensor Event: Poll Response",
            Sensors::SensorEventChangeSensitivity => "Sensor Event: Change Sensitivity",
            Sensors::SensorEventRangeMaximumReached => "Sensor Event: Range Maximum Reached",
            Sensors::SensorEventRangeMinimumReached => "Sensor Event: Range Minimum Reached",
            Sensors::SensorEventHighThresholdCrossUpward => {
                "Sensor Event: High Threshold Cross Upward"
            }
            Sensors::SensorEventHighThresholdCrossDownward => {
                "Sensor Event: High Threshold Cross Downward"
            }
            Sensors::SensorEventLowThresholdCrossUpward => {
                "Sensor Event: Low Threshold Cross Upward"
            }
            Sensors::SensorEventLowThresholdCrossDownward => {
                "Sensor Event: Low Threshold Cross Downward"
            }
            Sensors::SensorEventZeroThresholdCrossUpward => {
                "Sensor Event: Zero Threshold Cross Upward"
            }
            Sensors::SensorEventZeroThresholdCrossDownward => {
                "Sensor Event: Zero Threshold Cross Downward"
            }
            Sensors::SensorEventPeriodExceeded => "Sensor Event: Period Exceeded",
            Sensors::SensorEventFrequencyExceeded => "Sensor Event: Frequency Exceeded",
            Sensors::SensorEventComplexTrigger => "Sensor Event: Complex Trigger",
            Sensors::ConnectionTypePCIntegrated => "Connection Type: PC Integrated",
            Sensors::ConnectionTypePCAttached => "Connection Type: PC Attached",
            Sensors::ConnectionTypePCExternal => "Connection Type: PC External",
            Sensors::ReportingStateReportNoEvents => "Reporting State: Report No Events",
            Sensors::ReportingStateReportAllEvents => "Reporting State: Report All Events",
            Sensors::ReportingStateReportThresholdEvents => {
                "Reporting State: Report Threshold Events"
            }
            Sensors::ReportingStateWakeOnNoEvents => "Reporting State: Wake On No Events",
            Sensors::ReportingStateWakeOnAllEvents => "Reporting State: Wake On All Events",
            Sensors::ReportingStateWakeOnThresholdEvents => {
                "Reporting State: Wake On Threshold Events"
            }
            Sensors::ReportingStateAnytime => "Reporting State: Anytime",
            Sensors::PowerStateUndefined => "Power State: Undefined",
            Sensors::PowerStateD0FullPower => "Power State: D0 Full Power",
            Sensors::PowerStateD1LowPower => "Power State: D1 Low Power",
            Sensors::PowerStateD2StandbyPowerwithWakeup => {
                "Power State: D2 Standby Power with Wakeup"
            }
            Sensors::PowerStateD3SleepwithWakeup => "Power State: D3 Sleep with Wakeup",
            Sensors::PowerStateD4PowerOff => "Power State: D4 Power Off",
            Sensors::AccuracyDefault => "Accuracy: Default",
            Sensors::AccuracyHigh => "Accuracy: High",
            Sensors::AccuracyMedium => "Accuracy: Medium",
            Sensors::AccuracyLow => "Accuracy: Low",
            Sensors::FixQualityNoFix => "Fix Quality: No Fix",
            Sensors::FixQualityGPS => "Fix Quality: GPS",
            Sensors::FixQualityDGPS => "Fix Quality: DGPS",
            Sensors::FixTypeNoFix => "Fix Type: No Fix",
            Sensors::FixTypeGPSSPSModeFixValid => "Fix Type: GPS SPS Mode, Fix Valid",
            Sensors::FixTypeDGPSSPSModeFixValid => "Fix Type: DGPS SPS Mode, Fix Valid",
            Sensors::FixTypeGPSPPSModeFixValid => "Fix Type: GPS PPS Mode, Fix Valid",
            Sensors::FixTypeRealTimeKinematic => "Fix Type: Real Time Kinematic",
            Sensors::FixTypeFloatRTK => "Fix Type: Float RTK",
            Sensors::FixTypeEstimateddeadreckoned => "Fix Type: Estimated (dead reckoned)",
            Sensors::FixTypeManualInputMode => "Fix Type: Manual Input Mode",
            Sensors::FixTypeSimulatorMode => "Fix Type: Simulator Mode",
            Sensors::GPSOperationModeManual => "GPS Operation Mode: Manual",
            Sensors::GPSOperationModeAutomatic => "GPS Operation Mode: Automatic",
            Sensors::GPSSelectionModeAutonomous => "GPS Selection Mode: Autonomous",
            Sensors::GPSSelectionModeDGPS => "GPS Selection Mode: DGPS",
            Sensors::GPSSelectionModeEstimateddeadreckoned => {
                "GPS Selection Mode: Estimated (dead reckoned)"
            }
            Sensors::GPSSelectionModeManualInput => "GPS Selection Mode: Manual Input",
            Sensors::GPSSelectionModeSimulator => "GPS Selection Mode: Simulator",
            Sensors::GPSSelectionModeDataNotValid => "GPS Selection Mode: Data Not Valid",
            Sensors::GPSStatusDataValid => "GPS Status Data: Valid",
            Sensors::GPSStatusDataNotValid => "GPS Status Data: Not Valid",
            Sensors::DayofWeekSunday => "Day of Week: Sunday",
            Sensors::DayofWeekMonday => "Day of Week: Monday",
            Sensors::DayofWeekTuesday => "Day of Week: Tuesday",
            Sensors::DayofWeekWednesday => "Day of Week: Wednesday",
            Sensors::DayofWeekThursday => "Day of Week: Thursday",
            Sensors::DayofWeekFriday => "Day of Week: Friday",
            Sensors::DayofWeekSaturday => "Day of Week: Saturday",
            Sensors::KindCategory => "Kind: Category",
            Sensors::KindType => "Kind: Type",
            Sensors::KindEvent => "Kind: Event",
            Sensors::KindProperty => "Kind: Property",
            Sensors::KindDataField => "Kind: Data Field",
            Sensors::MagnetometerAccuracyLow => "Magnetometer Accuracy: Low",
            Sensors::MagnetometerAccuracyMedium => "Magnetometer Accuracy: Medium",
            Sensors::MagnetometerAccuracyHigh => "Magnetometer Accuracy: High",
            Sensors::SimpleOrientationDirectionNotRotated => {
                "Simple Orientation Direction: Not Rotated"
            }
            Sensors::SimpleOrientationDirectionRotated90DegreesCCW => {
                "Simple Orientation Direction: Rotated 90 Degrees CCW"
            }
            Sensors::SimpleOrientationDirectionRotated180DegreesCCW => {
                "Simple Orientation Direction: Rotated 180 Degrees CCW"
            }
            Sensors::SimpleOrientationDirectionRotated270DegreesCCW => {
                "Simple Orientation Direction: Rotated 270 Degrees CCW"
            }
            Sensors::SimpleOrientationDirectionFaceUp => "Simple Orientation Direction: Face Up",
            Sensors::SimpleOrientationDirectionFaceDown => {
                "Simple Orientation Direction: Face Down"
            }
            Sensors::VT_NULL => "VT_NULL",
            Sensors::VT_BOOL => "VT_BOOL",
            Sensors::VT_UI1 => "VT_UI1",
            Sensors::VT_I1 => "VT_I1",
            Sensors::VT_UI2 => "VT_UI2",
            Sensors::VT_I2 => "VT_I2",
            Sensors::VT_UI4 => "VT_UI4",
            Sensors::VT_I4 => "VT_I4",
            Sensors::VT_UI8 => "VT_UI8",
            Sensors::VT_I8 => "VT_I8",
            Sensors::VT_R4 => "VT_R4",
            Sensors::VT_R8 => "VT_R8",
            Sensors::VT_WSTR => "VT_WSTR",
            Sensors::VT_STR => "VT_STR",
            Sensors::VT_CLSID => "VT_CLSID",
            Sensors::VT_VECTORVT_UI1 => "VT_VECTOR VT_UI1",
            Sensors::VT_F16E0 => "VT_F16E0",
            Sensors::VT_F16E1 => "VT_F16E1",
            Sensors::VT_F16E2 => "VT_F16E2",
            Sensors::VT_F16E3 => "VT_F16E3",
            Sensors::VT_F16E4 => "VT_F16E4",
            Sensors::VT_F16E5 => "VT_F16E5",
            Sensors::VT_F16E6 => "VT_F16E6",
            Sensors::VT_F16E7 => "VT_F16E7",
            Sensors::VT_F16E8 => "VT_F16E8",
            Sensors::VT_F16E9 => "VT_F16E9",
            Sensors::VT_F16EA => "VT_F16EA",
            Sensors::VT_F16EB => "VT_F16EB",
            Sensors::VT_F16EC => "VT_F16EC",
            Sensors::VT_F16ED => "VT_F16ED",
            Sensors::VT_F16EE => "VT_F16EE",
            Sensors::VT_F16EF => "VT_F16EF",
            Sensors::VT_F32E0 => "VT_F32E0",
            Sensors::VT_F32E1 => "VT_F32E1",
            Sensors::VT_F32E2 => "VT_F32E2",
            Sensors::VT_F32E3 => "VT_F32E3",
            Sensors::VT_F32E4 => "VT_F32E4",
            Sensors::VT_F32E5 => "VT_F32E5",
            Sensors::VT_F32E6 => "VT_F32E6",
            Sensors::VT_F32E7 => "VT_F32E7",
            Sensors::VT_F32E8 => "VT_F32E8",
            Sensors::VT_F32E9 => "VT_F32E9",
            Sensors::VT_F32EA => "VT_F32EA",
            Sensors::VT_F32EB => "VT_F32EB",
            Sensors::VT_F32EC => "VT_F32EC",
            Sensors::VT_F32ED => "VT_F32ED",
            Sensors::VT_F32EE => "VT_F32EE",
            Sensors::VT_F32EF => "VT_F32EF",
            Sensors::ActivityTypeUnknown => "Activity Type: Unknown",
            Sensors::ActivityTypeStationary => "Activity Type: Stationary",
            Sensors::ActivityTypeFidgeting => "Activity Type: Fidgeting",
            Sensors::ActivityTypeWalking => "Activity Type: Walking",
            Sensors::ActivityTypeRunning => "Activity Type: Running",
            Sensors::ActivityTypeInVehicle => "Activity Type: In Vehicle",
            Sensors::ActivityTypeBiking => "Activity Type: Biking",
            Sensors::ActivityTypeIdle => "Activity Type: Idle",
            Sensors::UnitNotSpecified => "Unit: Not Specified",
            Sensors::UnitLux => "Unit: Lux",
            Sensors::UnitDegreesKelvin => "Unit: Degrees Kelvin",
            Sensors::UnitDegreesCelsius => "Unit: Degrees Celsius",
            Sensors::UnitPascal => "Unit: Pascal",
            Sensors::UnitNewton => "Unit: Newton",
            Sensors::UnitMetersSecond => "Unit: Meters/Second",
            Sensors::UnitKilogram => "Unit: Kilogram",
            Sensors::UnitMeter => "Unit: Meter",
            Sensors::UnitMetersSecondSecond => "Unit: Meters/Second/Second",
            Sensors::UnitFarad => "Unit: Farad",
            Sensors::UnitAmpere => "Unit: Ampere",
            Sensors::UnitWatt => "Unit: Watt",
            Sensors::UnitHenry => "Unit: Henry",
            Sensors::UnitOhm => "Unit: Ohm",
            Sensors::UnitVolt => "Unit: Volt",
            Sensors::UnitHertz => "Unit: Hertz",
            Sensors::UnitBar => "Unit: Bar",
            Sensors::UnitDegreesAnticlockwise => "Unit: Degrees Anti-clockwise",
            Sensors::UnitDegreesClockwise => "Unit: Degrees Clockwise",
            Sensors::UnitDegrees => "Unit: Degrees",
            Sensors::UnitDegreesSecond => "Unit: Degrees/Second",
            Sensors::UnitDegreesSecondSecond => "Unit: Degrees/Second/Second",
            Sensors::UnitKnot => "Unit: Knot",
            Sensors::UnitPercent => "Unit: Percent",
            Sensors::UnitSecond => "Unit: Second",
            Sensors::UnitMillisecond => "Unit: Millisecond",
            Sensors::UnitG => "Unit: G",
            Sensors::UnitBytes => "Unit: Bytes",
            Sensors::UnitMilligauss => "Unit: Milligauss",
            Sensors::UnitBits => "Unit: Bits",
            Sensors::ActivityStateNoStateChange => "Activity State: No State Change",
            Sensors::ActivityStateStartActivity => "Activity State: Start Activity",
            Sensors::ActivityStateEndActivity => "Activity State: End Activity",
            Sensors::Exponent0 => "Exponent 0",
            Sensors::Exponent1 => "Exponent 1",
            Sensors::Exponent2 => "Exponent 2",
            Sensors::Exponent3 => "Exponent 3",
            Sensors::Exponent4 => "Exponent 4",
            Sensors::Exponent5 => "Exponent 5",
            Sensors::Exponent6 => "Exponent 6",
            Sensors::Exponent7 => "Exponent 7",
            Sensors::Exponent8 => "Exponent 8",
            Sensors::Exponent9 => "Exponent 9",
            Sensors::ExponentA => "Exponent A",
            Sensors::ExponentB => "Exponent B",
            Sensors::ExponentC => "Exponent C",
            Sensors::ExponentD => "Exponent D",
            Sensors::ExponentE => "Exponent E",
            Sensors::ExponentF => "Exponent F",
            Sensors::DevicePositionUnknown => "Device Position: Unknown",
            Sensors::DevicePositionUnchanged => "Device Position: Unchanged",
            Sensors::DevicePositionOnDesk => "Device Position: On Desk",
            Sensors::DevicePositionInHand => "Device Position: In Hand",
            Sensors::DevicePositionMovinginBag => "Device Position: Moving in Bag",
            Sensors::DevicePositionStationaryinBag => "Device Position: Stationary in Bag",
            Sensors::StepTypeUnknown => "Step Type: Unknown",
            Sensors::StepTypeWalking => "Step Type: Walking",
            Sensors::StepTypeRunning => "Step Type: Running",
            Sensors::GestureStateUnknown => "Gesture State: Unknown",
            Sensors::GestureStateStarted => "Gesture State: Started",
            Sensors::GestureStateCompleted => "Gesture State: Completed",
            Sensors::GestureStateCancelled => "Gesture State: Cancelled",
            Sensors::HingeFoldContributingPanelUnknown => "Hinge Fold Contributing Panel: Unknown",
            Sensors::HingeFoldContributingPanelPanel1 => "Hinge Fold Contributing Panel: Panel 1",
            Sensors::HingeFoldContributingPanelPanel2 => "Hinge Fold Contributing Panel: Panel 2",
            Sensors::HingeFoldContributingPanelBoth => "Hinge Fold Contributing Panel: Both",
            Sensors::HingeFoldTypeUnknown => "Hinge Fold Type: Unknown",
            Sensors::HingeFoldTypeIncreasing => "Hinge Fold Type: Increasing",
            Sensors::HingeFoldTypeDecreasing => "Hinge Fold Type: Decreasing",
            Sensors::HumanPresenceDetectionTypeVendorDefinedNonBiometric => {
                "Human Presence Detection Type: Vendor-Defined Non-Biometric"
            }
            Sensors::HumanPresenceDetectionTypeVendorDefinedBiometric => {
                "Human Presence Detection Type: Vendor-Defined Biometric"
            }
            Sensors::HumanPresenceDetectionTypeFacialBiometric => {
                "Human Presence Detection Type: Facial Biometric"
            }
            Sensors::HumanPresenceDetectionTypeAudioBiometric => {
                "Human Presence Detection Type: Audio Biometric"
            }
            Sensors::ModifierChangeSensitivityAbsolute => "Modifier: Change Sensitivity Absolute",
            Sensors::ModifierMaximum => "Modifier: Maximum",
            Sensors::ModifierMinimum => "Modifier: Minimum",
            Sensors::ModifierAccuracy => "Modifier: Accuracy",
            Sensors::ModifierResolution => "Modifier: Resolution",
            Sensors::ModifierThresholdHigh => "Modifier: Threshold High",
            Sensors::ModifierThresholdLow => "Modifier: Threshold Low",
            Sensors::ModifierCalibrationOffset => "Modifier: Calibration Offset",
            Sensors::ModifierCalibrationMultiplier => "Modifier: Calibration Multiplier",
            Sensors::ModifierReportInterval => "Modifier: Report Interval",
            Sensors::ModifierFrequencyMax => "Modifier: Frequency Max",
            Sensors::ModifierPeriodMax => "Modifier: Period Max",
            Sensors::ModifierChangeSensitivityPercentofRange => {
                "Modifier: Change Sensitivity Percent of Range"
            }
            Sensors::ModifierChangeSensitivityPercentRelative => {
                "Modifier: Change Sensitivity Percent Relative"
            }
            Sensors::ModifierVendorReserved => "Modifier: Vendor Reserved",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Sensors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Sensors {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Sensors(self)](Usage::Sensors)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Sensors {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x20` for [Sensors]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Sensors]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Sensors> for u16 {
    fn from(sensors: &Sensors) -> u16 {
        *sensors as u16
    }
}

impl From<Sensors> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Sensors::usage_page_value()].
    fn from(sensors: Sensors) -> u16 {
        u16::from(&sensors)
    }
}

impl From<&Sensors> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Sensors::usage_value()].
    fn from(sensors: &Sensors) -> u32 {
        let up = UsagePage::from(sensors);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(sensors) as u32;
        up | id
    }
}

impl From<&Sensors> for UsagePage {
    /// Always returns [UsagePage::Sensors] and is
    /// identical to [Sensors::usage_page()].
    fn from(_: &Sensors) -> UsagePage {
        UsagePage::Sensors
    }
}

impl From<Sensors> for UsagePage {
    /// Always returns [UsagePage::Sensors] and is
    /// identical to [Sensors::usage_page()].
    fn from(_: Sensors) -> UsagePage {
        UsagePage::Sensors
    }
}

impl From<&Sensors> for Usage {
    fn from(sensors: &Sensors) -> Usage {
        Usage::try_from(u32::from(sensors)).unwrap()
    }
}

impl From<Sensors> for Usage {
    fn from(sensors: Sensors) -> Usage {
        Usage::from(&sensors)
    }
}

impl TryFrom<u16> for Sensors {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Sensors> {
        match usage_id {
            1 => Ok(Sensors::Sensor),
            16 => Ok(Sensors::Biometric),
            17 => Ok(Sensors::BiometricHumanPresence),
            18 => Ok(Sensors::BiometricHumanProximity),
            19 => Ok(Sensors::BiometricHumanTouch),
            20 => Ok(Sensors::BiometricBloodPressure),
            21 => Ok(Sensors::BiometricBodyTemperature),
            22 => Ok(Sensors::BiometricHeartRate),
            23 => Ok(Sensors::BiometricHeartRateVariability),
            24 => Ok(Sensors::BiometricPeripheralOxygenSaturation),
            25 => Ok(Sensors::BiometricRespiratoryRate),
            32 => Ok(Sensors::Electrical),
            33 => Ok(Sensors::ElectricalCapacitance),
            34 => Ok(Sensors::ElectricalCurrent),
            35 => Ok(Sensors::ElectricalPower),
            36 => Ok(Sensors::ElectricalInductance),
            37 => Ok(Sensors::ElectricalResistance),
            38 => Ok(Sensors::ElectricalVoltage),
            39 => Ok(Sensors::ElectricalPotentiometer),
            40 => Ok(Sensors::ElectricalFrequency),
            41 => Ok(Sensors::ElectricalPeriod),
            48 => Ok(Sensors::Environmental),
            49 => Ok(Sensors::EnvironmentalAtmosphericPressure),
            50 => Ok(Sensors::EnvironmentalHumidity),
            51 => Ok(Sensors::EnvironmentalTemperature),
            52 => Ok(Sensors::EnvironmentalWindDirection),
            53 => Ok(Sensors::EnvironmentalWindSpeed),
            54 => Ok(Sensors::EnvironmentalAirQuality),
            55 => Ok(Sensors::EnvironmentalHeatIndex),
            56 => Ok(Sensors::EnvironmentalSurfaceTemperature),
            57 => Ok(Sensors::EnvironmentalVolatileOrganicCompounds),
            58 => Ok(Sensors::EnvironmentalObjectPresence),
            59 => Ok(Sensors::EnvironmentalObjectProximity),
            64 => Ok(Sensors::Light),
            65 => Ok(Sensors::LightAmbientLight),
            66 => Ok(Sensors::LightConsumerInfrared),
            67 => Ok(Sensors::LightInfraredLight),
            68 => Ok(Sensors::LightVisibleLight),
            69 => Ok(Sensors::LightUltravioletLight),
            80 => Ok(Sensors::Location),
            81 => Ok(Sensors::LocationBroadcast),
            82 => Ok(Sensors::LocationDeadReckoning),
            83 => Ok(Sensors::LocationGPSGlobalPositioningSystem),
            84 => Ok(Sensors::LocationLookup),
            85 => Ok(Sensors::LocationOther),
            86 => Ok(Sensors::LocationStatic),
            87 => Ok(Sensors::LocationTriangulation),
            96 => Ok(Sensors::Mechanical),
            97 => Ok(Sensors::MechanicalBooleanSwitch),
            98 => Ok(Sensors::MechanicalBooleanSwitchArray),
            99 => Ok(Sensors::MechanicalMultivalueSwitch),
            100 => Ok(Sensors::MechanicalForce),
            101 => Ok(Sensors::MechanicalPressure),
            102 => Ok(Sensors::MechanicalStrain),
            103 => Ok(Sensors::MechanicalWeight),
            104 => Ok(Sensors::MechanicalHapticVibrator),
            105 => Ok(Sensors::MechanicalHallEffectSwitch),
            112 => Ok(Sensors::Motion),
            113 => Ok(Sensors::MotionAccelerometer1D),
            114 => Ok(Sensors::MotionAccelerometer2D),
            115 => Ok(Sensors::MotionAccelerometer3D),
            116 => Ok(Sensors::MotionGyrometer1D),
            117 => Ok(Sensors::MotionGyrometer2D),
            118 => Ok(Sensors::MotionGyrometer3D),
            119 => Ok(Sensors::MotionMotionDetector),
            120 => Ok(Sensors::MotionSpeedometer),
            121 => Ok(Sensors::MotionAccelerometer),
            122 => Ok(Sensors::MotionGyrometer),
            123 => Ok(Sensors::MotionGravityVector),
            124 => Ok(Sensors::MotionLinearAccelerometer),
            128 => Ok(Sensors::Orientation),
            129 => Ok(Sensors::OrientationCompass1D),
            130 => Ok(Sensors::OrientationCompass2D),
            131 => Ok(Sensors::OrientationCompass3D),
            132 => Ok(Sensors::OrientationInclinometer1D),
            133 => Ok(Sensors::OrientationInclinometer2D),
            134 => Ok(Sensors::OrientationInclinometer3D),
            135 => Ok(Sensors::OrientationDistance1D),
            136 => Ok(Sensors::OrientationDistance2D),
            137 => Ok(Sensors::OrientationDistance3D),
            138 => Ok(Sensors::OrientationDeviceOrientation),
            139 => Ok(Sensors::OrientationCompass),
            140 => Ok(Sensors::OrientationInclinometer),
            141 => Ok(Sensors::OrientationDistance),
            142 => Ok(Sensors::OrientationRelativeOrientation),
            143 => Ok(Sensors::OrientationSimpleOrientation),
            144 => Ok(Sensors::Scanner),
            145 => Ok(Sensors::ScannerBarcode),
            146 => Ok(Sensors::ScannerRFID),
            147 => Ok(Sensors::ScannerNFC),
            160 => Ok(Sensors::Time),
            161 => Ok(Sensors::TimeAlarmTimer),
            162 => Ok(Sensors::TimeRealTimeClock),
            176 => Ok(Sensors::PersonalActivity),
            177 => Ok(Sensors::PersonalActivityActivityDetection),
            178 => Ok(Sensors::PersonalActivityDevicePosition),
            179 => Ok(Sensors::PersonalActivityFloorTracker),
            180 => Ok(Sensors::PersonalActivityPedometer),
            181 => Ok(Sensors::PersonalActivityStepDetection),
            192 => Ok(Sensors::OrientationExtended),
            193 => Ok(Sensors::OrientationExtendedGeomagneticOrientation),
            194 => Ok(Sensors::OrientationExtendedMagnetometer),
            208 => Ok(Sensors::Gesture),
            209 => Ok(Sensors::GestureChassisFlipGesture),
            210 => Ok(Sensors::GestureHingeFoldGesture),
            224 => Ok(Sensors::Other),
            225 => Ok(Sensors::OtherCustom),
            226 => Ok(Sensors::OtherGeneric),
            227 => Ok(Sensors::OtherGenericEnumerator),
            228 => Ok(Sensors::OtherHingeAngle),
            240 => Ok(Sensors::VendorReserved1),
            241 => Ok(Sensors::VendorReserved2),
            242 => Ok(Sensors::VendorReserved3),
            243 => Ok(Sensors::VendorReserved4),
            244 => Ok(Sensors::VendorReserved5),
            245 => Ok(Sensors::VendorReserved6),
            246 => Ok(Sensors::VendorReserved7),
            247 => Ok(Sensors::VendorReserved8),
            248 => Ok(Sensors::VendorReserved9),
            249 => Ok(Sensors::VendorReserved10),
            250 => Ok(Sensors::VendorReserved11),
            251 => Ok(Sensors::VendorReserved12),
            252 => Ok(Sensors::VendorReserved13),
            253 => Ok(Sensors::VendorReserved14),
            254 => Ok(Sensors::VendorReserved15),
            255 => Ok(Sensors::VendorReserved16),
            512 => Ok(Sensors::Event),
            513 => Ok(Sensors::EventSensorState),
            514 => Ok(Sensors::EventSensorEvent),
            768 => Ok(Sensors::Property),
            769 => Ok(Sensors::PropertyFriendlyName),
            770 => Ok(Sensors::PropertyPersistentUniqueID),
            771 => Ok(Sensors::PropertySensorStatus),
            772 => Ok(Sensors::PropertyMinimumReportInterval),
            773 => Ok(Sensors::PropertySensorManufacturer),
            774 => Ok(Sensors::PropertySensorModel),
            775 => Ok(Sensors::PropertySensorSerialNumber),
            776 => Ok(Sensors::PropertySensorDescription),
            777 => Ok(Sensors::PropertySensorConnectionType),
            778 => Ok(Sensors::PropertySensorDevicePath),
            779 => Ok(Sensors::PropertyHardwareRevision),
            780 => Ok(Sensors::PropertyFirmwareVersion),
            781 => Ok(Sensors::PropertyReleaseDate),
            782 => Ok(Sensors::PropertyReportInterval),
            783 => Ok(Sensors::PropertyChangeSensitivityAbsolute),
            784 => Ok(Sensors::PropertyChangeSensitivityPercentofRange),
            785 => Ok(Sensors::PropertyChangeSensitivityPercentRelative),
            786 => Ok(Sensors::PropertyAccuracy),
            787 => Ok(Sensors::PropertyResolution),
            788 => Ok(Sensors::PropertyMaximum),
            789 => Ok(Sensors::PropertyMinimum),
            790 => Ok(Sensors::PropertyReportingState),
            791 => Ok(Sensors::PropertySamplingRate),
            792 => Ok(Sensors::PropertyResponseCurve),
            793 => Ok(Sensors::PropertyPowerState),
            794 => Ok(Sensors::PropertyMaximumFIFOEvents),
            795 => Ok(Sensors::PropertyReportLatency),
            796 => Ok(Sensors::PropertyFlushFIFOEvents),
            797 => Ok(Sensors::PropertyMaximumPowerConsumption),
            798 => Ok(Sensors::PropertyIsPrimary),
            799 => Ok(Sensors::PropertyHumanPresenceDetectionType),
            1024 => Ok(Sensors::DataFieldLocation),
            1026 => Ok(Sensors::DataFieldAltitudeAntennaSeaLevel),
            1027 => Ok(Sensors::DataFieldDifferentialReferenceStationID),
            1028 => Ok(Sensors::DataFieldAltitudeEllipsoidError),
            1029 => Ok(Sensors::DataFieldAltitudeEllipsoid),
            1030 => Ok(Sensors::DataFieldAltitudeSeaLevelError),
            1031 => Ok(Sensors::DataFieldAltitudeSeaLevel),
            1032 => Ok(Sensors::DataFieldDifferentialGPSDataAge),
            1033 => Ok(Sensors::DataFieldErrorRadius),
            1034 => Ok(Sensors::DataFieldFixQuality),
            1035 => Ok(Sensors::DataFieldFixType),
            1036 => Ok(Sensors::DataFieldGeoidalSeparation),
            1037 => Ok(Sensors::DataFieldGPSOperationMode),
            1038 => Ok(Sensors::DataFieldGPSSelectionMode),
            1039 => Ok(Sensors::DataFieldGPSStatus),
            1040 => Ok(Sensors::DataFieldPositionDilutionofPrecision),
            1041 => Ok(Sensors::DataFieldHorizontalDilutionofPrecision),
            1042 => Ok(Sensors::DataFieldVerticalDilutionofPrecision),
            1043 => Ok(Sensors::DataFieldLatitude),
            1044 => Ok(Sensors::DataFieldLongitude),
            1045 => Ok(Sensors::DataFieldTrueHeading),
            1046 => Ok(Sensors::DataFieldMagneticHeading),
            1047 => Ok(Sensors::DataFieldMagneticVariation),
            1048 => Ok(Sensors::DataFieldSpeed),
            1049 => Ok(Sensors::DataFieldSatellitesinView),
            1050 => Ok(Sensors::DataFieldSatellitesinViewAzimuth),
            1051 => Ok(Sensors::DataFieldSatellitesinViewElevation),
            1052 => Ok(Sensors::DataFieldSatellitesinViewIDs),
            1053 => Ok(Sensors::DataFieldSatellitesinViewPRNs),
            1054 => Ok(Sensors::DataFieldSatellitesinViewSNRatios),
            1055 => Ok(Sensors::DataFieldSatellitesUsedCount),
            1056 => Ok(Sensors::DataFieldSatellitesUsedPRNs),
            1057 => Ok(Sensors::DataFieldNMEASentence),
            1058 => Ok(Sensors::DataFieldAddressLine1),
            1059 => Ok(Sensors::DataFieldAddressLine2),
            1060 => Ok(Sensors::DataFieldCity),
            1061 => Ok(Sensors::DataFieldStateorProvince),
            1062 => Ok(Sensors::DataFieldCountryorRegion),
            1063 => Ok(Sensors::DataFieldPostalCode),
            1066 => Ok(Sensors::PropertyLocation),
            1067 => Ok(Sensors::PropertyLocationDesiredAccuracy),
            1072 => Ok(Sensors::DataFieldEnvironmental),
            1073 => Ok(Sensors::DataFieldAtmosphericPressure),
            1075 => Ok(Sensors::DataFieldRelativeHumidity),
            1076 => Ok(Sensors::DataFieldTemperature),
            1077 => Ok(Sensors::DataFieldWindDirection),
            1078 => Ok(Sensors::DataFieldWindSpeed),
            1079 => Ok(Sensors::DataFieldAirQualityIndex),
            1080 => Ok(Sensors::DataFieldEquivalentCO2),
            1081 => Ok(Sensors::DataFieldVolatileOrganicCompoundConcentration),
            1082 => Ok(Sensors::DataFieldObjectPresence),
            1083 => Ok(Sensors::DataFieldObjectProximityRange),
            1084 => Ok(Sensors::DataFieldObjectProximityOutofRange),
            1088 => Ok(Sensors::PropertyEnvironmental),
            1089 => Ok(Sensors::PropertyReferencePressure),
            1104 => Ok(Sensors::DataFieldMotion),
            1105 => Ok(Sensors::DataFieldMotionState),
            1106 => Ok(Sensors::DataFieldAcceleration),
            1107 => Ok(Sensors::DataFieldAccelerationAxisX),
            1108 => Ok(Sensors::DataFieldAccelerationAxisY),
            1109 => Ok(Sensors::DataFieldAccelerationAxisZ),
            1110 => Ok(Sensors::DataFieldAngularVelocity),
            1111 => Ok(Sensors::DataFieldAngularVelocityaboutXAxis),
            1112 => Ok(Sensors::DataFieldAngularVelocityaboutYAxis),
            1113 => Ok(Sensors::DataFieldAngularVelocityaboutZAxis),
            1114 => Ok(Sensors::DataFieldAngularPosition),
            1115 => Ok(Sensors::DataFieldAngularPositionaboutXAxis),
            1116 => Ok(Sensors::DataFieldAngularPositionaboutYAxis),
            1117 => Ok(Sensors::DataFieldAngularPositionaboutZAxis),
            1118 => Ok(Sensors::DataFieldMotionSpeed),
            1119 => Ok(Sensors::DataFieldMotionIntensity),
            1136 => Ok(Sensors::DataFieldOrientation),
            1137 => Ok(Sensors::DataFieldHeading),
            1138 => Ok(Sensors::DataFieldHeadingXAxis),
            1139 => Ok(Sensors::DataFieldHeadingYAxis),
            1140 => Ok(Sensors::DataFieldHeadingZAxis),
            1141 => Ok(Sensors::DataFieldHeadingCompensatedMagneticNorth),
            1142 => Ok(Sensors::DataFieldHeadingCompensatedTrueNorth),
            1143 => Ok(Sensors::DataFieldHeadingMagneticNorth),
            1144 => Ok(Sensors::DataFieldHeadingTrueNorth),
            1145 => Ok(Sensors::DataFieldDistance),
            1146 => Ok(Sensors::DataFieldDistanceXAxis),
            1147 => Ok(Sensors::DataFieldDistanceYAxis),
            1148 => Ok(Sensors::DataFieldDistanceZAxis),
            1149 => Ok(Sensors::DataFieldDistanceOutofRange),
            1150 => Ok(Sensors::DataFieldTilt),
            1151 => Ok(Sensors::DataFieldTiltXAxis),
            1152 => Ok(Sensors::DataFieldTiltYAxis),
            1153 => Ok(Sensors::DataFieldTiltZAxis),
            1154 => Ok(Sensors::DataFieldRotationMatrix),
            1155 => Ok(Sensors::DataFieldQuaternion),
            1156 => Ok(Sensors::DataFieldMagneticFlux),
            1157 => Ok(Sensors::DataFieldMagneticFluxXAxis),
            1158 => Ok(Sensors::DataFieldMagneticFluxYAxis),
            1159 => Ok(Sensors::DataFieldMagneticFluxZAxis),
            1160 => Ok(Sensors::DataFieldMagnetometerAccuracy),
            1161 => Ok(Sensors::DataFieldSimpleOrientationDirection),
            1168 => Ok(Sensors::DataFieldMechanical),
            1169 => Ok(Sensors::DataFieldBooleanSwitchState),
            1170 => Ok(Sensors::DataFieldBooleanSwitchArrayStates),
            1171 => Ok(Sensors::DataFieldMultivalueSwitchValue),
            1172 => Ok(Sensors::DataFieldForce),
            1173 => Ok(Sensors::DataFieldAbsolutePressure),
            1174 => Ok(Sensors::DataFieldGaugePressure),
            1175 => Ok(Sensors::DataFieldStrain),
            1176 => Ok(Sensors::DataFieldWeight),
            1184 => Ok(Sensors::PropertyMechanical),
            1185 => Ok(Sensors::PropertyVibrationState),
            1186 => Ok(Sensors::PropertyForwardVibrationSpeed),
            1187 => Ok(Sensors::PropertyBackwardVibrationSpeed),
            1200 => Ok(Sensors::DataFieldBiometric),
            1201 => Ok(Sensors::DataFieldHumanPresence),
            1202 => Ok(Sensors::DataFieldHumanProximityRange),
            1203 => Ok(Sensors::DataFieldHumanProximityOutofRange),
            1204 => Ok(Sensors::DataFieldHumanTouchState),
            1205 => Ok(Sensors::DataFieldBloodPressure),
            1206 => Ok(Sensors::DataFieldBloodPressureDiastolic),
            1207 => Ok(Sensors::DataFieldBloodPressureSystolic),
            1208 => Ok(Sensors::DataFieldHeartRate),
            1209 => Ok(Sensors::DataFieldRestingHeartRate),
            1210 => Ok(Sensors::DataFieldHeartbeatInterval),
            1211 => Ok(Sensors::DataFieldRespiratoryRate),
            1212 => Ok(Sensors::DataFieldSpO2),
            1213 => Ok(Sensors::DataFieldHumanAttentionDetected),
            1214 => Ok(Sensors::DataFieldHumanHeadAzimuth),
            1215 => Ok(Sensors::DataFieldHumanHeadAltitude),
            1216 => Ok(Sensors::DataFieldHumanHeadRoll),
            1217 => Ok(Sensors::DataFieldHumanHeadPitch),
            1218 => Ok(Sensors::DataFieldHumanHeadYaw),
            1219 => Ok(Sensors::DataFieldHumanCorrelationId),
            1232 => Ok(Sensors::DataFieldLight),
            1233 => Ok(Sensors::DataFieldIlluminance),
            1234 => Ok(Sensors::DataFieldColorTemperature),
            1235 => Ok(Sensors::DataFieldChromaticity),
            1236 => Ok(Sensors::DataFieldChromaticityX),
            1237 => Ok(Sensors::DataFieldChromaticityY),
            1238 => Ok(Sensors::DataFieldConsumerIRSentenceReceive),
            1239 => Ok(Sensors::DataFieldInfraredLight),
            1240 => Ok(Sensors::DataFieldRedLight),
            1241 => Ok(Sensors::DataFieldGreenLight),
            1242 => Ok(Sensors::DataFieldBlueLight),
            1243 => Ok(Sensors::DataFieldUltravioletALight),
            1244 => Ok(Sensors::DataFieldUltravioletBLight),
            1245 => Ok(Sensors::DataFieldUltravioletIndex),
            1246 => Ok(Sensors::DataFieldNearInfraredLight),
            1247 => Ok(Sensors::PropertyLight),
            1248 => Ok(Sensors::PropertyConsumerIRSentenceSend),
            1250 => Ok(Sensors::PropertyAutoBrightnessPreferred),
            1251 => Ok(Sensors::PropertyAutoColorPreferred),
            1264 => Ok(Sensors::DataFieldScanner),
            1265 => Ok(Sensors::DataFieldRFIDTag40Bit),
            1266 => Ok(Sensors::DataFieldNFCSentenceReceive),
            1272 => Ok(Sensors::PropertyScanner),
            1273 => Ok(Sensors::PropertyNFCSentenceSend),
            1280 => Ok(Sensors::DataFieldElectrical),
            1281 => Ok(Sensors::DataFieldCapacitance),
            1282 => Ok(Sensors::DataFieldCurrent),
            1283 => Ok(Sensors::DataFieldElectricalPower),
            1284 => Ok(Sensors::DataFieldInductance),
            1285 => Ok(Sensors::DataFieldResistance),
            1286 => Ok(Sensors::DataFieldVoltage),
            1287 => Ok(Sensors::DataFieldFrequency),
            1288 => Ok(Sensors::DataFieldPeriod),
            1289 => Ok(Sensors::DataFieldPercentofRange),
            1312 => Ok(Sensors::DataFieldTime),
            1313 => Ok(Sensors::DataFieldYear),
            1314 => Ok(Sensors::DataFieldMonth),
            1315 => Ok(Sensors::DataFieldDay),
            1316 => Ok(Sensors::DataFieldDayofWeek),
            1317 => Ok(Sensors::DataFieldHour),
            1318 => Ok(Sensors::DataFieldMinute),
            1319 => Ok(Sensors::DataFieldSecond),
            1320 => Ok(Sensors::DataFieldMillisecond),
            1321 => Ok(Sensors::DataFieldTimestamp),
            1322 => Ok(Sensors::DataFieldJulianDayofYear),
            1323 => Ok(Sensors::DataFieldTimeSinceSystemBoot),
            1328 => Ok(Sensors::PropertyTime),
            1329 => Ok(Sensors::PropertyTimeZoneOffsetfromUTC),
            1330 => Ok(Sensors::PropertyTimeZoneName),
            1331 => Ok(Sensors::PropertyDaylightSavingsTimeObserved),
            1332 => Ok(Sensors::PropertyTimeTrimAdjustment),
            1333 => Ok(Sensors::PropertyArmAlarm),
            1344 => Ok(Sensors::DataFieldCustom),
            1345 => Ok(Sensors::DataFieldCustomUsage),
            1346 => Ok(Sensors::DataFieldCustomBooleanArray),
            1347 => Ok(Sensors::DataFieldCustomValue),
            1348 => Ok(Sensors::DataFieldCustomValue1),
            1349 => Ok(Sensors::DataFieldCustomValue2),
            1350 => Ok(Sensors::DataFieldCustomValue3),
            1351 => Ok(Sensors::DataFieldCustomValue4),
            1352 => Ok(Sensors::DataFieldCustomValue5),
            1353 => Ok(Sensors::DataFieldCustomValue6),
            1354 => Ok(Sensors::DataFieldCustomValue7),
            1355 => Ok(Sensors::DataFieldCustomValue8),
            1356 => Ok(Sensors::DataFieldCustomValue9),
            1357 => Ok(Sensors::DataFieldCustomValue10),
            1358 => Ok(Sensors::DataFieldCustomValue11),
            1359 => Ok(Sensors::DataFieldCustomValue12),
            1360 => Ok(Sensors::DataFieldCustomValue13),
            1361 => Ok(Sensors::DataFieldCustomValue14),
            1362 => Ok(Sensors::DataFieldCustomValue15),
            1363 => Ok(Sensors::DataFieldCustomValue16),
            1364 => Ok(Sensors::DataFieldCustomValue17),
            1365 => Ok(Sensors::DataFieldCustomValue18),
            1366 => Ok(Sensors::DataFieldCustomValue19),
            1367 => Ok(Sensors::DataFieldCustomValue20),
            1368 => Ok(Sensors::DataFieldCustomValue21),
            1369 => Ok(Sensors::DataFieldCustomValue22),
            1370 => Ok(Sensors::DataFieldCustomValue23),
            1371 => Ok(Sensors::DataFieldCustomValue24),
            1372 => Ok(Sensors::DataFieldCustomValue25),
            1373 => Ok(Sensors::DataFieldCustomValue26),
            1374 => Ok(Sensors::DataFieldCustomValue27),
            1375 => Ok(Sensors::DataFieldCustomValue28),
            1376 => Ok(Sensors::DataFieldGeneric),
            1377 => Ok(Sensors::DataFieldGenericGUIDorPROPERTYKEY),
            1378 => Ok(Sensors::DataFieldGenericCategoryGUID),
            1379 => Ok(Sensors::DataFieldGenericTypeGUID),
            1380 => Ok(Sensors::DataFieldGenericEventPROPERTYKEY),
            1381 => Ok(Sensors::DataFieldGenericPropertyPROPERTYKEY),
            1382 => Ok(Sensors::DataFieldGenericDataFieldPROPERTYKEY),
            1383 => Ok(Sensors::DataFieldGenericEvent),
            1384 => Ok(Sensors::DataFieldGenericProperty),
            1385 => Ok(Sensors::DataFieldGenericDataField),
            1386 => Ok(Sensors::DataFieldEnumeratorTableRowIndex),
            1387 => Ok(Sensors::DataFieldEnumeratorTableRowCount),
            1388 => Ok(Sensors::DataFieldGenericGUIDorPROPERTYKEYkind),
            1389 => Ok(Sensors::DataFieldGenericGUID),
            1390 => Ok(Sensors::DataFieldGenericPROPERTYKEY),
            1391 => Ok(Sensors::DataFieldGenericTopLevelCollectionID),
            1392 => Ok(Sensors::DataFieldGenericReportID),
            1393 => Ok(Sensors::DataFieldGenericReportItemPositionIndex),
            1394 => Ok(Sensors::DataFieldGenericFirmwareVARTYPE),
            1395 => Ok(Sensors::DataFieldGenericUnitofMeasure),
            1396 => Ok(Sensors::DataFieldGenericUnitExponent),
            1397 => Ok(Sensors::DataFieldGenericReportSize),
            1398 => Ok(Sensors::DataFieldGenericReportCount),
            1408 => Ok(Sensors::PropertyGeneric),
            1409 => Ok(Sensors::PropertyEnumeratorTableRowIndex),
            1410 => Ok(Sensors::PropertyEnumeratorTableRowCount),
            1424 => Ok(Sensors::DataFieldPersonalActivity),
            1425 => Ok(Sensors::DataFieldActivityType),
            1426 => Ok(Sensors::DataFieldActivityState),
            1427 => Ok(Sensors::DataFieldDevicePosition),
            1428 => Ok(Sensors::DataFieldStepCount),
            1429 => Ok(Sensors::DataFieldStepCountReset),
            1430 => Ok(Sensors::DataFieldStepDuration),
            1431 => Ok(Sensors::DataFieldStepType),
            1440 => Ok(Sensors::PropertyMinimumActivityDetectionInterval),
            1441 => Ok(Sensors::PropertySupportedActivityTypes),
            1442 => Ok(Sensors::PropertySubscribedActivityTypes),
            1443 => Ok(Sensors::PropertySupportedStepTypes),
            1444 => Ok(Sensors::PropertySubscribedStepTypes),
            1445 => Ok(Sensors::PropertyFloorHeight),
            1456 => Ok(Sensors::DataFieldCustomTypeID),
            1472 => Ok(Sensors::PropertyCustom),
            1473 => Ok(Sensors::PropertyCustomValue1),
            1474 => Ok(Sensors::PropertyCustomValue2),
            1475 => Ok(Sensors::PropertyCustomValue3),
            1476 => Ok(Sensors::PropertyCustomValue4),
            1477 => Ok(Sensors::PropertyCustomValue5),
            1478 => Ok(Sensors::PropertyCustomValue6),
            1479 => Ok(Sensors::PropertyCustomValue7),
            1480 => Ok(Sensors::PropertyCustomValue8),
            1481 => Ok(Sensors::PropertyCustomValue9),
            1482 => Ok(Sensors::PropertyCustomValue10),
            1483 => Ok(Sensors::PropertyCustomValue11),
            1484 => Ok(Sensors::PropertyCustomValue12),
            1485 => Ok(Sensors::PropertyCustomValue13),
            1486 => Ok(Sensors::PropertyCustomValue14),
            1487 => Ok(Sensors::PropertyCustomValue15),
            1488 => Ok(Sensors::PropertyCustomValue16),
            1504 => Ok(Sensors::DataFieldHinge),
            1505 => Ok(Sensors::DataFieldHingeAngle),
            1520 => Ok(Sensors::DataFieldGestureSensor),
            1521 => Ok(Sensors::DataFieldGestureState),
            1522 => Ok(Sensors::DataFieldHingeFoldInitialAngle),
            1523 => Ok(Sensors::DataFieldHingeFoldFinalAngle),
            1524 => Ok(Sensors::DataFieldHingeFoldContributingPanel),
            1525 => Ok(Sensors::DataFieldHingeFoldType),
            2048 => Ok(Sensors::SensorStateUndefined),
            2049 => Ok(Sensors::SensorStateReady),
            2050 => Ok(Sensors::SensorStateNotAvailable),
            2051 => Ok(Sensors::SensorStateNoData),
            2052 => Ok(Sensors::SensorStateInitializing),
            2053 => Ok(Sensors::SensorStateAccessDenied),
            2054 => Ok(Sensors::SensorStateError),
            2064 => Ok(Sensors::SensorEventUnknown),
            2065 => Ok(Sensors::SensorEventStateChanged),
            2066 => Ok(Sensors::SensorEventPropertyChanged),
            2067 => Ok(Sensors::SensorEventDataUpdated),
            2068 => Ok(Sensors::SensorEventPollResponse),
            2069 => Ok(Sensors::SensorEventChangeSensitivity),
            2070 => Ok(Sensors::SensorEventRangeMaximumReached),
            2071 => Ok(Sensors::SensorEventRangeMinimumReached),
            2072 => Ok(Sensors::SensorEventHighThresholdCrossUpward),
            2073 => Ok(Sensors::SensorEventHighThresholdCrossDownward),
            2074 => Ok(Sensors::SensorEventLowThresholdCrossUpward),
            2075 => Ok(Sensors::SensorEventLowThresholdCrossDownward),
            2076 => Ok(Sensors::SensorEventZeroThresholdCrossUpward),
            2077 => Ok(Sensors::SensorEventZeroThresholdCrossDownward),
            2078 => Ok(Sensors::SensorEventPeriodExceeded),
            2079 => Ok(Sensors::SensorEventFrequencyExceeded),
            2080 => Ok(Sensors::SensorEventComplexTrigger),
            2096 => Ok(Sensors::ConnectionTypePCIntegrated),
            2097 => Ok(Sensors::ConnectionTypePCAttached),
            2098 => Ok(Sensors::ConnectionTypePCExternal),
            2112 => Ok(Sensors::ReportingStateReportNoEvents),
            2113 => Ok(Sensors::ReportingStateReportAllEvents),
            2114 => Ok(Sensors::ReportingStateReportThresholdEvents),
            2115 => Ok(Sensors::ReportingStateWakeOnNoEvents),
            2116 => Ok(Sensors::ReportingStateWakeOnAllEvents),
            2117 => Ok(Sensors::ReportingStateWakeOnThresholdEvents),
            2118 => Ok(Sensors::ReportingStateAnytime),
            2128 => Ok(Sensors::PowerStateUndefined),
            2129 => Ok(Sensors::PowerStateD0FullPower),
            2130 => Ok(Sensors::PowerStateD1LowPower),
            2131 => Ok(Sensors::PowerStateD2StandbyPowerwithWakeup),
            2132 => Ok(Sensors::PowerStateD3SleepwithWakeup),
            2133 => Ok(Sensors::PowerStateD4PowerOff),
            2144 => Ok(Sensors::AccuracyDefault),
            2145 => Ok(Sensors::AccuracyHigh),
            2146 => Ok(Sensors::AccuracyMedium),
            2147 => Ok(Sensors::AccuracyLow),
            2160 => Ok(Sensors::FixQualityNoFix),
            2161 => Ok(Sensors::FixQualityGPS),
            2162 => Ok(Sensors::FixQualityDGPS),
            2176 => Ok(Sensors::FixTypeNoFix),
            2177 => Ok(Sensors::FixTypeGPSSPSModeFixValid),
            2178 => Ok(Sensors::FixTypeDGPSSPSModeFixValid),
            2179 => Ok(Sensors::FixTypeGPSPPSModeFixValid),
            2180 => Ok(Sensors::FixTypeRealTimeKinematic),
            2181 => Ok(Sensors::FixTypeFloatRTK),
            2182 => Ok(Sensors::FixTypeEstimateddeadreckoned),
            2183 => Ok(Sensors::FixTypeManualInputMode),
            2184 => Ok(Sensors::FixTypeSimulatorMode),
            2192 => Ok(Sensors::GPSOperationModeManual),
            2193 => Ok(Sensors::GPSOperationModeAutomatic),
            2208 => Ok(Sensors::GPSSelectionModeAutonomous),
            2209 => Ok(Sensors::GPSSelectionModeDGPS),
            2210 => Ok(Sensors::GPSSelectionModeEstimateddeadreckoned),
            2211 => Ok(Sensors::GPSSelectionModeManualInput),
            2212 => Ok(Sensors::GPSSelectionModeSimulator),
            2213 => Ok(Sensors::GPSSelectionModeDataNotValid),
            2224 => Ok(Sensors::GPSStatusDataValid),
            2225 => Ok(Sensors::GPSStatusDataNotValid),
            2240 => Ok(Sensors::DayofWeekSunday),
            2241 => Ok(Sensors::DayofWeekMonday),
            2242 => Ok(Sensors::DayofWeekTuesday),
            2243 => Ok(Sensors::DayofWeekWednesday),
            2244 => Ok(Sensors::DayofWeekThursday),
            2245 => Ok(Sensors::DayofWeekFriday),
            2246 => Ok(Sensors::DayofWeekSaturday),
            2256 => Ok(Sensors::KindCategory),
            2257 => Ok(Sensors::KindType),
            2258 => Ok(Sensors::KindEvent),
            2259 => Ok(Sensors::KindProperty),
            2260 => Ok(Sensors::KindDataField),
            2272 => Ok(Sensors::MagnetometerAccuracyLow),
            2273 => Ok(Sensors::MagnetometerAccuracyMedium),
            2274 => Ok(Sensors::MagnetometerAccuracyHigh),
            2288 => Ok(Sensors::SimpleOrientationDirectionNotRotated),
            2289 => Ok(Sensors::SimpleOrientationDirectionRotated90DegreesCCW),
            2290 => Ok(Sensors::SimpleOrientationDirectionRotated180DegreesCCW),
            2291 => Ok(Sensors::SimpleOrientationDirectionRotated270DegreesCCW),
            2292 => Ok(Sensors::SimpleOrientationDirectionFaceUp),
            2293 => Ok(Sensors::SimpleOrientationDirectionFaceDown),
            2304 => Ok(Sensors::VT_NULL),
            2305 => Ok(Sensors::VT_BOOL),
            2306 => Ok(Sensors::VT_UI1),
            2307 => Ok(Sensors::VT_I1),
            2308 => Ok(Sensors::VT_UI2),
            2309 => Ok(Sensors::VT_I2),
            2310 => Ok(Sensors::VT_UI4),
            2311 => Ok(Sensors::VT_I4),
            2312 => Ok(Sensors::VT_UI8),
            2313 => Ok(Sensors::VT_I8),
            2314 => Ok(Sensors::VT_R4),
            2315 => Ok(Sensors::VT_R8),
            2316 => Ok(Sensors::VT_WSTR),
            2317 => Ok(Sensors::VT_STR),
            2318 => Ok(Sensors::VT_CLSID),
            2319 => Ok(Sensors::VT_VECTORVT_UI1),
            2320 => Ok(Sensors::VT_F16E0),
            2321 => Ok(Sensors::VT_F16E1),
            2322 => Ok(Sensors::VT_F16E2),
            2323 => Ok(Sensors::VT_F16E3),
            2324 => Ok(Sensors::VT_F16E4),
            2325 => Ok(Sensors::VT_F16E5),
            2326 => Ok(Sensors::VT_F16E6),
            2327 => Ok(Sensors::VT_F16E7),
            2328 => Ok(Sensors::VT_F16E8),
            2329 => Ok(Sensors::VT_F16E9),
            2330 => Ok(Sensors::VT_F16EA),
            2331 => Ok(Sensors::VT_F16EB),
            2332 => Ok(Sensors::VT_F16EC),
            2333 => Ok(Sensors::VT_F16ED),
            2334 => Ok(Sensors::VT_F16EE),
            2335 => Ok(Sensors::VT_F16EF),
            2336 => Ok(Sensors::VT_F32E0),
            2337 => Ok(Sensors::VT_F32E1),
            2338 => Ok(Sensors::VT_F32E2),
            2339 => Ok(Sensors::VT_F32E3),
            2340 => Ok(Sensors::VT_F32E4),
            2341 => Ok(Sensors::VT_F32E5),
            2342 => Ok(Sensors::VT_F32E6),
            2343 => Ok(Sensors::VT_F32E7),
            2344 => Ok(Sensors::VT_F32E8),
            2345 => Ok(Sensors::VT_F32E9),
            2346 => Ok(Sensors::VT_F32EA),
            2347 => Ok(Sensors::VT_F32EB),
            2348 => Ok(Sensors::VT_F32EC),
            2349 => Ok(Sensors::VT_F32ED),
            2350 => Ok(Sensors::VT_F32EE),
            2351 => Ok(Sensors::VT_F32EF),
            2352 => Ok(Sensors::ActivityTypeUnknown),
            2353 => Ok(Sensors::ActivityTypeStationary),
            2354 => Ok(Sensors::ActivityTypeFidgeting),
            2355 => Ok(Sensors::ActivityTypeWalking),
            2356 => Ok(Sensors::ActivityTypeRunning),
            2357 => Ok(Sensors::ActivityTypeInVehicle),
            2358 => Ok(Sensors::ActivityTypeBiking),
            2359 => Ok(Sensors::ActivityTypeIdle),
            2368 => Ok(Sensors::UnitNotSpecified),
            2369 => Ok(Sensors::UnitLux),
            2370 => Ok(Sensors::UnitDegreesKelvin),
            2371 => Ok(Sensors::UnitDegreesCelsius),
            2372 => Ok(Sensors::UnitPascal),
            2373 => Ok(Sensors::UnitNewton),
            2374 => Ok(Sensors::UnitMetersSecond),
            2375 => Ok(Sensors::UnitKilogram),
            2376 => Ok(Sensors::UnitMeter),
            2377 => Ok(Sensors::UnitMetersSecondSecond),
            2378 => Ok(Sensors::UnitFarad),
            2379 => Ok(Sensors::UnitAmpere),
            2380 => Ok(Sensors::UnitWatt),
            2381 => Ok(Sensors::UnitHenry),
            2382 => Ok(Sensors::UnitOhm),
            2383 => Ok(Sensors::UnitVolt),
            2384 => Ok(Sensors::UnitHertz),
            2385 => Ok(Sensors::UnitBar),
            2386 => Ok(Sensors::UnitDegreesAnticlockwise),
            2387 => Ok(Sensors::UnitDegreesClockwise),
            2388 => Ok(Sensors::UnitDegrees),
            2389 => Ok(Sensors::UnitDegreesSecond),
            2390 => Ok(Sensors::UnitDegreesSecondSecond),
            2391 => Ok(Sensors::UnitKnot),
            2392 => Ok(Sensors::UnitPercent),
            2393 => Ok(Sensors::UnitSecond),
            2394 => Ok(Sensors::UnitMillisecond),
            2395 => Ok(Sensors::UnitG),
            2396 => Ok(Sensors::UnitBytes),
            2397 => Ok(Sensors::UnitMilligauss),
            2398 => Ok(Sensors::UnitBits),
            2400 => Ok(Sensors::ActivityStateNoStateChange),
            2401 => Ok(Sensors::ActivityStateStartActivity),
            2402 => Ok(Sensors::ActivityStateEndActivity),
            2416 => Ok(Sensors::Exponent0),
            2417 => Ok(Sensors::Exponent1),
            2418 => Ok(Sensors::Exponent2),
            2419 => Ok(Sensors::Exponent3),
            2420 => Ok(Sensors::Exponent4),
            2421 => Ok(Sensors::Exponent5),
            2422 => Ok(Sensors::Exponent6),
            2423 => Ok(Sensors::Exponent7),
            2424 => Ok(Sensors::Exponent8),
            2425 => Ok(Sensors::Exponent9),
            2426 => Ok(Sensors::ExponentA),
            2427 => Ok(Sensors::ExponentB),
            2428 => Ok(Sensors::ExponentC),
            2429 => Ok(Sensors::ExponentD),
            2430 => Ok(Sensors::ExponentE),
            2431 => Ok(Sensors::ExponentF),
            2432 => Ok(Sensors::DevicePositionUnknown),
            2433 => Ok(Sensors::DevicePositionUnchanged),
            2434 => Ok(Sensors::DevicePositionOnDesk),
            2435 => Ok(Sensors::DevicePositionInHand),
            2436 => Ok(Sensors::DevicePositionMovinginBag),
            2437 => Ok(Sensors::DevicePositionStationaryinBag),
            2448 => Ok(Sensors::StepTypeUnknown),
            2449 => Ok(Sensors::StepTypeWalking),
            2450 => Ok(Sensors::StepTypeRunning),
            2464 => Ok(Sensors::GestureStateUnknown),
            2465 => Ok(Sensors::GestureStateStarted),
            2466 => Ok(Sensors::GestureStateCompleted),
            2467 => Ok(Sensors::GestureStateCancelled),
            2480 => Ok(Sensors::HingeFoldContributingPanelUnknown),
            2481 => Ok(Sensors::HingeFoldContributingPanelPanel1),
            2482 => Ok(Sensors::HingeFoldContributingPanelPanel2),
            2483 => Ok(Sensors::HingeFoldContributingPanelBoth),
            2484 => Ok(Sensors::HingeFoldTypeUnknown),
            2485 => Ok(Sensors::HingeFoldTypeIncreasing),
            2486 => Ok(Sensors::HingeFoldTypeDecreasing),
            2496 => Ok(Sensors::HumanPresenceDetectionTypeVendorDefinedNonBiometric),
            2497 => Ok(Sensors::HumanPresenceDetectionTypeVendorDefinedBiometric),
            2498 => Ok(Sensors::HumanPresenceDetectionTypeFacialBiometric),
            2499 => Ok(Sensors::HumanPresenceDetectionTypeAudioBiometric),
            4096 => Ok(Sensors::ModifierChangeSensitivityAbsolute),
            8192 => Ok(Sensors::ModifierMaximum),
            12288 => Ok(Sensors::ModifierMinimum),
            16384 => Ok(Sensors::ModifierAccuracy),
            20480 => Ok(Sensors::ModifierResolution),
            24576 => Ok(Sensors::ModifierThresholdHigh),
            28672 => Ok(Sensors::ModifierThresholdLow),
            32768 => Ok(Sensors::ModifierCalibrationOffset),
            36864 => Ok(Sensors::ModifierCalibrationMultiplier),
            40960 => Ok(Sensors::ModifierReportInterval),
            45056 => Ok(Sensors::ModifierFrequencyMax),
            49152 => Ok(Sensors::ModifierPeriodMax),
            53248 => Ok(Sensors::ModifierChangeSensitivityPercentofRange),
            57344 => Ok(Sensors::ModifierChangeSensitivityPercentRelative),
            61440 => Ok(Sensors::ModifierVendorReserved),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Sensors {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x40`: "Medical Instrument"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::MedicalInstrument(MedicalInstrument::VCRAcquisition);
/// let u2 = Usage::new_from_page_and_id(0x40, 0x20).unwrap();
/// let u3 = Usage::from(MedicalInstrument::VCRAcquisition);
/// let u4: Usage = MedicalInstrument::VCRAcquisition.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::MedicalInstrument));
/// assert_eq!(0x40, u1.usage_page_value());
/// assert_eq!(0x20, u1.usage_id_value());
/// assert_eq!((0x40 << 16) | 0x20, u1.usage_value());
/// assert_eq!("VCR/Acquisition", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum MedicalInstrument {
    /// Usage ID `0x1`: "Medical Ultrasound"
    MedicalUltrasound = 0x1,
    /// Usage ID `0x20`: "VCR/Acquisition"
    VCRAcquisition = 0x20,
    /// Usage ID `0x21`: "Freeze/Thaw"
    FreezeThaw = 0x21,
    /// Usage ID `0x22`: "Clip Store"
    ClipStore = 0x22,
    /// Usage ID `0x23`: "Update"
    Update = 0x23,
    /// Usage ID `0x24`: "Next"
    Next = 0x24,
    /// Usage ID `0x25`: "Save"
    Save = 0x25,
    /// Usage ID `0x26`: "Print"
    Print = 0x26,
    /// Usage ID `0x27`: "Microphone Enable"
    MicrophoneEnable = 0x27,
    /// Usage ID `0x40`: "Cine"
    Cine = 0x40,
    /// Usage ID `0x41`: "Transmit Power"
    TransmitPower = 0x41,
    /// Usage ID `0x42`: "Volume"
    Volume = 0x42,
    /// Usage ID `0x43`: "Focus"
    Focus = 0x43,
    /// Usage ID `0x44`: "Depth"
    Depth = 0x44,
    /// Usage ID `0x60`: "Soft Step - Primary"
    SoftStepPrimary = 0x60,
    /// Usage ID `0x61`: "Soft Step - Secondary"
    SoftStepSecondary = 0x61,
    /// Usage ID `0x70`: "Depth Gain Compensation"
    DepthGainCompensation = 0x70,
    /// Usage ID `0x80`: "Zoom Select"
    ZoomSelect = 0x80,
    /// Usage ID `0x81`: "Zoom Adjust"
    ZoomAdjust = 0x81,
    /// Usage ID `0x82`: "Spectral Doppler Mode Select"
    SpectralDopplerModeSelect = 0x82,
    /// Usage ID `0x83`: "Spectral Doppler Adjust"
    SpectralDopplerAdjust = 0x83,
    /// Usage ID `0x84`: "Color Doppler Mode Select"
    ColorDopplerModeSelect = 0x84,
    /// Usage ID `0x85`: "Color Doppler Adjust"
    ColorDopplerAdjust = 0x85,
    /// Usage ID `0x86`: "Motion Mode Select"
    MotionModeSelect = 0x86,
    /// Usage ID `0x87`: "Motion Mode Adjust"
    MotionModeAdjust = 0x87,
    /// Usage ID `0x88`: "2-D Mode Select"
    TwoDModeSelect = 0x88,
    /// Usage ID `0x89`: "2-D Mode Adjust"
    TwoDModeAdjust = 0x89,
    /// Usage ID `0xA0`: "Soft Control Select"
    SoftControlSelect = 0xA0,
    /// Usage ID `0xA1`: "Soft Control Adjust"
    SoftControlAdjust = 0xA1,
}

impl MedicalInstrument {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            MedicalInstrument::MedicalUltrasound => "Medical Ultrasound",
            MedicalInstrument::VCRAcquisition => "VCR/Acquisition",
            MedicalInstrument::FreezeThaw => "Freeze/Thaw",
            MedicalInstrument::ClipStore => "Clip Store",
            MedicalInstrument::Update => "Update",
            MedicalInstrument::Next => "Next",
            MedicalInstrument::Save => "Save",
            MedicalInstrument::Print => "Print",
            MedicalInstrument::MicrophoneEnable => "Microphone Enable",
            MedicalInstrument::Cine => "Cine",
            MedicalInstrument::TransmitPower => "Transmit Power",
            MedicalInstrument::Volume => "Volume",
            MedicalInstrument::Focus => "Focus",
            MedicalInstrument::Depth => "Depth",
            MedicalInstrument::SoftStepPrimary => "Soft Step - Primary",
            MedicalInstrument::SoftStepSecondary => "Soft Step - Secondary",
            MedicalInstrument::DepthGainCompensation => "Depth Gain Compensation",
            MedicalInstrument::ZoomSelect => "Zoom Select",
            MedicalInstrument::ZoomAdjust => "Zoom Adjust",
            MedicalInstrument::SpectralDopplerModeSelect => "Spectral Doppler Mode Select",
            MedicalInstrument::SpectralDopplerAdjust => "Spectral Doppler Adjust",
            MedicalInstrument::ColorDopplerModeSelect => "Color Doppler Mode Select",
            MedicalInstrument::ColorDopplerAdjust => "Color Doppler Adjust",
            MedicalInstrument::MotionModeSelect => "Motion Mode Select",
            MedicalInstrument::MotionModeAdjust => "Motion Mode Adjust",
            MedicalInstrument::TwoDModeSelect => "2-D Mode Select",
            MedicalInstrument::TwoDModeAdjust => "2-D Mode Adjust",
            MedicalInstrument::SoftControlSelect => "Soft Control Select",
            MedicalInstrument::SoftControlAdjust => "Soft Control Adjust",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for MedicalInstrument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for MedicalInstrument {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::MedicalInstrument(self)](Usage::MedicalInstrument)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for MedicalInstrument {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x40` for [MedicalInstrument]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::MedicalInstrument]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&MedicalInstrument> for u16 {
    fn from(medicalinstrument: &MedicalInstrument) -> u16 {
        *medicalinstrument as u16
    }
}

impl From<MedicalInstrument> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [MedicalInstrument::usage_page_value()].
    fn from(medicalinstrument: MedicalInstrument) -> u16 {
        u16::from(&medicalinstrument)
    }
}

impl From<&MedicalInstrument> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [MedicalInstrument::usage_value()].
    fn from(medicalinstrument: &MedicalInstrument) -> u32 {
        let up = UsagePage::from(medicalinstrument);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(medicalinstrument) as u32;
        up | id
    }
}

impl From<&MedicalInstrument> for UsagePage {
    /// Always returns [UsagePage::MedicalInstrument] and is
    /// identical to [MedicalInstrument::usage_page()].
    fn from(_: &MedicalInstrument) -> UsagePage {
        UsagePage::MedicalInstrument
    }
}

impl From<MedicalInstrument> for UsagePage {
    /// Always returns [UsagePage::MedicalInstrument] and is
    /// identical to [MedicalInstrument::usage_page()].
    fn from(_: MedicalInstrument) -> UsagePage {
        UsagePage::MedicalInstrument
    }
}

impl From<&MedicalInstrument> for Usage {
    fn from(medicalinstrument: &MedicalInstrument) -> Usage {
        Usage::try_from(u32::from(medicalinstrument)).unwrap()
    }
}

impl From<MedicalInstrument> for Usage {
    fn from(medicalinstrument: MedicalInstrument) -> Usage {
        Usage::from(&medicalinstrument)
    }
}

impl TryFrom<u16> for MedicalInstrument {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<MedicalInstrument> {
        match usage_id {
            1 => Ok(MedicalInstrument::MedicalUltrasound),
            32 => Ok(MedicalInstrument::VCRAcquisition),
            33 => Ok(MedicalInstrument::FreezeThaw),
            34 => Ok(MedicalInstrument::ClipStore),
            35 => Ok(MedicalInstrument::Update),
            36 => Ok(MedicalInstrument::Next),
            37 => Ok(MedicalInstrument::Save),
            38 => Ok(MedicalInstrument::Print),
            39 => Ok(MedicalInstrument::MicrophoneEnable),
            64 => Ok(MedicalInstrument::Cine),
            65 => Ok(MedicalInstrument::TransmitPower),
            66 => Ok(MedicalInstrument::Volume),
            67 => Ok(MedicalInstrument::Focus),
            68 => Ok(MedicalInstrument::Depth),
            96 => Ok(MedicalInstrument::SoftStepPrimary),
            97 => Ok(MedicalInstrument::SoftStepSecondary),
            112 => Ok(MedicalInstrument::DepthGainCompensation),
            128 => Ok(MedicalInstrument::ZoomSelect),
            129 => Ok(MedicalInstrument::ZoomAdjust),
            130 => Ok(MedicalInstrument::SpectralDopplerModeSelect),
            131 => Ok(MedicalInstrument::SpectralDopplerAdjust),
            132 => Ok(MedicalInstrument::ColorDopplerModeSelect),
            133 => Ok(MedicalInstrument::ColorDopplerAdjust),
            134 => Ok(MedicalInstrument::MotionModeSelect),
            135 => Ok(MedicalInstrument::MotionModeAdjust),
            136 => Ok(MedicalInstrument::TwoDModeSelect),
            137 => Ok(MedicalInstrument::TwoDModeAdjust),
            160 => Ok(MedicalInstrument::SoftControlSelect),
            161 => Ok(MedicalInstrument::SoftControlAdjust),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for MedicalInstrument {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x41`: "Braille Display"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::BrailleDisplay(BrailleDisplay::BrailleRow);
/// let u2 = Usage::new_from_page_and_id(0x41, 0x2).unwrap();
/// let u3 = Usage::from(BrailleDisplay::BrailleRow);
/// let u4: Usage = BrailleDisplay::BrailleRow.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::BrailleDisplay));
/// assert_eq!(0x41, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x41 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Braille Row", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum BrailleDisplay {
    /// Usage ID `0x1`: "Braille Display"
    BrailleDisplay = 0x1,
    /// Usage ID `0x2`: "Braille Row"
    BrailleRow = 0x2,
    /// Usage ID `0x3`: "8 Dot Braille Cell"
    EightDotBrailleCell = 0x3,
    /// Usage ID `0x4`: "6 Dot Braille Cell"
    SixDotBrailleCell = 0x4,
    /// Usage ID `0x5`: "Number of Braille Cells"
    NumberofBrailleCells = 0x5,
    /// Usage ID `0x6`: "Screen Reader Control"
    ScreenReaderControl = 0x6,
    /// Usage ID `0x7`: "Screen Reader Identifier"
    ScreenReaderIdentifier = 0x7,
    /// Usage ID `0xFA`: "Router Set 1"
    RouterSet1 = 0xFA,
    /// Usage ID `0xFB`: "Router Set 2"
    RouterSet2 = 0xFB,
    /// Usage ID `0xFC`: "Router Set 3"
    RouterSet3 = 0xFC,
    /// Usage ID `0x100`: "Router Key"
    RouterKey = 0x100,
    /// Usage ID `0x101`: "Row Router Key"
    RowRouterKey = 0x101,
    /// Usage ID `0x200`: "Braille Buttons"
    BrailleButtons = 0x200,
    /// Usage ID `0x201`: "Braille Keyboard Dot 1"
    BrailleKeyboardDot1 = 0x201,
    /// Usage ID `0x202`: "Braille Keyboard Dot 2"
    BrailleKeyboardDot2 = 0x202,
    /// Usage ID `0x203`: "Braille Keyboard Dot 3"
    BrailleKeyboardDot3 = 0x203,
    /// Usage ID `0x204`: "Braille Keyboard Dot 4"
    BrailleKeyboardDot4 = 0x204,
    /// Usage ID `0x205`: "Braille Keyboard Dot 5"
    BrailleKeyboardDot5 = 0x205,
    /// Usage ID `0x206`: "Braille Keyboard Dot 6"
    BrailleKeyboardDot6 = 0x206,
    /// Usage ID `0x207`: "Braille Keyboard Dot 7"
    BrailleKeyboardDot7 = 0x207,
    /// Usage ID `0x208`: "Braille Keyboard Dot 8"
    BrailleKeyboardDot8 = 0x208,
    /// Usage ID `0x209`: "Braille Keyboard Space"
    BrailleKeyboardSpace = 0x209,
    /// Usage ID `0x20A`: "Braille Keyboard Left Space"
    BrailleKeyboardLeftSpace = 0x20A,
    /// Usage ID `0x20B`: "Braille Keyboard Right Space"
    BrailleKeyboardRightSpace = 0x20B,
    /// Usage ID `0x20C`: "Braille Face Controls"
    BrailleFaceControls = 0x20C,
    /// Usage ID `0x20D`: "Braille Left Controls"
    BrailleLeftControls = 0x20D,
    /// Usage ID `0x20E`: "Braille Right Controls"
    BrailleRightControls = 0x20E,
    /// Usage ID `0x20F`: "Braille Top Controls"
    BrailleTopControls = 0x20F,
    /// Usage ID `0x210`: "Braille Joystick Center"
    BrailleJoystickCenter = 0x210,
    /// Usage ID `0x211`: "Braille Joystick Up"
    BrailleJoystickUp = 0x211,
    /// Usage ID `0x212`: "Braille Joystick Down"
    BrailleJoystickDown = 0x212,
    /// Usage ID `0x213`: "Braille Joystick Left"
    BrailleJoystickLeft = 0x213,
    /// Usage ID `0x214`: "Braille Joystick Right"
    BrailleJoystickRight = 0x214,
    /// Usage ID `0x215`: "Braille D-Pad Center"
    BrailleDPadCenter = 0x215,
    /// Usage ID `0x216`: "Braille D-Pad Up"
    BrailleDPadUp = 0x216,
    /// Usage ID `0x217`: "Braille D-Pad Down"
    BrailleDPadDown = 0x217,
    /// Usage ID `0x218`: "Braille D-Pad Left"
    BrailleDPadLeft = 0x218,
    /// Usage ID `0x219`: "Braille D-Pad Right"
    BrailleDPadRight = 0x219,
    /// Usage ID `0x21A`: "Braille Pan Left"
    BraillePanLeft = 0x21A,
    /// Usage ID `0x21B`: "Braille Pan Right"
    BraillePanRight = 0x21B,
    /// Usage ID `0x21C`: "Braille Rocker Up"
    BrailleRockerUp = 0x21C,
    /// Usage ID `0x21D`: "Braille Rocker Down"
    BrailleRockerDown = 0x21D,
    /// Usage ID `0x21E`: "Braille Rocker Press"
    BrailleRockerPress = 0x21E,
}

impl BrailleDisplay {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            BrailleDisplay::BrailleDisplay => "Braille Display",
            BrailleDisplay::BrailleRow => "Braille Row",
            BrailleDisplay::EightDotBrailleCell => "8 Dot Braille Cell",
            BrailleDisplay::SixDotBrailleCell => "6 Dot Braille Cell",
            BrailleDisplay::NumberofBrailleCells => "Number of Braille Cells",
            BrailleDisplay::ScreenReaderControl => "Screen Reader Control",
            BrailleDisplay::ScreenReaderIdentifier => "Screen Reader Identifier",
            BrailleDisplay::RouterSet1 => "Router Set 1",
            BrailleDisplay::RouterSet2 => "Router Set 2",
            BrailleDisplay::RouterSet3 => "Router Set 3",
            BrailleDisplay::RouterKey => "Router Key",
            BrailleDisplay::RowRouterKey => "Row Router Key",
            BrailleDisplay::BrailleButtons => "Braille Buttons",
            BrailleDisplay::BrailleKeyboardDot1 => "Braille Keyboard Dot 1",
            BrailleDisplay::BrailleKeyboardDot2 => "Braille Keyboard Dot 2",
            BrailleDisplay::BrailleKeyboardDot3 => "Braille Keyboard Dot 3",
            BrailleDisplay::BrailleKeyboardDot4 => "Braille Keyboard Dot 4",
            BrailleDisplay::BrailleKeyboardDot5 => "Braille Keyboard Dot 5",
            BrailleDisplay::BrailleKeyboardDot6 => "Braille Keyboard Dot 6",
            BrailleDisplay::BrailleKeyboardDot7 => "Braille Keyboard Dot 7",
            BrailleDisplay::BrailleKeyboardDot8 => "Braille Keyboard Dot 8",
            BrailleDisplay::BrailleKeyboardSpace => "Braille Keyboard Space",
            BrailleDisplay::BrailleKeyboardLeftSpace => "Braille Keyboard Left Space",
            BrailleDisplay::BrailleKeyboardRightSpace => "Braille Keyboard Right Space",
            BrailleDisplay::BrailleFaceControls => "Braille Face Controls",
            BrailleDisplay::BrailleLeftControls => "Braille Left Controls",
            BrailleDisplay::BrailleRightControls => "Braille Right Controls",
            BrailleDisplay::BrailleTopControls => "Braille Top Controls",
            BrailleDisplay::BrailleJoystickCenter => "Braille Joystick Center",
            BrailleDisplay::BrailleJoystickUp => "Braille Joystick Up",
            BrailleDisplay::BrailleJoystickDown => "Braille Joystick Down",
            BrailleDisplay::BrailleJoystickLeft => "Braille Joystick Left",
            BrailleDisplay::BrailleJoystickRight => "Braille Joystick Right",
            BrailleDisplay::BrailleDPadCenter => "Braille D-Pad Center",
            BrailleDisplay::BrailleDPadUp => "Braille D-Pad Up",
            BrailleDisplay::BrailleDPadDown => "Braille D-Pad Down",
            BrailleDisplay::BrailleDPadLeft => "Braille D-Pad Left",
            BrailleDisplay::BrailleDPadRight => "Braille D-Pad Right",
            BrailleDisplay::BraillePanLeft => "Braille Pan Left",
            BrailleDisplay::BraillePanRight => "Braille Pan Right",
            BrailleDisplay::BrailleRockerUp => "Braille Rocker Up",
            BrailleDisplay::BrailleRockerDown => "Braille Rocker Down",
            BrailleDisplay::BrailleRockerPress => "Braille Rocker Press",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for BrailleDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for BrailleDisplay {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::BrailleDisplay(self)](Usage::BrailleDisplay)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for BrailleDisplay {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x41` for [BrailleDisplay]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::BrailleDisplay]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&BrailleDisplay> for u16 {
    fn from(brailledisplay: &BrailleDisplay) -> u16 {
        *brailledisplay as u16
    }
}

impl From<BrailleDisplay> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [BrailleDisplay::usage_page_value()].
    fn from(brailledisplay: BrailleDisplay) -> u16 {
        u16::from(&brailledisplay)
    }
}

impl From<&BrailleDisplay> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [BrailleDisplay::usage_value()].
    fn from(brailledisplay: &BrailleDisplay) -> u32 {
        let up = UsagePage::from(brailledisplay);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(brailledisplay) as u32;
        up | id
    }
}

impl From<&BrailleDisplay> for UsagePage {
    /// Always returns [UsagePage::BrailleDisplay] and is
    /// identical to [BrailleDisplay::usage_page()].
    fn from(_: &BrailleDisplay) -> UsagePage {
        UsagePage::BrailleDisplay
    }
}

impl From<BrailleDisplay> for UsagePage {
    /// Always returns [UsagePage::BrailleDisplay] and is
    /// identical to [BrailleDisplay::usage_page()].
    fn from(_: BrailleDisplay) -> UsagePage {
        UsagePage::BrailleDisplay
    }
}

impl From<&BrailleDisplay> for Usage {
    fn from(brailledisplay: &BrailleDisplay) -> Usage {
        Usage::try_from(u32::from(brailledisplay)).unwrap()
    }
}

impl From<BrailleDisplay> for Usage {
    fn from(brailledisplay: BrailleDisplay) -> Usage {
        Usage::from(&brailledisplay)
    }
}

impl TryFrom<u16> for BrailleDisplay {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<BrailleDisplay> {
        match usage_id {
            1 => Ok(BrailleDisplay::BrailleDisplay),
            2 => Ok(BrailleDisplay::BrailleRow),
            3 => Ok(BrailleDisplay::EightDotBrailleCell),
            4 => Ok(BrailleDisplay::SixDotBrailleCell),
            5 => Ok(BrailleDisplay::NumberofBrailleCells),
            6 => Ok(BrailleDisplay::ScreenReaderControl),
            7 => Ok(BrailleDisplay::ScreenReaderIdentifier),
            250 => Ok(BrailleDisplay::RouterSet1),
            251 => Ok(BrailleDisplay::RouterSet2),
            252 => Ok(BrailleDisplay::RouterSet3),
            256 => Ok(BrailleDisplay::RouterKey),
            257 => Ok(BrailleDisplay::RowRouterKey),
            512 => Ok(BrailleDisplay::BrailleButtons),
            513 => Ok(BrailleDisplay::BrailleKeyboardDot1),
            514 => Ok(BrailleDisplay::BrailleKeyboardDot2),
            515 => Ok(BrailleDisplay::BrailleKeyboardDot3),
            516 => Ok(BrailleDisplay::BrailleKeyboardDot4),
            517 => Ok(BrailleDisplay::BrailleKeyboardDot5),
            518 => Ok(BrailleDisplay::BrailleKeyboardDot6),
            519 => Ok(BrailleDisplay::BrailleKeyboardDot7),
            520 => Ok(BrailleDisplay::BrailleKeyboardDot8),
            521 => Ok(BrailleDisplay::BrailleKeyboardSpace),
            522 => Ok(BrailleDisplay::BrailleKeyboardLeftSpace),
            523 => Ok(BrailleDisplay::BrailleKeyboardRightSpace),
            524 => Ok(BrailleDisplay::BrailleFaceControls),
            525 => Ok(BrailleDisplay::BrailleLeftControls),
            526 => Ok(BrailleDisplay::BrailleRightControls),
            527 => Ok(BrailleDisplay::BrailleTopControls),
            528 => Ok(BrailleDisplay::BrailleJoystickCenter),
            529 => Ok(BrailleDisplay::BrailleJoystickUp),
            530 => Ok(BrailleDisplay::BrailleJoystickDown),
            531 => Ok(BrailleDisplay::BrailleJoystickLeft),
            532 => Ok(BrailleDisplay::BrailleJoystickRight),
            533 => Ok(BrailleDisplay::BrailleDPadCenter),
            534 => Ok(BrailleDisplay::BrailleDPadUp),
            535 => Ok(BrailleDisplay::BrailleDPadDown),
            536 => Ok(BrailleDisplay::BrailleDPadLeft),
            537 => Ok(BrailleDisplay::BrailleDPadRight),
            538 => Ok(BrailleDisplay::BraillePanLeft),
            539 => Ok(BrailleDisplay::BraillePanRight),
            540 => Ok(BrailleDisplay::BrailleRockerUp),
            541 => Ok(BrailleDisplay::BrailleRockerDown),
            542 => Ok(BrailleDisplay::BrailleRockerPress),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for BrailleDisplay {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x59`: "Lighting And Illumination"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::LightingAndIllumination(LightingAndIllumination::LampArrayAttributesReport);
/// let u2 = Usage::new_from_page_and_id(0x59, 0x2).unwrap();
/// let u3 = Usage::from(LightingAndIllumination::LampArrayAttributesReport);
/// let u4: Usage = LightingAndIllumination::LampArrayAttributesReport.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::LightingAndIllumination));
/// assert_eq!(0x59, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x59 << 16) | 0x2, u1.usage_value());
/// assert_eq!("LampArrayAttributesReport", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum LightingAndIllumination {
    /// Usage ID `0x1`: "LampArray"
    LampArray = 0x1,
    /// Usage ID `0x2`: "LampArrayAttributesReport"
    LampArrayAttributesReport = 0x2,
    /// Usage ID `0x3`: "LampCount"
    LampCount = 0x3,
    /// Usage ID `0x4`: "BoundingBoxWidthInMicrometers"
    BoundingBoxWidthInMicrometers = 0x4,
    /// Usage ID `0x5`: "BoundingBoxHeightInMicrometers"
    BoundingBoxHeightInMicrometers = 0x5,
    /// Usage ID `0x6`: "BoundingBoxDepthInMicrometers"
    BoundingBoxDepthInMicrometers = 0x6,
    /// Usage ID `0x7`: "LampArrayKind"
    LampArrayKind = 0x7,
    /// Usage ID `0x8`: "MinUpdateIntervalInMicroseconds"
    MinUpdateIntervalInMicroseconds = 0x8,
    /// Usage ID `0x20`: "LampAttributesRequestReport"
    LampAttributesRequestReport = 0x20,
    /// Usage ID `0x21`: "LampId"
    LampId = 0x21,
    /// Usage ID `0x22`: "LampAttributesResponseReport"
    LampAttributesResponseReport = 0x22,
    /// Usage ID `0x23`: "PositionXInMicrometers"
    PositionXInMicrometers = 0x23,
    /// Usage ID `0x24`: "PositionYInMicrometers"
    PositionYInMicrometers = 0x24,
    /// Usage ID `0x25`: "PositionZInMicrometers"
    PositionZInMicrometers = 0x25,
    /// Usage ID `0x26`: "LampPurposes"
    LampPurposes = 0x26,
    /// Usage ID `0x27`: "UpdateLatencyInMicroseconds"
    UpdateLatencyInMicroseconds = 0x27,
    /// Usage ID `0x28`: "RedLevelCount"
    RedLevelCount = 0x28,
    /// Usage ID `0x29`: "GreenLevelCount"
    GreenLevelCount = 0x29,
    /// Usage ID `0x2A`: "BlueLevelCount"
    BlueLevelCount = 0x2A,
    /// Usage ID `0x2B`: "IntensityLevelCount"
    IntensityLevelCount = 0x2B,
    /// Usage ID `0x2C`: "IsProgrammable"
    IsProgrammable = 0x2C,
    /// Usage ID `0x2D`: "InputBinding"
    InputBinding = 0x2D,
    /// Usage ID `0x50`: "LampMultiUpdateReport"
    LampMultiUpdateReport = 0x50,
    /// Usage ID `0x51`: "RedUpdateChannel"
    RedUpdateChannel = 0x51,
    /// Usage ID `0x52`: "GreenUpdateChannel"
    GreenUpdateChannel = 0x52,
    /// Usage ID `0x53`: "BlueUpdateChannel"
    BlueUpdateChannel = 0x53,
    /// Usage ID `0x54`: "IntensityUpdateChannel"
    IntensityUpdateChannel = 0x54,
    /// Usage ID `0x55`: "LampUpdateFlags"
    LampUpdateFlags = 0x55,
    /// Usage ID `0x60`: "LampRangeUpdateReport"
    LampRangeUpdateReport = 0x60,
    /// Usage ID `0x61`: "LampIdStart"
    LampIdStart = 0x61,
    /// Usage ID `0x62`: "LampIdEnd"
    LampIdEnd = 0x62,
    /// Usage ID `0x70`: "LampArrayControlReport"
    LampArrayControlReport = 0x70,
    /// Usage ID `0x71`: "AutonomousMode"
    AutonomousMode = 0x71,
}

impl LightingAndIllumination {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            LightingAndIllumination::LampArray => "LampArray",
            LightingAndIllumination::LampArrayAttributesReport => "LampArrayAttributesReport",
            LightingAndIllumination::LampCount => "LampCount",
            LightingAndIllumination::BoundingBoxWidthInMicrometers => {
                "BoundingBoxWidthInMicrometers"
            }
            LightingAndIllumination::BoundingBoxHeightInMicrometers => {
                "BoundingBoxHeightInMicrometers"
            }
            LightingAndIllumination::BoundingBoxDepthInMicrometers => {
                "BoundingBoxDepthInMicrometers"
            }
            LightingAndIllumination::LampArrayKind => "LampArrayKind",
            LightingAndIllumination::MinUpdateIntervalInMicroseconds => {
                "MinUpdateIntervalInMicroseconds"
            }
            LightingAndIllumination::LampAttributesRequestReport => "LampAttributesRequestReport",
            LightingAndIllumination::LampId => "LampId",
            LightingAndIllumination::LampAttributesResponseReport => "LampAttributesResponseReport",
            LightingAndIllumination::PositionXInMicrometers => "PositionXInMicrometers",
            LightingAndIllumination::PositionYInMicrometers => "PositionYInMicrometers",
            LightingAndIllumination::PositionZInMicrometers => "PositionZInMicrometers",
            LightingAndIllumination::LampPurposes => "LampPurposes",
            LightingAndIllumination::UpdateLatencyInMicroseconds => "UpdateLatencyInMicroseconds",
            LightingAndIllumination::RedLevelCount => "RedLevelCount",
            LightingAndIllumination::GreenLevelCount => "GreenLevelCount",
            LightingAndIllumination::BlueLevelCount => "BlueLevelCount",
            LightingAndIllumination::IntensityLevelCount => "IntensityLevelCount",
            LightingAndIllumination::IsProgrammable => "IsProgrammable",
            LightingAndIllumination::InputBinding => "InputBinding",
            LightingAndIllumination::LampMultiUpdateReport => "LampMultiUpdateReport",
            LightingAndIllumination::RedUpdateChannel => "RedUpdateChannel",
            LightingAndIllumination::GreenUpdateChannel => "GreenUpdateChannel",
            LightingAndIllumination::BlueUpdateChannel => "BlueUpdateChannel",
            LightingAndIllumination::IntensityUpdateChannel => "IntensityUpdateChannel",
            LightingAndIllumination::LampUpdateFlags => "LampUpdateFlags",
            LightingAndIllumination::LampRangeUpdateReport => "LampRangeUpdateReport",
            LightingAndIllumination::LampIdStart => "LampIdStart",
            LightingAndIllumination::LampIdEnd => "LampIdEnd",
            LightingAndIllumination::LampArrayControlReport => "LampArrayControlReport",
            LightingAndIllumination::AutonomousMode => "AutonomousMode",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for LightingAndIllumination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for LightingAndIllumination {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::LightingAndIllumination(self)](Usage::LightingAndIllumination)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for LightingAndIllumination {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x59` for [LightingAndIllumination]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::LightingAndIllumination]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&LightingAndIllumination> for u16 {
    fn from(lightingandillumination: &LightingAndIllumination) -> u16 {
        *lightingandillumination as u16
    }
}

impl From<LightingAndIllumination> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [LightingAndIllumination::usage_page_value()].
    fn from(lightingandillumination: LightingAndIllumination) -> u16 {
        u16::from(&lightingandillumination)
    }
}

impl From<&LightingAndIllumination> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [LightingAndIllumination::usage_value()].
    fn from(lightingandillumination: &LightingAndIllumination) -> u32 {
        let up = UsagePage::from(lightingandillumination);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(lightingandillumination) as u32;
        up | id
    }
}

impl From<&LightingAndIllumination> for UsagePage {
    /// Always returns [UsagePage::LightingAndIllumination] and is
    /// identical to [LightingAndIllumination::usage_page()].
    fn from(_: &LightingAndIllumination) -> UsagePage {
        UsagePage::LightingAndIllumination
    }
}

impl From<LightingAndIllumination> for UsagePage {
    /// Always returns [UsagePage::LightingAndIllumination] and is
    /// identical to [LightingAndIllumination::usage_page()].
    fn from(_: LightingAndIllumination) -> UsagePage {
        UsagePage::LightingAndIllumination
    }
}

impl From<&LightingAndIllumination> for Usage {
    fn from(lightingandillumination: &LightingAndIllumination) -> Usage {
        Usage::try_from(u32::from(lightingandillumination)).unwrap()
    }
}

impl From<LightingAndIllumination> for Usage {
    fn from(lightingandillumination: LightingAndIllumination) -> Usage {
        Usage::from(&lightingandillumination)
    }
}

impl TryFrom<u16> for LightingAndIllumination {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<LightingAndIllumination> {
        match usage_id {
            1 => Ok(LightingAndIllumination::LampArray),
            2 => Ok(LightingAndIllumination::LampArrayAttributesReport),
            3 => Ok(LightingAndIllumination::LampCount),
            4 => Ok(LightingAndIllumination::BoundingBoxWidthInMicrometers),
            5 => Ok(LightingAndIllumination::BoundingBoxHeightInMicrometers),
            6 => Ok(LightingAndIllumination::BoundingBoxDepthInMicrometers),
            7 => Ok(LightingAndIllumination::LampArrayKind),
            8 => Ok(LightingAndIllumination::MinUpdateIntervalInMicroseconds),
            32 => Ok(LightingAndIllumination::LampAttributesRequestReport),
            33 => Ok(LightingAndIllumination::LampId),
            34 => Ok(LightingAndIllumination::LampAttributesResponseReport),
            35 => Ok(LightingAndIllumination::PositionXInMicrometers),
            36 => Ok(LightingAndIllumination::PositionYInMicrometers),
            37 => Ok(LightingAndIllumination::PositionZInMicrometers),
            38 => Ok(LightingAndIllumination::LampPurposes),
            39 => Ok(LightingAndIllumination::UpdateLatencyInMicroseconds),
            40 => Ok(LightingAndIllumination::RedLevelCount),
            41 => Ok(LightingAndIllumination::GreenLevelCount),
            42 => Ok(LightingAndIllumination::BlueLevelCount),
            43 => Ok(LightingAndIllumination::IntensityLevelCount),
            44 => Ok(LightingAndIllumination::IsProgrammable),
            45 => Ok(LightingAndIllumination::InputBinding),
            80 => Ok(LightingAndIllumination::LampMultiUpdateReport),
            81 => Ok(LightingAndIllumination::RedUpdateChannel),
            82 => Ok(LightingAndIllumination::GreenUpdateChannel),
            83 => Ok(LightingAndIllumination::BlueUpdateChannel),
            84 => Ok(LightingAndIllumination::IntensityUpdateChannel),
            85 => Ok(LightingAndIllumination::LampUpdateFlags),
            96 => Ok(LightingAndIllumination::LampRangeUpdateReport),
            97 => Ok(LightingAndIllumination::LampIdStart),
            98 => Ok(LightingAndIllumination::LampIdEnd),
            112 => Ok(LightingAndIllumination::LampArrayControlReport),
            113 => Ok(LightingAndIllumination::AutonomousMode),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for LightingAndIllumination {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x80`: "Monitor"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Monitor(Monitor::EDIDInformation);
/// let u2 = Usage::new_from_page_and_id(0x80, 0x2).unwrap();
/// let u3 = Usage::from(Monitor::EDIDInformation);
/// let u4: Usage = Monitor::EDIDInformation.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Monitor));
/// assert_eq!(0x80, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x80 << 16) | 0x2, u1.usage_value());
/// assert_eq!("EDID Information", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Monitor {
    /// Usage ID `0x1`: "Monitor Control"
    MonitorControl = 0x1,
    /// Usage ID `0x2`: "EDID Information"
    EDIDInformation = 0x2,
    /// Usage ID `0x3`: "VDIF Information"
    VDIFInformation = 0x3,
    /// Usage ID `0x4`: "VESA Version"
    VESAVersion = 0x4,
}

impl Monitor {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Monitor::MonitorControl => "Monitor Control",
            Monitor::EDIDInformation => "EDID Information",
            Monitor::VDIFInformation => "VDIF Information",
            Monitor::VESAVersion => "VESA Version",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Monitor {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Monitor(self)](Usage::Monitor)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Monitor {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x80` for [Monitor]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Monitor]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Monitor> for u16 {
    fn from(monitor: &Monitor) -> u16 {
        *monitor as u16
    }
}

impl From<Monitor> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Monitor::usage_page_value()].
    fn from(monitor: Monitor) -> u16 {
        u16::from(&monitor)
    }
}

impl From<&Monitor> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Monitor::usage_value()].
    fn from(monitor: &Monitor) -> u32 {
        let up = UsagePage::from(monitor);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(monitor) as u32;
        up | id
    }
}

impl From<&Monitor> for UsagePage {
    /// Always returns [UsagePage::Monitor] and is
    /// identical to [Monitor::usage_page()].
    fn from(_: &Monitor) -> UsagePage {
        UsagePage::Monitor
    }
}

impl From<Monitor> for UsagePage {
    /// Always returns [UsagePage::Monitor] and is
    /// identical to [Monitor::usage_page()].
    fn from(_: Monitor) -> UsagePage {
        UsagePage::Monitor
    }
}

impl From<&Monitor> for Usage {
    fn from(monitor: &Monitor) -> Usage {
        Usage::try_from(u32::from(monitor)).unwrap()
    }
}

impl From<Monitor> for Usage {
    fn from(monitor: Monitor) -> Usage {
        Usage::from(&monitor)
    }
}

impl TryFrom<u16> for Monitor {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Monitor> {
        match usage_id {
            1 => Ok(Monitor::MonitorControl),
            2 => Ok(Monitor::EDIDInformation),
            3 => Ok(Monitor::VDIFInformation),
            4 => Ok(Monitor::VESAVersion),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Monitor {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x81`: "Monitor Enumerated"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
///
/// This Usage Page is generated, not defined, any Usage IDs in this Usage
/// Page are simply the enumerate number.
///
/// ```
/// # use hut::*;
/// let u1 = Usage::MonitorEnumerated(MonitorEnumerated::MonitorEnumerated(3));
/// let u2 = Usage::new_from_page_and_id(0x81, 3).unwrap();
/// let u3 = Usage::from(MonitorEnumerated::MonitorEnumerated(3));
/// let u4: Usage = MonitorEnumerated::MonitorEnumerated(3).into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::MonitorEnumerated));
/// assert_eq!(0x81, u1.usage_page_value());
/// assert_eq!(3, u1.usage_id_value());
/// assert_eq!((0x81 << 16) | 3, u1.usage_value());
/// assert_eq!("Enumerate 3", u1.name());
/// ```
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum MonitorEnumerated {
    MonitorEnumerated(u16),
}

impl MonitorEnumerated {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            MonitorEnumerated::MonitorEnumerated(enumerate) => format!("Enumerate {enumerate}"),
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for MonitorEnumerated {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for MonitorEnumerated {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::MonitorEnumerated(self)](Usage::MonitorEnumerated)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for MonitorEnumerated {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x81` for [MonitorEnumerated]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::MonitorEnumerated]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&MonitorEnumerated> for u16 {
    fn from(monitorenumerated: &MonitorEnumerated) -> u16 {
        match *monitorenumerated {
            MonitorEnumerated::MonitorEnumerated(enumerate) => enumerate,
        }
    }
}

impl From<MonitorEnumerated> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [MonitorEnumerated::usage_page_value()].
    fn from(monitorenumerated: MonitorEnumerated) -> u16 {
        u16::from(&monitorenumerated)
    }
}

impl From<&MonitorEnumerated> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [MonitorEnumerated::usage_value()].
    fn from(monitorenumerated: &MonitorEnumerated) -> u32 {
        let up = UsagePage::from(monitorenumerated);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(monitorenumerated) as u32;
        up | id
    }
}

impl From<&MonitorEnumerated> for UsagePage {
    /// Always returns [UsagePage::MonitorEnumerated] and is
    /// identical to [MonitorEnumerated::usage_page()].
    fn from(_: &MonitorEnumerated) -> UsagePage {
        UsagePage::MonitorEnumerated
    }
}

impl From<MonitorEnumerated> for UsagePage {
    /// Always returns [UsagePage::MonitorEnumerated] and is
    /// identical to [MonitorEnumerated::usage_page()].
    fn from(_: MonitorEnumerated) -> UsagePage {
        UsagePage::MonitorEnumerated
    }
}

impl From<&MonitorEnumerated> for Usage {
    fn from(monitorenumerated: &MonitorEnumerated) -> Usage {
        Usage::try_from(u32::from(monitorenumerated)).unwrap()
    }
}

impl From<MonitorEnumerated> for Usage {
    fn from(monitorenumerated: MonitorEnumerated) -> Usage {
        Usage::from(&monitorenumerated)
    }
}

impl TryFrom<u16> for MonitorEnumerated {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<MonitorEnumerated> {
        match usage_id {
            n => Ok(MonitorEnumerated::MonitorEnumerated(n)),
        }
    }
}

impl BitOr<u16> for MonitorEnumerated {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x82`: "VESA Virtual Controls"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::VESAVirtualControls(VESAVirtualControls::Brightness);
/// let u2 = Usage::new_from_page_and_id(0x82, 0x10).unwrap();
/// let u3 = Usage::from(VESAVirtualControls::Brightness);
/// let u4: Usage = VESAVirtualControls::Brightness.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::VESAVirtualControls));
/// assert_eq!(0x82, u1.usage_page_value());
/// assert_eq!(0x10, u1.usage_id_value());
/// assert_eq!((0x82 << 16) | 0x10, u1.usage_value());
/// assert_eq!("Brightness", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum VESAVirtualControls {
    /// Usage ID `0x1`: "Degauss"
    Degauss = 0x1,
    /// Usage ID `0x10`: "Brightness"
    Brightness = 0x10,
    /// Usage ID `0x12`: "Contrast"
    Contrast = 0x12,
    /// Usage ID `0x16`: "Red Video Gain"
    RedVideoGain = 0x16,
    /// Usage ID `0x18`: "Green Video Gain"
    GreenVideoGain = 0x18,
    /// Usage ID `0x1A`: "Blue Video Gain"
    BlueVideoGain = 0x1A,
    /// Usage ID `0x1C`: "Focus"
    Focus = 0x1C,
    /// Usage ID `0x20`: "Horizontal Position"
    HorizontalPosition = 0x20,
    /// Usage ID `0x22`: "Horizontal Size"
    HorizontalSize = 0x22,
    /// Usage ID `0x24`: "Horizontal Pincushion"
    HorizontalPincushion = 0x24,
    /// Usage ID `0x26`: "Horizontal Pincushion Balance"
    HorizontalPincushionBalance = 0x26,
    /// Usage ID `0x28`: "Horizontal Misconvergence"
    HorizontalMisconvergence = 0x28,
    /// Usage ID `0x2A`: "Horizontal Linearity"
    HorizontalLinearity = 0x2A,
    /// Usage ID `0x2C`: "Horizontal Linearity Balance"
    HorizontalLinearityBalance = 0x2C,
    /// Usage ID `0x30`: "Vertical Position"
    VerticalPosition = 0x30,
    /// Usage ID `0x32`: "Vertical Size"
    VerticalSize = 0x32,
    /// Usage ID `0x34`: "Vertical Pincushion"
    VerticalPincushion = 0x34,
    /// Usage ID `0x36`: "Vertical Pincushion Balance"
    VerticalPincushionBalance = 0x36,
    /// Usage ID `0x38`: "Vertical Misconvergence"
    VerticalMisconvergence = 0x38,
    /// Usage ID `0x3A`: "Vertical Linearity"
    VerticalLinearity = 0x3A,
    /// Usage ID `0x3C`: "Vertical Linearity Balance"
    VerticalLinearityBalance = 0x3C,
    /// Usage ID `0x40`: "Parallelogram Distortion (Key Balance)"
    ParallelogramDistortionKeyBalance = 0x40,
    /// Usage ID `0x42`: "Trapezoidal Distortion (Key)"
    TrapezoidalDistortionKey = 0x42,
    /// Usage ID `0x44`: "Tilt (Rotation)"
    TiltRotation = 0x44,
    /// Usage ID `0x46`: "Top Corner Distortion Control"
    TopCornerDistortionControl = 0x46,
    /// Usage ID `0x48`: "Top Corner Distortion Balance"
    TopCornerDistortionBalance = 0x48,
    /// Usage ID `0x4A`: "Bottom Corner Distortion Control"
    BottomCornerDistortionControl = 0x4A,
    /// Usage ID `0x4C`: "Bottom Corner Distortion Balance"
    BottomCornerDistortionBalance = 0x4C,
    /// Usage ID `0x56`: "Horizontal Moiré"
    HorizontalMoiré = 0x56,
    /// Usage ID `0x58`: "Vertical Moiré"
    VerticalMoiré = 0x58,
    /// Usage ID `0x5E`: "Input Level Select"
    InputLevelSelect = 0x5E,
    /// Usage ID `0x60`: "Input Source Select"
    InputSourceSelect = 0x60,
    /// Usage ID `0x6C`: "Red Video Black Level"
    RedVideoBlackLevel = 0x6C,
    /// Usage ID `0x6E`: "Green Video Black Level"
    GreenVideoBlackLevel = 0x6E,
    /// Usage ID `0x70`: "Blue Video Black Level"
    BlueVideoBlackLevel = 0x70,
    /// Usage ID `0xA2`: "Auto Size Center"
    AutoSizeCenter = 0xA2,
    /// Usage ID `0xA4`: "Polarity Horizontal Synchronization"
    PolarityHorizontalSynchronization = 0xA4,
    /// Usage ID `0xA6`: "Polarity Vertical Synchronization"
    PolarityVerticalSynchronization = 0xA6,
    /// Usage ID `0xA8`: "Synchronization Type"
    SynchronizationType = 0xA8,
    /// Usage ID `0xAA`: "Screen Orientation"
    ScreenOrientation = 0xAA,
    /// Usage ID `0xAC`: "Horizontal Frequency"
    HorizontalFrequency = 0xAC,
    /// Usage ID `0xAE`: "Vertical Frequency"
    VerticalFrequency = 0xAE,
    /// Usage ID `0xB0`: "Settings"
    Settings = 0xB0,
    /// Usage ID `0xCA`: "On Screen Display"
    OnScreenDisplay = 0xCA,
    /// Usage ID `0xD4`: "Stereo Mode"
    StereoMode = 0xD4,
}

impl VESAVirtualControls {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            VESAVirtualControls::Degauss => "Degauss",
            VESAVirtualControls::Brightness => "Brightness",
            VESAVirtualControls::Contrast => "Contrast",
            VESAVirtualControls::RedVideoGain => "Red Video Gain",
            VESAVirtualControls::GreenVideoGain => "Green Video Gain",
            VESAVirtualControls::BlueVideoGain => "Blue Video Gain",
            VESAVirtualControls::Focus => "Focus",
            VESAVirtualControls::HorizontalPosition => "Horizontal Position",
            VESAVirtualControls::HorizontalSize => "Horizontal Size",
            VESAVirtualControls::HorizontalPincushion => "Horizontal Pincushion",
            VESAVirtualControls::HorizontalPincushionBalance => "Horizontal Pincushion Balance",
            VESAVirtualControls::HorizontalMisconvergence => "Horizontal Misconvergence",
            VESAVirtualControls::HorizontalLinearity => "Horizontal Linearity",
            VESAVirtualControls::HorizontalLinearityBalance => "Horizontal Linearity Balance",
            VESAVirtualControls::VerticalPosition => "Vertical Position",
            VESAVirtualControls::VerticalSize => "Vertical Size",
            VESAVirtualControls::VerticalPincushion => "Vertical Pincushion",
            VESAVirtualControls::VerticalPincushionBalance => "Vertical Pincushion Balance",
            VESAVirtualControls::VerticalMisconvergence => "Vertical Misconvergence",
            VESAVirtualControls::VerticalLinearity => "Vertical Linearity",
            VESAVirtualControls::VerticalLinearityBalance => "Vertical Linearity Balance",
            VESAVirtualControls::ParallelogramDistortionKeyBalance => {
                "Parallelogram Distortion (Key Balance)"
            }
            VESAVirtualControls::TrapezoidalDistortionKey => "Trapezoidal Distortion (Key)",
            VESAVirtualControls::TiltRotation => "Tilt (Rotation)",
            VESAVirtualControls::TopCornerDistortionControl => "Top Corner Distortion Control",
            VESAVirtualControls::TopCornerDistortionBalance => "Top Corner Distortion Balance",
            VESAVirtualControls::BottomCornerDistortionControl => {
                "Bottom Corner Distortion Control"
            }
            VESAVirtualControls::BottomCornerDistortionBalance => {
                "Bottom Corner Distortion Balance"
            }
            VESAVirtualControls::HorizontalMoiré => "Horizontal Moiré",
            VESAVirtualControls::VerticalMoiré => "Vertical Moiré",
            VESAVirtualControls::InputLevelSelect => "Input Level Select",
            VESAVirtualControls::InputSourceSelect => "Input Source Select",
            VESAVirtualControls::RedVideoBlackLevel => "Red Video Black Level",
            VESAVirtualControls::GreenVideoBlackLevel => "Green Video Black Level",
            VESAVirtualControls::BlueVideoBlackLevel => "Blue Video Black Level",
            VESAVirtualControls::AutoSizeCenter => "Auto Size Center",
            VESAVirtualControls::PolarityHorizontalSynchronization => {
                "Polarity Horizontal Synchronization"
            }
            VESAVirtualControls::PolarityVerticalSynchronization => {
                "Polarity Vertical Synchronization"
            }
            VESAVirtualControls::SynchronizationType => "Synchronization Type",
            VESAVirtualControls::ScreenOrientation => "Screen Orientation",
            VESAVirtualControls::HorizontalFrequency => "Horizontal Frequency",
            VESAVirtualControls::VerticalFrequency => "Vertical Frequency",
            VESAVirtualControls::Settings => "Settings",
            VESAVirtualControls::OnScreenDisplay => "On Screen Display",
            VESAVirtualControls::StereoMode => "Stereo Mode",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for VESAVirtualControls {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for VESAVirtualControls {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::VESAVirtualControls(self)](Usage::VESAVirtualControls)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for VESAVirtualControls {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x82` for [VESAVirtualControls]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::VESAVirtualControls]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&VESAVirtualControls> for u16 {
    fn from(vesavirtualcontrols: &VESAVirtualControls) -> u16 {
        *vesavirtualcontrols as u16
    }
}

impl From<VESAVirtualControls> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [VESAVirtualControls::usage_page_value()].
    fn from(vesavirtualcontrols: VESAVirtualControls) -> u16 {
        u16::from(&vesavirtualcontrols)
    }
}

impl From<&VESAVirtualControls> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [VESAVirtualControls::usage_value()].
    fn from(vesavirtualcontrols: &VESAVirtualControls) -> u32 {
        let up = UsagePage::from(vesavirtualcontrols);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(vesavirtualcontrols) as u32;
        up | id
    }
}

impl From<&VESAVirtualControls> for UsagePage {
    /// Always returns [UsagePage::VESAVirtualControls] and is
    /// identical to [VESAVirtualControls::usage_page()].
    fn from(_: &VESAVirtualControls) -> UsagePage {
        UsagePage::VESAVirtualControls
    }
}

impl From<VESAVirtualControls> for UsagePage {
    /// Always returns [UsagePage::VESAVirtualControls] and is
    /// identical to [VESAVirtualControls::usage_page()].
    fn from(_: VESAVirtualControls) -> UsagePage {
        UsagePage::VESAVirtualControls
    }
}

impl From<&VESAVirtualControls> for Usage {
    fn from(vesavirtualcontrols: &VESAVirtualControls) -> Usage {
        Usage::try_from(u32::from(vesavirtualcontrols)).unwrap()
    }
}

impl From<VESAVirtualControls> for Usage {
    fn from(vesavirtualcontrols: VESAVirtualControls) -> Usage {
        Usage::from(&vesavirtualcontrols)
    }
}

impl TryFrom<u16> for VESAVirtualControls {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<VESAVirtualControls> {
        match usage_id {
            1 => Ok(VESAVirtualControls::Degauss),
            16 => Ok(VESAVirtualControls::Brightness),
            18 => Ok(VESAVirtualControls::Contrast),
            22 => Ok(VESAVirtualControls::RedVideoGain),
            24 => Ok(VESAVirtualControls::GreenVideoGain),
            26 => Ok(VESAVirtualControls::BlueVideoGain),
            28 => Ok(VESAVirtualControls::Focus),
            32 => Ok(VESAVirtualControls::HorizontalPosition),
            34 => Ok(VESAVirtualControls::HorizontalSize),
            36 => Ok(VESAVirtualControls::HorizontalPincushion),
            38 => Ok(VESAVirtualControls::HorizontalPincushionBalance),
            40 => Ok(VESAVirtualControls::HorizontalMisconvergence),
            42 => Ok(VESAVirtualControls::HorizontalLinearity),
            44 => Ok(VESAVirtualControls::HorizontalLinearityBalance),
            48 => Ok(VESAVirtualControls::VerticalPosition),
            50 => Ok(VESAVirtualControls::VerticalSize),
            52 => Ok(VESAVirtualControls::VerticalPincushion),
            54 => Ok(VESAVirtualControls::VerticalPincushionBalance),
            56 => Ok(VESAVirtualControls::VerticalMisconvergence),
            58 => Ok(VESAVirtualControls::VerticalLinearity),
            60 => Ok(VESAVirtualControls::VerticalLinearityBalance),
            64 => Ok(VESAVirtualControls::ParallelogramDistortionKeyBalance),
            66 => Ok(VESAVirtualControls::TrapezoidalDistortionKey),
            68 => Ok(VESAVirtualControls::TiltRotation),
            70 => Ok(VESAVirtualControls::TopCornerDistortionControl),
            72 => Ok(VESAVirtualControls::TopCornerDistortionBalance),
            74 => Ok(VESAVirtualControls::BottomCornerDistortionControl),
            76 => Ok(VESAVirtualControls::BottomCornerDistortionBalance),
            86 => Ok(VESAVirtualControls::HorizontalMoiré),
            88 => Ok(VESAVirtualControls::VerticalMoiré),
            94 => Ok(VESAVirtualControls::InputLevelSelect),
            96 => Ok(VESAVirtualControls::InputSourceSelect),
            108 => Ok(VESAVirtualControls::RedVideoBlackLevel),
            110 => Ok(VESAVirtualControls::GreenVideoBlackLevel),
            112 => Ok(VESAVirtualControls::BlueVideoBlackLevel),
            162 => Ok(VESAVirtualControls::AutoSizeCenter),
            164 => Ok(VESAVirtualControls::PolarityHorizontalSynchronization),
            166 => Ok(VESAVirtualControls::PolarityVerticalSynchronization),
            168 => Ok(VESAVirtualControls::SynchronizationType),
            170 => Ok(VESAVirtualControls::ScreenOrientation),
            172 => Ok(VESAVirtualControls::HorizontalFrequency),
            174 => Ok(VESAVirtualControls::VerticalFrequency),
            176 => Ok(VESAVirtualControls::Settings),
            202 => Ok(VESAVirtualControls::OnScreenDisplay),
            212 => Ok(VESAVirtualControls::StereoMode),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for VESAVirtualControls {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x84`: "Power"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Power(Power::PresentStatus);
/// let u2 = Usage::new_from_page_and_id(0x84, 0x2).unwrap();
/// let u3 = Usage::from(Power::PresentStatus);
/// let u4: Usage = Power::PresentStatus.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Power));
/// assert_eq!(0x84, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x84 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Present Status", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Power {
    /// Usage ID `0x1`: "iName"
    iName = 0x1,
    /// Usage ID `0x2`: "Present Status"
    PresentStatus = 0x2,
    /// Usage ID `0x3`: "Changed Status"
    ChangedStatus = 0x3,
    /// Usage ID `0x4`: "UPS"
    UPS = 0x4,
    /// Usage ID `0x5`: "Power Supply"
    PowerSupply = 0x5,
    /// Usage ID `0x10`: "Battery System"
    BatterySystem = 0x10,
    /// Usage ID `0x11`: "Battery System Id"
    BatterySystemId = 0x11,
    /// Usage ID `0x12`: "Battery"
    Battery = 0x12,
    /// Usage ID `0x13`: "Battery Id"
    BatteryId = 0x13,
    /// Usage ID `0x14`: "Charger"
    Charger = 0x14,
    /// Usage ID `0x15`: "Charger Id"
    ChargerId = 0x15,
    /// Usage ID `0x16`: "Power Converter"
    PowerConverter = 0x16,
    /// Usage ID `0x17`: "Power Converter Id"
    PowerConverterId = 0x17,
    /// Usage ID `0x18`: "Outlet System"
    OutletSystem = 0x18,
    /// Usage ID `0x19`: "Outlet System Id"
    OutletSystemId = 0x19,
    /// Usage ID `0x1A`: "Input"
    Input = 0x1A,
    /// Usage ID `0x1B`: "Input Id"
    InputId = 0x1B,
    /// Usage ID `0x1C`: "Output"
    Output = 0x1C,
    /// Usage ID `0x1D`: "Output Id"
    OutputId = 0x1D,
    /// Usage ID `0x1E`: "Flow"
    Flow = 0x1E,
    /// Usage ID `0x1F`: "Flow Id"
    FlowId = 0x1F,
    /// Usage ID `0x20`: "Outlet"
    Outlet = 0x20,
    /// Usage ID `0x21`: "Outlet Id"
    OutletId = 0x21,
    /// Usage ID `0x22`: "Gang"
    Gang = 0x22,
    /// Usage ID `0x23`: "Gang Id"
    GangId = 0x23,
    /// Usage ID `0x24`: "Power Summary"
    PowerSummary = 0x24,
    /// Usage ID `0x25`: "Power Summary Id"
    PowerSummaryId = 0x25,
    /// Usage ID `0x30`: "Voltage"
    Voltage = 0x30,
    /// Usage ID `0x31`: "Current"
    Current = 0x31,
    /// Usage ID `0x32`: "Frequency"
    Frequency = 0x32,
    /// Usage ID `0x33`: "Apparent Power"
    ApparentPower = 0x33,
    /// Usage ID `0x34`: "Active Power"
    ActivePower = 0x34,
    /// Usage ID `0x35`: "Percent Load"
    PercentLoad = 0x35,
    /// Usage ID `0x36`: "Temperature"
    Temperature = 0x36,
    /// Usage ID `0x37`: "Humidity"
    Humidity = 0x37,
    /// Usage ID `0x38`: "Bad Count"
    BadCount = 0x38,
    /// Usage ID `0x40`: "Config Voltage"
    ConfigVoltage = 0x40,
    /// Usage ID `0x41`: "Config Current"
    ConfigCurrent = 0x41,
    /// Usage ID `0x42`: "Config Frequency"
    ConfigFrequency = 0x42,
    /// Usage ID `0x43`: "Config Apparent Power"
    ConfigApparentPower = 0x43,
    /// Usage ID `0x44`: "Config Active Power"
    ConfigActivePower = 0x44,
    /// Usage ID `0x45`: "Config Percent Load"
    ConfigPercentLoad = 0x45,
    /// Usage ID `0x46`: "Config Temperature"
    ConfigTemperature = 0x46,
    /// Usage ID `0x47`: "Config Humidity"
    ConfigHumidity = 0x47,
    /// Usage ID `0x50`: "Switch On Control"
    SwitchOnControl = 0x50,
    /// Usage ID `0x51`: "Switch Off Control"
    SwitchOffControl = 0x51,
    /// Usage ID `0x52`: "Toggle Control"
    ToggleControl = 0x52,
    /// Usage ID `0x53`: "Low Voltage Transfer"
    LowVoltageTransfer = 0x53,
    /// Usage ID `0x54`: "High Voltage Transfer"
    HighVoltageTransfer = 0x54,
    /// Usage ID `0x55`: "Delay Before Reboot"
    DelayBeforeReboot = 0x55,
    /// Usage ID `0x56`: "Delay Before Startup"
    DelayBeforeStartup = 0x56,
    /// Usage ID `0x57`: "Delay Before Shutdown"
    DelayBeforeShutdown = 0x57,
    /// Usage ID `0x58`: "Test"
    Test = 0x58,
    /// Usage ID `0x59`: "Module Reset"
    ModuleReset = 0x59,
    /// Usage ID `0x5A`: "Audible Alarm Control"
    AudibleAlarmControl = 0x5A,
    /// Usage ID `0x60`: "Present"
    Present = 0x60,
    /// Usage ID `0x61`: "Good"
    Good = 0x61,
    /// Usage ID `0x62`: "Internal Failure"
    InternalFailure = 0x62,
    /// Usage ID `0x63`: "Voltag Out Of Range"
    VoltagOutOfRange = 0x63,
    /// Usage ID `0x64`: "Frequency Out Of Range"
    FrequencyOutOfRange = 0x64,
    /// Usage ID `0x65`: "Overload"
    Overload = 0x65,
    /// Usage ID `0x66`: "Over Charged"
    OverCharged = 0x66,
    /// Usage ID `0x67`: "Over Temperature"
    OverTemperature = 0x67,
    /// Usage ID `0x68`: "Shutdown Requested"
    ShutdownRequested = 0x68,
    /// Usage ID `0x69`: "Shutdown Imminent"
    ShutdownImminent = 0x69,
    /// Usage ID `0x6B`: "Switch On/Off"
    SwitchOnOff = 0x6B,
    /// Usage ID `0x6C`: "Switchable"
    Switchable = 0x6C,
    /// Usage ID `0x6D`: "Used"
    Used = 0x6D,
    /// Usage ID `0x6E`: "Boost"
    Boost = 0x6E,
    /// Usage ID `0x6F`: "Buck"
    Buck = 0x6F,
    /// Usage ID `0x70`: "Initialized"
    Initialized = 0x70,
    /// Usage ID `0x71`: "Tested"
    Tested = 0x71,
    /// Usage ID `0x72`: "Awaiting Power"
    AwaitingPower = 0x72,
    /// Usage ID `0x73`: "Communication Lost"
    CommunicationLost = 0x73,
    /// Usage ID `0xFD`: "iManufacturer"
    iManufacturer = 0xFD,
    /// Usage ID `0xFE`: "iProduct"
    iProduct = 0xFE,
    /// Usage ID `0xFF`: "iSerialNumber"
    iSerialNumber = 0xFF,
}

impl Power {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Power::iName => "iName",
            Power::PresentStatus => "Present Status",
            Power::ChangedStatus => "Changed Status",
            Power::UPS => "UPS",
            Power::PowerSupply => "Power Supply",
            Power::BatterySystem => "Battery System",
            Power::BatterySystemId => "Battery System Id",
            Power::Battery => "Battery",
            Power::BatteryId => "Battery Id",
            Power::Charger => "Charger",
            Power::ChargerId => "Charger Id",
            Power::PowerConverter => "Power Converter",
            Power::PowerConverterId => "Power Converter Id",
            Power::OutletSystem => "Outlet System",
            Power::OutletSystemId => "Outlet System Id",
            Power::Input => "Input",
            Power::InputId => "Input Id",
            Power::Output => "Output",
            Power::OutputId => "Output Id",
            Power::Flow => "Flow",
            Power::FlowId => "Flow Id",
            Power::Outlet => "Outlet",
            Power::OutletId => "Outlet Id",
            Power::Gang => "Gang",
            Power::GangId => "Gang Id",
            Power::PowerSummary => "Power Summary",
            Power::PowerSummaryId => "Power Summary Id",
            Power::Voltage => "Voltage",
            Power::Current => "Current",
            Power::Frequency => "Frequency",
            Power::ApparentPower => "Apparent Power",
            Power::ActivePower => "Active Power",
            Power::PercentLoad => "Percent Load",
            Power::Temperature => "Temperature",
            Power::Humidity => "Humidity",
            Power::BadCount => "Bad Count",
            Power::ConfigVoltage => "Config Voltage",
            Power::ConfigCurrent => "Config Current",
            Power::ConfigFrequency => "Config Frequency",
            Power::ConfigApparentPower => "Config Apparent Power",
            Power::ConfigActivePower => "Config Active Power",
            Power::ConfigPercentLoad => "Config Percent Load",
            Power::ConfigTemperature => "Config Temperature",
            Power::ConfigHumidity => "Config Humidity",
            Power::SwitchOnControl => "Switch On Control",
            Power::SwitchOffControl => "Switch Off Control",
            Power::ToggleControl => "Toggle Control",
            Power::LowVoltageTransfer => "Low Voltage Transfer",
            Power::HighVoltageTransfer => "High Voltage Transfer",
            Power::DelayBeforeReboot => "Delay Before Reboot",
            Power::DelayBeforeStartup => "Delay Before Startup",
            Power::DelayBeforeShutdown => "Delay Before Shutdown",
            Power::Test => "Test",
            Power::ModuleReset => "Module Reset",
            Power::AudibleAlarmControl => "Audible Alarm Control",
            Power::Present => "Present",
            Power::Good => "Good",
            Power::InternalFailure => "Internal Failure",
            Power::VoltagOutOfRange => "Voltag Out Of Range",
            Power::FrequencyOutOfRange => "Frequency Out Of Range",
            Power::Overload => "Overload",
            Power::OverCharged => "Over Charged",
            Power::OverTemperature => "Over Temperature",
            Power::ShutdownRequested => "Shutdown Requested",
            Power::ShutdownImminent => "Shutdown Imminent",
            Power::SwitchOnOff => "Switch On/Off",
            Power::Switchable => "Switchable",
            Power::Used => "Used",
            Power::Boost => "Boost",
            Power::Buck => "Buck",
            Power::Initialized => "Initialized",
            Power::Tested => "Tested",
            Power::AwaitingPower => "Awaiting Power",
            Power::CommunicationLost => "Communication Lost",
            Power::iManufacturer => "iManufacturer",
            Power::iProduct => "iProduct",
            Power::iSerialNumber => "iSerialNumber",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Power {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Power {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Power(self)](Usage::Power)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Power {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x84` for [Power]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Power]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Power> for u16 {
    fn from(power: &Power) -> u16 {
        *power as u16
    }
}

impl From<Power> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Power::usage_page_value()].
    fn from(power: Power) -> u16 {
        u16::from(&power)
    }
}

impl From<&Power> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Power::usage_value()].
    fn from(power: &Power) -> u32 {
        let up = UsagePage::from(power);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(power) as u32;
        up | id
    }
}

impl From<&Power> for UsagePage {
    /// Always returns [UsagePage::Power] and is
    /// identical to [Power::usage_page()].
    fn from(_: &Power) -> UsagePage {
        UsagePage::Power
    }
}

impl From<Power> for UsagePage {
    /// Always returns [UsagePage::Power] and is
    /// identical to [Power::usage_page()].
    fn from(_: Power) -> UsagePage {
        UsagePage::Power
    }
}

impl From<&Power> for Usage {
    fn from(power: &Power) -> Usage {
        Usage::try_from(u32::from(power)).unwrap()
    }
}

impl From<Power> for Usage {
    fn from(power: Power) -> Usage {
        Usage::from(&power)
    }
}

impl TryFrom<u16> for Power {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Power> {
        match usage_id {
            1 => Ok(Power::iName),
            2 => Ok(Power::PresentStatus),
            3 => Ok(Power::ChangedStatus),
            4 => Ok(Power::UPS),
            5 => Ok(Power::PowerSupply),
            16 => Ok(Power::BatterySystem),
            17 => Ok(Power::BatterySystemId),
            18 => Ok(Power::Battery),
            19 => Ok(Power::BatteryId),
            20 => Ok(Power::Charger),
            21 => Ok(Power::ChargerId),
            22 => Ok(Power::PowerConverter),
            23 => Ok(Power::PowerConverterId),
            24 => Ok(Power::OutletSystem),
            25 => Ok(Power::OutletSystemId),
            26 => Ok(Power::Input),
            27 => Ok(Power::InputId),
            28 => Ok(Power::Output),
            29 => Ok(Power::OutputId),
            30 => Ok(Power::Flow),
            31 => Ok(Power::FlowId),
            32 => Ok(Power::Outlet),
            33 => Ok(Power::OutletId),
            34 => Ok(Power::Gang),
            35 => Ok(Power::GangId),
            36 => Ok(Power::PowerSummary),
            37 => Ok(Power::PowerSummaryId),
            48 => Ok(Power::Voltage),
            49 => Ok(Power::Current),
            50 => Ok(Power::Frequency),
            51 => Ok(Power::ApparentPower),
            52 => Ok(Power::ActivePower),
            53 => Ok(Power::PercentLoad),
            54 => Ok(Power::Temperature),
            55 => Ok(Power::Humidity),
            56 => Ok(Power::BadCount),
            64 => Ok(Power::ConfigVoltage),
            65 => Ok(Power::ConfigCurrent),
            66 => Ok(Power::ConfigFrequency),
            67 => Ok(Power::ConfigApparentPower),
            68 => Ok(Power::ConfigActivePower),
            69 => Ok(Power::ConfigPercentLoad),
            70 => Ok(Power::ConfigTemperature),
            71 => Ok(Power::ConfigHumidity),
            80 => Ok(Power::SwitchOnControl),
            81 => Ok(Power::SwitchOffControl),
            82 => Ok(Power::ToggleControl),
            83 => Ok(Power::LowVoltageTransfer),
            84 => Ok(Power::HighVoltageTransfer),
            85 => Ok(Power::DelayBeforeReboot),
            86 => Ok(Power::DelayBeforeStartup),
            87 => Ok(Power::DelayBeforeShutdown),
            88 => Ok(Power::Test),
            89 => Ok(Power::ModuleReset),
            90 => Ok(Power::AudibleAlarmControl),
            96 => Ok(Power::Present),
            97 => Ok(Power::Good),
            98 => Ok(Power::InternalFailure),
            99 => Ok(Power::VoltagOutOfRange),
            100 => Ok(Power::FrequencyOutOfRange),
            101 => Ok(Power::Overload),
            102 => Ok(Power::OverCharged),
            103 => Ok(Power::OverTemperature),
            104 => Ok(Power::ShutdownRequested),
            105 => Ok(Power::ShutdownImminent),
            107 => Ok(Power::SwitchOnOff),
            108 => Ok(Power::Switchable),
            109 => Ok(Power::Used),
            110 => Ok(Power::Boost),
            111 => Ok(Power::Buck),
            112 => Ok(Power::Initialized),
            113 => Ok(Power::Tested),
            114 => Ok(Power::AwaitingPower),
            115 => Ok(Power::CommunicationLost),
            253 => Ok(Power::iManufacturer),
            254 => Ok(Power::iProduct),
            255 => Ok(Power::iSerialNumber),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Power {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x85`: "Battery System"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::BatterySystem(BatterySystem::SmartBatteryBatteryStatus);
/// let u2 = Usage::new_from_page_and_id(0x85, 0x2).unwrap();
/// let u3 = Usage::from(BatterySystem::SmartBatteryBatteryStatus);
/// let u4: Usage = BatterySystem::SmartBatteryBatteryStatus.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::BatterySystem));
/// assert_eq!(0x85, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x85 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Smart Battery Battery Status", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum BatterySystem {
    /// Usage ID `0x1`: "Smart Battery Battery Mode"
    SmartBatteryBatteryMode = 0x1,
    /// Usage ID `0x2`: "Smart Battery Battery Status"
    SmartBatteryBatteryStatus = 0x2,
    /// Usage ID `0x3`: "Smart Battery Alarm Warning"
    SmartBatteryAlarmWarning = 0x3,
    /// Usage ID `0x4`: "Smart Battery Charger Mode"
    SmartBatteryChargerMode = 0x4,
    /// Usage ID `0x5`: "Smart Battery Charger Status"
    SmartBatteryChargerStatus = 0x5,
    /// Usage ID `0x6`: "Smart Battery Charger Spec Info"
    SmartBatteryChargerSpecInfo = 0x6,
    /// Usage ID `0x7`: "Smart Battery Selector State"
    SmartBatterySelectorState = 0x7,
    /// Usage ID `0x8`: "Smart Battery Selector Presets"
    SmartBatterySelectorPresets = 0x8,
    /// Usage ID `0x9`: "Smart Battery Selector Info"
    SmartBatterySelectorInfo = 0x9,
    /// Usage ID `0x10`: "Optional Mfg Function 1"
    OptionalMfgFunction1 = 0x10,
    /// Usage ID `0x11`: "Optional Mfg Function 2"
    OptionalMfgFunction2 = 0x11,
    /// Usage ID `0x12`: "Optional Mfg Function 3"
    OptionalMfgFunction3 = 0x12,
    /// Usage ID `0x13`: "Optional Mfg Function 4"
    OptionalMfgFunction4 = 0x13,
    /// Usage ID `0x14`: "Optional Mfg Function 5"
    OptionalMfgFunction5 = 0x14,
    /// Usage ID `0x15`: "Connection To SM Bus"
    ConnectionToSMBus = 0x15,
    /// Usage ID `0x16`: "Output Connection"
    OutputConnection = 0x16,
    /// Usage ID `0x17`: "Charger Connection"
    ChargerConnection = 0x17,
    /// Usage ID `0x18`: "Battery Insertion"
    BatteryInsertion = 0x18,
    /// Usage ID `0x19`: "Use Next"
    UseNext = 0x19,
    /// Usage ID `0x1A`: "OK To Use"
    OKToUse = 0x1A,
    /// Usage ID `0x1B`: "Battery Supported"
    BatterySupported = 0x1B,
    /// Usage ID `0x1C`: "Selector Revision"
    SelectorRevision = 0x1C,
    /// Usage ID `0x1D`: "Charging Indicator"
    ChargingIndicator = 0x1D,
    /// Usage ID `0x28`: "Manufacturer Access"
    ManufacturerAccess = 0x28,
    /// Usage ID `0x29`: "Remaining Capacity Limit"
    RemainingCapacityLimit = 0x29,
    /// Usage ID `0x2A`: "Remaining Time Limit"
    RemainingTimeLimit = 0x2A,
    /// Usage ID `0x2B`: "At Rate"
    AtRate = 0x2B,
    /// Usage ID `0x2C`: "Capacity Mode"
    CapacityMode = 0x2C,
    /// Usage ID `0x2D`: "Broadcast To Charger"
    BroadcastToCharger = 0x2D,
    /// Usage ID `0x2E`: "Primary Battery"
    PrimaryBattery = 0x2E,
    /// Usage ID `0x2F`: "Charge Controller"
    ChargeController = 0x2F,
    /// Usage ID `0x40`: "Terminate Charge"
    TerminateCharge = 0x40,
    /// Usage ID `0x41`: "Terminate Discharge"
    TerminateDischarge = 0x41,
    /// Usage ID `0x42`: "Below Remaining Capacity Limit"
    BelowRemainingCapacityLimit = 0x42,
    /// Usage ID `0x43`: "Remaining Time Limit Expired"
    RemainingTimeLimitExpired = 0x43,
    /// Usage ID `0x44`: "Charging"
    Charging = 0x44,
    /// Usage ID `0x45`: "Discharging"
    Discharging = 0x45,
    /// Usage ID `0x46`: "Fully Charged"
    FullyCharged = 0x46,
    /// Usage ID `0x47`: "Fully Discharged"
    FullyDischarged = 0x47,
    /// Usage ID `0x48`: "Conditioning Flag"
    ConditioningFlag = 0x48,
    /// Usage ID `0x49`: "At Rate OK"
    AtRateOK = 0x49,
    /// Usage ID `0x4A`: "Smart Battery Error Code"
    SmartBatteryErrorCode = 0x4A,
    /// Usage ID `0x4B`: "Need Replacement"
    NeedReplacement = 0x4B,
    /// Usage ID `0x60`: "At Rate Time To Full"
    AtRateTimeToFull = 0x60,
    /// Usage ID `0x61`: "At Rate Time To Empty"
    AtRateTimeToEmpty = 0x61,
    /// Usage ID `0x62`: "Average Current"
    AverageCurrent = 0x62,
    /// Usage ID `0x63`: "Max Error"
    MaxError = 0x63,
    /// Usage ID `0x64`: "Relative State Of Charge"
    RelativeStateOfCharge = 0x64,
    /// Usage ID `0x65`: "Absolute State Of Charge"
    AbsoluteStateOfCharge = 0x65,
    /// Usage ID `0x66`: "Remaining Capacity"
    RemainingCapacity = 0x66,
    /// Usage ID `0x67`: "Full Charge Capacity"
    FullChargeCapacity = 0x67,
    /// Usage ID `0x68`: "Run Time To Empty"
    RunTimeToEmpty = 0x68,
    /// Usage ID `0x69`: "Average Time To Empty"
    AverageTimeToEmpty = 0x69,
    /// Usage ID `0x6A`: "Average Time To Full"
    AverageTimeToFull = 0x6A,
    /// Usage ID `0x6B`: "Cycle Count"
    CycleCount = 0x6B,
    /// Usage ID `0x80`: "Battery Pack Model Level"
    BatteryPackModelLevel = 0x80,
    /// Usage ID `0x81`: "Internal Charge Controller"
    InternalChargeController = 0x81,
    /// Usage ID `0x82`: "Primary Battery Support"
    PrimaryBatterySupport = 0x82,
    /// Usage ID `0x83`: "Design Capacity"
    DesignCapacity = 0x83,
    /// Usage ID `0x84`: "Specification Info"
    SpecificationInfo = 0x84,
    /// Usage ID `0x85`: "Manufacture Date"
    ManufactureDate = 0x85,
    /// Usage ID `0x86`: "Serial Number"
    SerialNumber = 0x86,
    /// Usage ID `0x87`: "iManufacturer Name"
    iManufacturerName = 0x87,
    /// Usage ID `0x88`: "iDevice Name"
    iDeviceName = 0x88,
    /// Usage ID `0x89`: "iDevice Chemistry"
    iDeviceChemistry = 0x89,
    /// Usage ID `0x8A`: "Manufacturer Data"
    ManufacturerData = 0x8A,
    /// Usage ID `0x8B`: "Rechargable"
    Rechargable = 0x8B,
    /// Usage ID `0x8C`: "Warning Capacity Limit"
    WarningCapacityLimit = 0x8C,
    /// Usage ID `0x8D`: "Capacity Granularity 1"
    CapacityGranularity1 = 0x8D,
    /// Usage ID `0x8E`: "Capacity Granularity 2"
    CapacityGranularity2 = 0x8E,
    /// Usage ID `0x8F`: "iOEM Information"
    iOEMInformation = 0x8F,
    /// Usage ID `0xC0`: "Inhibit Charge"
    InhibitCharge = 0xC0,
    /// Usage ID `0xC1`: "Enable Polling"
    EnablePolling = 0xC1,
    /// Usage ID `0xC2`: "Reset To Zero"
    ResetToZero = 0xC2,
    /// Usage ID `0xD0`: "AC Present"
    ACPresent = 0xD0,
    /// Usage ID `0xD1`: "Battery Present"
    BatteryPresent = 0xD1,
    /// Usage ID `0xD2`: "Power Fail"
    PowerFail = 0xD2,
    /// Usage ID `0xD3`: "Alarm Inhibited"
    AlarmInhibited = 0xD3,
    /// Usage ID `0xD4`: "Thermistor Under Range"
    ThermistorUnderRange = 0xD4,
    /// Usage ID `0xD5`: "Thermistor Hot"
    ThermistorHot = 0xD5,
    /// Usage ID `0xD6`: "Thermistor Cold"
    ThermistorCold = 0xD6,
    /// Usage ID `0xD7`: "Thermistor Over Range"
    ThermistorOverRange = 0xD7,
    /// Usage ID `0xD8`: "Voltage Out Of Range"
    VoltageOutOfRange = 0xD8,
    /// Usage ID `0xD9`: "Current Out Of Range"
    CurrentOutOfRange = 0xD9,
    /// Usage ID `0xDA`: "Current Not Regulated"
    CurrentNotRegulated = 0xDA,
    /// Usage ID `0xDB`: "Voltage Not Regulated"
    VoltageNotRegulated = 0xDB,
    /// Usage ID `0xDC`: "Master Mode"
    MasterMode = 0xDC,
    /// Usage ID `0xF0`: "Charger Selector Support"
    ChargerSelectorSupport = 0xF0,
    /// Usage ID `0xF1`: "Charger Spec"
    ChargerSpec = 0xF1,
    /// Usage ID `0xF2`: "Level 2"
    Level2 = 0xF2,
    /// Usage ID `0xF3`: "Level 3"
    Level3 = 0xF3,
}

impl BatterySystem {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            BatterySystem::SmartBatteryBatteryMode => "Smart Battery Battery Mode",
            BatterySystem::SmartBatteryBatteryStatus => "Smart Battery Battery Status",
            BatterySystem::SmartBatteryAlarmWarning => "Smart Battery Alarm Warning",
            BatterySystem::SmartBatteryChargerMode => "Smart Battery Charger Mode",
            BatterySystem::SmartBatteryChargerStatus => "Smart Battery Charger Status",
            BatterySystem::SmartBatteryChargerSpecInfo => "Smart Battery Charger Spec Info",
            BatterySystem::SmartBatterySelectorState => "Smart Battery Selector State",
            BatterySystem::SmartBatterySelectorPresets => "Smart Battery Selector Presets",
            BatterySystem::SmartBatterySelectorInfo => "Smart Battery Selector Info",
            BatterySystem::OptionalMfgFunction1 => "Optional Mfg Function 1",
            BatterySystem::OptionalMfgFunction2 => "Optional Mfg Function 2",
            BatterySystem::OptionalMfgFunction3 => "Optional Mfg Function 3",
            BatterySystem::OptionalMfgFunction4 => "Optional Mfg Function 4",
            BatterySystem::OptionalMfgFunction5 => "Optional Mfg Function 5",
            BatterySystem::ConnectionToSMBus => "Connection To SM Bus",
            BatterySystem::OutputConnection => "Output Connection",
            BatterySystem::ChargerConnection => "Charger Connection",
            BatterySystem::BatteryInsertion => "Battery Insertion",
            BatterySystem::UseNext => "Use Next",
            BatterySystem::OKToUse => "OK To Use",
            BatterySystem::BatterySupported => "Battery Supported",
            BatterySystem::SelectorRevision => "Selector Revision",
            BatterySystem::ChargingIndicator => "Charging Indicator",
            BatterySystem::ManufacturerAccess => "Manufacturer Access",
            BatterySystem::RemainingCapacityLimit => "Remaining Capacity Limit",
            BatterySystem::RemainingTimeLimit => "Remaining Time Limit",
            BatterySystem::AtRate => "At Rate",
            BatterySystem::CapacityMode => "Capacity Mode",
            BatterySystem::BroadcastToCharger => "Broadcast To Charger",
            BatterySystem::PrimaryBattery => "Primary Battery",
            BatterySystem::ChargeController => "Charge Controller",
            BatterySystem::TerminateCharge => "Terminate Charge",
            BatterySystem::TerminateDischarge => "Terminate Discharge",
            BatterySystem::BelowRemainingCapacityLimit => "Below Remaining Capacity Limit",
            BatterySystem::RemainingTimeLimitExpired => "Remaining Time Limit Expired",
            BatterySystem::Charging => "Charging",
            BatterySystem::Discharging => "Discharging",
            BatterySystem::FullyCharged => "Fully Charged",
            BatterySystem::FullyDischarged => "Fully Discharged",
            BatterySystem::ConditioningFlag => "Conditioning Flag",
            BatterySystem::AtRateOK => "At Rate OK",
            BatterySystem::SmartBatteryErrorCode => "Smart Battery Error Code",
            BatterySystem::NeedReplacement => "Need Replacement",
            BatterySystem::AtRateTimeToFull => "At Rate Time To Full",
            BatterySystem::AtRateTimeToEmpty => "At Rate Time To Empty",
            BatterySystem::AverageCurrent => "Average Current",
            BatterySystem::MaxError => "Max Error",
            BatterySystem::RelativeStateOfCharge => "Relative State Of Charge",
            BatterySystem::AbsoluteStateOfCharge => "Absolute State Of Charge",
            BatterySystem::RemainingCapacity => "Remaining Capacity",
            BatterySystem::FullChargeCapacity => "Full Charge Capacity",
            BatterySystem::RunTimeToEmpty => "Run Time To Empty",
            BatterySystem::AverageTimeToEmpty => "Average Time To Empty",
            BatterySystem::AverageTimeToFull => "Average Time To Full",
            BatterySystem::CycleCount => "Cycle Count",
            BatterySystem::BatteryPackModelLevel => "Battery Pack Model Level",
            BatterySystem::InternalChargeController => "Internal Charge Controller",
            BatterySystem::PrimaryBatterySupport => "Primary Battery Support",
            BatterySystem::DesignCapacity => "Design Capacity",
            BatterySystem::SpecificationInfo => "Specification Info",
            BatterySystem::ManufactureDate => "Manufacture Date",
            BatterySystem::SerialNumber => "Serial Number",
            BatterySystem::iManufacturerName => "iManufacturer Name",
            BatterySystem::iDeviceName => "iDevice Name",
            BatterySystem::iDeviceChemistry => "iDevice Chemistry",
            BatterySystem::ManufacturerData => "Manufacturer Data",
            BatterySystem::Rechargable => "Rechargable",
            BatterySystem::WarningCapacityLimit => "Warning Capacity Limit",
            BatterySystem::CapacityGranularity1 => "Capacity Granularity 1",
            BatterySystem::CapacityGranularity2 => "Capacity Granularity 2",
            BatterySystem::iOEMInformation => "iOEM Information",
            BatterySystem::InhibitCharge => "Inhibit Charge",
            BatterySystem::EnablePolling => "Enable Polling",
            BatterySystem::ResetToZero => "Reset To Zero",
            BatterySystem::ACPresent => "AC Present",
            BatterySystem::BatteryPresent => "Battery Present",
            BatterySystem::PowerFail => "Power Fail",
            BatterySystem::AlarmInhibited => "Alarm Inhibited",
            BatterySystem::ThermistorUnderRange => "Thermistor Under Range",
            BatterySystem::ThermistorHot => "Thermistor Hot",
            BatterySystem::ThermistorCold => "Thermistor Cold",
            BatterySystem::ThermistorOverRange => "Thermistor Over Range",
            BatterySystem::VoltageOutOfRange => "Voltage Out Of Range",
            BatterySystem::CurrentOutOfRange => "Current Out Of Range",
            BatterySystem::CurrentNotRegulated => "Current Not Regulated",
            BatterySystem::VoltageNotRegulated => "Voltage Not Regulated",
            BatterySystem::MasterMode => "Master Mode",
            BatterySystem::ChargerSelectorSupport => "Charger Selector Support",
            BatterySystem::ChargerSpec => "Charger Spec",
            BatterySystem::Level2 => "Level 2",
            BatterySystem::Level3 => "Level 3",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for BatterySystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for BatterySystem {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::BatterySystem(self)](Usage::BatterySystem)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for BatterySystem {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x85` for [BatterySystem]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::BatterySystem]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&BatterySystem> for u16 {
    fn from(batterysystem: &BatterySystem) -> u16 {
        *batterysystem as u16
    }
}

impl From<BatterySystem> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [BatterySystem::usage_page_value()].
    fn from(batterysystem: BatterySystem) -> u16 {
        u16::from(&batterysystem)
    }
}

impl From<&BatterySystem> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [BatterySystem::usage_value()].
    fn from(batterysystem: &BatterySystem) -> u32 {
        let up = UsagePage::from(batterysystem);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(batterysystem) as u32;
        up | id
    }
}

impl From<&BatterySystem> for UsagePage {
    /// Always returns [UsagePage::BatterySystem] and is
    /// identical to [BatterySystem::usage_page()].
    fn from(_: &BatterySystem) -> UsagePage {
        UsagePage::BatterySystem
    }
}

impl From<BatterySystem> for UsagePage {
    /// Always returns [UsagePage::BatterySystem] and is
    /// identical to [BatterySystem::usage_page()].
    fn from(_: BatterySystem) -> UsagePage {
        UsagePage::BatterySystem
    }
}

impl From<&BatterySystem> for Usage {
    fn from(batterysystem: &BatterySystem) -> Usage {
        Usage::try_from(u32::from(batterysystem)).unwrap()
    }
}

impl From<BatterySystem> for Usage {
    fn from(batterysystem: BatterySystem) -> Usage {
        Usage::from(&batterysystem)
    }
}

impl TryFrom<u16> for BatterySystem {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<BatterySystem> {
        match usage_id {
            1 => Ok(BatterySystem::SmartBatteryBatteryMode),
            2 => Ok(BatterySystem::SmartBatteryBatteryStatus),
            3 => Ok(BatterySystem::SmartBatteryAlarmWarning),
            4 => Ok(BatterySystem::SmartBatteryChargerMode),
            5 => Ok(BatterySystem::SmartBatteryChargerStatus),
            6 => Ok(BatterySystem::SmartBatteryChargerSpecInfo),
            7 => Ok(BatterySystem::SmartBatterySelectorState),
            8 => Ok(BatterySystem::SmartBatterySelectorPresets),
            9 => Ok(BatterySystem::SmartBatterySelectorInfo),
            16 => Ok(BatterySystem::OptionalMfgFunction1),
            17 => Ok(BatterySystem::OptionalMfgFunction2),
            18 => Ok(BatterySystem::OptionalMfgFunction3),
            19 => Ok(BatterySystem::OptionalMfgFunction4),
            20 => Ok(BatterySystem::OptionalMfgFunction5),
            21 => Ok(BatterySystem::ConnectionToSMBus),
            22 => Ok(BatterySystem::OutputConnection),
            23 => Ok(BatterySystem::ChargerConnection),
            24 => Ok(BatterySystem::BatteryInsertion),
            25 => Ok(BatterySystem::UseNext),
            26 => Ok(BatterySystem::OKToUse),
            27 => Ok(BatterySystem::BatterySupported),
            28 => Ok(BatterySystem::SelectorRevision),
            29 => Ok(BatterySystem::ChargingIndicator),
            40 => Ok(BatterySystem::ManufacturerAccess),
            41 => Ok(BatterySystem::RemainingCapacityLimit),
            42 => Ok(BatterySystem::RemainingTimeLimit),
            43 => Ok(BatterySystem::AtRate),
            44 => Ok(BatterySystem::CapacityMode),
            45 => Ok(BatterySystem::BroadcastToCharger),
            46 => Ok(BatterySystem::PrimaryBattery),
            47 => Ok(BatterySystem::ChargeController),
            64 => Ok(BatterySystem::TerminateCharge),
            65 => Ok(BatterySystem::TerminateDischarge),
            66 => Ok(BatterySystem::BelowRemainingCapacityLimit),
            67 => Ok(BatterySystem::RemainingTimeLimitExpired),
            68 => Ok(BatterySystem::Charging),
            69 => Ok(BatterySystem::Discharging),
            70 => Ok(BatterySystem::FullyCharged),
            71 => Ok(BatterySystem::FullyDischarged),
            72 => Ok(BatterySystem::ConditioningFlag),
            73 => Ok(BatterySystem::AtRateOK),
            74 => Ok(BatterySystem::SmartBatteryErrorCode),
            75 => Ok(BatterySystem::NeedReplacement),
            96 => Ok(BatterySystem::AtRateTimeToFull),
            97 => Ok(BatterySystem::AtRateTimeToEmpty),
            98 => Ok(BatterySystem::AverageCurrent),
            99 => Ok(BatterySystem::MaxError),
            100 => Ok(BatterySystem::RelativeStateOfCharge),
            101 => Ok(BatterySystem::AbsoluteStateOfCharge),
            102 => Ok(BatterySystem::RemainingCapacity),
            103 => Ok(BatterySystem::FullChargeCapacity),
            104 => Ok(BatterySystem::RunTimeToEmpty),
            105 => Ok(BatterySystem::AverageTimeToEmpty),
            106 => Ok(BatterySystem::AverageTimeToFull),
            107 => Ok(BatterySystem::CycleCount),
            128 => Ok(BatterySystem::BatteryPackModelLevel),
            129 => Ok(BatterySystem::InternalChargeController),
            130 => Ok(BatterySystem::PrimaryBatterySupport),
            131 => Ok(BatterySystem::DesignCapacity),
            132 => Ok(BatterySystem::SpecificationInfo),
            133 => Ok(BatterySystem::ManufactureDate),
            134 => Ok(BatterySystem::SerialNumber),
            135 => Ok(BatterySystem::iManufacturerName),
            136 => Ok(BatterySystem::iDeviceName),
            137 => Ok(BatterySystem::iDeviceChemistry),
            138 => Ok(BatterySystem::ManufacturerData),
            139 => Ok(BatterySystem::Rechargable),
            140 => Ok(BatterySystem::WarningCapacityLimit),
            141 => Ok(BatterySystem::CapacityGranularity1),
            142 => Ok(BatterySystem::CapacityGranularity2),
            143 => Ok(BatterySystem::iOEMInformation),
            192 => Ok(BatterySystem::InhibitCharge),
            193 => Ok(BatterySystem::EnablePolling),
            194 => Ok(BatterySystem::ResetToZero),
            208 => Ok(BatterySystem::ACPresent),
            209 => Ok(BatterySystem::BatteryPresent),
            210 => Ok(BatterySystem::PowerFail),
            211 => Ok(BatterySystem::AlarmInhibited),
            212 => Ok(BatterySystem::ThermistorUnderRange),
            213 => Ok(BatterySystem::ThermistorHot),
            214 => Ok(BatterySystem::ThermistorCold),
            215 => Ok(BatterySystem::ThermistorOverRange),
            216 => Ok(BatterySystem::VoltageOutOfRange),
            217 => Ok(BatterySystem::CurrentOutOfRange),
            218 => Ok(BatterySystem::CurrentNotRegulated),
            219 => Ok(BatterySystem::VoltageNotRegulated),
            220 => Ok(BatterySystem::MasterMode),
            240 => Ok(BatterySystem::ChargerSelectorSupport),
            241 => Ok(BatterySystem::ChargerSpec),
            242 => Ok(BatterySystem::Level2),
            243 => Ok(BatterySystem::Level3),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for BatterySystem {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x8C`: "Barcode Scanner"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::BarcodeScanner(BarcodeScanner::BarcodeScanner);
/// let u2 = Usage::new_from_page_and_id(0x8C, 0x2).unwrap();
/// let u3 = Usage::from(BarcodeScanner::BarcodeScanner);
/// let u4: Usage = BarcodeScanner::BarcodeScanner.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::BarcodeScanner));
/// assert_eq!(0x8C, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x8C << 16) | 0x2, u1.usage_value());
/// assert_eq!("Barcode Scanner", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum BarcodeScanner {
    /// Usage ID `0x1`: "Barcode Badge Reader"
    BarcodeBadgeReader = 0x1,
    /// Usage ID `0x2`: "Barcode Scanner"
    BarcodeScanner = 0x2,
    /// Usage ID `0x3`: "Dumb Bar Code Scanner"
    DumbBarCodeScanner = 0x3,
    /// Usage ID `0x4`: "Cordless Scanner Base"
    CordlessScannerBase = 0x4,
    /// Usage ID `0x5`: "Bar Code Scanner Cradle"
    BarCodeScannerCradle = 0x5,
    /// Usage ID `0x10`: "Attribute Report"
    AttributeReport = 0x10,
    /// Usage ID `0x11`: "Settings Report"
    SettingsReport = 0x11,
    /// Usage ID `0x12`: "Scanned Data Report"
    ScannedDataReport = 0x12,
    /// Usage ID `0x13`: "Raw Scanned Data Report"
    RawScannedDataReport = 0x13,
    /// Usage ID `0x14`: "Trigger Report"
    TriggerReport = 0x14,
    /// Usage ID `0x15`: "Status Report"
    StatusReport = 0x15,
    /// Usage ID `0x16`: "UPC/EAN Control Report"
    UPCEANControlReport = 0x16,
    /// Usage ID `0x17`: "EAN 2/3 Label Control Report"
    EAN23LabelControlReport = 0x17,
    /// Usage ID `0x18`: "Code 39 Control Report"
    Code39ControlReport = 0x18,
    /// Usage ID `0x19`: "Interleaved 2 of 5 Control Report"
    Interleaved2of5ControlReport = 0x19,
    /// Usage ID `0x1A`: "Standard 2 of 5 Control Report"
    Standard2of5ControlReport = 0x1A,
    /// Usage ID `0x1B`: "MSI Plessey Control Report"
    MSIPlesseyControlReport = 0x1B,
    /// Usage ID `0x1C`: "Codabar Control Report"
    CodabarControlReport = 0x1C,
    /// Usage ID `0x1D`: "Code 128 Control Report"
    Code128ControlReport = 0x1D,
    /// Usage ID `0x1E`: "Misc 1D Control Report"
    Misc1DControlReport = 0x1E,
    /// Usage ID `0x1F`: "2D Control Report"
    TwoDControlReport = 0x1F,
    /// Usage ID `0x30`: "Aiming/Pointer Mode"
    AimingPointerMode = 0x30,
    /// Usage ID `0x31`: "Bar Code Present Sensor"
    BarCodePresentSensor = 0x31,
    /// Usage ID `0x32`: "Class 1A Laser"
    Class1ALaser = 0x32,
    /// Usage ID `0x33`: "Class 2 Laser"
    Class2Laser = 0x33,
    /// Usage ID `0x34`: "Heater Present"
    HeaterPresent = 0x34,
    /// Usage ID `0x35`: "Contact Scanner"
    ContactScanner = 0x35,
    /// Usage ID `0x36`: "Electronic Article Surveillance Notification"
    ElectronicArticleSurveillanceNotification = 0x36,
    /// Usage ID `0x37`: "Constant Electronic Article Surveillance"
    ConstantElectronicArticleSurveillance = 0x37,
    /// Usage ID `0x38`: "Error Indication"
    ErrorIndication = 0x38,
    /// Usage ID `0x39`: "Fixed Beeper"
    FixedBeeper = 0x39,
    /// Usage ID `0x3A`: "Good Decode Indication"
    GoodDecodeIndication = 0x3A,
    /// Usage ID `0x3B`: "Hands Free Scanning"
    HandsFreeScanning = 0x3B,
    /// Usage ID `0x3C`: "Intrinsically Safe"
    IntrinsicallySafe = 0x3C,
    /// Usage ID `0x3D`: "Klasse Eins Laser"
    KlasseEinsLaser = 0x3D,
    /// Usage ID `0x3E`: "Long Range Scanner"
    LongRangeScanner = 0x3E,
    /// Usage ID `0x3F`: "Mirror Speed Control"
    MirrorSpeedControl = 0x3F,
    /// Usage ID `0x40`: "Not On File Indication"
    NotOnFileIndication = 0x40,
    /// Usage ID `0x41`: "Programmable Beeper"
    ProgrammableBeeper = 0x41,
    /// Usage ID `0x42`: "Triggerless"
    Triggerless = 0x42,
    /// Usage ID `0x43`: "Wand"
    Wand = 0x43,
    /// Usage ID `0x44`: "Water Resistant"
    WaterResistant = 0x44,
    /// Usage ID `0x45`: "Multi-Range Scanner"
    MultiRangeScanner = 0x45,
    /// Usage ID `0x46`: "Proximity Sensor"
    ProximitySensor = 0x46,
    /// Usage ID `0x4D`: "Fragment Decoding"
    FragmentDecoding = 0x4D,
    /// Usage ID `0x4E`: "Scanner Read Confidence"
    ScannerReadConfidence = 0x4E,
    /// Usage ID `0x4F`: "Data Prefix"
    DataPrefix = 0x4F,
    /// Usage ID `0x50`: "Prefix AIMI"
    PrefixAIMI = 0x50,
    /// Usage ID `0x51`: "Prefix None"
    PrefixNone = 0x51,
    /// Usage ID `0x52`: "Prefix Proprietary"
    PrefixProprietary = 0x52,
    /// Usage ID `0x55`: "Active Time"
    ActiveTime = 0x55,
    /// Usage ID `0x56`: "Aiming Laser Pattern"
    AimingLaserPattern = 0x56,
    /// Usage ID `0x57`: "Bar Code Present"
    BarCodePresent = 0x57,
    /// Usage ID `0x58`: "Beeper State"
    BeeperState = 0x58,
    /// Usage ID `0x59`: "Laser On Time"
    LaserOnTime = 0x59,
    /// Usage ID `0x5A`: "Laser State"
    LaserState = 0x5A,
    /// Usage ID `0x5B`: "Lockout Time"
    LockoutTime = 0x5B,
    /// Usage ID `0x5C`: "Motor State"
    MotorState = 0x5C,
    /// Usage ID `0x5D`: "Motor Timeout"
    MotorTimeout = 0x5D,
    /// Usage ID `0x5E`: "Power On Reset Scanner"
    PowerOnResetScanner = 0x5E,
    /// Usage ID `0x5F`: "Prevent Read of Barcodes"
    PreventReadofBarcodes = 0x5F,
    /// Usage ID `0x60`: "Initiate Barcode Read"
    InitiateBarcodeRead = 0x60,
    /// Usage ID `0x61`: "Trigger State"
    TriggerState = 0x61,
    /// Usage ID `0x62`: "Trigger Mode"
    TriggerMode = 0x62,
    /// Usage ID `0x63`: "Trigger Mode Blinking Laser On"
    TriggerModeBlinkingLaserOn = 0x63,
    /// Usage ID `0x64`: "Trigger Mode Continuous Laser On"
    TriggerModeContinuousLaserOn = 0x64,
    /// Usage ID `0x65`: "Trigger Mode Laser on while Pulled"
    TriggerModeLaseronwhilePulled = 0x65,
    /// Usage ID `0x66`: "Trigger Mode Laser stays on after release"
    TriggerModeLaserstaysonafterrelease = 0x66,
    /// Usage ID `0x6D`: "Commit Parameters to NVM"
    CommitParameterstoNVM = 0x6D,
    /// Usage ID `0x6E`: "Parameter Scanning"
    ParameterScanning = 0x6E,
    /// Usage ID `0x6F`: "Parameters Changed"
    ParametersChanged = 0x6F,
    /// Usage ID `0x70`: "Set parameter default values"
    Setparameterdefaultvalues = 0x70,
    /// Usage ID `0x75`: "Scanner In Cradle"
    ScannerInCradle = 0x75,
    /// Usage ID `0x76`: "Scanner In Range"
    ScannerInRange = 0x76,
    /// Usage ID `0x7A`: "Aim Duration"
    AimDuration = 0x7A,
    /// Usage ID `0x7B`: "Good Read Lamp Duration"
    GoodReadLampDuration = 0x7B,
    /// Usage ID `0x7C`: "Good Read Lamp Intensity"
    GoodReadLampIntensity = 0x7C,
    /// Usage ID `0x7D`: "Good Read LED"
    GoodReadLED = 0x7D,
    /// Usage ID `0x7E`: "Good Read Tone Frequency"
    GoodReadToneFrequency = 0x7E,
    /// Usage ID `0x7F`: "Good Read Tone Length"
    GoodReadToneLength = 0x7F,
    /// Usage ID `0x80`: "Good Read Tone Volume"
    GoodReadToneVolume = 0x80,
    /// Usage ID `0x82`: "No Read Message"
    NoReadMessage = 0x82,
    /// Usage ID `0x83`: "Not on File Volume"
    NotonFileVolume = 0x83,
    /// Usage ID `0x84`: "Powerup Beep"
    PowerupBeep = 0x84,
    /// Usage ID `0x85`: "Sound Error Beep"
    SoundErrorBeep = 0x85,
    /// Usage ID `0x86`: "Sound Good Read Beep"
    SoundGoodReadBeep = 0x86,
    /// Usage ID `0x87`: "Sound Not On File Beep"
    SoundNotOnFileBeep = 0x87,
    /// Usage ID `0x88`: "Good Read When to Write"
    GoodReadWhentoWrite = 0x88,
    /// Usage ID `0x89`: "GRWTI After Decode"
    GRWTIAfterDecode = 0x89,
    /// Usage ID `0x8A`: "GRWTI Beep/Lamp after transmit"
    GRWTIBeepLampaftertransmit = 0x8A,
    /// Usage ID `0x8B`: "GRWTI No Beep/Lamp use at all"
    GRWTINoBeepLampuseatall = 0x8B,
    /// Usage ID `0x91`: "Bookland EAN"
    BooklandEAN = 0x91,
    /// Usage ID `0x92`: "Convert EAN 8 to 13 Type"
    ConvertEAN8to13Type = 0x92,
    /// Usage ID `0x93`: "Convert UPC A to EAN-13"
    ConvertUPCAtoEAN13 = 0x93,
    /// Usage ID `0x94`: "Convert UPC-E to A"
    ConvertUPCEtoA = 0x94,
    /// Usage ID `0x95`: "EAN-13"
    EAN13 = 0x95,
    /// Usage ID `0x96`: "EAN-8"
    EAN8 = 0x96,
    /// Usage ID `0x97`: "EAN-99 128 Mandatory"
    EAN99128Mandatory = 0x97,
    /// Usage ID `0x98`: "EAN-99 P5/128 Optional"
    EAN99P5128Optional = 0x98,
    /// Usage ID `0x99`: "Enable EAN Two Label"
    EnableEANTwoLabel = 0x99,
    /// Usage ID `0x9A`: "UPC/EAN"
    UPCEAN = 0x9A,
    /// Usage ID `0x9B`: "UPC/EAN Coupon Code"
    UPCEANCouponCode = 0x9B,
    /// Usage ID `0x9C`: "UPC/EAN Periodicals"
    UPCEANPeriodicals = 0x9C,
    /// Usage ID `0x9D`: "UPC-A"
    UPCA = 0x9D,
    /// Usage ID `0x9E`: "UPC-A with 128 Mandatory"
    UPCAwith128Mandatory = 0x9E,
    /// Usage ID `0x9F`: "UPC-A with 128 Optional"
    UPCAwith128Optional = 0x9F,
    /// Usage ID `0xA0`: "UPC-A with P5 Optional"
    UPCAwithP5Optional = 0xA0,
    /// Usage ID `0xA1`: "UPC-E"
    UPCE = 0xA1,
    /// Usage ID `0xA2`: "UPC-E1"
    UPCE1 = 0xA2,
    /// Usage ID `0xA9`: "Periodical"
    Periodical = 0xA9,
    /// Usage ID `0xAA`: "Periodical Auto-Discriminate +2"
    PeriodicalAutoDiscriminatePlus2 = 0xAA,
    /// Usage ID `0xAB`: "Periodical Only Decode with +2"
    PeriodicalOnlyDecodewithPlus2 = 0xAB,
    /// Usage ID `0xAC`: "Periodical Ignore +2"
    PeriodicalIgnorePlus2 = 0xAC,
    /// Usage ID `0xAD`: "Periodical Auto-Discriminate +5"
    PeriodicalAutoDiscriminatePlus5 = 0xAD,
    /// Usage ID `0xAE`: "Periodical Only Decode with +5"
    PeriodicalOnlyDecodewithPlus5 = 0xAE,
    /// Usage ID `0xAF`: "Periodical Ignore +5"
    PeriodicalIgnorePlus5 = 0xAF,
    /// Usage ID `0xB0`: "Check"
    Check = 0xB0,
    /// Usage ID `0xB1`: "Check Disable Price"
    CheckDisablePrice = 0xB1,
    /// Usage ID `0xB2`: "Check Enable 4 digit Price"
    CheckEnable4digitPrice = 0xB2,
    /// Usage ID `0xB3`: "Check Enable 5 digit Price"
    CheckEnable5digitPrice = 0xB3,
    /// Usage ID `0xB4`: "Check Enable European 4 digit Price"
    CheckEnableEuropean4digitPrice = 0xB4,
    /// Usage ID `0xB5`: "Check Enable European 5 digit Price"
    CheckEnableEuropean5digitPrice = 0xB5,
    /// Usage ID `0xB7`: "EAN Two Label"
    EANTwoLabel = 0xB7,
    /// Usage ID `0xB8`: "EAN Three Label"
    EANThreeLabel = 0xB8,
    /// Usage ID `0xB9`: "EAN 8 Flag Digit 1"
    EAN8FlagDigit1 = 0xB9,
    /// Usage ID `0xBA`: "EAN 8 Flag Digit 2"
    EAN8FlagDigit2 = 0xBA,
    /// Usage ID `0xBB`: "EAN 8 Flag Digit 3"
    EAN8FlagDigit3 = 0xBB,
    /// Usage ID `0xBC`: "EAN 13 Flag Digit 1"
    EAN13FlagDigit1 = 0xBC,
    /// Usage ID `0xBD`: "EAN 13 Flag Digit 2"
    EAN13FlagDigit2 = 0xBD,
    /// Usage ID `0xBE`: "EAN 13 Flag Digit 3"
    EAN13FlagDigit3 = 0xBE,
    /// Usage ID `0xBF`: "Add EAN 2/3 Label Definition"
    AddEAN23LabelDefinition = 0xBF,
    /// Usage ID `0xC0`: "Clear all EAN 2/3 Label Definitions"
    ClearallEAN23LabelDefinitions = 0xC0,
    /// Usage ID `0xC3`: "Codabar"
    Codabar = 0xC3,
    /// Usage ID `0xC4`: "Code 128"
    Code128 = 0xC4,
    /// Usage ID `0xC7`: "Code 39"
    Code39 = 0xC7,
    /// Usage ID `0xC8`: "Code 93"
    Code93 = 0xC8,
    /// Usage ID `0xC9`: "Full ASCII Conversion"
    FullASCIIConversion = 0xC9,
    /// Usage ID `0xCA`: "Interleaved 2 of 5"
    Interleaved2of5 = 0xCA,
    /// Usage ID `0xCB`: "Italian Pharmacy Code"
    ItalianPharmacyCode = 0xCB,
    /// Usage ID `0xCC`: "MSI/Plessey"
    MSIPlessey = 0xCC,
    /// Usage ID `0xCD`: "Standard 2 of 5 IATA"
    Standard2of5IATA = 0xCD,
    /// Usage ID `0xCE`: "Standard 2 of 5"
    Standard2of5 = 0xCE,
    /// Usage ID `0xD3`: "Transmit Start/Stop"
    TransmitStartStop = 0xD3,
    /// Usage ID `0xD4`: "Tri-Optic"
    TriOptic = 0xD4,
    /// Usage ID `0xD5`: "UCC/EAN-128"
    UCCEAN128 = 0xD5,
    /// Usage ID `0xD6`: "Check Digit"
    CheckDigit = 0xD6,
    /// Usage ID `0xD7`: "Check Digit Disable"
    CheckDigitDisable = 0xD7,
    /// Usage ID `0xD8`: "Check Digit Enable Interleaved 2 of 5 OPCC"
    CheckDigitEnableInterleaved2of5OPCC = 0xD8,
    /// Usage ID `0xD9`: "Check Digit Enable Interleaved 2 of 5 USS"
    CheckDigitEnableInterleaved2of5USS = 0xD9,
    /// Usage ID `0xDA`: "Check Digit Enable Standard 2 of 5 OPCC"
    CheckDigitEnableStandard2of5OPCC = 0xDA,
    /// Usage ID `0xDB`: "Check Digit Enable Standard 2 of 5 USS"
    CheckDigitEnableStandard2of5USS = 0xDB,
    /// Usage ID `0xDC`: "Check Digit Enable One MSI Plessey"
    CheckDigitEnableOneMSIPlessey = 0xDC,
    /// Usage ID `0xDD`: "Check Digit Enable Two MSI Plessey"
    CheckDigitEnableTwoMSIPlessey = 0xDD,
    /// Usage ID `0xDE`: "Check Digit Codabar Enable"
    CheckDigitCodabarEnable = 0xDE,
    /// Usage ID `0xDF`: "Check Digit Code 39 Enable"
    CheckDigitCode39Enable = 0xDF,
    /// Usage ID `0xF0`: "Transmit Check Digit"
    TransmitCheckDigit = 0xF0,
    /// Usage ID `0xF1`: "Disable Check Digit Transmit"
    DisableCheckDigitTransmit = 0xF1,
    /// Usage ID `0xF2`: "Enable Check Digit Transmit"
    EnableCheckDigitTransmit = 0xF2,
    /// Usage ID `0xFB`: "Symbology Identifier 1"
    SymbologyIdentifier1 = 0xFB,
    /// Usage ID `0xFC`: "Symbology Identifier 2"
    SymbologyIdentifier2 = 0xFC,
    /// Usage ID `0xFD`: "Symbology Identifier 3"
    SymbologyIdentifier3 = 0xFD,
    /// Usage ID `0xFE`: "Decoded Data"
    DecodedData = 0xFE,
    /// Usage ID `0xFF`: "Decode Data Continued"
    DecodeDataContinued = 0xFF,
    /// Usage ID `0x100`: "Bar Space Data"
    BarSpaceData = 0x100,
    /// Usage ID `0x101`: "Scanner Data Accuracy"
    ScannerDataAccuracy = 0x101,
    /// Usage ID `0x102`: "Raw Data Polarity"
    RawDataPolarity = 0x102,
    /// Usage ID `0x103`: "Polarity Inverted Bar Code"
    PolarityInvertedBarCode = 0x103,
    /// Usage ID `0x104`: "Polarity Normal Bar Code"
    PolarityNormalBarCode = 0x104,
    /// Usage ID `0x106`: "Minimum Length to Decode"
    MinimumLengthtoDecode = 0x106,
    /// Usage ID `0x107`: "Maximum Length to Decode"
    MaximumLengthtoDecode = 0x107,
    /// Usage ID `0x108`: "Discrete Length to Decode 1"
    DiscreteLengthtoDecode1 = 0x108,
    /// Usage ID `0x109`: "Discrete Length to Decode 2"
    DiscreteLengthtoDecode2 = 0x109,
    /// Usage ID `0x10A`: "Data Length Method"
    DataLengthMethod = 0x10A,
    /// Usage ID `0x10B`: "DL Method Read any"
    DLMethodReadany = 0x10B,
    /// Usage ID `0x10C`: "DL Method Check in Range"
    DLMethodCheckinRange = 0x10C,
    /// Usage ID `0x10D`: "DL Method Check for Discrete"
    DLMethodCheckforDiscrete = 0x10D,
    /// Usage ID `0x110`: "Aztec Code"
    AztecCode = 0x110,
    /// Usage ID `0x111`: "BC412"
    BC412 = 0x111,
    /// Usage ID `0x112`: "Channel Code"
    ChannelCode = 0x112,
    /// Usage ID `0x113`: "Code 16"
    Code16 = 0x113,
    /// Usage ID `0x114`: "Code 32"
    Code32 = 0x114,
    /// Usage ID `0x115`: "Code 49"
    Code49 = 0x115,
    /// Usage ID `0x116`: "Code One"
    CodeOne = 0x116,
    /// Usage ID `0x117`: "Colorcode"
    Colorcode = 0x117,
    /// Usage ID `0x118`: "Data Matrix"
    DataMatrix = 0x118,
    /// Usage ID `0x119`: "MaxiCode"
    MaxiCode = 0x119,
    /// Usage ID `0x11A`: "MicroPDF"
    MicroPDF = 0x11A,
    /// Usage ID `0x11B`: "PDF-417"
    PDF417 = 0x11B,
    /// Usage ID `0x11C`: "PosiCode"
    PosiCode = 0x11C,
    /// Usage ID `0x11D`: "QR Code"
    QRCode = 0x11D,
    /// Usage ID `0x11E`: "SuperCode"
    SuperCode = 0x11E,
    /// Usage ID `0x11F`: "UltraCode"
    UltraCode = 0x11F,
    /// Usage ID `0x120`: "USD-5 (Slug Code)"
    USD5SlugCode = 0x120,
    /// Usage ID `0x121`: "VeriCode"
    VeriCode = 0x121,
}

impl BarcodeScanner {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            BarcodeScanner::BarcodeBadgeReader => "Barcode Badge Reader",
            BarcodeScanner::BarcodeScanner => "Barcode Scanner",
            BarcodeScanner::DumbBarCodeScanner => "Dumb Bar Code Scanner",
            BarcodeScanner::CordlessScannerBase => "Cordless Scanner Base",
            BarcodeScanner::BarCodeScannerCradle => "Bar Code Scanner Cradle",
            BarcodeScanner::AttributeReport => "Attribute Report",
            BarcodeScanner::SettingsReport => "Settings Report",
            BarcodeScanner::ScannedDataReport => "Scanned Data Report",
            BarcodeScanner::RawScannedDataReport => "Raw Scanned Data Report",
            BarcodeScanner::TriggerReport => "Trigger Report",
            BarcodeScanner::StatusReport => "Status Report",
            BarcodeScanner::UPCEANControlReport => "UPC/EAN Control Report",
            BarcodeScanner::EAN23LabelControlReport => "EAN 2/3 Label Control Report",
            BarcodeScanner::Code39ControlReport => "Code 39 Control Report",
            BarcodeScanner::Interleaved2of5ControlReport => "Interleaved 2 of 5 Control Report",
            BarcodeScanner::Standard2of5ControlReport => "Standard 2 of 5 Control Report",
            BarcodeScanner::MSIPlesseyControlReport => "MSI Plessey Control Report",
            BarcodeScanner::CodabarControlReport => "Codabar Control Report",
            BarcodeScanner::Code128ControlReport => "Code 128 Control Report",
            BarcodeScanner::Misc1DControlReport => "Misc 1D Control Report",
            BarcodeScanner::TwoDControlReport => "2D Control Report",
            BarcodeScanner::AimingPointerMode => "Aiming/Pointer Mode",
            BarcodeScanner::BarCodePresentSensor => "Bar Code Present Sensor",
            BarcodeScanner::Class1ALaser => "Class 1A Laser",
            BarcodeScanner::Class2Laser => "Class 2 Laser",
            BarcodeScanner::HeaterPresent => "Heater Present",
            BarcodeScanner::ContactScanner => "Contact Scanner",
            BarcodeScanner::ElectronicArticleSurveillanceNotification => {
                "Electronic Article Surveillance Notification"
            }
            BarcodeScanner::ConstantElectronicArticleSurveillance => {
                "Constant Electronic Article Surveillance"
            }
            BarcodeScanner::ErrorIndication => "Error Indication",
            BarcodeScanner::FixedBeeper => "Fixed Beeper",
            BarcodeScanner::GoodDecodeIndication => "Good Decode Indication",
            BarcodeScanner::HandsFreeScanning => "Hands Free Scanning",
            BarcodeScanner::IntrinsicallySafe => "Intrinsically Safe",
            BarcodeScanner::KlasseEinsLaser => "Klasse Eins Laser",
            BarcodeScanner::LongRangeScanner => "Long Range Scanner",
            BarcodeScanner::MirrorSpeedControl => "Mirror Speed Control",
            BarcodeScanner::NotOnFileIndication => "Not On File Indication",
            BarcodeScanner::ProgrammableBeeper => "Programmable Beeper",
            BarcodeScanner::Triggerless => "Triggerless",
            BarcodeScanner::Wand => "Wand",
            BarcodeScanner::WaterResistant => "Water Resistant",
            BarcodeScanner::MultiRangeScanner => "Multi-Range Scanner",
            BarcodeScanner::ProximitySensor => "Proximity Sensor",
            BarcodeScanner::FragmentDecoding => "Fragment Decoding",
            BarcodeScanner::ScannerReadConfidence => "Scanner Read Confidence",
            BarcodeScanner::DataPrefix => "Data Prefix",
            BarcodeScanner::PrefixAIMI => "Prefix AIMI",
            BarcodeScanner::PrefixNone => "Prefix None",
            BarcodeScanner::PrefixProprietary => "Prefix Proprietary",
            BarcodeScanner::ActiveTime => "Active Time",
            BarcodeScanner::AimingLaserPattern => "Aiming Laser Pattern",
            BarcodeScanner::BarCodePresent => "Bar Code Present",
            BarcodeScanner::BeeperState => "Beeper State",
            BarcodeScanner::LaserOnTime => "Laser On Time",
            BarcodeScanner::LaserState => "Laser State",
            BarcodeScanner::LockoutTime => "Lockout Time",
            BarcodeScanner::MotorState => "Motor State",
            BarcodeScanner::MotorTimeout => "Motor Timeout",
            BarcodeScanner::PowerOnResetScanner => "Power On Reset Scanner",
            BarcodeScanner::PreventReadofBarcodes => "Prevent Read of Barcodes",
            BarcodeScanner::InitiateBarcodeRead => "Initiate Barcode Read",
            BarcodeScanner::TriggerState => "Trigger State",
            BarcodeScanner::TriggerMode => "Trigger Mode",
            BarcodeScanner::TriggerModeBlinkingLaserOn => "Trigger Mode Blinking Laser On",
            BarcodeScanner::TriggerModeContinuousLaserOn => "Trigger Mode Continuous Laser On",
            BarcodeScanner::TriggerModeLaseronwhilePulled => "Trigger Mode Laser on while Pulled",
            BarcodeScanner::TriggerModeLaserstaysonafterrelease => {
                "Trigger Mode Laser stays on after release"
            }
            BarcodeScanner::CommitParameterstoNVM => "Commit Parameters to NVM",
            BarcodeScanner::ParameterScanning => "Parameter Scanning",
            BarcodeScanner::ParametersChanged => "Parameters Changed",
            BarcodeScanner::Setparameterdefaultvalues => "Set parameter default values",
            BarcodeScanner::ScannerInCradle => "Scanner In Cradle",
            BarcodeScanner::ScannerInRange => "Scanner In Range",
            BarcodeScanner::AimDuration => "Aim Duration",
            BarcodeScanner::GoodReadLampDuration => "Good Read Lamp Duration",
            BarcodeScanner::GoodReadLampIntensity => "Good Read Lamp Intensity",
            BarcodeScanner::GoodReadLED => "Good Read LED",
            BarcodeScanner::GoodReadToneFrequency => "Good Read Tone Frequency",
            BarcodeScanner::GoodReadToneLength => "Good Read Tone Length",
            BarcodeScanner::GoodReadToneVolume => "Good Read Tone Volume",
            BarcodeScanner::NoReadMessage => "No Read Message",
            BarcodeScanner::NotonFileVolume => "Not on File Volume",
            BarcodeScanner::PowerupBeep => "Powerup Beep",
            BarcodeScanner::SoundErrorBeep => "Sound Error Beep",
            BarcodeScanner::SoundGoodReadBeep => "Sound Good Read Beep",
            BarcodeScanner::SoundNotOnFileBeep => "Sound Not On File Beep",
            BarcodeScanner::GoodReadWhentoWrite => "Good Read When to Write",
            BarcodeScanner::GRWTIAfterDecode => "GRWTI After Decode",
            BarcodeScanner::GRWTIBeepLampaftertransmit => "GRWTI Beep/Lamp after transmit",
            BarcodeScanner::GRWTINoBeepLampuseatall => "GRWTI No Beep/Lamp use at all",
            BarcodeScanner::BooklandEAN => "Bookland EAN",
            BarcodeScanner::ConvertEAN8to13Type => "Convert EAN 8 to 13 Type",
            BarcodeScanner::ConvertUPCAtoEAN13 => "Convert UPC A to EAN-13",
            BarcodeScanner::ConvertUPCEtoA => "Convert UPC-E to A",
            BarcodeScanner::EAN13 => "EAN-13",
            BarcodeScanner::EAN8 => "EAN-8",
            BarcodeScanner::EAN99128Mandatory => "EAN-99 128 Mandatory",
            BarcodeScanner::EAN99P5128Optional => "EAN-99 P5/128 Optional",
            BarcodeScanner::EnableEANTwoLabel => "Enable EAN Two Label",
            BarcodeScanner::UPCEAN => "UPC/EAN",
            BarcodeScanner::UPCEANCouponCode => "UPC/EAN Coupon Code",
            BarcodeScanner::UPCEANPeriodicals => "UPC/EAN Periodicals",
            BarcodeScanner::UPCA => "UPC-A",
            BarcodeScanner::UPCAwith128Mandatory => "UPC-A with 128 Mandatory",
            BarcodeScanner::UPCAwith128Optional => "UPC-A with 128 Optional",
            BarcodeScanner::UPCAwithP5Optional => "UPC-A with P5 Optional",
            BarcodeScanner::UPCE => "UPC-E",
            BarcodeScanner::UPCE1 => "UPC-E1",
            BarcodeScanner::Periodical => "Periodical",
            BarcodeScanner::PeriodicalAutoDiscriminatePlus2 => "Periodical Auto-Discriminate +2",
            BarcodeScanner::PeriodicalOnlyDecodewithPlus2 => "Periodical Only Decode with +2",
            BarcodeScanner::PeriodicalIgnorePlus2 => "Periodical Ignore +2",
            BarcodeScanner::PeriodicalAutoDiscriminatePlus5 => "Periodical Auto-Discriminate +5",
            BarcodeScanner::PeriodicalOnlyDecodewithPlus5 => "Periodical Only Decode with +5",
            BarcodeScanner::PeriodicalIgnorePlus5 => "Periodical Ignore +5",
            BarcodeScanner::Check => "Check",
            BarcodeScanner::CheckDisablePrice => "Check Disable Price",
            BarcodeScanner::CheckEnable4digitPrice => "Check Enable 4 digit Price",
            BarcodeScanner::CheckEnable5digitPrice => "Check Enable 5 digit Price",
            BarcodeScanner::CheckEnableEuropean4digitPrice => "Check Enable European 4 digit Price",
            BarcodeScanner::CheckEnableEuropean5digitPrice => "Check Enable European 5 digit Price",
            BarcodeScanner::EANTwoLabel => "EAN Two Label",
            BarcodeScanner::EANThreeLabel => "EAN Three Label",
            BarcodeScanner::EAN8FlagDigit1 => "EAN 8 Flag Digit 1",
            BarcodeScanner::EAN8FlagDigit2 => "EAN 8 Flag Digit 2",
            BarcodeScanner::EAN8FlagDigit3 => "EAN 8 Flag Digit 3",
            BarcodeScanner::EAN13FlagDigit1 => "EAN 13 Flag Digit 1",
            BarcodeScanner::EAN13FlagDigit2 => "EAN 13 Flag Digit 2",
            BarcodeScanner::EAN13FlagDigit3 => "EAN 13 Flag Digit 3",
            BarcodeScanner::AddEAN23LabelDefinition => "Add EAN 2/3 Label Definition",
            BarcodeScanner::ClearallEAN23LabelDefinitions => "Clear all EAN 2/3 Label Definitions",
            BarcodeScanner::Codabar => "Codabar",
            BarcodeScanner::Code128 => "Code 128",
            BarcodeScanner::Code39 => "Code 39",
            BarcodeScanner::Code93 => "Code 93",
            BarcodeScanner::FullASCIIConversion => "Full ASCII Conversion",
            BarcodeScanner::Interleaved2of5 => "Interleaved 2 of 5",
            BarcodeScanner::ItalianPharmacyCode => "Italian Pharmacy Code",
            BarcodeScanner::MSIPlessey => "MSI/Plessey",
            BarcodeScanner::Standard2of5IATA => "Standard 2 of 5 IATA",
            BarcodeScanner::Standard2of5 => "Standard 2 of 5",
            BarcodeScanner::TransmitStartStop => "Transmit Start/Stop",
            BarcodeScanner::TriOptic => "Tri-Optic",
            BarcodeScanner::UCCEAN128 => "UCC/EAN-128",
            BarcodeScanner::CheckDigit => "Check Digit",
            BarcodeScanner::CheckDigitDisable => "Check Digit Disable",
            BarcodeScanner::CheckDigitEnableInterleaved2of5OPCC => {
                "Check Digit Enable Interleaved 2 of 5 OPCC"
            }
            BarcodeScanner::CheckDigitEnableInterleaved2of5USS => {
                "Check Digit Enable Interleaved 2 of 5 USS"
            }
            BarcodeScanner::CheckDigitEnableStandard2of5OPCC => {
                "Check Digit Enable Standard 2 of 5 OPCC"
            }
            BarcodeScanner::CheckDigitEnableStandard2of5USS => {
                "Check Digit Enable Standard 2 of 5 USS"
            }
            BarcodeScanner::CheckDigitEnableOneMSIPlessey => "Check Digit Enable One MSI Plessey",
            BarcodeScanner::CheckDigitEnableTwoMSIPlessey => "Check Digit Enable Two MSI Plessey",
            BarcodeScanner::CheckDigitCodabarEnable => "Check Digit Codabar Enable",
            BarcodeScanner::CheckDigitCode39Enable => "Check Digit Code 39 Enable",
            BarcodeScanner::TransmitCheckDigit => "Transmit Check Digit",
            BarcodeScanner::DisableCheckDigitTransmit => "Disable Check Digit Transmit",
            BarcodeScanner::EnableCheckDigitTransmit => "Enable Check Digit Transmit",
            BarcodeScanner::SymbologyIdentifier1 => "Symbology Identifier 1",
            BarcodeScanner::SymbologyIdentifier2 => "Symbology Identifier 2",
            BarcodeScanner::SymbologyIdentifier3 => "Symbology Identifier 3",
            BarcodeScanner::DecodedData => "Decoded Data",
            BarcodeScanner::DecodeDataContinued => "Decode Data Continued",
            BarcodeScanner::BarSpaceData => "Bar Space Data",
            BarcodeScanner::ScannerDataAccuracy => "Scanner Data Accuracy",
            BarcodeScanner::RawDataPolarity => "Raw Data Polarity",
            BarcodeScanner::PolarityInvertedBarCode => "Polarity Inverted Bar Code",
            BarcodeScanner::PolarityNormalBarCode => "Polarity Normal Bar Code",
            BarcodeScanner::MinimumLengthtoDecode => "Minimum Length to Decode",
            BarcodeScanner::MaximumLengthtoDecode => "Maximum Length to Decode",
            BarcodeScanner::DiscreteLengthtoDecode1 => "Discrete Length to Decode 1",
            BarcodeScanner::DiscreteLengthtoDecode2 => "Discrete Length to Decode 2",
            BarcodeScanner::DataLengthMethod => "Data Length Method",
            BarcodeScanner::DLMethodReadany => "DL Method Read any",
            BarcodeScanner::DLMethodCheckinRange => "DL Method Check in Range",
            BarcodeScanner::DLMethodCheckforDiscrete => "DL Method Check for Discrete",
            BarcodeScanner::AztecCode => "Aztec Code",
            BarcodeScanner::BC412 => "BC412",
            BarcodeScanner::ChannelCode => "Channel Code",
            BarcodeScanner::Code16 => "Code 16",
            BarcodeScanner::Code32 => "Code 32",
            BarcodeScanner::Code49 => "Code 49",
            BarcodeScanner::CodeOne => "Code One",
            BarcodeScanner::Colorcode => "Colorcode",
            BarcodeScanner::DataMatrix => "Data Matrix",
            BarcodeScanner::MaxiCode => "MaxiCode",
            BarcodeScanner::MicroPDF => "MicroPDF",
            BarcodeScanner::PDF417 => "PDF-417",
            BarcodeScanner::PosiCode => "PosiCode",
            BarcodeScanner::QRCode => "QR Code",
            BarcodeScanner::SuperCode => "SuperCode",
            BarcodeScanner::UltraCode => "UltraCode",
            BarcodeScanner::USD5SlugCode => "USD-5 (Slug Code)",
            BarcodeScanner::VeriCode => "VeriCode",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for BarcodeScanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for BarcodeScanner {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::BarcodeScanner(self)](Usage::BarcodeScanner)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for BarcodeScanner {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x8C` for [BarcodeScanner]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::BarcodeScanner]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&BarcodeScanner> for u16 {
    fn from(barcodescanner: &BarcodeScanner) -> u16 {
        *barcodescanner as u16
    }
}

impl From<BarcodeScanner> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [BarcodeScanner::usage_page_value()].
    fn from(barcodescanner: BarcodeScanner) -> u16 {
        u16::from(&barcodescanner)
    }
}

impl From<&BarcodeScanner> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [BarcodeScanner::usage_value()].
    fn from(barcodescanner: &BarcodeScanner) -> u32 {
        let up = UsagePage::from(barcodescanner);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(barcodescanner) as u32;
        up | id
    }
}

impl From<&BarcodeScanner> for UsagePage {
    /// Always returns [UsagePage::BarcodeScanner] and is
    /// identical to [BarcodeScanner::usage_page()].
    fn from(_: &BarcodeScanner) -> UsagePage {
        UsagePage::BarcodeScanner
    }
}

impl From<BarcodeScanner> for UsagePage {
    /// Always returns [UsagePage::BarcodeScanner] and is
    /// identical to [BarcodeScanner::usage_page()].
    fn from(_: BarcodeScanner) -> UsagePage {
        UsagePage::BarcodeScanner
    }
}

impl From<&BarcodeScanner> for Usage {
    fn from(barcodescanner: &BarcodeScanner) -> Usage {
        Usage::try_from(u32::from(barcodescanner)).unwrap()
    }
}

impl From<BarcodeScanner> for Usage {
    fn from(barcodescanner: BarcodeScanner) -> Usage {
        Usage::from(&barcodescanner)
    }
}

impl TryFrom<u16> for BarcodeScanner {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<BarcodeScanner> {
        match usage_id {
            1 => Ok(BarcodeScanner::BarcodeBadgeReader),
            2 => Ok(BarcodeScanner::BarcodeScanner),
            3 => Ok(BarcodeScanner::DumbBarCodeScanner),
            4 => Ok(BarcodeScanner::CordlessScannerBase),
            5 => Ok(BarcodeScanner::BarCodeScannerCradle),
            16 => Ok(BarcodeScanner::AttributeReport),
            17 => Ok(BarcodeScanner::SettingsReport),
            18 => Ok(BarcodeScanner::ScannedDataReport),
            19 => Ok(BarcodeScanner::RawScannedDataReport),
            20 => Ok(BarcodeScanner::TriggerReport),
            21 => Ok(BarcodeScanner::StatusReport),
            22 => Ok(BarcodeScanner::UPCEANControlReport),
            23 => Ok(BarcodeScanner::EAN23LabelControlReport),
            24 => Ok(BarcodeScanner::Code39ControlReport),
            25 => Ok(BarcodeScanner::Interleaved2of5ControlReport),
            26 => Ok(BarcodeScanner::Standard2of5ControlReport),
            27 => Ok(BarcodeScanner::MSIPlesseyControlReport),
            28 => Ok(BarcodeScanner::CodabarControlReport),
            29 => Ok(BarcodeScanner::Code128ControlReport),
            30 => Ok(BarcodeScanner::Misc1DControlReport),
            31 => Ok(BarcodeScanner::TwoDControlReport),
            48 => Ok(BarcodeScanner::AimingPointerMode),
            49 => Ok(BarcodeScanner::BarCodePresentSensor),
            50 => Ok(BarcodeScanner::Class1ALaser),
            51 => Ok(BarcodeScanner::Class2Laser),
            52 => Ok(BarcodeScanner::HeaterPresent),
            53 => Ok(BarcodeScanner::ContactScanner),
            54 => Ok(BarcodeScanner::ElectronicArticleSurveillanceNotification),
            55 => Ok(BarcodeScanner::ConstantElectronicArticleSurveillance),
            56 => Ok(BarcodeScanner::ErrorIndication),
            57 => Ok(BarcodeScanner::FixedBeeper),
            58 => Ok(BarcodeScanner::GoodDecodeIndication),
            59 => Ok(BarcodeScanner::HandsFreeScanning),
            60 => Ok(BarcodeScanner::IntrinsicallySafe),
            61 => Ok(BarcodeScanner::KlasseEinsLaser),
            62 => Ok(BarcodeScanner::LongRangeScanner),
            63 => Ok(BarcodeScanner::MirrorSpeedControl),
            64 => Ok(BarcodeScanner::NotOnFileIndication),
            65 => Ok(BarcodeScanner::ProgrammableBeeper),
            66 => Ok(BarcodeScanner::Triggerless),
            67 => Ok(BarcodeScanner::Wand),
            68 => Ok(BarcodeScanner::WaterResistant),
            69 => Ok(BarcodeScanner::MultiRangeScanner),
            70 => Ok(BarcodeScanner::ProximitySensor),
            77 => Ok(BarcodeScanner::FragmentDecoding),
            78 => Ok(BarcodeScanner::ScannerReadConfidence),
            79 => Ok(BarcodeScanner::DataPrefix),
            80 => Ok(BarcodeScanner::PrefixAIMI),
            81 => Ok(BarcodeScanner::PrefixNone),
            82 => Ok(BarcodeScanner::PrefixProprietary),
            85 => Ok(BarcodeScanner::ActiveTime),
            86 => Ok(BarcodeScanner::AimingLaserPattern),
            87 => Ok(BarcodeScanner::BarCodePresent),
            88 => Ok(BarcodeScanner::BeeperState),
            89 => Ok(BarcodeScanner::LaserOnTime),
            90 => Ok(BarcodeScanner::LaserState),
            91 => Ok(BarcodeScanner::LockoutTime),
            92 => Ok(BarcodeScanner::MotorState),
            93 => Ok(BarcodeScanner::MotorTimeout),
            94 => Ok(BarcodeScanner::PowerOnResetScanner),
            95 => Ok(BarcodeScanner::PreventReadofBarcodes),
            96 => Ok(BarcodeScanner::InitiateBarcodeRead),
            97 => Ok(BarcodeScanner::TriggerState),
            98 => Ok(BarcodeScanner::TriggerMode),
            99 => Ok(BarcodeScanner::TriggerModeBlinkingLaserOn),
            100 => Ok(BarcodeScanner::TriggerModeContinuousLaserOn),
            101 => Ok(BarcodeScanner::TriggerModeLaseronwhilePulled),
            102 => Ok(BarcodeScanner::TriggerModeLaserstaysonafterrelease),
            109 => Ok(BarcodeScanner::CommitParameterstoNVM),
            110 => Ok(BarcodeScanner::ParameterScanning),
            111 => Ok(BarcodeScanner::ParametersChanged),
            112 => Ok(BarcodeScanner::Setparameterdefaultvalues),
            117 => Ok(BarcodeScanner::ScannerInCradle),
            118 => Ok(BarcodeScanner::ScannerInRange),
            122 => Ok(BarcodeScanner::AimDuration),
            123 => Ok(BarcodeScanner::GoodReadLampDuration),
            124 => Ok(BarcodeScanner::GoodReadLampIntensity),
            125 => Ok(BarcodeScanner::GoodReadLED),
            126 => Ok(BarcodeScanner::GoodReadToneFrequency),
            127 => Ok(BarcodeScanner::GoodReadToneLength),
            128 => Ok(BarcodeScanner::GoodReadToneVolume),
            130 => Ok(BarcodeScanner::NoReadMessage),
            131 => Ok(BarcodeScanner::NotonFileVolume),
            132 => Ok(BarcodeScanner::PowerupBeep),
            133 => Ok(BarcodeScanner::SoundErrorBeep),
            134 => Ok(BarcodeScanner::SoundGoodReadBeep),
            135 => Ok(BarcodeScanner::SoundNotOnFileBeep),
            136 => Ok(BarcodeScanner::GoodReadWhentoWrite),
            137 => Ok(BarcodeScanner::GRWTIAfterDecode),
            138 => Ok(BarcodeScanner::GRWTIBeepLampaftertransmit),
            139 => Ok(BarcodeScanner::GRWTINoBeepLampuseatall),
            145 => Ok(BarcodeScanner::BooklandEAN),
            146 => Ok(BarcodeScanner::ConvertEAN8to13Type),
            147 => Ok(BarcodeScanner::ConvertUPCAtoEAN13),
            148 => Ok(BarcodeScanner::ConvertUPCEtoA),
            149 => Ok(BarcodeScanner::EAN13),
            150 => Ok(BarcodeScanner::EAN8),
            151 => Ok(BarcodeScanner::EAN99128Mandatory),
            152 => Ok(BarcodeScanner::EAN99P5128Optional),
            153 => Ok(BarcodeScanner::EnableEANTwoLabel),
            154 => Ok(BarcodeScanner::UPCEAN),
            155 => Ok(BarcodeScanner::UPCEANCouponCode),
            156 => Ok(BarcodeScanner::UPCEANPeriodicals),
            157 => Ok(BarcodeScanner::UPCA),
            158 => Ok(BarcodeScanner::UPCAwith128Mandatory),
            159 => Ok(BarcodeScanner::UPCAwith128Optional),
            160 => Ok(BarcodeScanner::UPCAwithP5Optional),
            161 => Ok(BarcodeScanner::UPCE),
            162 => Ok(BarcodeScanner::UPCE1),
            169 => Ok(BarcodeScanner::Periodical),
            170 => Ok(BarcodeScanner::PeriodicalAutoDiscriminatePlus2),
            171 => Ok(BarcodeScanner::PeriodicalOnlyDecodewithPlus2),
            172 => Ok(BarcodeScanner::PeriodicalIgnorePlus2),
            173 => Ok(BarcodeScanner::PeriodicalAutoDiscriminatePlus5),
            174 => Ok(BarcodeScanner::PeriodicalOnlyDecodewithPlus5),
            175 => Ok(BarcodeScanner::PeriodicalIgnorePlus5),
            176 => Ok(BarcodeScanner::Check),
            177 => Ok(BarcodeScanner::CheckDisablePrice),
            178 => Ok(BarcodeScanner::CheckEnable4digitPrice),
            179 => Ok(BarcodeScanner::CheckEnable5digitPrice),
            180 => Ok(BarcodeScanner::CheckEnableEuropean4digitPrice),
            181 => Ok(BarcodeScanner::CheckEnableEuropean5digitPrice),
            183 => Ok(BarcodeScanner::EANTwoLabel),
            184 => Ok(BarcodeScanner::EANThreeLabel),
            185 => Ok(BarcodeScanner::EAN8FlagDigit1),
            186 => Ok(BarcodeScanner::EAN8FlagDigit2),
            187 => Ok(BarcodeScanner::EAN8FlagDigit3),
            188 => Ok(BarcodeScanner::EAN13FlagDigit1),
            189 => Ok(BarcodeScanner::EAN13FlagDigit2),
            190 => Ok(BarcodeScanner::EAN13FlagDigit3),
            191 => Ok(BarcodeScanner::AddEAN23LabelDefinition),
            192 => Ok(BarcodeScanner::ClearallEAN23LabelDefinitions),
            195 => Ok(BarcodeScanner::Codabar),
            196 => Ok(BarcodeScanner::Code128),
            199 => Ok(BarcodeScanner::Code39),
            200 => Ok(BarcodeScanner::Code93),
            201 => Ok(BarcodeScanner::FullASCIIConversion),
            202 => Ok(BarcodeScanner::Interleaved2of5),
            203 => Ok(BarcodeScanner::ItalianPharmacyCode),
            204 => Ok(BarcodeScanner::MSIPlessey),
            205 => Ok(BarcodeScanner::Standard2of5IATA),
            206 => Ok(BarcodeScanner::Standard2of5),
            211 => Ok(BarcodeScanner::TransmitStartStop),
            212 => Ok(BarcodeScanner::TriOptic),
            213 => Ok(BarcodeScanner::UCCEAN128),
            214 => Ok(BarcodeScanner::CheckDigit),
            215 => Ok(BarcodeScanner::CheckDigitDisable),
            216 => Ok(BarcodeScanner::CheckDigitEnableInterleaved2of5OPCC),
            217 => Ok(BarcodeScanner::CheckDigitEnableInterleaved2of5USS),
            218 => Ok(BarcodeScanner::CheckDigitEnableStandard2of5OPCC),
            219 => Ok(BarcodeScanner::CheckDigitEnableStandard2of5USS),
            220 => Ok(BarcodeScanner::CheckDigitEnableOneMSIPlessey),
            221 => Ok(BarcodeScanner::CheckDigitEnableTwoMSIPlessey),
            222 => Ok(BarcodeScanner::CheckDigitCodabarEnable),
            223 => Ok(BarcodeScanner::CheckDigitCode39Enable),
            240 => Ok(BarcodeScanner::TransmitCheckDigit),
            241 => Ok(BarcodeScanner::DisableCheckDigitTransmit),
            242 => Ok(BarcodeScanner::EnableCheckDigitTransmit),
            251 => Ok(BarcodeScanner::SymbologyIdentifier1),
            252 => Ok(BarcodeScanner::SymbologyIdentifier2),
            253 => Ok(BarcodeScanner::SymbologyIdentifier3),
            254 => Ok(BarcodeScanner::DecodedData),
            255 => Ok(BarcodeScanner::DecodeDataContinued),
            256 => Ok(BarcodeScanner::BarSpaceData),
            257 => Ok(BarcodeScanner::ScannerDataAccuracy),
            258 => Ok(BarcodeScanner::RawDataPolarity),
            259 => Ok(BarcodeScanner::PolarityInvertedBarCode),
            260 => Ok(BarcodeScanner::PolarityNormalBarCode),
            262 => Ok(BarcodeScanner::MinimumLengthtoDecode),
            263 => Ok(BarcodeScanner::MaximumLengthtoDecode),
            264 => Ok(BarcodeScanner::DiscreteLengthtoDecode1),
            265 => Ok(BarcodeScanner::DiscreteLengthtoDecode2),
            266 => Ok(BarcodeScanner::DataLengthMethod),
            267 => Ok(BarcodeScanner::DLMethodReadany),
            268 => Ok(BarcodeScanner::DLMethodCheckinRange),
            269 => Ok(BarcodeScanner::DLMethodCheckforDiscrete),
            272 => Ok(BarcodeScanner::AztecCode),
            273 => Ok(BarcodeScanner::BC412),
            274 => Ok(BarcodeScanner::ChannelCode),
            275 => Ok(BarcodeScanner::Code16),
            276 => Ok(BarcodeScanner::Code32),
            277 => Ok(BarcodeScanner::Code49),
            278 => Ok(BarcodeScanner::CodeOne),
            279 => Ok(BarcodeScanner::Colorcode),
            280 => Ok(BarcodeScanner::DataMatrix),
            281 => Ok(BarcodeScanner::MaxiCode),
            282 => Ok(BarcodeScanner::MicroPDF),
            283 => Ok(BarcodeScanner::PDF417),
            284 => Ok(BarcodeScanner::PosiCode),
            285 => Ok(BarcodeScanner::QRCode),
            286 => Ok(BarcodeScanner::SuperCode),
            287 => Ok(BarcodeScanner::UltraCode),
            288 => Ok(BarcodeScanner::USD5SlugCode),
            289 => Ok(BarcodeScanner::VeriCode),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for BarcodeScanner {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x8D`: "Scales"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Scales(Scales::ScaleDevice);
/// let u2 = Usage::new_from_page_and_id(0x8D, 0x20).unwrap();
/// let u3 = Usage::from(Scales::ScaleDevice);
/// let u4: Usage = Scales::ScaleDevice.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Scales));
/// assert_eq!(0x8D, u1.usage_page_value());
/// assert_eq!(0x20, u1.usage_id_value());
/// assert_eq!((0x8D << 16) | 0x20, u1.usage_value());
/// assert_eq!("Scale Device", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Scales {
    /// Usage ID `0x1`: "Scales"
    Scales = 0x1,
    /// Usage ID `0x20`: "Scale Device"
    ScaleDevice = 0x20,
    /// Usage ID `0x21`: "Scale Class"
    ScaleClass = 0x21,
    /// Usage ID `0x22`: "Scale Class I Metric"
    ScaleClassIMetric = 0x22,
    /// Usage ID `0x23`: "Scale Class II Metric"
    ScaleClassIIMetric = 0x23,
    /// Usage ID `0x24`: "Scale Class III Metric"
    ScaleClassIIIMetric = 0x24,
    /// Usage ID `0x25`: "Scale Class IIIL Metric"
    ScaleClassIIILMetric = 0x25,
    /// Usage ID `0x26`: "Scale Class IV Metric"
    ScaleClassIVMetric = 0x26,
    /// Usage ID `0x27`: "Scale Class III English"
    ScaleClassIIIEnglish = 0x27,
    /// Usage ID `0x28`: "Scale Class IIIL English"
    ScaleClassIIILEnglish = 0x28,
    /// Usage ID `0x29`: "Scale Class IV English"
    ScaleClassIVEnglish = 0x29,
    /// Usage ID `0x2A`: "Scale Class Generic"
    ScaleClassGeneric = 0x2A,
    /// Usage ID `0x30`: "Scale Attribute Report"
    ScaleAttributeReport = 0x30,
    /// Usage ID `0x31`: "Scale Control Report"
    ScaleControlReport = 0x31,
    /// Usage ID `0x32`: "Scale Data Report"
    ScaleDataReport = 0x32,
    /// Usage ID `0x33`: "Scale Status Report"
    ScaleStatusReport = 0x33,
    /// Usage ID `0x34`: "Scale Weight Limit Report"
    ScaleWeightLimitReport = 0x34,
    /// Usage ID `0x35`: "Scale Statistics Report"
    ScaleStatisticsReport = 0x35,
    /// Usage ID `0x40`: "Data Weight"
    DataWeight = 0x40,
    /// Usage ID `0x41`: "Data Scaling"
    DataScaling = 0x41,
    /// Usage ID `0x50`: "Weight Unit"
    WeightUnit = 0x50,
    /// Usage ID `0x51`: "Weight Unit Milligram"
    WeightUnitMilligram = 0x51,
    /// Usage ID `0x52`: "Weight Unit Gram"
    WeightUnitGram = 0x52,
    /// Usage ID `0x53`: "Weight Unit Kilogram"
    WeightUnitKilogram = 0x53,
    /// Usage ID `0x54`: "Weight Unit Carats"
    WeightUnitCarats = 0x54,
    /// Usage ID `0x55`: "Weight Unit Taels"
    WeightUnitTaels = 0x55,
    /// Usage ID `0x56`: "Weight Unit Grains"
    WeightUnitGrains = 0x56,
    /// Usage ID `0x57`: "Weight Unit Pennyweights"
    WeightUnitPennyweights = 0x57,
    /// Usage ID `0x58`: "Weight Unit Metric Ton"
    WeightUnitMetricTon = 0x58,
    /// Usage ID `0x59`: "Weight Unit Avoir Ton"
    WeightUnitAvoirTon = 0x59,
    /// Usage ID `0x5A`: "Weight Unit Troy Ounce"
    WeightUnitTroyOunce = 0x5A,
    /// Usage ID `0x5B`: "Weight Unit Ounce"
    WeightUnitOunce = 0x5B,
    /// Usage ID `0x5C`: "Weight Unit Pound"
    WeightUnitPound = 0x5C,
    /// Usage ID `0x60`: "Calibration Count"
    CalibrationCount = 0x60,
    /// Usage ID `0x61`: "Re-Zero Count"
    ReZeroCount = 0x61,
    /// Usage ID `0x70`: "Scale Status"
    ScaleStatus = 0x70,
    /// Usage ID `0x71`: "Scale Status Fault"
    ScaleStatusFault = 0x71,
    /// Usage ID `0x72`: "Scale Status Stable at Center of Zero"
    ScaleStatusStableatCenterofZero = 0x72,
    /// Usage ID `0x73`: "Scale Status In Motion"
    ScaleStatusInMotion = 0x73,
    /// Usage ID `0x74`: "Scale Status Weight Stable"
    ScaleStatusWeightStable = 0x74,
    /// Usage ID `0x75`: "Scale Status Under Zero"
    ScaleStatusUnderZero = 0x75,
    /// Usage ID `0x76`: "Scale Status Over Weight Limit"
    ScaleStatusOverWeightLimit = 0x76,
    /// Usage ID `0x77`: "Scale Status Requires Calibration"
    ScaleStatusRequiresCalibration = 0x77,
    /// Usage ID `0x78`: "Scale Status Requires Rezeroing"
    ScaleStatusRequiresRezeroing = 0x78,
    /// Usage ID `0x80`: "Zero Scale"
    ZeroScale = 0x80,
    /// Usage ID `0x81`: "Enforced Zero Return"
    EnforcedZeroReturn = 0x81,
}

impl Scales {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Scales::Scales => "Scales",
            Scales::ScaleDevice => "Scale Device",
            Scales::ScaleClass => "Scale Class",
            Scales::ScaleClassIMetric => "Scale Class I Metric",
            Scales::ScaleClassIIMetric => "Scale Class II Metric",
            Scales::ScaleClassIIIMetric => "Scale Class III Metric",
            Scales::ScaleClassIIILMetric => "Scale Class IIIL Metric",
            Scales::ScaleClassIVMetric => "Scale Class IV Metric",
            Scales::ScaleClassIIIEnglish => "Scale Class III English",
            Scales::ScaleClassIIILEnglish => "Scale Class IIIL English",
            Scales::ScaleClassIVEnglish => "Scale Class IV English",
            Scales::ScaleClassGeneric => "Scale Class Generic",
            Scales::ScaleAttributeReport => "Scale Attribute Report",
            Scales::ScaleControlReport => "Scale Control Report",
            Scales::ScaleDataReport => "Scale Data Report",
            Scales::ScaleStatusReport => "Scale Status Report",
            Scales::ScaleWeightLimitReport => "Scale Weight Limit Report",
            Scales::ScaleStatisticsReport => "Scale Statistics Report",
            Scales::DataWeight => "Data Weight",
            Scales::DataScaling => "Data Scaling",
            Scales::WeightUnit => "Weight Unit",
            Scales::WeightUnitMilligram => "Weight Unit Milligram",
            Scales::WeightUnitGram => "Weight Unit Gram",
            Scales::WeightUnitKilogram => "Weight Unit Kilogram",
            Scales::WeightUnitCarats => "Weight Unit Carats",
            Scales::WeightUnitTaels => "Weight Unit Taels",
            Scales::WeightUnitGrains => "Weight Unit Grains",
            Scales::WeightUnitPennyweights => "Weight Unit Pennyweights",
            Scales::WeightUnitMetricTon => "Weight Unit Metric Ton",
            Scales::WeightUnitAvoirTon => "Weight Unit Avoir Ton",
            Scales::WeightUnitTroyOunce => "Weight Unit Troy Ounce",
            Scales::WeightUnitOunce => "Weight Unit Ounce",
            Scales::WeightUnitPound => "Weight Unit Pound",
            Scales::CalibrationCount => "Calibration Count",
            Scales::ReZeroCount => "Re-Zero Count",
            Scales::ScaleStatus => "Scale Status",
            Scales::ScaleStatusFault => "Scale Status Fault",
            Scales::ScaleStatusStableatCenterofZero => "Scale Status Stable at Center of Zero",
            Scales::ScaleStatusInMotion => "Scale Status In Motion",
            Scales::ScaleStatusWeightStable => "Scale Status Weight Stable",
            Scales::ScaleStatusUnderZero => "Scale Status Under Zero",
            Scales::ScaleStatusOverWeightLimit => "Scale Status Over Weight Limit",
            Scales::ScaleStatusRequiresCalibration => "Scale Status Requires Calibration",
            Scales::ScaleStatusRequiresRezeroing => "Scale Status Requires Rezeroing",
            Scales::ZeroScale => "Zero Scale",
            Scales::EnforcedZeroReturn => "Enforced Zero Return",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Scales {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Scales {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Scales(self)](Usage::Scales)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Scales {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x8D` for [Scales]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Scales]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Scales> for u16 {
    fn from(scales: &Scales) -> u16 {
        *scales as u16
    }
}

impl From<Scales> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Scales::usage_page_value()].
    fn from(scales: Scales) -> u16 {
        u16::from(&scales)
    }
}

impl From<&Scales> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Scales::usage_value()].
    fn from(scales: &Scales) -> u32 {
        let up = UsagePage::from(scales);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(scales) as u32;
        up | id
    }
}

impl From<&Scales> for UsagePage {
    /// Always returns [UsagePage::Scales] and is
    /// identical to [Scales::usage_page()].
    fn from(_: &Scales) -> UsagePage {
        UsagePage::Scales
    }
}

impl From<Scales> for UsagePage {
    /// Always returns [UsagePage::Scales] and is
    /// identical to [Scales::usage_page()].
    fn from(_: Scales) -> UsagePage {
        UsagePage::Scales
    }
}

impl From<&Scales> for Usage {
    fn from(scales: &Scales) -> Usage {
        Usage::try_from(u32::from(scales)).unwrap()
    }
}

impl From<Scales> for Usage {
    fn from(scales: Scales) -> Usage {
        Usage::from(&scales)
    }
}

impl TryFrom<u16> for Scales {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Scales> {
        match usage_id {
            1 => Ok(Scales::Scales),
            32 => Ok(Scales::ScaleDevice),
            33 => Ok(Scales::ScaleClass),
            34 => Ok(Scales::ScaleClassIMetric),
            35 => Ok(Scales::ScaleClassIIMetric),
            36 => Ok(Scales::ScaleClassIIIMetric),
            37 => Ok(Scales::ScaleClassIIILMetric),
            38 => Ok(Scales::ScaleClassIVMetric),
            39 => Ok(Scales::ScaleClassIIIEnglish),
            40 => Ok(Scales::ScaleClassIIILEnglish),
            41 => Ok(Scales::ScaleClassIVEnglish),
            42 => Ok(Scales::ScaleClassGeneric),
            48 => Ok(Scales::ScaleAttributeReport),
            49 => Ok(Scales::ScaleControlReport),
            50 => Ok(Scales::ScaleDataReport),
            51 => Ok(Scales::ScaleStatusReport),
            52 => Ok(Scales::ScaleWeightLimitReport),
            53 => Ok(Scales::ScaleStatisticsReport),
            64 => Ok(Scales::DataWeight),
            65 => Ok(Scales::DataScaling),
            80 => Ok(Scales::WeightUnit),
            81 => Ok(Scales::WeightUnitMilligram),
            82 => Ok(Scales::WeightUnitGram),
            83 => Ok(Scales::WeightUnitKilogram),
            84 => Ok(Scales::WeightUnitCarats),
            85 => Ok(Scales::WeightUnitTaels),
            86 => Ok(Scales::WeightUnitGrains),
            87 => Ok(Scales::WeightUnitPennyweights),
            88 => Ok(Scales::WeightUnitMetricTon),
            89 => Ok(Scales::WeightUnitAvoirTon),
            90 => Ok(Scales::WeightUnitTroyOunce),
            91 => Ok(Scales::WeightUnitOunce),
            92 => Ok(Scales::WeightUnitPound),
            96 => Ok(Scales::CalibrationCount),
            97 => Ok(Scales::ReZeroCount),
            112 => Ok(Scales::ScaleStatus),
            113 => Ok(Scales::ScaleStatusFault),
            114 => Ok(Scales::ScaleStatusStableatCenterofZero),
            115 => Ok(Scales::ScaleStatusInMotion),
            116 => Ok(Scales::ScaleStatusWeightStable),
            117 => Ok(Scales::ScaleStatusUnderZero),
            118 => Ok(Scales::ScaleStatusOverWeightLimit),
            119 => Ok(Scales::ScaleStatusRequiresCalibration),
            120 => Ok(Scales::ScaleStatusRequiresRezeroing),
            128 => Ok(Scales::ZeroScale),
            129 => Ok(Scales::EnforcedZeroReturn),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Scales {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x8E`: "Magnetic Stripe Reader"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::MagneticStripeReader(MagneticStripeReader::Track1Length);
/// let u2 = Usage::new_from_page_and_id(0x8E, 0x11).unwrap();
/// let u3 = Usage::from(MagneticStripeReader::Track1Length);
/// let u4: Usage = MagneticStripeReader::Track1Length.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::MagneticStripeReader));
/// assert_eq!(0x8E, u1.usage_page_value());
/// assert_eq!(0x11, u1.usage_id_value());
/// assert_eq!((0x8E << 16) | 0x11, u1.usage_value());
/// assert_eq!("Track 1 Length", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum MagneticStripeReader {
    /// Usage ID `0x1`: "MSR Device Read-Only"
    MSRDeviceReadOnly = 0x1,
    /// Usage ID `0x11`: "Track 1 Length"
    Track1Length = 0x11,
    /// Usage ID `0x12`: "Track 2 Length"
    Track2Length = 0x12,
    /// Usage ID `0x13`: "Track 3 Length"
    Track3Length = 0x13,
    /// Usage ID `0x14`: "Track JIS Length"
    TrackJISLength = 0x14,
    /// Usage ID `0x20`: "Track Data"
    TrackData = 0x20,
    /// Usage ID `0x21`: "Track 1 Data"
    Track1Data = 0x21,
    /// Usage ID `0x22`: "Track 2 Data"
    Track2Data = 0x22,
    /// Usage ID `0x23`: "Track 3 Data"
    Track3Data = 0x23,
    /// Usage ID `0x24`: "Track JIS Data"
    TrackJISData = 0x24,
}

impl MagneticStripeReader {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            MagneticStripeReader::MSRDeviceReadOnly => "MSR Device Read-Only",
            MagneticStripeReader::Track1Length => "Track 1 Length",
            MagneticStripeReader::Track2Length => "Track 2 Length",
            MagneticStripeReader::Track3Length => "Track 3 Length",
            MagneticStripeReader::TrackJISLength => "Track JIS Length",
            MagneticStripeReader::TrackData => "Track Data",
            MagneticStripeReader::Track1Data => "Track 1 Data",
            MagneticStripeReader::Track2Data => "Track 2 Data",
            MagneticStripeReader::Track3Data => "Track 3 Data",
            MagneticStripeReader::TrackJISData => "Track JIS Data",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for MagneticStripeReader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for MagneticStripeReader {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::MagneticStripeReader(self)](Usage::MagneticStripeReader)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for MagneticStripeReader {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x8E` for [MagneticStripeReader]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::MagneticStripeReader]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&MagneticStripeReader> for u16 {
    fn from(magneticstripereader: &MagneticStripeReader) -> u16 {
        *magneticstripereader as u16
    }
}

impl From<MagneticStripeReader> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [MagneticStripeReader::usage_page_value()].
    fn from(magneticstripereader: MagneticStripeReader) -> u16 {
        u16::from(&magneticstripereader)
    }
}

impl From<&MagneticStripeReader> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [MagneticStripeReader::usage_value()].
    fn from(magneticstripereader: &MagneticStripeReader) -> u32 {
        let up = UsagePage::from(magneticstripereader);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(magneticstripereader) as u32;
        up | id
    }
}

impl From<&MagneticStripeReader> for UsagePage {
    /// Always returns [UsagePage::MagneticStripeReader] and is
    /// identical to [MagneticStripeReader::usage_page()].
    fn from(_: &MagneticStripeReader) -> UsagePage {
        UsagePage::MagneticStripeReader
    }
}

impl From<MagneticStripeReader> for UsagePage {
    /// Always returns [UsagePage::MagneticStripeReader] and is
    /// identical to [MagneticStripeReader::usage_page()].
    fn from(_: MagneticStripeReader) -> UsagePage {
        UsagePage::MagneticStripeReader
    }
}

impl From<&MagneticStripeReader> for Usage {
    fn from(magneticstripereader: &MagneticStripeReader) -> Usage {
        Usage::try_from(u32::from(magneticstripereader)).unwrap()
    }
}

impl From<MagneticStripeReader> for Usage {
    fn from(magneticstripereader: MagneticStripeReader) -> Usage {
        Usage::from(&magneticstripereader)
    }
}

impl TryFrom<u16> for MagneticStripeReader {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<MagneticStripeReader> {
        match usage_id {
            1 => Ok(MagneticStripeReader::MSRDeviceReadOnly),
            17 => Ok(MagneticStripeReader::Track1Length),
            18 => Ok(MagneticStripeReader::Track2Length),
            19 => Ok(MagneticStripeReader::Track3Length),
            20 => Ok(MagneticStripeReader::TrackJISLength),
            32 => Ok(MagneticStripeReader::TrackData),
            33 => Ok(MagneticStripeReader::Track1Data),
            34 => Ok(MagneticStripeReader::Track2Data),
            35 => Ok(MagneticStripeReader::Track3Data),
            36 => Ok(MagneticStripeReader::TrackJISData),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for MagneticStripeReader {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x90`: "Camera Control"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::CameraControl(CameraControl::CameraShutter);
/// let u2 = Usage::new_from_page_and_id(0x90, 0x21).unwrap();
/// let u3 = Usage::from(CameraControl::CameraShutter);
/// let u4: Usage = CameraControl::CameraShutter.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::CameraControl));
/// assert_eq!(0x90, u1.usage_page_value());
/// assert_eq!(0x21, u1.usage_id_value());
/// assert_eq!((0x90 << 16) | 0x21, u1.usage_value());
/// assert_eq!("Camera Shutter", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum CameraControl {
    /// Usage ID `0x20`: "Camera Auto-focus"
    CameraAutofocus = 0x20,
    /// Usage ID `0x21`: "Camera Shutter"
    CameraShutter = 0x21,
}

impl CameraControl {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            CameraControl::CameraAutofocus => "Camera Auto-focus",
            CameraControl::CameraShutter => "Camera Shutter",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for CameraControl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for CameraControl {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::CameraControl(self)](Usage::CameraControl)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for CameraControl {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x90` for [CameraControl]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::CameraControl]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&CameraControl> for u16 {
    fn from(cameracontrol: &CameraControl) -> u16 {
        *cameracontrol as u16
    }
}

impl From<CameraControl> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [CameraControl::usage_page_value()].
    fn from(cameracontrol: CameraControl) -> u16 {
        u16::from(&cameracontrol)
    }
}

impl From<&CameraControl> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [CameraControl::usage_value()].
    fn from(cameracontrol: &CameraControl) -> u32 {
        let up = UsagePage::from(cameracontrol);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(cameracontrol) as u32;
        up | id
    }
}

impl From<&CameraControl> for UsagePage {
    /// Always returns [UsagePage::CameraControl] and is
    /// identical to [CameraControl::usage_page()].
    fn from(_: &CameraControl) -> UsagePage {
        UsagePage::CameraControl
    }
}

impl From<CameraControl> for UsagePage {
    /// Always returns [UsagePage::CameraControl] and is
    /// identical to [CameraControl::usage_page()].
    fn from(_: CameraControl) -> UsagePage {
        UsagePage::CameraControl
    }
}

impl From<&CameraControl> for Usage {
    fn from(cameracontrol: &CameraControl) -> Usage {
        Usage::try_from(u32::from(cameracontrol)).unwrap()
    }
}

impl From<CameraControl> for Usage {
    fn from(cameracontrol: CameraControl) -> Usage {
        Usage::from(&cameracontrol)
    }
}

impl TryFrom<u16> for CameraControl {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<CameraControl> {
        match usage_id {
            32 => Ok(CameraControl::CameraAutofocus),
            33 => Ok(CameraControl::CameraShutter),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for CameraControl {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0x91`: "Arcade"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Arcade(Arcade::CoinDoor);
/// let u2 = Usage::new_from_page_and_id(0x91, 0x2).unwrap();
/// let u3 = Usage::from(Arcade::CoinDoor);
/// let u4: Usage = Arcade::CoinDoor.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Arcade));
/// assert_eq!(0x91, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x91 << 16) | 0x2, u1.usage_value());
/// assert_eq!("Coin Door", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Arcade {
    /// Usage ID `0x1`: "General Purpose IO Card"
    GeneralPurposeIOCard = 0x1,
    /// Usage ID `0x2`: "Coin Door"
    CoinDoor = 0x2,
    /// Usage ID `0x3`: "Watchdog Timer"
    WatchdogTimer = 0x3,
    /// Usage ID `0x30`: "General Purpose Analog Input State"
    GeneralPurposeAnalogInputState = 0x30,
    /// Usage ID `0x31`: "General Purpose Digital Input State"
    GeneralPurposeDigitalInputState = 0x31,
    /// Usage ID `0x32`: "General Purpose Optical Input State"
    GeneralPurposeOpticalInputState = 0x32,
    /// Usage ID `0x33`: "General Purpose Digital Output State"
    GeneralPurposeDigitalOutputState = 0x33,
    /// Usage ID `0x34`: "Number of Coin Doors"
    NumberofCoinDoors = 0x34,
    /// Usage ID `0x35`: "Coin Drawer Drop Count"
    CoinDrawerDropCount = 0x35,
    /// Usage ID `0x36`: "Coin Drawer Start"
    CoinDrawerStart = 0x36,
    /// Usage ID `0x37`: "Coin Drawer Service"
    CoinDrawerService = 0x37,
    /// Usage ID `0x38`: "Coin Drawer Tilt"
    CoinDrawerTilt = 0x38,
    /// Usage ID `0x39`: "Coin Door Test"
    CoinDoorTest = 0x39,
    /// Usage ID `0x40`: "Coin Door Lockout"
    CoinDoorLockout = 0x40,
    /// Usage ID `0x41`: "Watchdog Timeout"
    WatchdogTimeout = 0x41,
    /// Usage ID `0x42`: "Watchdog Action"
    WatchdogAction = 0x42,
    /// Usage ID `0x43`: "Watchdog Reboot"
    WatchdogReboot = 0x43,
    /// Usage ID `0x44`: "Watchdog Restart"
    WatchdogRestart = 0x44,
    /// Usage ID `0x45`: "Alarm Input"
    AlarmInput = 0x45,
    /// Usage ID `0x46`: "Coin Door Counter"
    CoinDoorCounter = 0x46,
    /// Usage ID `0x47`: "I/O Direction Mapping"
    IODirectionMapping = 0x47,
    /// Usage ID `0x48`: "Set I/O Direction Mapping"
    SetIODirectionMapping = 0x48,
    /// Usage ID `0x49`: "Extended Optical Input State"
    ExtendedOpticalInputState = 0x49,
    /// Usage ID `0x4A`: "Pin Pad Input State"
    PinPadInputState = 0x4A,
    /// Usage ID `0x4B`: "Pin Pad Status"
    PinPadStatus = 0x4B,
    /// Usage ID `0x4C`: "Pin Pad Output"
    PinPadOutput = 0x4C,
    /// Usage ID `0x4D`: "Pin Pad Command"
    PinPadCommand = 0x4D,
}

impl Arcade {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Arcade::GeneralPurposeIOCard => "General Purpose IO Card",
            Arcade::CoinDoor => "Coin Door",
            Arcade::WatchdogTimer => "Watchdog Timer",
            Arcade::GeneralPurposeAnalogInputState => "General Purpose Analog Input State",
            Arcade::GeneralPurposeDigitalInputState => "General Purpose Digital Input State",
            Arcade::GeneralPurposeOpticalInputState => "General Purpose Optical Input State",
            Arcade::GeneralPurposeDigitalOutputState => "General Purpose Digital Output State",
            Arcade::NumberofCoinDoors => "Number of Coin Doors",
            Arcade::CoinDrawerDropCount => "Coin Drawer Drop Count",
            Arcade::CoinDrawerStart => "Coin Drawer Start",
            Arcade::CoinDrawerService => "Coin Drawer Service",
            Arcade::CoinDrawerTilt => "Coin Drawer Tilt",
            Arcade::CoinDoorTest => "Coin Door Test",
            Arcade::CoinDoorLockout => "Coin Door Lockout",
            Arcade::WatchdogTimeout => "Watchdog Timeout",
            Arcade::WatchdogAction => "Watchdog Action",
            Arcade::WatchdogReboot => "Watchdog Reboot",
            Arcade::WatchdogRestart => "Watchdog Restart",
            Arcade::AlarmInput => "Alarm Input",
            Arcade::CoinDoorCounter => "Coin Door Counter",
            Arcade::IODirectionMapping => "I/O Direction Mapping",
            Arcade::SetIODirectionMapping => "Set I/O Direction Mapping",
            Arcade::ExtendedOpticalInputState => "Extended Optical Input State",
            Arcade::PinPadInputState => "Pin Pad Input State",
            Arcade::PinPadStatus => "Pin Pad Status",
            Arcade::PinPadOutput => "Pin Pad Output",
            Arcade::PinPadCommand => "Pin Pad Command",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Arcade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Arcade {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Arcade(self)](Usage::Arcade)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Arcade {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0x91` for [Arcade]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Arcade]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Arcade> for u16 {
    fn from(arcade: &Arcade) -> u16 {
        *arcade as u16
    }
}

impl From<Arcade> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Arcade::usage_page_value()].
    fn from(arcade: Arcade) -> u16 {
        u16::from(&arcade)
    }
}

impl From<&Arcade> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Arcade::usage_value()].
    fn from(arcade: &Arcade) -> u32 {
        let up = UsagePage::from(arcade);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(arcade) as u32;
        up | id
    }
}

impl From<&Arcade> for UsagePage {
    /// Always returns [UsagePage::Arcade] and is
    /// identical to [Arcade::usage_page()].
    fn from(_: &Arcade) -> UsagePage {
        UsagePage::Arcade
    }
}

impl From<Arcade> for UsagePage {
    /// Always returns [UsagePage::Arcade] and is
    /// identical to [Arcade::usage_page()].
    fn from(_: Arcade) -> UsagePage {
        UsagePage::Arcade
    }
}

impl From<&Arcade> for Usage {
    fn from(arcade: &Arcade) -> Usage {
        Usage::try_from(u32::from(arcade)).unwrap()
    }
}

impl From<Arcade> for Usage {
    fn from(arcade: Arcade) -> Usage {
        Usage::from(&arcade)
    }
}

impl TryFrom<u16> for Arcade {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Arcade> {
        match usage_id {
            1 => Ok(Arcade::GeneralPurposeIOCard),
            2 => Ok(Arcade::CoinDoor),
            3 => Ok(Arcade::WatchdogTimer),
            48 => Ok(Arcade::GeneralPurposeAnalogInputState),
            49 => Ok(Arcade::GeneralPurposeDigitalInputState),
            50 => Ok(Arcade::GeneralPurposeOpticalInputState),
            51 => Ok(Arcade::GeneralPurposeDigitalOutputState),
            52 => Ok(Arcade::NumberofCoinDoors),
            53 => Ok(Arcade::CoinDrawerDropCount),
            54 => Ok(Arcade::CoinDrawerStart),
            55 => Ok(Arcade::CoinDrawerService),
            56 => Ok(Arcade::CoinDrawerTilt),
            57 => Ok(Arcade::CoinDoorTest),
            64 => Ok(Arcade::CoinDoorLockout),
            65 => Ok(Arcade::WatchdogTimeout),
            66 => Ok(Arcade::WatchdogAction),
            67 => Ok(Arcade::WatchdogReboot),
            68 => Ok(Arcade::WatchdogRestart),
            69 => Ok(Arcade::AlarmInput),
            70 => Ok(Arcade::CoinDoorCounter),
            71 => Ok(Arcade::IODirectionMapping),
            72 => Ok(Arcade::SetIODirectionMapping),
            73 => Ok(Arcade::ExtendedOpticalInputState),
            74 => Ok(Arcade::PinPadInputState),
            75 => Ok(Arcade::PinPadStatus),
            76 => Ok(Arcade::PinPadOutput),
            77 => Ok(Arcade::PinPadCommand),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Arcade {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xF1D0`: "FIDO Alliance"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::FIDOAlliance(FIDOAlliance::InputReportData);
/// let u2 = Usage::new_from_page_and_id(0xF1D0, 0x20).unwrap();
/// let u3 = Usage::from(FIDOAlliance::InputReportData);
/// let u4: Usage = FIDOAlliance::InputReportData.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::FIDOAlliance));
/// assert_eq!(0xF1D0, u1.usage_page_value());
/// assert_eq!(0x20, u1.usage_id_value());
/// assert_eq!((0xF1D0 << 16) | 0x20, u1.usage_value());
/// assert_eq!("Input Report Data", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum FIDOAlliance {
    /// Usage ID `0x1`: "U2F Authenticator Device"
    U2FAuthenticatorDevice = 0x1,
    /// Usage ID `0x20`: "Input Report Data"
    InputReportData = 0x20,
    /// Usage ID `0x21`: "Output Report Data"
    OutputReportData = 0x21,
}

impl FIDOAlliance {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            FIDOAlliance::U2FAuthenticatorDevice => "U2F Authenticator Device",
            FIDOAlliance::InputReportData => "Input Report Data",
            FIDOAlliance::OutputReportData => "Output Report Data",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for FIDOAlliance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for FIDOAlliance {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::FIDOAlliance(self)](Usage::FIDOAlliance)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for FIDOAlliance {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xF1D0` for [FIDOAlliance]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::FIDOAlliance]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&FIDOAlliance> for u16 {
    fn from(fidoalliance: &FIDOAlliance) -> u16 {
        *fidoalliance as u16
    }
}

impl From<FIDOAlliance> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [FIDOAlliance::usage_page_value()].
    fn from(fidoalliance: FIDOAlliance) -> u16 {
        u16::from(&fidoalliance)
    }
}

impl From<&FIDOAlliance> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [FIDOAlliance::usage_value()].
    fn from(fidoalliance: &FIDOAlliance) -> u32 {
        let up = UsagePage::from(fidoalliance);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(fidoalliance) as u32;
        up | id
    }
}

impl From<&FIDOAlliance> for UsagePage {
    /// Always returns [UsagePage::FIDOAlliance] and is
    /// identical to [FIDOAlliance::usage_page()].
    fn from(_: &FIDOAlliance) -> UsagePage {
        UsagePage::FIDOAlliance
    }
}

impl From<FIDOAlliance> for UsagePage {
    /// Always returns [UsagePage::FIDOAlliance] and is
    /// identical to [FIDOAlliance::usage_page()].
    fn from(_: FIDOAlliance) -> UsagePage {
        UsagePage::FIDOAlliance
    }
}

impl From<&FIDOAlliance> for Usage {
    fn from(fidoalliance: &FIDOAlliance) -> Usage {
        Usage::try_from(u32::from(fidoalliance)).unwrap()
    }
}

impl From<FIDOAlliance> for Usage {
    fn from(fidoalliance: FIDOAlliance) -> Usage {
        Usage::from(&fidoalliance)
    }
}

impl TryFrom<u16> for FIDOAlliance {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<FIDOAlliance> {
        match usage_id {
            1 => Ok(FIDOAlliance::U2FAuthenticatorDevice),
            32 => Ok(FIDOAlliance::InputReportData),
            33 => Ok(FIDOAlliance::OutputReportData),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for FIDOAlliance {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Usage Page `0xFF0D`: "Wacom"*
///
/// **This enum is autogenerated from the HID Usage Tables**.
/// ```
/// # use hut::*;
/// let u1 = Usage::Wacom(Wacom::WacomPen);
/// let u2 = Usage::new_from_page_and_id(0xFF0D, 0x2).unwrap();
/// let u3 = Usage::from(Wacom::WacomPen);
/// let u4: Usage = Wacom::WacomPen.into();
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
/// assert_eq!(u1, u4);
///
/// assert!(matches!(u1.usage_page(), UsagePage::Wacom));
/// assert_eq!(0xFF0D, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0xFF0D << 16) | 0x2, u1.usage_value());
/// assert_eq!("Wacom Pen", u1.name());
/// ```
///
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[repr(u16)]
pub enum Wacom {
    /// Usage ID `0x1`: "Wacom Digitizer"
    WacomDigitizer = 0x1,
    /// Usage ID `0x2`: "Wacom Pen"
    WacomPen = 0x2,
    /// Usage ID `0x3`: "Light Pen"
    LightPen = 0x3,
    /// Usage ID `0x4`: "Touch Screen"
    TouchScreen = 0x4,
    /// Usage ID `0x5`: "Touch Pad"
    TouchPad = 0x5,
    /// Usage ID `0x6`: "White Board"
    WhiteBoard = 0x6,
    /// Usage ID `0x7`: "Coordinate Measuring Machine"
    CoordinateMeasuringMachine = 0x7,
    /// Usage ID `0x8`: "3-D Digitizer"
    ThreeDDigitizer = 0x8,
    /// Usage ID `0x9`: "Stereo Plotter"
    StereoPlotter = 0x9,
    /// Usage ID `0xA`: "Articulated Arm"
    ArticulatedArm = 0xA,
    /// Usage ID `0xB`: "Armature"
    Armature = 0xB,
    /// Usage ID `0xC`: "Multiple Point Digitizer"
    MultiplePointDigitizer = 0xC,
    /// Usage ID `0xD`: "Free Space Wand"
    FreeSpaceWand = 0xD,
    /// Usage ID `0xE`: "Device Configuration"
    DeviceConfiguration = 0xE,
    /// Usage ID `0x20`: "Stylus"
    Stylus = 0x20,
    /// Usage ID `0x21`: "Puck"
    Puck = 0x21,
    /// Usage ID `0x22`: "Finger"
    Finger = 0x22,
    /// Usage ID `0x23`: "Device Settings"
    DeviceSettings = 0x23,
    /// Usage ID `0x30`: "Tip Pressure"
    TipPressure = 0x30,
    /// Usage ID `0x31`: "Barrel Pressure"
    BarrelPressure = 0x31,
    /// Usage ID `0x32`: "In Range"
    InRange = 0x32,
    /// Usage ID `0x33`: "Touch"
    Touch = 0x33,
    /// Usage ID `0x34`: "Untouch"
    Untouch = 0x34,
    /// Usage ID `0x35`: "Tap"
    Tap = 0x35,
    /// Usage ID `0x36`: "Wacom Sense"
    WacomSense = 0x36,
    /// Usage ID `0x37`: "Data Valid"
    DataValid = 0x37,
    /// Usage ID `0x38`: "Transducer Index"
    TransducerIndex = 0x38,
    /// Usage ID `0x39`: "Wacom DigitizerFnKeys"
    WacomDigitizerFnKeys = 0x39,
    /// Usage ID `0x3A`: "Program Change Keys"
    ProgramChangeKeys = 0x3A,
    /// Usage ID `0x3B`: "Battery Strength"
    BatteryStrength = 0x3B,
    /// Usage ID `0x3C`: "Invert"
    Invert = 0x3C,
    /// Usage ID `0x3D`: "X Tilt"
    XTilt = 0x3D,
    /// Usage ID `0x3E`: "Y Tilt"
    YTilt = 0x3E,
    /// Usage ID `0x3F`: "Azimuth"
    Azimuth = 0x3F,
    /// Usage ID `0x40`: "Altitude"
    Altitude = 0x40,
    /// Usage ID `0x41`: "Twist"
    Twist = 0x41,
    /// Usage ID `0x42`: "Tip Switch"
    TipSwitch = 0x42,
    /// Usage ID `0x43`: "Secondary Tip Switch"
    SecondaryTipSwitch = 0x43,
    /// Usage ID `0x44`: "Barrel Switch"
    BarrelSwitch = 0x44,
    /// Usage ID `0x45`: "Eraser"
    Eraser = 0x45,
    /// Usage ID `0x46`: "Tablet Pick"
    TabletPick = 0x46,
    /// Usage ID `0x47`: "Confidence"
    Confidence = 0x47,
    /// Usage ID `0x48`: "Width"
    Width = 0x48,
    /// Usage ID `0x49`: "Height"
    Height = 0x49,
    /// Usage ID `0x51`: "Contact Id"
    ContactId = 0x51,
    /// Usage ID `0x52`: "Inputmode"
    Inputmode = 0x52,
    /// Usage ID `0x53`: "Device Index"
    DeviceIndex = 0x53,
    /// Usage ID `0x54`: "Contact Count"
    ContactCount = 0x54,
    /// Usage ID `0x55`: "Contact Max"
    ContactMax = 0x55,
    /// Usage ID `0x56`: "Scan Time"
    ScanTime = 0x56,
    /// Usage ID `0x57`: "Surface Switch"
    SurfaceSwitch = 0x57,
    /// Usage ID `0x58`: "Button Switch"
    ButtonSwitch = 0x58,
    /// Usage ID `0x59`: "Button Type"
    ButtonType = 0x59,
    /// Usage ID `0x5A`: "Secondary Barrel Switch"
    SecondaryBarrelSwitch = 0x5A,
    /// Usage ID `0x5B`: "Transducer Serial Number"
    TransducerSerialNumber = 0x5B,
    /// Usage ID `0x5C`: "Wacom SerialHi"
    WacomSerialHi = 0x5C,
    /// Usage ID `0x5D`: "Preferred Color is Locked"
    PreferredColorisLocked = 0x5D,
    /// Usage ID `0x5E`: "Preferred Line Width"
    PreferredLineWidth = 0x5E,
    /// Usage ID `0x5F`: "Preferred Line Width is Locked"
    PreferredLineWidthisLocked = 0x5F,
    /// Usage ID `0x70`: "Preferred Line Style"
    PreferredLineStyle = 0x70,
    /// Usage ID `0x71`: "Preferred Line Style is Locked"
    PreferredLineStyleisLocked = 0x71,
    /// Usage ID `0x72`: "Ink"
    Ink = 0x72,
    /// Usage ID `0x73`: "Pencil"
    Pencil = 0x73,
    /// Usage ID `0x74`: "Highlighter"
    Highlighter = 0x74,
    /// Usage ID `0x75`: "Chisel Marker"
    ChiselMarker = 0x75,
    /// Usage ID `0x76`: "Brush"
    Brush = 0x76,
    /// Usage ID `0x77`: "Wacom ToolType"
    WacomToolType = 0x77,
    /// Usage ID `0x80`: "Digitizer Diagnostic"
    DigitizerDiagnostic = 0x80,
    /// Usage ID `0x81`: "Digitizer Error"
    DigitizerError = 0x81,
    /// Usage ID `0x82`: "Err Normal Status"
    ErrNormalStatus = 0x82,
    /// Usage ID `0x83`: "Err Transducers Exceeded"
    ErrTransducersExceeded = 0x83,
    /// Usage ID `0x84`: "Err Full Trans Features Unavail"
    ErrFullTransFeaturesUnavail = 0x84,
    /// Usage ID `0x85`: "Err Charge Low"
    ErrChargeLow = 0x85,
    /// Usage ID `0x130`: "X"
    X = 0x130,
    /// Usage ID `0x131`: "Y"
    Y = 0x131,
    /// Usage ID `0x132`: "Wacom Distance"
    WacomDistance = 0x132,
    /// Usage ID `0x136`: "Wacom TouchStrip"
    WacomTouchStrip = 0x136,
    /// Usage ID `0x137`: "Wacom TouchStrip2"
    WacomTouchStrip2 = 0x137,
    /// Usage ID `0x138`: "Wacom TouchRing"
    WacomTouchRing = 0x138,
    /// Usage ID `0x139`: "Wacom TouchRingStatus"
    WacomTouchRingStatus = 0x139,
    /// Usage ID `0x401`: "Wacom Accelerometer X"
    WacomAccelerometerX = 0x401,
    /// Usage ID `0x402`: "Wacom Accelerometer Y"
    WacomAccelerometerY = 0x402,
    /// Usage ID `0x403`: "Wacom Accelerometer Z"
    WacomAccelerometerZ = 0x403,
    /// Usage ID `0x404`: "Wacom Battery Charging"
    WacomBatteryCharging = 0x404,
    /// Usage ID `0x43B`: "Wacom Battery Level"
    WacomBatteryLevel = 0x43B,
    /// Usage ID `0x454`: "Wacom TouchOnOff"
    WacomTouchOnOff = 0x454,
    /// Usage ID `0x910`: "Wacom ExpressKey00"
    WacomExpressKey00 = 0x910,
    /// Usage ID `0x950`: "Wacom ExpressKeyCap00"
    WacomExpressKeyCap00 = 0x950,
    /// Usage ID `0x980`: "Wacom Mode Change"
    WacomModeChange = 0x980,
    /// Usage ID `0x981`: "Wacom Button Desktop Center"
    WacomButtonDesktopCenter = 0x981,
    /// Usage ID `0x982`: "Wacom Button On Screen Keyboard"
    WacomButtonOnScreenKeyboard = 0x982,
    /// Usage ID `0x983`: "Wacom Button Display Setting"
    WacomButtonDisplaySetting = 0x983,
    /// Usage ID `0x986`: "Wacom Button Touch On/Off"
    WacomButtonTouchOnOff = 0x986,
    /// Usage ID `0x990`: "Wacom Button Home"
    WacomButtonHome = 0x990,
    /// Usage ID `0x991`: "Wacom Button Up"
    WacomButtonUp = 0x991,
    /// Usage ID `0x992`: "Wacom Button Down"
    WacomButtonDown = 0x992,
    /// Usage ID `0x993`: "Wacom Button Left"
    WacomButtonLeft = 0x993,
    /// Usage ID `0x994`: "Wacom Button Right"
    WacomButtonRight = 0x994,
    /// Usage ID `0x995`: "Wacom Button Center"
    WacomButtonCenter = 0x995,
    /// Usage ID `0xD03`: "Wacom FingerWheel"
    WacomFingerWheel = 0xD03,
    /// Usage ID `0xD30`: "Wacom Offset Left"
    WacomOffsetLeft = 0xD30,
    /// Usage ID `0xD31`: "Wacom Offset Top"
    WacomOffsetTop = 0xD31,
    /// Usage ID `0xD32`: "Wacom Offset Right"
    WacomOffsetRight = 0xD32,
    /// Usage ID `0xD33`: "Wacom Offset Bottom"
    WacomOffsetBottom = 0xD33,
    /// Usage ID `0x1002`: "Wacom DataMode"
    WacomDataMode = 0x1002,
    /// Usage ID `0x1013`: "Wacom Digitizer Info"
    WacomDigitizerInfo = 0x1013,
}

impl Wacom {
    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Wacom::WacomDigitizer => "Wacom Digitizer",
            Wacom::WacomPen => "Wacom Pen",
            Wacom::LightPen => "Light Pen",
            Wacom::TouchScreen => "Touch Screen",
            Wacom::TouchPad => "Touch Pad",
            Wacom::WhiteBoard => "White Board",
            Wacom::CoordinateMeasuringMachine => "Coordinate Measuring Machine",
            Wacom::ThreeDDigitizer => "3-D Digitizer",
            Wacom::StereoPlotter => "Stereo Plotter",
            Wacom::ArticulatedArm => "Articulated Arm",
            Wacom::Armature => "Armature",
            Wacom::MultiplePointDigitizer => "Multiple Point Digitizer",
            Wacom::FreeSpaceWand => "Free Space Wand",
            Wacom::DeviceConfiguration => "Device Configuration",
            Wacom::Stylus => "Stylus",
            Wacom::Puck => "Puck",
            Wacom::Finger => "Finger",
            Wacom::DeviceSettings => "Device Settings",
            Wacom::TipPressure => "Tip Pressure",
            Wacom::BarrelPressure => "Barrel Pressure",
            Wacom::InRange => "In Range",
            Wacom::Touch => "Touch",
            Wacom::Untouch => "Untouch",
            Wacom::Tap => "Tap",
            Wacom::WacomSense => "Wacom Sense",
            Wacom::DataValid => "Data Valid",
            Wacom::TransducerIndex => "Transducer Index",
            Wacom::WacomDigitizerFnKeys => "Wacom DigitizerFnKeys",
            Wacom::ProgramChangeKeys => "Program Change Keys",
            Wacom::BatteryStrength => "Battery Strength",
            Wacom::Invert => "Invert",
            Wacom::XTilt => "X Tilt",
            Wacom::YTilt => "Y Tilt",
            Wacom::Azimuth => "Azimuth",
            Wacom::Altitude => "Altitude",
            Wacom::Twist => "Twist",
            Wacom::TipSwitch => "Tip Switch",
            Wacom::SecondaryTipSwitch => "Secondary Tip Switch",
            Wacom::BarrelSwitch => "Barrel Switch",
            Wacom::Eraser => "Eraser",
            Wacom::TabletPick => "Tablet Pick",
            Wacom::Confidence => "Confidence",
            Wacom::Width => "Width",
            Wacom::Height => "Height",
            Wacom::ContactId => "Contact Id",
            Wacom::Inputmode => "Inputmode",
            Wacom::DeviceIndex => "Device Index",
            Wacom::ContactCount => "Contact Count",
            Wacom::ContactMax => "Contact Max",
            Wacom::ScanTime => "Scan Time",
            Wacom::SurfaceSwitch => "Surface Switch",
            Wacom::ButtonSwitch => "Button Switch",
            Wacom::ButtonType => "Button Type",
            Wacom::SecondaryBarrelSwitch => "Secondary Barrel Switch",
            Wacom::TransducerSerialNumber => "Transducer Serial Number",
            Wacom::WacomSerialHi => "Wacom SerialHi",
            Wacom::PreferredColorisLocked => "Preferred Color is Locked",
            Wacom::PreferredLineWidth => "Preferred Line Width",
            Wacom::PreferredLineWidthisLocked => "Preferred Line Width is Locked",
            Wacom::PreferredLineStyle => "Preferred Line Style",
            Wacom::PreferredLineStyleisLocked => "Preferred Line Style is Locked",
            Wacom::Ink => "Ink",
            Wacom::Pencil => "Pencil",
            Wacom::Highlighter => "Highlighter",
            Wacom::ChiselMarker => "Chisel Marker",
            Wacom::Brush => "Brush",
            Wacom::WacomToolType => "Wacom ToolType",
            Wacom::DigitizerDiagnostic => "Digitizer Diagnostic",
            Wacom::DigitizerError => "Digitizer Error",
            Wacom::ErrNormalStatus => "Err Normal Status",
            Wacom::ErrTransducersExceeded => "Err Transducers Exceeded",
            Wacom::ErrFullTransFeaturesUnavail => "Err Full Trans Features Unavail",
            Wacom::ErrChargeLow => "Err Charge Low",
            Wacom::X => "X",
            Wacom::Y => "Y",
            Wacom::WacomDistance => "Wacom Distance",
            Wacom::WacomTouchStrip => "Wacom TouchStrip",
            Wacom::WacomTouchStrip2 => "Wacom TouchStrip2",
            Wacom::WacomTouchRing => "Wacom TouchRing",
            Wacom::WacomTouchRingStatus => "Wacom TouchRingStatus",
            Wacom::WacomAccelerometerX => "Wacom Accelerometer X",
            Wacom::WacomAccelerometerY => "Wacom Accelerometer Y",
            Wacom::WacomAccelerometerZ => "Wacom Accelerometer Z",
            Wacom::WacomBatteryCharging => "Wacom Battery Charging",
            Wacom::WacomBatteryLevel => "Wacom Battery Level",
            Wacom::WacomTouchOnOff => "Wacom TouchOnOff",
            Wacom::WacomExpressKey00 => "Wacom ExpressKey00",
            Wacom::WacomExpressKeyCap00 => "Wacom ExpressKeyCap00",
            Wacom::WacomModeChange => "Wacom Mode Change",
            Wacom::WacomButtonDesktopCenter => "Wacom Button Desktop Center",
            Wacom::WacomButtonOnScreenKeyboard => "Wacom Button On Screen Keyboard",
            Wacom::WacomButtonDisplaySetting => "Wacom Button Display Setting",
            Wacom::WacomButtonTouchOnOff => "Wacom Button Touch On/Off",
            Wacom::WacomButtonHome => "Wacom Button Home",
            Wacom::WacomButtonUp => "Wacom Button Up",
            Wacom::WacomButtonDown => "Wacom Button Down",
            Wacom::WacomButtonLeft => "Wacom Button Left",
            Wacom::WacomButtonRight => "Wacom Button Right",
            Wacom::WacomButtonCenter => "Wacom Button Center",
            Wacom::WacomFingerWheel => "Wacom FingerWheel",
            Wacom::WacomOffsetLeft => "Wacom Offset Left",
            Wacom::WacomOffsetTop => "Wacom Offset Top",
            Wacom::WacomOffsetRight => "Wacom Offset Right",
            Wacom::WacomOffsetBottom => "Wacom Offset Bottom",
            Wacom::WacomDataMode => "Wacom DataMode",
            Wacom::WacomDigitizerInfo => "Wacom Digitizer Info",
        }
        .into()
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Wacom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AsUsage for Wacom {
    /// Returns the 32 bit Usage value of this Usage
    fn usage_value(&self) -> u32 {
        u32::from(self)
    }

    /// Returns the 16 bit Usage ID value of this Usage
    fn usage_id_value(&self) -> u16 {
        u16::from(self)
    }

    /// Returns this usage as [Usage::Wacom(self)](Usage::Wacom)
    /// This is a convenience function to avoid having
    /// to implement `From` for every used type in the caller.
    ///
    /// ```
    /// # use hut::*;
    /// let gd_x = GenericDesktop::X;
    /// let usage = Usage::from(GenericDesktop::X);
    /// assert!(matches!(gd_x.usage(), usage));
    /// ```
    fn usage(&self) -> Usage {
        Usage::from(self)
    }
}

impl AsUsagePage for Wacom {
    /// Returns the 16 bit value of this UsagePage
    ///
    /// This value is `0xFF0D` for [Wacom]
    fn usage_page_value(&self) -> u16 {
        let up = UsagePage::from(self);
        u16::from(up)
    }

    /// Returns [UsagePage::Wacom]]
    fn usage_page(&self) -> UsagePage {
        UsagePage::from(self)
    }
}

impl From<&Wacom> for u16 {
    fn from(wacom: &Wacom) -> u16 {
        *wacom as u16
    }
}

impl From<Wacom> for u16 {
    /// Returns the 16bit value of this usage. This is identical
    /// to [Wacom::usage_page_value()].
    fn from(wacom: Wacom) -> u16 {
        u16::from(&wacom)
    }
}

impl From<&Wacom> for u32 {
    /// Returns the 32 bit value of this usage. This is identical
    /// to [Wacom::usage_value()].
    fn from(wacom: &Wacom) -> u32 {
        let up = UsagePage::from(wacom);
        let up = (u16::from(&up) as u32) << 16;
        let id = u16::from(wacom) as u32;
        up | id
    }
}

impl From<&Wacom> for UsagePage {
    /// Always returns [UsagePage::Wacom] and is
    /// identical to [Wacom::usage_page()].
    fn from(_: &Wacom) -> UsagePage {
        UsagePage::Wacom
    }
}

impl From<Wacom> for UsagePage {
    /// Always returns [UsagePage::Wacom] and is
    /// identical to [Wacom::usage_page()].
    fn from(_: Wacom) -> UsagePage {
        UsagePage::Wacom
    }
}

impl From<&Wacom> for Usage {
    fn from(wacom: &Wacom) -> Usage {
        Usage::try_from(u32::from(wacom)).unwrap()
    }
}

impl From<Wacom> for Usage {
    fn from(wacom: Wacom) -> Usage {
        Usage::from(&wacom)
    }
}

impl TryFrom<u16> for Wacom {
    type Error = HutError;

    fn try_from(usage_id: u16) -> Result<Wacom> {
        match usage_id {
            1 => Ok(Wacom::WacomDigitizer),
            2 => Ok(Wacom::WacomPen),
            3 => Ok(Wacom::LightPen),
            4 => Ok(Wacom::TouchScreen),
            5 => Ok(Wacom::TouchPad),
            6 => Ok(Wacom::WhiteBoard),
            7 => Ok(Wacom::CoordinateMeasuringMachine),
            8 => Ok(Wacom::ThreeDDigitizer),
            9 => Ok(Wacom::StereoPlotter),
            10 => Ok(Wacom::ArticulatedArm),
            11 => Ok(Wacom::Armature),
            12 => Ok(Wacom::MultiplePointDigitizer),
            13 => Ok(Wacom::FreeSpaceWand),
            14 => Ok(Wacom::DeviceConfiguration),
            32 => Ok(Wacom::Stylus),
            33 => Ok(Wacom::Puck),
            34 => Ok(Wacom::Finger),
            35 => Ok(Wacom::DeviceSettings),
            48 => Ok(Wacom::TipPressure),
            49 => Ok(Wacom::BarrelPressure),
            50 => Ok(Wacom::InRange),
            51 => Ok(Wacom::Touch),
            52 => Ok(Wacom::Untouch),
            53 => Ok(Wacom::Tap),
            54 => Ok(Wacom::WacomSense),
            55 => Ok(Wacom::DataValid),
            56 => Ok(Wacom::TransducerIndex),
            57 => Ok(Wacom::WacomDigitizerFnKeys),
            58 => Ok(Wacom::ProgramChangeKeys),
            59 => Ok(Wacom::BatteryStrength),
            60 => Ok(Wacom::Invert),
            61 => Ok(Wacom::XTilt),
            62 => Ok(Wacom::YTilt),
            63 => Ok(Wacom::Azimuth),
            64 => Ok(Wacom::Altitude),
            65 => Ok(Wacom::Twist),
            66 => Ok(Wacom::TipSwitch),
            67 => Ok(Wacom::SecondaryTipSwitch),
            68 => Ok(Wacom::BarrelSwitch),
            69 => Ok(Wacom::Eraser),
            70 => Ok(Wacom::TabletPick),
            71 => Ok(Wacom::Confidence),
            72 => Ok(Wacom::Width),
            73 => Ok(Wacom::Height),
            81 => Ok(Wacom::ContactId),
            82 => Ok(Wacom::Inputmode),
            83 => Ok(Wacom::DeviceIndex),
            84 => Ok(Wacom::ContactCount),
            85 => Ok(Wacom::ContactMax),
            86 => Ok(Wacom::ScanTime),
            87 => Ok(Wacom::SurfaceSwitch),
            88 => Ok(Wacom::ButtonSwitch),
            89 => Ok(Wacom::ButtonType),
            90 => Ok(Wacom::SecondaryBarrelSwitch),
            91 => Ok(Wacom::TransducerSerialNumber),
            92 => Ok(Wacom::WacomSerialHi),
            93 => Ok(Wacom::PreferredColorisLocked),
            94 => Ok(Wacom::PreferredLineWidth),
            95 => Ok(Wacom::PreferredLineWidthisLocked),
            112 => Ok(Wacom::PreferredLineStyle),
            113 => Ok(Wacom::PreferredLineStyleisLocked),
            114 => Ok(Wacom::Ink),
            115 => Ok(Wacom::Pencil),
            116 => Ok(Wacom::Highlighter),
            117 => Ok(Wacom::ChiselMarker),
            118 => Ok(Wacom::Brush),
            119 => Ok(Wacom::WacomToolType),
            128 => Ok(Wacom::DigitizerDiagnostic),
            129 => Ok(Wacom::DigitizerError),
            130 => Ok(Wacom::ErrNormalStatus),
            131 => Ok(Wacom::ErrTransducersExceeded),
            132 => Ok(Wacom::ErrFullTransFeaturesUnavail),
            133 => Ok(Wacom::ErrChargeLow),
            304 => Ok(Wacom::X),
            305 => Ok(Wacom::Y),
            306 => Ok(Wacom::WacomDistance),
            310 => Ok(Wacom::WacomTouchStrip),
            311 => Ok(Wacom::WacomTouchStrip2),
            312 => Ok(Wacom::WacomTouchRing),
            313 => Ok(Wacom::WacomTouchRingStatus),
            1025 => Ok(Wacom::WacomAccelerometerX),
            1026 => Ok(Wacom::WacomAccelerometerY),
            1027 => Ok(Wacom::WacomAccelerometerZ),
            1028 => Ok(Wacom::WacomBatteryCharging),
            1083 => Ok(Wacom::WacomBatteryLevel),
            1108 => Ok(Wacom::WacomTouchOnOff),
            2320 => Ok(Wacom::WacomExpressKey00),
            2384 => Ok(Wacom::WacomExpressKeyCap00),
            2432 => Ok(Wacom::WacomModeChange),
            2433 => Ok(Wacom::WacomButtonDesktopCenter),
            2434 => Ok(Wacom::WacomButtonOnScreenKeyboard),
            2435 => Ok(Wacom::WacomButtonDisplaySetting),
            2438 => Ok(Wacom::WacomButtonTouchOnOff),
            2448 => Ok(Wacom::WacomButtonHome),
            2449 => Ok(Wacom::WacomButtonUp),
            2450 => Ok(Wacom::WacomButtonDown),
            2451 => Ok(Wacom::WacomButtonLeft),
            2452 => Ok(Wacom::WacomButtonRight),
            2453 => Ok(Wacom::WacomButtonCenter),
            3331 => Ok(Wacom::WacomFingerWheel),
            3376 => Ok(Wacom::WacomOffsetLeft),
            3377 => Ok(Wacom::WacomOffsetTop),
            3378 => Ok(Wacom::WacomOffsetRight),
            3379 => Ok(Wacom::WacomOffsetBottom),
            4098 => Ok(Wacom::WacomDataMode),
            4115 => Ok(Wacom::WacomDigitizerInfo),
            n => Err(HutError::UnknownUsageId { usage_id: n }),
        }
    }
}

impl BitOr<u16> for Wacom {
    type Output = Usage;

    /// A convenience function to combine a Usage Page with
    /// a value.
    ///
    /// This function panics if the Usage ID value results in
    /// an unknown Usage. Where error checking is required,
    /// use [UsagePage::to_usage_from_value].
    fn bitor(self, usage: u16) -> Usage {
        let up = u16::from(self) as u32;
        let u = usage as u32;
        Usage::try_from(up | u).expect("Invalid Usage ID for this Usage Page")
    }
}

/// *Reserved Usage Pages*
///
/// This Usage Page has no named Usage IDs, any Usages in this Usage Page are
/// reserved implementation. In a future version of the HUT standard a reserved
/// Usage Page may become a defined Usage Page.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum ReservedUsagePage {
    Undefined,
    ReservedUsage { usage_id: u16 },
}

impl ReservedUsagePage {
    #[cfg(feature = "std")]
    fn name(&self) -> String {
        match self {
            ReservedUsagePage::Undefined => "Reserved Usage Undefined".to_string(),
            ReservedUsagePage::ReservedUsage { usage_id } => {
                format!("Reserved Usage 0x{usage_id:02x}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for ReservedUsagePage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<&ReservedUsagePage> for u16 {
    fn from(v: &ReservedUsagePage) -> u16 {
        match v {
            ReservedUsagePage::Undefined => 0x00,
            ReservedUsagePage::ReservedUsage { usage_id } => *usage_id,
        }
    }
}

/// *Usage Page `0xFF00` to `0xFFFF`: The Vendor Defined Pages*
///
/// This Usage Page has no named Usage IDs, any Usages in this Usage Page are
/// private to a vendor implementation.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum VendorDefinedPage {
    Undefined,
    VendorUsage { usage_id: u16 },
}

impl VendorDefinedPage {
    #[cfg(feature = "std")]
    fn name(&self) -> String {
        match self {
            VendorDefinedPage::Undefined => "Vendor Usage Undefined".to_string(),
            VendorDefinedPage::VendorUsage { usage_id } => {
                format!("Vendor Usage 0x{usage_id:02x}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for VendorDefinedPage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<&VendorDefinedPage> for u16 {
    fn from(v: &VendorDefinedPage) -> u16 {
        match v {
            VendorDefinedPage::Undefined => 0x00,
            VendorDefinedPage::VendorUsage { usage_id } => *usage_id,
        }
    }
}

impl From<&Usage> for UsagePage {
    #[allow(clippy::unneeded_struct_pattern)]
    fn from(usage: &Usage) -> UsagePage {
        match usage {
            Usage::GenericDesktop { .. } => UsagePage::GenericDesktop,
            Usage::SimulationControls { .. } => UsagePage::SimulationControls,
            Usage::VRControls { .. } => UsagePage::VRControls,
            Usage::SportControls { .. } => UsagePage::SportControls,
            Usage::GameControls { .. } => UsagePage::GameControls,
            Usage::GenericDeviceControls { .. } => UsagePage::GenericDeviceControls,
            Usage::KeyboardKeypad { .. } => UsagePage::KeyboardKeypad,
            Usage::LED { .. } => UsagePage::LED,
            Usage::Button { .. } => UsagePage::Button,
            Usage::Ordinal { .. } => UsagePage::Ordinal,
            Usage::TelephonyDevice { .. } => UsagePage::TelephonyDevice,
            Usage::Consumer { .. } => UsagePage::Consumer,
            Usage::Digitizers { .. } => UsagePage::Digitizers,
            Usage::Haptics { .. } => UsagePage::Haptics,
            Usage::PhysicalInputDevice { .. } => UsagePage::PhysicalInputDevice,
            Usage::Unicode { .. } => UsagePage::Unicode,
            Usage::SoC { .. } => UsagePage::SoC,
            Usage::EyeandHeadTrackers { .. } => UsagePage::EyeandHeadTrackers,
            Usage::AuxiliaryDisplay { .. } => UsagePage::AuxiliaryDisplay,
            Usage::Sensors { .. } => UsagePage::Sensors,
            Usage::MedicalInstrument { .. } => UsagePage::MedicalInstrument,
            Usage::BrailleDisplay { .. } => UsagePage::BrailleDisplay,
            Usage::LightingAndIllumination { .. } => UsagePage::LightingAndIllumination,
            Usage::Monitor { .. } => UsagePage::Monitor,
            Usage::MonitorEnumerated { .. } => UsagePage::MonitorEnumerated,
            Usage::VESAVirtualControls { .. } => UsagePage::VESAVirtualControls,
            Usage::Power { .. } => UsagePage::Power,
            Usage::BatterySystem { .. } => UsagePage::BatterySystem,
            Usage::BarcodeScanner { .. } => UsagePage::BarcodeScanner,
            Usage::Scales { .. } => UsagePage::Scales,
            Usage::MagneticStripeReader { .. } => UsagePage::MagneticStripeReader,
            Usage::CameraControl { .. } => UsagePage::CameraControl,
            Usage::Arcade { .. } => UsagePage::Arcade,
            Usage::FIDOAlliance { .. } => UsagePage::FIDOAlliance,
            Usage::Wacom { .. } => UsagePage::Wacom,
            Usage::ReservedUsagePage { reserved_page, .. } => {
                UsagePage::ReservedUsagePage(*reserved_page)
            }
            Usage::VendorDefinedPage { vendor_page, .. } => {
                UsagePage::VendorDefinedPage(*vendor_page)
            }
        }
    }
}

impl From<&UsagePage> for u16 {
    /// Returns the UsagePage as 16-bit value. This is equivalent to the
    /// upper 16 bits of a full 32-bit Usage value shifted down.
    #[allow(clippy::unneeded_struct_pattern)]
    fn from(usage_page: &UsagePage) -> u16 {
        match usage_page {
            UsagePage::GenericDesktop { .. } => 1,
            UsagePage::SimulationControls { .. } => 2,
            UsagePage::VRControls { .. } => 3,
            UsagePage::SportControls { .. } => 4,
            UsagePage::GameControls { .. } => 5,
            UsagePage::GenericDeviceControls { .. } => 6,
            UsagePage::KeyboardKeypad { .. } => 7,
            UsagePage::LED { .. } => 8,
            UsagePage::Button { .. } => 9,
            UsagePage::Ordinal { .. } => 10,
            UsagePage::TelephonyDevice { .. } => 11,
            UsagePage::Consumer { .. } => 12,
            UsagePage::Digitizers { .. } => 13,
            UsagePage::Haptics { .. } => 14,
            UsagePage::PhysicalInputDevice { .. } => 15,
            UsagePage::Unicode { .. } => 16,
            UsagePage::SoC { .. } => 17,
            UsagePage::EyeandHeadTrackers { .. } => 18,
            UsagePage::AuxiliaryDisplay { .. } => 20,
            UsagePage::Sensors { .. } => 32,
            UsagePage::MedicalInstrument { .. } => 64,
            UsagePage::BrailleDisplay { .. } => 65,
            UsagePage::LightingAndIllumination { .. } => 89,
            UsagePage::Monitor { .. } => 128,
            UsagePage::MonitorEnumerated { .. } => 129,
            UsagePage::VESAVirtualControls { .. } => 130,
            UsagePage::Power { .. } => 132,
            UsagePage::BatterySystem { .. } => 133,
            UsagePage::BarcodeScanner { .. } => 140,
            UsagePage::Scales { .. } => 141,
            UsagePage::MagneticStripeReader { .. } => 142,
            UsagePage::CameraControl { .. } => 144,
            UsagePage::Arcade { .. } => 145,
            UsagePage::FIDOAlliance { .. } => 61904,
            UsagePage::Wacom { .. } => 65293,
            UsagePage::ReservedUsagePage(reserved_page) => u16::from(reserved_page),
            UsagePage::VendorDefinedPage(vendor_page) => u16::from(vendor_page),
        }
    }
}

impl From<UsagePage> for u16 {
    fn from(usage_page: UsagePage) -> u16 {
        u16::from(&usage_page)
    }
}

impl TryFrom<u16> for UsagePage {
    type Error = HutError;

    fn try_from(usage_page: u16) -> Result<UsagePage> {
        match usage_page {
            1 => Ok(UsagePage::GenericDesktop),
            2 => Ok(UsagePage::SimulationControls),
            3 => Ok(UsagePage::VRControls),
            4 => Ok(UsagePage::SportControls),
            5 => Ok(UsagePage::GameControls),
            6 => Ok(UsagePage::GenericDeviceControls),
            7 => Ok(UsagePage::KeyboardKeypad),
            8 => Ok(UsagePage::LED),
            9 => Ok(UsagePage::Button),
            10 => Ok(UsagePage::Ordinal),
            11 => Ok(UsagePage::TelephonyDevice),
            12 => Ok(UsagePage::Consumer),
            13 => Ok(UsagePage::Digitizers),
            14 => Ok(UsagePage::Haptics),
            15 => Ok(UsagePage::PhysicalInputDevice),
            16 => Ok(UsagePage::Unicode),
            17 => Ok(UsagePage::SoC),
            18 => Ok(UsagePage::EyeandHeadTrackers),
            20 => Ok(UsagePage::AuxiliaryDisplay),
            32 => Ok(UsagePage::Sensors),
            64 => Ok(UsagePage::MedicalInstrument),
            65 => Ok(UsagePage::BrailleDisplay),
            89 => Ok(UsagePage::LightingAndIllumination),
            128 => Ok(UsagePage::Monitor),
            129 => Ok(UsagePage::MonitorEnumerated),
            130 => Ok(UsagePage::VESAVirtualControls),
            132 => Ok(UsagePage::Power),
            133 => Ok(UsagePage::BatterySystem),
            140 => Ok(UsagePage::BarcodeScanner),
            141 => Ok(UsagePage::Scales),
            142 => Ok(UsagePage::MagneticStripeReader),
            144 => Ok(UsagePage::CameraControl),
            145 => Ok(UsagePage::Arcade),
            61904 => Ok(UsagePage::FIDOAlliance),
            65293 => Ok(UsagePage::Wacom),
            page @ 0xff00..=0xffff => Ok(UsagePage::VendorDefinedPage(VendorPage(page))),
            n => match ReservedPage::try_from(n) {
                Ok(r) => Ok(UsagePage::ReservedUsagePage(r)),
                Err(_) => Err(HutError::UnknownUsagePage { usage_page: n }),
            },
        }
    }
}

impl TryFrom<u32> for UsagePage {
    type Error = HutError;

    fn try_from(usage_page: u32) -> Result<UsagePage> {
        UsagePage::try_from((usage_page >> 16) as u16)
    }
}

#[cfg(feature = "std")]
impl fmt::Display for UsagePage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// An enum wrapping all known Usages in the HUT.
/// ```
/// # use hut::*;
/// let u1 = Usage::GenericDesktop(GenericDesktop::Mouse);
/// let u2 = Usage::new_from_page_and_id(0x01, 0x02).unwrap();
/// let u3 = Usage::from(GenericDesktop::Mouse);
/// assert_eq!(u1, u2);
/// assert_eq!(u1, u3);
///
/// assert_eq!(0x1, u1.usage_page_value());
/// assert_eq!(0x2, u1.usage_id_value());
/// assert_eq!((0x1 << 16) | 0x2, u1.usage_value());
/// ```
/// Note: this enum is generated from the HUT documents.
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[non_exhaustive]
pub enum Usage {
    /// "Generic Desktop"
    GenericDesktop(GenericDesktop),
    /// "Simulation Controls"
    SimulationControls(SimulationControls),
    /// "VR Controls"
    VRControls(VRControls),
    /// "Sport Controls"
    SportControls(SportControls),
    /// "Game Controls"
    GameControls(GameControls),
    /// "Generic Device Controls"
    GenericDeviceControls(GenericDeviceControls),
    /// "Keyboard/Keypad"
    KeyboardKeypad(KeyboardKeypad),
    /// "LED"
    LED(LED),
    /// "Button"
    Button(Button),
    /// "Ordinal"
    Ordinal(Ordinal),
    /// "Telephony Device"
    TelephonyDevice(TelephonyDevice),
    /// "Consumer"
    Consumer(Consumer),
    /// "Digitizers"
    Digitizers(Digitizers),
    /// "Haptics"
    Haptics(Haptics),
    /// "Physical Input Device"
    PhysicalInputDevice(PhysicalInputDevice),
    /// "Unicode"
    Unicode(Unicode),
    /// "SoC"
    SoC(SoC),
    /// "Eye and Head Trackers"
    EyeandHeadTrackers(EyeandHeadTrackers),
    /// "Auxiliary Display"
    AuxiliaryDisplay(AuxiliaryDisplay),
    /// "Sensors"
    Sensors(Sensors),
    /// "Medical Instrument"
    MedicalInstrument(MedicalInstrument),
    /// "Braille Display"
    BrailleDisplay(BrailleDisplay),
    /// "Lighting And Illumination"
    LightingAndIllumination(LightingAndIllumination),
    /// "Monitor"
    Monitor(Monitor),
    /// "Monitor Enumerated"
    MonitorEnumerated(MonitorEnumerated),
    /// "VESA Virtual Controls"
    VESAVirtualControls(VESAVirtualControls),
    /// "Power"
    Power(Power),
    /// "Battery System"
    BatterySystem(BatterySystem),
    /// "Barcode Scanner"
    BarcodeScanner(BarcodeScanner),
    /// "Scales"
    Scales(Scales),
    /// "Magnetic Stripe Reader"
    MagneticStripeReader(MagneticStripeReader),
    /// "Camera Control"
    CameraControl(CameraControl),
    /// "Arcade"
    Arcade(Arcade),
    /// "FIDO Alliance"
    FIDOAlliance(FIDOAlliance),
    /// "Wacom"
    Wacom(Wacom),
    ReservedUsagePage {
        reserved_page: ReservedPage,
        usage: ReservedUsagePage,
    },
    VendorDefinedPage {
        vendor_page: VendorPage,
        usage: VendorDefinedPage,
    },
}

impl Usage {
    #[cfg(feature = "std")]
    pub fn new_from_page_and_id(usage_page: u16, usage_id: u16) -> Result<Usage> {
        Usage::try_from(((usage_page as u32) << 16) | usage_id as u32)
    }

    #[cfg(feature = "std")]
    pub fn name(&self) -> String {
        match self {
            Usage::GenericDesktop(usage) => usage.name(),
            Usage::SimulationControls(usage) => usage.name(),
            Usage::VRControls(usage) => usage.name(),
            Usage::SportControls(usage) => usage.name(),
            Usage::GameControls(usage) => usage.name(),
            Usage::GenericDeviceControls(usage) => usage.name(),
            Usage::KeyboardKeypad(usage) => usage.name(),
            Usage::LED(usage) => usage.name(),
            Usage::Button(usage) => usage.name(),
            Usage::Ordinal(usage) => usage.name(),
            Usage::TelephonyDevice(usage) => usage.name(),
            Usage::Consumer(usage) => usage.name(),
            Usage::Digitizers(usage) => usage.name(),
            Usage::Haptics(usage) => usage.name(),
            Usage::PhysicalInputDevice(usage) => usage.name(),
            Usage::Unicode(usage) => usage.name(),
            Usage::SoC(usage) => usage.name(),
            Usage::EyeandHeadTrackers(usage) => usage.name(),
            Usage::AuxiliaryDisplay(usage) => usage.name(),
            Usage::Sensors(usage) => usage.name(),
            Usage::MedicalInstrument(usage) => usage.name(),
            Usage::BrailleDisplay(usage) => usage.name(),
            Usage::LightingAndIllumination(usage) => usage.name(),
            Usage::Monitor(usage) => usage.name(),
            Usage::MonitorEnumerated(usage) => usage.name(),
            Usage::VESAVirtualControls(usage) => usage.name(),
            Usage::Power(usage) => usage.name(),
            Usage::BatterySystem(usage) => usage.name(),
            Usage::BarcodeScanner(usage) => usage.name(),
            Usage::Scales(usage) => usage.name(),
            Usage::MagneticStripeReader(usage) => usage.name(),
            Usage::CameraControl(usage) => usage.name(),
            Usage::Arcade(usage) => usage.name(),
            Usage::FIDOAlliance(usage) => usage.name(),
            Usage::Wacom(usage) => usage.name(),
            Usage::ReservedUsagePage { usage, .. } => usage.name(),
            Usage::VendorDefinedPage { usage, .. } => usage.name(),
        }
    }
}

impl AsUsage for Usage {
    /// Returns the 32 bit Usage value for this usage.
    fn usage_value(&self) -> u32 {
        self.into()
    }

    /// Returns the 16-bit Usage ID value for this usage.
    fn usage_id_value(&self) -> u16 {
        self.into()
    }

    /// Returns [Self]
    fn usage(&self) -> Usage {
        Usage::try_from(self.usage_value()).unwrap()
    }
}

impl PartialEq for Usage {
    fn eq(&self, other: &Self) -> bool {
        u32::from(self) == u32::from(other)
    }
}

impl AsUsagePage for Usage {
    fn usage_page_value(&self) -> u16 {
        UsagePage::from(self).into()
    }

    fn usage_page(&self) -> UsagePage {
        match self {
            Usage::GenericDesktop(_) => UsagePage::GenericDesktop,
            Usage::SimulationControls(_) => UsagePage::SimulationControls,
            Usage::VRControls(_) => UsagePage::VRControls,
            Usage::SportControls(_) => UsagePage::SportControls,
            Usage::GameControls(_) => UsagePage::GameControls,
            Usage::GenericDeviceControls(_) => UsagePage::GenericDeviceControls,
            Usage::KeyboardKeypad(_) => UsagePage::KeyboardKeypad,
            Usage::LED(_) => UsagePage::LED,
            Usage::Button(_) => UsagePage::Button,
            Usage::Ordinal(_) => UsagePage::Ordinal,
            Usage::TelephonyDevice(_) => UsagePage::TelephonyDevice,
            Usage::Consumer(_) => UsagePage::Consumer,
            Usage::Digitizers(_) => UsagePage::Digitizers,
            Usage::Haptics(_) => UsagePage::Haptics,
            Usage::PhysicalInputDevice(_) => UsagePage::PhysicalInputDevice,
            Usage::Unicode(_) => UsagePage::Unicode,
            Usage::SoC(_) => UsagePage::SoC,
            Usage::EyeandHeadTrackers(_) => UsagePage::EyeandHeadTrackers,
            Usage::AuxiliaryDisplay(_) => UsagePage::AuxiliaryDisplay,
            Usage::Sensors(_) => UsagePage::Sensors,
            Usage::MedicalInstrument(_) => UsagePage::MedicalInstrument,
            Usage::BrailleDisplay(_) => UsagePage::BrailleDisplay,
            Usage::LightingAndIllumination(_) => UsagePage::LightingAndIllumination,
            Usage::Monitor(_) => UsagePage::Monitor,
            Usage::MonitorEnumerated(_) => UsagePage::MonitorEnumerated,
            Usage::VESAVirtualControls(_) => UsagePage::VESAVirtualControls,
            Usage::Power(_) => UsagePage::Power,
            Usage::BatterySystem(_) => UsagePage::BatterySystem,
            Usage::BarcodeScanner(_) => UsagePage::BarcodeScanner,
            Usage::Scales(_) => UsagePage::Scales,
            Usage::MagneticStripeReader(_) => UsagePage::MagneticStripeReader,
            Usage::CameraControl(_) => UsagePage::CameraControl,
            Usage::Arcade(_) => UsagePage::Arcade,
            Usage::FIDOAlliance(_) => UsagePage::FIDOAlliance,
            Usage::Wacom(_) => UsagePage::Wacom,
            Usage::ReservedUsagePage { reserved_page, .. } => {
                UsagePage::ReservedUsagePage(*reserved_page)
            }
            Usage::VendorDefinedPage { vendor_page, .. } => {
                UsagePage::VendorDefinedPage(*vendor_page)
            }
        }
    }
}

#[cfg(feature = "std")]
impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<&Usage> for u16 {
    fn from(usage: &Usage) -> u16 {
        let u: u32 = u32::from(usage);
        (u & 0xFFFF) as u16
    }
}

impl From<Usage> for u16 {
    fn from(usage: Usage) -> u16 {
        u16::from(&usage)
    }
}

impl From<&Usage> for u32 {
    fn from(usage: &Usage) -> u32 {
        match usage {
            Usage::GenericDesktop(usage) => (1 << 16) | u16::from(usage) as u32,
            Usage::SimulationControls(usage) => (2 << 16) | u16::from(usage) as u32,
            Usage::VRControls(usage) => (3 << 16) | u16::from(usage) as u32,
            Usage::SportControls(usage) => (4 << 16) | u16::from(usage) as u32,
            Usage::GameControls(usage) => (5 << 16) | u16::from(usage) as u32,
            Usage::GenericDeviceControls(usage) => (6 << 16) | u16::from(usage) as u32,
            Usage::KeyboardKeypad(usage) => (7 << 16) | u16::from(usage) as u32,
            Usage::LED(usage) => (8 << 16) | u16::from(usage) as u32,
            Usage::Button(usage) => (9 << 16) | u16::from(usage) as u32,
            Usage::Ordinal(usage) => (10 << 16) | u16::from(usage) as u32,
            Usage::TelephonyDevice(usage) => (11 << 16) | u16::from(usage) as u32,
            Usage::Consumer(usage) => (12 << 16) | u16::from(usage) as u32,
            Usage::Digitizers(usage) => (13 << 16) | u16::from(usage) as u32,
            Usage::Haptics(usage) => (14 << 16) | u16::from(usage) as u32,
            Usage::PhysicalInputDevice(usage) => (15 << 16) | u16::from(usage) as u32,
            Usage::Unicode(usage) => (16 << 16) | u16::from(usage) as u32,
            Usage::SoC(usage) => (17 << 16) | u16::from(usage) as u32,
            Usage::EyeandHeadTrackers(usage) => (18 << 16) | u16::from(usage) as u32,
            Usage::AuxiliaryDisplay(usage) => (20 << 16) | u16::from(usage) as u32,
            Usage::Sensors(usage) => (32 << 16) | u16::from(usage) as u32,
            Usage::MedicalInstrument(usage) => (64 << 16) | u16::from(usage) as u32,
            Usage::BrailleDisplay(usage) => (65 << 16) | u16::from(usage) as u32,
            Usage::LightingAndIllumination(usage) => (89 << 16) | u16::from(usage) as u32,
            Usage::Monitor(usage) => (128 << 16) | u16::from(usage) as u32,
            Usage::MonitorEnumerated(usage) => (129 << 16) | u16::from(usage) as u32,
            Usage::VESAVirtualControls(usage) => (130 << 16) | u16::from(usage) as u32,
            Usage::Power(usage) => (132 << 16) | u16::from(usage) as u32,
            Usage::BatterySystem(usage) => (133 << 16) | u16::from(usage) as u32,
            Usage::BarcodeScanner(usage) => (140 << 16) | u16::from(usage) as u32,
            Usage::Scales(usage) => (141 << 16) | u16::from(usage) as u32,
            Usage::MagneticStripeReader(usage) => (142 << 16) | u16::from(usage) as u32,
            Usage::CameraControl(usage) => (144 << 16) | u16::from(usage) as u32,
            Usage::Arcade(usage) => (145 << 16) | u16::from(usage) as u32,
            Usage::FIDOAlliance(usage) => (61904 << 16) | u16::from(usage) as u32,
            Usage::Wacom(usage) => (65293 << 16) | u16::from(usage) as u32,
            Usage::ReservedUsagePage {
                reserved_page,
                usage,
            } => ((u16::from(reserved_page) as u32) << 16) | u16::from(usage) as u32,
            Usage::VendorDefinedPage { vendor_page, usage } => {
                ((u16::from(vendor_page) as u32) << 16) | u16::from(usage) as u32
            }
        }
    }
}

impl TryFrom<u32> for Usage {
    type Error = HutError;

    fn try_from(up: u32) -> Result<Usage> {
        match (up >> 16, up & 0xFFFF) {
            (1, n) => Ok(Usage::GenericDesktop(GenericDesktop::try_from(n as u16)?)),
            (2, n) => Ok(Usage::SimulationControls(SimulationControls::try_from(
                n as u16,
            )?)),
            (3, n) => Ok(Usage::VRControls(VRControls::try_from(n as u16)?)),
            (4, n) => Ok(Usage::SportControls(SportControls::try_from(n as u16)?)),
            (5, n) => Ok(Usage::GameControls(GameControls::try_from(n as u16)?)),
            (6, n) => Ok(Usage::GenericDeviceControls(
                GenericDeviceControls::try_from(n as u16)?,
            )),
            (7, n) => Ok(Usage::KeyboardKeypad(KeyboardKeypad::try_from(n as u16)?)),
            (8, n) => Ok(Usage::LED(LED::try_from(n as u16)?)),
            (9, n) => Ok(Usage::Button(Button::try_from(n as u16)?)),
            (10, n) => Ok(Usage::Ordinal(Ordinal::try_from(n as u16)?)),
            (11, n) => Ok(Usage::TelephonyDevice(TelephonyDevice::try_from(n as u16)?)),
            (12, n) => Ok(Usage::Consumer(Consumer::try_from(n as u16)?)),
            (13, n) => Ok(Usage::Digitizers(Digitizers::try_from(n as u16)?)),
            (14, n) => Ok(Usage::Haptics(Haptics::try_from(n as u16)?)),
            (15, n) => Ok(Usage::PhysicalInputDevice(PhysicalInputDevice::try_from(
                n as u16,
            )?)),
            (16, n) => Ok(Usage::Unicode(Unicode::try_from(n as u16)?)),
            (17, n) => Ok(Usage::SoC(SoC::try_from(n as u16)?)),
            (18, n) => Ok(Usage::EyeandHeadTrackers(EyeandHeadTrackers::try_from(
                n as u16,
            )?)),
            (20, n) => Ok(Usage::AuxiliaryDisplay(AuxiliaryDisplay::try_from(
                n as u16,
            )?)),
            (32, n) => Ok(Usage::Sensors(Sensors::try_from(n as u16)?)),
            (64, n) => Ok(Usage::MedicalInstrument(MedicalInstrument::try_from(
                n as u16,
            )?)),
            (65, n) => Ok(Usage::BrailleDisplay(BrailleDisplay::try_from(n as u16)?)),
            (89, n) => Ok(Usage::LightingAndIllumination(
                LightingAndIllumination::try_from(n as u16)?,
            )),
            (128, n) => Ok(Usage::Monitor(Monitor::try_from(n as u16)?)),
            (129, n) => Ok(Usage::MonitorEnumerated(MonitorEnumerated::try_from(
                n as u16,
            )?)),
            (130, n) => Ok(Usage::VESAVirtualControls(VESAVirtualControls::try_from(
                n as u16,
            )?)),
            (132, n) => Ok(Usage::Power(Power::try_from(n as u16)?)),
            (133, n) => Ok(Usage::BatterySystem(BatterySystem::try_from(n as u16)?)),
            (140, n) => Ok(Usage::BarcodeScanner(BarcodeScanner::try_from(n as u16)?)),
            (141, n) => Ok(Usage::Scales(Scales::try_from(n as u16)?)),
            (142, n) => Ok(Usage::MagneticStripeReader(MagneticStripeReader::try_from(
                n as u16,
            )?)),
            (144, n) => Ok(Usage::CameraControl(CameraControl::try_from(n as u16)?)),
            (145, n) => Ok(Usage::Arcade(Arcade::try_from(n as u16)?)),
            (61904, n) => Ok(Usage::FIDOAlliance(FIDOAlliance::try_from(n as u16)?)),
            (65293, n) => Ok(Usage::Wacom(Wacom::try_from(n as u16)?)),
            (p @ 0xff00..=0xffff, n) => Ok(Usage::VendorDefinedPage {
                vendor_page: VendorPage(p as u16),
                usage: VendorDefinedPage::VendorUsage { usage_id: n as u16 },
            }),
            (p, n) => match ReservedPage::try_from(p as u16) {
                Ok(r) => Ok(Usage::ReservedUsagePage {
                    reserved_page: r,
                    usage: ReservedUsagePage::ReservedUsage { usage_id: n as u16 },
                }),
                _ => Err(HutError::UnknownUsage),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let hid_usage_page: u16 = 0x01; // Generic Desktop
        let hid_usage_id: u16 = 0x02; // Mouse
        let hid_usage: u32 = ((hid_usage_page as u32) << 16) | hid_usage_id as u32;

        let u = GenericDesktop::Mouse;
        // 32 bit usage to enum
        assert!(matches!(
            Usage::try_from(hid_usage).unwrap(),
            Usage::GenericDesktop(_)
        ));

        // Usage to u32
        assert_eq!(u32::from(&u), hid_usage);
        assert_eq!(u.usage_value(), hid_usage);

        // Usage to u16 usage_id
        assert_eq!(hid_usage_id, u16::from(&u));
        assert_eq!(hid_usage_id, u.usage_id_value());

        // Usage to UsagePage
        assert!(matches!(UsagePage::from(&u), UsagePage::GenericDesktop));

        // UsagePage to u16
        let up = UsagePage::from(&u);
        assert_eq!(hid_usage_page, u16::from(&up));

        // UsagePage to u16 via AsUsagePage trait
        assert_eq!(hid_usage_page, up.usage_page_value());
    }

    #[test]
    fn buttons() {
        let hid_usage_page: u16 = 0x9;
        let hid_usage_id: u16 = 0x5;
        let hid_usage: u32 = ((hid_usage_page as u32) << 16) | hid_usage_id as u32;

        let u = Button::Button(5);
        assert!(matches!(
            Usage::try_from(hid_usage).unwrap(),
            Usage::Button(_)
        ));

        // Usage to u32
        assert_eq!(u32::from(&u), hid_usage);
        assert_eq!(u.usage_value(), hid_usage);

        // Usage to u16 usage_id
        assert_eq!(hid_usage_id, u16::from(&u));
        assert_eq!(hid_usage_id, u.usage_id_value());

        // Usage to UsagePage
        assert!(matches!(UsagePage::from(&u), UsagePage::Button));

        // UsagePage to u16
        let up = UsagePage::from(&u);
        assert_eq!(hid_usage_page, u16::from(&up));

        // UsagePage to u16 via AsUsagePage trait
        assert_eq!(hid_usage_page, up.usage_page_value());
    }

    #[test]
    fn ordinals() {
        let hid_usage_page: u16 = 0xA;
        let hid_usage_id: u16 = 0x8;
        let hid_usage: u32 = ((hid_usage_page as u32) << 16) | hid_usage_id as u32;

        let u = Ordinal::Ordinal(8);
        assert!(matches!(
            Usage::try_from(hid_usage).unwrap(),
            Usage::Ordinal(_)
        ));

        // Usage to u32
        assert_eq!(u32::from(&u), hid_usage);
        assert_eq!(u.usage_value(), hid_usage);

        // Usage to u16 usage_id
        assert_eq!(hid_usage_id, u16::from(&u));
        assert_eq!(hid_usage_id, u.usage_id_value());

        // Usage to UsagePage
        assert!(matches!(UsagePage::from(&u), UsagePage::Ordinal));

        // UsagePage to u16
        let up = UsagePage::from(&u);
        assert_eq!(hid_usage_page, u16::from(&up));

        // UsagePage to u16 via AsUsagePage trait
        assert_eq!(hid_usage_page, up.usage_page_value());
    }

    #[cfg(feature = "std")]
    #[test]
    fn names() {
        assert_eq!(UsagePage::GenericDesktop.name().as_str(), "Generic Desktop");
        assert_eq!(
            UsagePage::PhysicalInputDevice.name().as_str(),
            "Physical Input Device"
        );
        assert_eq!(GenericDesktop::CallMuteLED.name().as_str(), "Call Mute LED");
        assert_eq!(VRControls::HeadTracker.name().as_str(), "Head Tracker");
    }

    #[test]
    fn usages() {
        let mouse = GenericDesktop::Mouse;
        let usage = Usage::GenericDesktop(GenericDesktop::Mouse);
        assert_eq!(mouse.usage(), usage);
    }
}
