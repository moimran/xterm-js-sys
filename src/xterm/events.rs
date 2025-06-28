//! Event data structures and related types for xterm.js bindings.

use super::types::Str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Corresponds to `{ key: string, domEvent: KeyboardEvent }`.
    ///
    /// Produced by [`Terminal::on_key`].
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type KeyEventData;

    /// Gets the key that was pressed.
    #[wasm_bindgen(structural, method, getter = key)]
    pub fn key(this: &KeyEventData) -> Str;

    /// Gets the DOM event that triggered this.
    #[wasm_bindgen(structural, method, getter = domEvent)]
    pub fn dom_event(this: &KeyEventData) -> web_sys::KeyboardEvent;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Corresponds to `{ start: number, end: number }`.
    ///
    /// Produced by [`Terminal::on_render`].
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type RenderEventData;

    /// Gets the index of the row at the start of the rendered area.
    ///
    /// This will be ∈ `[` `0`, [`Terminal::rows`] `)`.
    #[wasm_bindgen(structural, method, getter = start)]
    pub fn start(this: &RenderEventData) -> u16;

    /// Gets the index of the row at the end of the rendered area.
    ///
    /// This will be ∈ `[` `0`, [`Terminal::rows`] `)`.
    #[wasm_bindgen(structural, method, getter = end)]
    pub fn end(this: &RenderEventData) -> u16;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Corresponds to `{ cols: number, rows: number }`.
    ///
    /// Produced by [`Terminal::on_resize`].
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type ResizeEventData;

    /// Gets the new number of columns.
    #[wasm_bindgen(structural, method, getter = cols)]
    pub fn cols(this: &ResizeEventData) -> u16;

    /// Gets the new number of rows.
    #[wasm_bindgen(structural, method, getter = rows)]
    pub fn rows(this: &ResizeEventData) -> u16;
}
