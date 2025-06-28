//! The main Terminal implementation for xterm.js bindings.

use super::buffer::*;
use super::events::*;
use super::interfaces::*;
use super::options::*;
use super::types::*;
use crate::ReadOnlyArray;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// The class that represents an xterm.js terminal.
    #[wasm_bindgen(extends = Disposable)]
    #[derive(Debug, Clone)]
    pub type Terminal;

    /// Creates a new Terminal object.
    ///
    /// Takes `options`: an object containing a set of options.
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<TerminalOptions>) -> Terminal;

    /////////////////////////////// Properties ///////////////////////////////

    /// **[EXPERIMENTAL]** The terminal's current buffer, this might be either
    /// the normal buffer or the alt buffer depending on what's running in the
    /// terminal.
    #[wasm_bindgen(method, getter = buffer)]
    pub fn buffer(this: &Terminal) -> BufferNamespace;

    /// The number of columns in the terminal's viewport. Use
    /// [`TerminalOptions::cols`] to set this in the [constructor] and
    /// [`Terminal::resize`] for when the terminal exists.
    ///
    /// [constructor]: Terminal::new
    #[wasm_bindgen(method, getter = cols)]
    pub fn cols(this: &Terminal) -> u16;

    /// The [element] containing the terminal.
    ///
    /// [element]: web_sys::HtmlElement
    #[wasm_bindgen(method, getter = element)]
    pub fn element(this: &Terminal) -> Option<web_sys::HtmlElement>;

    /// **[EXPERIMENTAL]** Get all markers registered against the buffer. If the
    /// alt buffer is active this will always return `[]`.
    #[wasm_bindgen(method, getter = markers)]
    pub fn markers(this: &Terminal) -> ReadOnlyArray<Marker>;

    /// Get the parser interface to register custom escape sequence handlers.
    #[wasm_bindgen(method, getter = parser)]
    pub fn parser(this: &Terminal) -> Parser;

    /// The number of rows in the terminal's viewport. Use
    /// [`TerminalOptions::rows`] to set this in the [constructor] and
    /// [`Terminal::resize`] for when the terminal exists.
    ///
    /// [constructor]: Terminal::new
    #[wasm_bindgen(method, getter = rows)]
    pub fn rows(this: &Terminal) -> u16;

    /// The [textarea] that accepts input for the terminal.
    ///
    /// [textarea]: web_sys::HtmlTextAreaElement
    #[wasm_bindgen(method, getter = textarea)]
    pub fn textarea(this: &Terminal) -> Option<web_sys::HtmlTextAreaElement>;

    /// **[EXPERIMENTAL]** Get the Unicode handling interface.
    ///
    /// This can be used to register Unicode versions and switch the active
    /// Unicode version.
    #[wasm_bindgen(method, getter = unicode)]
    pub fn unicode(this: &Terminal) -> UnicodeHandling;

    /// Natural language strings that can be localized.
    #[wasm_bindgen(method, getter = strings)]
    pub fn strings(this: &Terminal) -> LocalizableStrings;

    /// Gets the terminal modes as set by SM/DECSET.
    #[wasm_bindgen(method, getter = modes)]
    pub fn modes(this: &Terminal) -> Modes;

    /// Gets the terminal options. This supports getting multiple options.
    ///
    /// Example:
    /// ```rust
    /// let font_size = terminal.options().font_size();
    /// ```
    #[wasm_bindgen(method, getter = options)]
    pub fn options(this: &Terminal) -> TerminalOptions;

    /// Sets the terminal options. This supports setting multiple options.
    ///
    /// Note that for options that are objects, a new object must be used in order
    /// to take effect as a reference comparison will be done.
    ///
    /// Example:
    /// ```rust
    /// let mut new_options = TerminalOptions::default();
    /// new_options.set_font_size(12.0);
    /// terminal.set_options(new_options);
    /// ```
    #[wasm_bindgen(method, setter = options)]
    pub fn set_options(this: &Terminal, options: TerminalOptions);

    /////////////////////////////// Event Listeners ///////////////////////////////

    /// Adds an event listener for when the bell is triggered.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_bell_event_listener`] (if the `ext` feature is enabled) for
    /// a friendlier version of this function.
    ///
    /// [`attach_bell_event_listener`]: Terminal::attach_bell_event_listener
    #[wasm_bindgen(method, js_name = onBell)]
    pub fn on_bell(this: &Terminal, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// Adds an event listener for when a binary event fires. This is used to
    /// enable non UTF-8 conformant binary messages to be sent to the backend.
    /// Currently this is only used for a certain type of mouse reports that
    /// happen to be not UTF-8 compatible. The event value is a JS string, pass
    /// it to the underlying pty as binary data, e.g.
    /// `pty.write(Buffer.from(data, 'binary'))`.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_binary_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`attach_binary_event_listener`]: Terminal::attach_binary_event_listener
    #[wasm_bindgen(method, js_name = onBinary)]
    pub fn on_binary(this: &Terminal, listener: &Closure<dyn FnMut(Str)>) -> Disposable;

    /// Adds an event listener for the cursor moves.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_cursor_move_event_listener`] (if the `ext` feature is
    /// enabled) for a friendlier version of this function.
    ///
    /// [`attach_cursor_move_event_listener`]: Terminal::attach_cursor_move_event_listener
    #[wasm_bindgen(method, js_name = onCursorMove)]
    pub fn on_cursor_move(this: &Terminal, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// Adds an event listener for when a data event fires. This happens for
    /// example when the user types or pastes into the terminal. The event value
    /// is whatever `string` results, in a typical setup, this should be passed
    /// on to the backing pty.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_data_event_listener`] (if the `ext` feature is enabled) for
    /// a friendlier version of this function.
    ///
    /// [`attach_data_event_listener`]: Terminal::attach_data_event_listener
    #[wasm_bindgen(method, js_name = onData)]
    pub fn on_data(this: &Terminal, listener: &Closure<dyn FnMut(Str)>) -> Disposable;

    /// Adds an event listener for when a key is pressed. The event value
    /// contains the string that will be sent in the data event as well as the
    /// DOM event that triggered it.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_key_event_listener`] (if the `ext` feature is enabled) for
    /// a friendlier version of this function.
    ///
    /// [`attach_key_event_listener`]: Terminal::attach_key_event_listener
    #[wasm_bindgen(method, js_name = onKey)]
    pub fn on_key(this: &Terminal, listener: &Closure<dyn FnMut(KeyEventData)>) -> Disposable;

    /// Adds an event listener for when a line feed is added.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_line_feed_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`attach_line_feed_event_listener`]: Terminal::attach_line_feed_event_listener
    #[wasm_bindgen(method, js_name = onLineFeed)]
    pub fn on_line_feed(this: &Terminal, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// Adds an event listener for when rows are rendered. The event value
    /// contains the start row and end rows of the rendered area (ranges from `0`
    /// to `Terminal.rows - 1`).
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_render_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`attach_render_event_listener`]: Terminal::attach_render_event_listener
    #[wasm_bindgen(method, js_name = onRender)]
    pub fn on_render(
        this: &Terminal,
        listener: &Closure<dyn FnMut(RenderEventData)>,
    ) -> Disposable;

    /// Adds an event listener for when data has been parsed by the terminal,
    /// after [`write`] is called. This event is useful to listen for any
    /// changes in the buffer.
    ///
    /// This fires at most once per frame, after data parsing completes. Note
    /// that this can fire when there are still writes pending if there is a lot
    /// of data.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// [`write`]: Terminal::write
    #[wasm_bindgen(method, js_name = onWriteParsed)]
    pub fn on_write_parsed(this: &Terminal, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// Adds an event listener for when the terminal is resized.
    ///
    /// The event value ([`ResizeEventData`]) contains the new size.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_resize_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`attach_resize_event_listener`]: Terminal::attach_resize_event_listener
    #[wasm_bindgen(method, js_name = onResize)]
    pub fn on_resize(
        this: &Terminal,
        listener: &Closure<dyn FnMut(ResizeEventData)>,
    ) -> Disposable;

    /// Adds an event listener for when a event listener for when a scroll
    /// occurs.
    ///
    /// The event value is the new position of the viewport.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_scroll_event_listener`] (if the `ext` feature is enabled)
    /// for a friendlier version of this function.
    ///
    /// [`attach_scroll_event_listener`]: Terminal::attach_scroll_event_listener
    #[wasm_bindgen(method, js_name = onScroll)]
    pub fn on_scroll(this: &Terminal, listener: &Closure<dyn FnMut(u32)>) -> Disposable;

    /// Adds an event listener for when a selection change occurs.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_selection_change_event_listener`] (if the `ext` feature is
    /// enabled) for a friendlier version of this function.
    ///
    /// [`attach_selection_change_event_listener`]: Terminal::attach_selection_change_event_listener
    #[wasm_bindgen(method, js_name = onSelectionChange)]
    pub fn on_selection_change(this: &Terminal, listener: &Closure<dyn FnMut()>) -> Disposable;

    /// Adds an event listener for when an OSC 0 or OSC 2 title change occurs.
    ///
    /// The event value is the new title.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_title_change_event_listener`] (if the `ext` feature is
    /// enabled) for a friendlier version of this function.
    ///
    /// [`attach_title_change_event_listener`]: Terminal::attach_title_change_event_listener
    #[wasm_bindgen(method, js_name = onTitleChange)]
    pub fn on_title_change(this: &Terminal, listener: &Closure<dyn FnMut(Str)>) -> Disposable;

    /////////////////////////////// Core Methods ///////////////////////////////

    /// Unfocus the terminal.
    #[wasm_bindgen(method, js_name = blur)]
    pub fn blur(this: &Terminal);

    /// Clear the entire buffer, making the prompt line the new first line.
    #[wasm_bindgen(method, js_name = clear)]
    pub fn clear(this: &Terminal);

    /// Clear the current line.
    #[wasm_bindgen(method, js_name = clearSelection)]
    pub fn clear_selection(this: &Terminal);

    /// Focus the terminal.
    #[wasm_bindgen(method, js_name = focus)]
    pub fn focus(this: &Terminal);

    /// Input data to application side. The data is treated the same way input
    /// typed into the terminal would (ie. the onData event will fire).
    ///
    /// Takes:
    ///   - `data`: The data to forward to the application.
    ///   - `was_user_input`: Whether the input is genuine user input. This is
    ///     true by default and triggers additional behavior like focus or
    ///     selection clearing. Set this to false if the data sent should not
    ///     be treated like user input would, for example passing an escape
    ///     sequence to the application.
    #[wasm_bindgen(method, js_name = input)]
    pub fn input(this: &Terminal, data: Str);

    /// Input data to application side with user input flag.
    #[wasm_bindgen(method, js_name = input)]
    pub fn input_with_user_flag(this: &Terminal, data: Str, was_user_input: bool);

    /// Gets whether the terminal has focus.
    #[wasm_bindgen(method, js_name = hasSelection)]
    pub fn has_selection(this: &Terminal) -> bool;

    /// Gets the terminal's current selection, this is useful for implementing
    /// copy behavior outside of xterm.js.
    #[wasm_bindgen(method, js_name = getSelection)]
    pub fn get_selection(this: &Terminal) -> Str;

    /// Gets the selection position or undefined if there is no selection.
    #[wasm_bindgen(method, js_name = getSelectionPosition)]
    pub fn get_selection_position(this: &Terminal) -> Option<js_sys::Object>;

    /// Loads an addon into this instance of the terminal.
    ///
    /// Takes `addon`: The addon to load.
    #[wasm_bindgen(method, js_name = loadAddon)]
    pub fn load_addon(this: &Terminal, addon: TerminalAddon);

    /// Opens the terminal within an element.
    ///
    /// Takes `parent`: The element to create the terminal within. This element
    /// must be visible (have dimensions) when `open` is called as several DOM-
    /// based measurements need to be performed when this function is called.
    #[wasm_bindgen(method, js_name = open)]
    pub fn open(this: &Terminal, parent: web_sys::HtmlElement);

    /// Attaches a custom key event handler which is run before keys are
    /// processed, giving consumers of xterm.js ultimate control as to what keys
    /// should be processed by the terminal and what keys should not.
    ///
    /// Takes `custom_key_event_handler`: The custom KeyboardEvent handler to attach.
    /// This is a function that takes a KeyboardEvent, allowing consumers to stop
    /// propagation and/or prevent the default action. The function returns
    /// whether the event should be processed by xterm.js.
    #[wasm_bindgen(method, js_name = attachCustomKeyEventHandler)]
    pub fn attach_custom_key_event_handler(
        this: &Terminal,
        custom_key_event_handler: &Closure<dyn FnMut(web_sys::KeyboardEvent) -> bool>,
    );

    /// Attaches a custom wheel event handler which is run before wheel events are
    /// processed, giving consumers of xterm.js control over whether to proceed
    /// or cancel terminal wheel events.
    ///
    /// Takes `custom_wheel_event_handler`: The custom WheelEvent handler to attach.
    /// This is a function that takes a WheelEvent, allowing consumers to stop
    /// propagation and/or prevent the default action. The function returns
    /// whether the event should be processed by xterm.js.
    #[wasm_bindgen(method, js_name = attachCustomWheelEventHandler)]
    pub fn attach_custom_wheel_event_handler(
        this: &Terminal,
        custom_wheel_event_handler: &Closure<dyn FnMut(web_sys::WheelEvent) -> bool>,
    );

    /// Pastes the given data to the terminal.
    ///
    /// Takes `data`: The data to paste.
    #[wasm_bindgen(method, js_name = paste)]
    pub fn paste(this: &Terminal, data: Str);

    /// Tells the renderer to refresh terminal content between two rows
    /// (inclusive) at the next opportunity.
    ///
    /// Takes:
    ///   - `start`: The row to start from (between 0 and this.rows - 1).
    ///   - `end`: The row to end at (between start and this.rows - 1).
    #[wasm_bindgen(method, js_name = refresh)]
    pub fn refresh(this: &Terminal, start: u16, end: u16);

    /// Clears the texture atlas of the canvas renderer if it's active. Doing
    /// this will force a redraw of all glyphs which can workaround issues
    /// causing the texture to become corrupt, for example Chromium/Nvidia has an
    /// issue where the texture gets messed up when resuming the OS from sleep.
    #[wasm_bindgen(method, js_name = clearTextureAtlas)]
    pub fn clear_texture_atlas(this: &Terminal);

    /// **[EXPERIMENTAL]** Registers a marker at the current cursor position.
    ///
    /// Returns the new marker or undefined if the alt buffer is active.
    #[wasm_bindgen(method, js_name = registerMarker)]
    pub fn register_marker(this: &Terminal, cursor_y_offset: Option<i16>) -> Option<Marker>;

    /// **[EXPERIMENTAL]** Registers a character joiner, allowing custom sequences of
    /// characters to be rendered as a single unit. This is useful in particular
    /// for rendering ligatures and graphemes, among other things.
    ///
    /// Each registered character joiner is called with a string of text
    /// representing a portion of a line in the terminal that can be rendered as
    /// a single unit. The joiner must return a sorted array, where each entry is
    /// itself an array of length two, containing the start (inclusive) and end
    /// (exclusive) index of a substring of the input that should be rendered as
    /// a single unit.
    ///
    /// NOTE: character joiners are only used by the canvas renderer.
    ///
    /// Takes `handler`: The function that determines character joins. It is called
    /// with a string of text that is eligible for joining and returns an array
    /// where each entry is an array containing the start (inclusive) and end
    /// (exclusive) indexes of ranges that should be rendered as a single unit.
    ///
    /// Returns the ID of the new joiner, this can be used to deregister.
    #[wasm_bindgen(method, js_name = registerCharacterJoiner)]
    pub fn register_character_joiner(
        this: &Terminal,
        handler: &Closure<dyn FnMut(Str) -> js_sys::Array>,
    ) -> u32;

    /// **[EXPERIMENTAL]** Deregisters the character joiner if one was registered.
    /// NOTE: character joiners are only used by the canvas renderer.
    ///
    /// Takes `joiner_id`: The character joiner's ID (returned after register).
    #[wasm_bindgen(method, js_name = deregisterCharacterJoiner)]
    pub fn deregister_character_joiner(this: &Terminal, joiner_id: u32);

    /// Resizes the terminal. It's best practice to debounce calls to resize,
    /// this will help ensure that the pty can respond to the resize event
    /// before another one occurs.
    ///
    /// Takes:
    ///   - `columns`: The number of columns to resize to.
    ///   - `rows`: The number of rows to resize to.
    #[wasm_bindgen(method, js_name = resize)]
    pub fn resize(this: &Terminal, columns: u16, rows: u16);

    /// Scroll the display of the terminal by a number of lines.
    ///
    /// Takes `amount`: The number of lines to scroll down (negative scroll up).
    #[wasm_bindgen(method, js_name = scrollLines)]
    pub fn scroll_lines(this: &Terminal, amount: i16);

    /// Scroll the display of the terminal to the bottom.
    #[wasm_bindgen(method, js_name = scrollToBottom)]
    pub fn scroll_to_bottom(this: &Terminal);

    /// Scroll the display of the terminal to the top.
    #[wasm_bindgen(method, js_name = scrollToTop)]
    pub fn scroll_to_top(this: &Terminal);

    /// Scrolls the display of the terminal to a line.
    ///
    /// Takes `line`: The line to scroll to.
    #[wasm_bindgen(method, js_name = scrollToLine)]
    pub fn scroll_to_line(this: &Terminal, line: u16);

    /// Selects text within the terminal.
    ///
    /// Takes:
    ///   - `column`: The column the selection starts at.
    ///   - `row`: The row the selection starts at.
    ///   - `length`: The length of the selection.
    #[wasm_bindgen(method, js_name = select)]
    pub fn select(this: &Terminal, column: u16, row: u16, length: u16);

    /// Selects all text within the terminal.
    #[wasm_bindgen(method, js_name = selectAll)]
    pub fn select_all(this: &Terminal);

    /// Selects text in the buffer between 2 lines.
    ///
    /// Takes:
    ///   - `start`: The 0-based line index to select from (inclusive).
    ///   - `end`: The 0-based line index to select to (inclusive).
    #[wasm_bindgen(method, js_name = selectLines)]
    pub fn select_lines(this: &Terminal, start: u16, end: u16);

    /// Writes data to the terminal.
    ///
    /// Takes:
    ///   - `data`: The data to write to the terminal. This can either be raw
    ///             bytes given as `Uint8Array` from the pty or a string. Raw
    ///             bytes will always be treated as UTF-8 encoded, string data
    ///             as UTF-16. For simplicity, we just take a `String`; this
    ///             shouldn't cause problems (going from UTF-8 encoded Rust
    ///             `String`s to UTF-16 JS strings) and just makes things
    ///             simpler.
    #[wasm_bindgen(method, js_name = write)]
    pub fn write(this: &Terminal, data: Str);

    /// Writes data to the terminal and takes a callback.
    ///
    /// This identical to [`write`] except it also takes a callback.
    ///
    /// Takes:
    ///   - `data`:     The data to write to the terminal. This can either be raw
    ///                 bytes given as `Uint8Array` from the pty or a string. Raw
    ///                 bytes will always be treated as UTF-8 encoded, string data
    ///                 as UTF-16. For simplicity, we just take a `String`; this
    ///                 shouldn't cause problems (going from UTF-8 encoded Rust
    ///                 `String`s to UTF-16 JS strings) and just makes things
    ///                 simpler.
    ///  - `callback`: Callback that fires when the data was processed by the
    ///                parser.
    ///
    /// [`write`]: Terminal::write
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_with_callback(
        this: &Terminal,
        data: Str,
        callback: &Closure<dyn FnMut()>,
    );

    /// Writes raw bytes to the terminal.
    ///
    /// Takes:
    ///   - `data`: The raw bytes to write to the terminal as `Uint8Array`.
    ///             Raw bytes will always be treated as UTF-8 encoded.
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_bytes(this: &Terminal, data: &[u8]);

    /// Writes raw bytes to the terminal using Uint8Array.
    ///
    /// Takes:
    ///   - `data`: The raw bytes to write to the terminal as `Uint8Array`.
    ///             Raw bytes will always be treated as UTF-8 encoded.
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_uint8_array(this: &Terminal, data: &js_sys::Uint8Array);

    /// Writes raw bytes to the terminal and takes a callback.
    ///
    /// Takes:
    ///   - `data`:     The raw bytes to write to the terminal as `Uint8Array`.
    ///                 Raw bytes will always be treated as UTF-8 encoded.
    ///   - `callback`: Callback that fires when the data was processed by the
    ///                 parser.
    #[wasm_bindgen(method, js_name = write)]
    pub fn write_bytes_with_callback(
        this: &Terminal,
        data: &[u8],
        callback: &Closure<dyn FnMut()>,
    );

    /// Writes data to the terminal, followed by a break line character (\n).
    ///
    /// Takes:
    ///   - `data`: The data to write to the terminal.
    #[wasm_bindgen(method, js_name = writeln)]
    pub fn writeln(this: &Terminal, data: Str);

    /// Writes data to the terminal, followed by a break line character (\n),
    /// and takes a callback.
    ///
    /// Takes:
    ///   - `data`:     The data to write to the terminal.
    ///   - `callback`: Callback that fires when the data was processed by the
    ///                 parser.
    #[wasm_bindgen(method, js_name = writeln)]
    pub fn writeln_with_callback(
        this: &Terminal,
        data: Str,
        callback: &Closure<dyn FnMut()>,
    );

    /// Writes raw bytes to the terminal, followed by a break line character (\n).
    ///
    /// Takes:
    ///   - `data`: The raw bytes to write to the terminal as `Uint8Array`.
    ///             Raw bytes will always be treated as UTF-8 encoded.
    #[wasm_bindgen(method, js_name = writeln)]
    pub fn writeln_bytes(this: &Terminal, data: &[u8]);

    /// Writes raw bytes to the terminal, followed by a break line character (\n),
    /// and takes a callback.
    ///
    /// Takes:
    ///   - `data`:     The raw bytes to write to the terminal as `Uint8Array`.
    ///                 Raw bytes will always be treated as UTF-8 encoded.
    ///   - `callback`: Callback that fires when the data was processed by the
    ///                 parser.
    #[wasm_bindgen(method, js_name = writeln)]
    pub fn writeln_bytes_with_callback(
        this: &Terminal,
        data: &[u8],
        callback: &Closure<dyn FnMut()>,
    );

// TODO: registerLinkProvider
}
