//! Core interfaces like Disposable, Marker, etc. for xterm.js bindings.

use super::types::*;
use crate::ReadOnlyArray;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// An object that can be disposed via a dispose function.
    ///
    /// (This is a [duck-typed interface]; its Rust dual is available [here]
    /// when the `ext` feature is enabled).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    /// [here]: crate::ext::disposable::XtermDisposable
    #[derive(Debug, Clone)]
    pub type Disposable;

    /// Disposes of the instance.
    ///
    /// This can involve unregistering an event listener or cleaning up
    /// resources or anything else that should happen when an instance is
    /// disposed of.
    #[wasm_bindgen(structural, method, js_name = dispose)]
    pub fn dispose(this: &Disposable);
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// The set of localizable strings.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type LocalizableStrings;

    /// The aria label for the underlying input textarea for the terminal.
    #[wasm_bindgen(structural, method, getter = promptLabel)]
    pub fn prompt_label(this: &LocalizableStrings) -> Str;

    /// Announcement for when line reading is suppressed due to too many lines
    /// being printed to the terminal when `screenReaderMode` is enabled.
    #[wasm_bindgen(structural, method, getter = tooMuchOutput)]
    pub fn too_much_output(this: &LocalizableStrings) -> Str;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a specific line in the terminal that is tracked when
    /// scrollback is trimmed and lines are added or removed. This is a single
    /// line that may be part of a larger wrapped line.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type Marker;

    /// A unique identifier for this marker.
    #[wasm_bindgen(structural, method, getter = id)]
    pub fn id(this: &Marker) -> u32;

    /// Whether this marker is disposed.
    #[wasm_bindgen(structural, method, getter = isDisposed)]
    pub fn is_disposed(this: &Marker) -> bool;

    /// The actual line index in the buffer at this point in time. This is set
    /// to -1 if the marker has been disposed.
    #[wasm_bindgen(structural, method, getter = line)]
    pub fn line(this: &Marker) -> i32;

    /// Adds an event listener for when the marker is disposed.
    ///
    /// Returns a [`Disposable`] to stop listening.
    #[wasm_bindgen(structural, method, js_name = onDispose)]
    pub fn on_dispose(this: &Marker, listener: &Closure<dyn FnMut()>) -> Disposable;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// **[EXPERIMENTAL]** Represents a decoration in the terminal that is
    /// associated with a particular marker and DOM element.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type Decoration;

    /// The marker for the decoration in the terminal.
    #[wasm_bindgen(structural, method, getter = marker)]
    pub fn marker(this: &Decoration) -> Marker;

    /// The element that the decoration is rendered to. This will be undefined
    /// until it is rendered for the first time by [`Decoration::on_render`].
    ///
    /// [`Decoration::on_render`]: Decoration::on_render
    #[wasm_bindgen(structural, method, getter = element)]
    pub fn element(this: &Decoration) -> Option<web_sys::HtmlElement>;

    /// Whether this decoration is disposed.
    #[wasm_bindgen(structural, method, getter = isDisposed)]
    pub fn is_disposed(this: &Decoration) -> bool;

    /// Adds an event listener for when the decoration is disposed.
    ///
    /// Returns a [`Disposable`] to stop listening.
    #[wasm_bindgen(structural, method, js_name = onDispose)]
    pub fn on_dispose(this: &Decoration, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// An event fired when the decoration is rendered, returns the dom element
    /// associated with the decoration.
    ///
    /// Returns a [`Disposable`] to stop listening.
    #[wasm_bindgen(structural, method, js_name = onRender)]
    pub fn on_render(
        this: &Decoration,
        listener: &Closure<dyn FnMut(web_sys::HtmlElement)>,
    ) -> Disposable;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// An addon that can provide additional functionality to the terminal.
    ///
    /// (This is a [duck-typed interface]; its Rust dual is available [here]
    /// when the `ext` feature is enabled).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    /// [here]: crate::ext::addon::XtermAddon
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type TerminalAddon;

    /// This is called when the addon is activated.
    #[wasm_bindgen(structural, method, js_name = activate)]
    pub fn activate(this: &TerminalAddon, terminal: super::terminal::Terminal);
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// **[EXPERIMENTAL]** Unicode handling interface.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type UnicodeHandling;

    /// Registers a [custom Unicode version provider].
    ///
    /// [custom Unicode version provider]: UnicodeVersionProvider
    #[wasm_bindgen(structural, method, js_name = register)]
    pub fn register(this: &UnicodeHandling, provider: UnicodeVersionProvider);

    /// Gets the active Unicode version.
    #[wasm_bindgen(structural, method, getter = activeVersion)]
    pub fn active_version(this: &UnicodeHandling) -> Str;

    /// Sets the active Unicode version.
    #[wasm_bindgen(structural, method, setter = activeVersion)]
    pub fn set_active_version(this: &UnicodeHandling, version: Str);

    /// Gets registered Unicode versions.
    #[wasm_bindgen(structural, method, getter = versions)]
    pub fn versions(this: &UnicodeHandling) -> ReadOnlyArray<js_sys::JsString>;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// **[EXPERIMENTAL]** Unicode version provider.
    ///
    /// Used to register custom Unicode versions with
    /// [`UnicodeHandling::register`] (obtained from [`Terminal::unicode`]).
    ///
    /// (This is a [duck-typed interface]; its Rust dual is available [here]
    /// when the `ext` feature is enabled).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    /// [here]: crate::ext::XtermUnicodeVersionProvider
    #[derive(Debug, Clone)]
    pub type UnicodeVersionProvider;

    /// Gets a string indicating the Unicode version provided.
    #[wasm_bindgen(structural, method, getter = version)]
    pub fn version(this: &UnicodeVersionProvider) -> Str;

    /// Unicode version dependent wcwidth implementation.
    #[wasm_bindgen(structural, method, js_name = wcwidth)]
    pub fn wcwidth(this: &UnicodeVersionProvider, codepoint: u32) -> WideCharacterWidth;

    /// Unicode version dependent character properties implementation.
    #[wasm_bindgen(structural, method, js_name = charProperties)]
    pub fn char_properties(this: &UnicodeVersionProvider, codepoint: u32, preceding: u32) -> u32;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// **[EXPERIMENTAL]** Parser interface.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type Parser;

    /// Adds a handler for CSI escape sequences.
    ///
    /// Takes:
    ///   - `id`: Specifies the function identifier under which the callback gets
    ///           registered, e.g. {final: 'm'} for SGR.
    ///   - `callback`: The function to handle the sequence. The callback is
    ///                 called with the numerical params. If the sequence has
    ///                 subparams the array will contain subarrays with their
    ///                 numerical values. Return `true` if the sequence was
    ///                 handled, `false` if the parser should try a previous
    ///                 handler. The most recently added handler is tried first.
    ///
    /// Returns a [`Disposable`] you can call to remove this handler.
    #[wasm_bindgen(structural, method, js_name = registerCsiHandler)]
    pub fn register_csi_handler(
        this: &Parser,
        id: super::options::FunctionIdentifier,
        callback: &Closure<dyn FnMut(js_sys::Array) -> bool>,
    ) -> Disposable;

    /// Adds a handler for DCS escape sequences.
    ///
    /// Takes:
    ///   - `id`: Specifies the function identifier under which the callback gets
    ///           registered, e.g. {intermediates: '$' final: 'q'} for DECRQSS.
    ///   - `callback`: The function to handle the sequence. Note that the
    ///                 function will only be called once if the sequence finished
    ///                 successfully. There is currently no way to intercept
    ///                 smaller data chunks, data chunks will be stored up until
    ///                 the sequence is finished. Since DCS sequences are not
    ///                 limited by the amount of data this might impose a problem
    ///                 for big payloads. Currently xterm.js limits DCS payload to
    ///                 10 MB which should give enough room for most use cases.
    ///                 The function gets the payload and numerical parameters as
    ///                 arguments. Return `true` if the sequence was handled,
    ///                 `false` if the parser should try a previous handler. The
    ///                 most recently added handler is tried first.
    ///
    /// Returns a [`Disposable`] you can call to remove this handler.
    #[wasm_bindgen(structural, method, js_name = registerDcsHandler)]
    pub fn register_dcs_handler(
        this: &Parser,
        id: super::options::FunctionIdentifier,
        callback: &Closure<dyn FnMut(Str, js_sys::Array) -> bool>,
    ) -> Disposable;

    /// Adds a handler for ESC escape sequences.
    ///
    /// Takes:
    ///   - `id`: Specifies the function identifier under which the callback gets
    ///           registered, e.g. {intermediates: '%' final: 'G'} for default
    ///           charset selection.
    ///   - `callback`: The function to handle the sequence. Return `true` if the
    ///                 sequence was handled, `false` if the parser should try a
    ///                 previous handler. The most recently added handler is tried
    ///                 first.
    ///
    /// Returns a [`Disposable`] you can call to remove this handler.
    #[wasm_bindgen(structural, method, js_name = registerEscHandler)]
    pub fn register_esc_handler(
        this: &Parser,
        id: super::options::FunctionIdentifier,
        callback: &Closure<dyn FnMut() -> bool>,
    ) -> Disposable;

    /// Adds a handler for OSC escape sequences.
    ///
    /// Takes:
    ///   - `ident`: The number (first parameter) of the sequence.
    ///   - `callback`: The function to handle the sequence. Note that the
    ///                 function will only be called once if the sequence finished
    ///                 successfully. There is currently no way to intercept
    ///                 smaller data chunks, data chunks will be stored up until
    ///                 the sequence is finished. Since OSC sequences are not
    ///                 limited by the amount of data this might impose a problem
    ///                 for big payloads. Currently xterm.js limits OSC payload to
    ///                 10 MB which should give enough room for most use cases.
    ///                 The callback is called with OSC data string. Return `true`
    ///                 if the sequence was handled, `false` if the parser should
    ///                 try a previous handler. The most recently added handler is
    ///                 tried first.
    ///
    /// Returns a [`Disposable`] you can call to remove this handler.
    #[wasm_bindgen(structural, method, js_name = registerOscHandler)]
    pub fn register_osc_handler(
        this: &Parser,
        ident: u16,
        callback: &Closure<dyn FnMut(Str) -> bool>,
    ) -> Disposable;
}
