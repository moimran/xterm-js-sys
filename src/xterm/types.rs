//! Basic types, enums, and constants for xterm.js bindings.

#![allow(missing_docs)] // wasm_bindgen generates additional undocumented items

use wasm_bindgen::prelude::*;

/// An alias for [`String`].
pub type Str = String;

/// A string representing the type of a bell notification.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum BellStyle {
    /// No bell notification.
    None = "none",
    /// [Removed](https://github.com/xtermjs/xterm.js/issues/1155).
    #[deprecated(
        since = "3.0.0",
        note = "See: https://github.com/xtermjs/xterm.js/issues/1155"
    )]
    /// A visual bell notification.
    Visual = "visual",
    /// An auditory bell notification.
    Sound = "sound",
    /// [Removed](https://github.com/xtermjs/xterm.js/issues/1155).
    #[deprecated(
        since = "3.0.0",
        note = "See: https://github.com/xtermjs/xterm.js/issues/1155"
    )]
    Both = "both",
}

/// The style of the cursor.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum CursorStyle {
    /// Block cursor style: `█`.
    Block = "block",
    /// Underline cursor style: `_`.
    Underline = "underline",
    /// Bar cursor style: `|`.
    Bar = "bar",
}

/// The style of the cursor when the terminal is not focused.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum CursorInactiveStyle {
    /// Outline cursor style when inactive.
    Outline = "outline",
    /// Block cursor style when inactive.
    Block = "block",
    /// Bar cursor style when inactive.
    Bar = "bar",
    /// Underline cursor style when inactive.
    Underline = "underline",
    /// No cursor when inactive.
    None = "none",
}

/// The modifier key hold to multiply scroll speed.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum FastScrollModifier {
    /// The 'alt' key.
    Alt = "alt",
    /// The 'ctrl' key.
    Ctrl = "ctrl",
    /// The 'shift' key.
    Shift = "shift",
}

/// A string representing text font weight.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum FontWeight {
    /// Normal font weight.
    Normal = "normal",
    /// Bold font weight.
    Bold = "bold",
    /// 100 font weight.
    _100 = "100",
    /// 200 font weight.
    _200 = "200",
    /// 300 font weight.
    _300 = "300",
    /// 400 font weight.
    _400 = "400",
    /// 500 font weight.
    _500 = "500",
    /// 600 font weight.
    _600 = "600",
    /// 700 font weight.
    _700 = "700",
    /// 800 font weight.
    _800 = "800",
    /// 900 font weight.
    _900 = "900",
}

/// A string representing log level.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum LogLevel {
    /// Show debug (and above) log level information (all logs).
    Debug = "debug",
    /// Show information (and above) log level information.
    Info = "info",
    /// Show warning (and above) log level information.
    Warn = "warn",
    /// Show errors.
    Error = "error",
    /// Show no logs.
    Off = "off",
}

/// A string representing a renderer type.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum RendererType {
    /// The DOM renderer. This is faster but doesn't support some features
    /// (letter spacing, blinking cursor). As such, this is the _fallback_.
    Dom = "dom",
    /// The Canvas renderer.
    Canvas = "canvas",
}

/// A string representing the type of a buffer.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum BufferType {
    /// A normal buffer.
    Normal = "normal",
    /// An alternate buffer.
    Alternate = "alternate",
}

/// Width of a Wide Character.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum WideCharacterWidth {
    /// Width of 0.
    _0 = 0,
    /// Width of 1.
    _1 = 1,
    /// Width of 2.
    _2 = 2,
}

/// Windows PTY backend type.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum WindowsPty {
    /// Auto-detect the Windows PTY backend.
    Auto = "auto",
    /// Use the ConPTY backend.
    Conpty = "conpty",
    /// Use the WinPTY backend.
    Winpty = "winpty",
}

/// Mouse tracking mode.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(missing_docs)] // wasm_bindgen generates additional undocumented items
pub enum MouseTrackingMode {
    /// No mouse tracking (default).
    None = "none",
    /// Send Mouse X & Y on button press (CSI ? 9 h).
    X10 = "x10",
    /// Send Mouse X & Y on button press and release (CSI ? 1000 h).
    Vt200 = "vt200",
    /// Use Cell Motion Mouse Tracking (CSI ? 1002 h).
    Drag = "drag",
    /// Use All Motion Mouse Tracking (CSI ? 1003 h).
    Any = "any",
}
