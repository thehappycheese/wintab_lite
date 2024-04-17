# Wintab; Lite-ly Rusted <!-- omit in toc -->

Defines a minimal set of types to get rust working with wintab.

- [1. Licence](#1-licence)
- [2. Example  `wintab_lite` with `winit` and `libloading`](#2-example--wintab_lite-with-winit-and-libloading)
  - [2.1. Usage](#21-usage)
  - [2.2. Limitations](#22-limitations)
- [3. Alternatives](#3-alternatives)
- [4. Things I learned](#4-things-i-learned)


## 1. Licence

The original wintab headers and documentation are marked with a copyright notice by Wacom.
See [here](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/Wintab/WINTAB.H#L1C1-L10C81)
and [here](https://developer-docs.wacom.com/docs/icbt/windows/wintab/wintab-reference/).
The example code provided by Wacom is MIT licensed [here](https://github.com/Wacom-Developer/wacom-device-kit-windows/blob/881d8e8303e858e53584e70235fe32e3c9ef06f2/Wintab%20Pressure%20Test/SampleCode/MIT-license.txt)

## 2. Example  `wintab_lite` with `winit` and `libloading`

```bash
cargo run --example winit_libloading
```

### 2.1. Usage

- Press `c` on the keyboard to clear the view.
- Only wintab input will cause anything to be drawn. Mouse won't do anything.

![screenshot](./readme_extras/screenshot.png)

### 2.2. Limitations

- This demo worked for me with my hardware, it does not strictly follow best practices, so you may find you need to tweek the setup code;
  - For example I found that the default LOGCONTEXT object was configured
     differently from what the documentation said on my system, so your milage
     may vary.

## 3. Alternatives

An alternative to this crate is to use
[bindgen](https://crates.io/crates/bindgen) and the original `wintab.h` header
files which are avaliable
[here](https://github.com/Wacom-Developer/wacom-device-kit-windows). I did not
have a good time with that approach

 - A lot of excess code gets generated
 - Missing useful trait definitions
 - Coordinates are represented as separate struct fields instead of being packed
   into an `XYZ` struct
 - Enums are represented as separate const declarations instead of being a rust
   `enum` or using the `bitflags` crate.

## 4. Things I learned

- When working in `winit`, the native `wintab` events (e.g. `WT_PACKET`) are
  unavailable.
  - Luckliy `wintab` supports polling methods and keeps a nice timestamped event
    queue. YOu only need access to the `hwnd` pointer. This is good news as it
    means it is likely-ish I can get this working in `bevy`, as long as the
    plugin lets me have the `hwnd` :P
- My tablet only reports button 1; the pen tip button. Other buttons are
  reported as keystrokes???! I think winit has misinterpreted something there.
  they should be mouse events. Perhaps I broke something in the event loop?
