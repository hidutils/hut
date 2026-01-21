# hut - HID Usage Tables

This crate provides access to the [HID Usage Tables (HUT)](https://usb.org/document-library/hid-usage-tables-15).

This module is created through code generation from the HID Usage Tables.

This crate merely provides enums and functions to convert between values, it
does not concern itself with how to obtain the values to be used. Look at e.g.
the `hidreport` crate for parsing HID Report descriptors.

```rust
use hut::*;

let usage = Usage::from(GenericDesktop::Mouse);

let usage_page_value: u16 = 0x01; // Generic Desktop
let usage_id_value: u16 = 0x02; // Mouse
let usage_value: u32 = ((usage_page_value as u32) << 16) | usage_id_value as u32;

let u: Usage = Usage::try_from(usage_value).unwrap();
```

See the documentation for more details.

# License

`hut` is [MIT-licensed](https://spdx.org/licenses/MIT.html), see the [COPYING](COPYING) file for details.
