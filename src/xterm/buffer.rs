//! Buffer-related interfaces and types for xterm.js bindings.

use super::types::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a single cell in the terminal's buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type BufferCell;

    /// Gets the number representation of the background color mode, this can be
    /// used to perform quick comparisons of 2 cells to see if they're the same.
    /// Use `is_bg_rgb`, `is_bg_palette` and `is_bg_default` to check what color
    /// mode a cell is.
    #[wasm_bindgen(structural, method, js_name = getBgColorMode)]
    pub fn get_bg_color_mode(this: &BufferCell) -> u8;

    /// Gets a cell's background color number, this differs depending on what the
    /// color mode of the cell is:
    ///
    ///   - Default: This should be 0, representing the default background color
    ///     (CSI 49 m).
    ///   - Palette: This is a number from 0 to 255 of ANSI colors
    ///     (CSI 4(0-7) m, CSI 10(0-7) m, CSI 48 ; 5 ; 0-255 m).
    ///   - RGB: A hex value representing a 'true color': 0xRRGGBB
    ///     (CSI 4 8 ; 2 ; Pi ; Pr ; Pg ; Pb)
    #[wasm_bindgen(structural, method, js_name = getBgColor)]
    pub fn get_bg_color(this: &BufferCell) -> u32;

    /// Gets the character(s) within the cell. Examples of what this can contain:
    ///
    ///   - A normal width character
    ///   - A wide character (eg. CJK)
    ///   - An emoji
    #[wasm_bindgen(structural, method, js_name = getChars)]
    pub fn get_chars(this: &BufferCell) -> Str;

    /// Gets the UTF32 codepoint of single characters, if content is a combined
    /// string it returns the codepoint of the last character in the string.
    #[wasm_bindgen(structural, method, js_name = getCode)]
    pub fn get_code(this: &BufferCell) -> u32;

    /// Gets the number representation of the foreground color mode, this can be
    /// used to perform quick comparisons of 2 cells to see if they're the same.
    /// Use `is_fg_rgb`, `is_fg_palette` and `is_fg_default` to check what color
    /// mode a cell is.
    #[wasm_bindgen(structural, method, js_name = getFgColorMode)]
    pub fn get_fg_color_mode(this: &BufferCell) -> u8;

    /// Gets a cell's foreground color number, this differs depending on what the
    /// color mode of the cell is:
    ///
    ///   - Default: This should be 0, representing the default foreground color
    ///     (CSI 39 m).
    ///   - Palette: This is a number from 0 to 255 of ANSI colors (CSI 3(0-7) m,
    ///     CSI 9(0-7) m, CSI 38 ; 5 ; 0-255 m).
    ///   - RGB: A hex value representing a 'true color': 0xRRGGBB.
    ///     (CSI 3 8 ; 2 ; Pi ; Pr ; Pg ; Pb)
    #[wasm_bindgen(structural, method, js_name = getFgColor)]
    pub fn get_fg_color(this: &BufferCell) -> u32;

    /// The width of the character. Some examples:
    ///
    ///   - `1` for most cells.
    ///   - `2` for wide character like CJK glyphs.
    ///   - `0` for cells immediately following cells with a width of `2`.
    #[wasm_bindgen(structural, method, js_name = getWidth)]
    pub fn get_width(this: &BufferCell) -> WideCharacterWidth;

    /// Whether the cell has the default attribute (no color or style).
    #[wasm_bindgen(structural, method, js_name = isAttributeDefault)]
    pub fn is_attribute_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the default background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgDefault)]
    pub fn is_bg_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the palette background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgPalette)]
    pub fn is_bg_palette(this: &BufferCell) -> bool;

    /// Whether the cell is using the RGB background color mode.
    #[wasm_bindgen(structural, method, js_name = isBgRGB)]
    pub fn is_bg_rgb(this: &BufferCell) -> bool;

    /// Whether the cell has the blink attribute (CSI 5 m).
    #[wasm_bindgen(structural, method, js_name = isBlink)]
    pub fn is_blink(this: &BufferCell) -> u8;

    /// Whether the cell has the bold attribute (CSI 1 m).
    #[wasm_bindgen(structural, method, js_name = isBold)]
    pub fn is_bold(this: &BufferCell) -> u8;

    /// Whether the cell has the dim attribute (CSI 2 m).
    #[wasm_bindgen(structural, method, js_name = isDim)]
    pub fn is_dim(this: &BufferCell) -> u8;

    /// Whether the cell is using the default foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgDefault)]
    pub fn is_fg_default(this: &BufferCell) -> bool;

    /// Whether the cell is using the palette foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgPalette)]
    pub fn is_fg_palette(this: &BufferCell) -> bool;

    /// Whether the cell is using the RGB foreground color mode.
    #[wasm_bindgen(structural, method, js_name = isFgRGB)]
    pub fn is_fg_rgb(this: &BufferCell) -> bool;

    /// Whether the cell has the invisible attribute (CSI 8 m).
    #[wasm_bindgen(structural, method, js_name = isInvisible)]
    pub fn is_invisible(this: &BufferCell) -> u8;

    /// Whether the cell has the inverse attribute (CSI 7 m).
    #[wasm_bindgen(structural, method, js_name = isInverse)]
    pub fn is_inverse(this: &BufferCell) -> u8;

    /// Whether the cell has the italic attribute (CSI 3 m).
    #[wasm_bindgen(structural, method, js_name = isItalic)]
    pub fn is_italic(this: &BufferCell) -> u8;

    /// Whether the cell has the overline attribute (CSI 53 m).
    #[wasm_bindgen(structural, method, js_name = isOverline)]
    pub fn is_overline(this: &BufferCell) -> u8;

    /// Whether the cell has the strikethrough attribute (CSI 9 m).
    #[wasm_bindgen(structural, method, js_name = isStrikethrough)]
    pub fn is_strikethrough(this: &BufferCell) -> u8;

    /// Whether the cell has the underline attribute (CSI 4 m).
    #[wasm_bindgen(structural, method, js_name = isUnderline)]
    pub fn is_underline(this: &BufferCell) -> u8;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a line in the terminal's buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    pub type BufferLine;

    /// Whether the line is wrapped from the previous line.
    #[wasm_bindgen(structural, method, getter = isWrapped)]
    pub fn is_wrapped(this: &BufferLine) -> bool;

    /// The length of the line.
    ///
    /// All calls to [`BufferLine::get_cell`] beyond the length will result in
    /// `None`.
    ///
    /// [`BufferLine::get_cell`]: BufferLine::get_cell
    #[wasm_bindgen(structural, method, getter = length)]
    pub fn length(this: &BufferLine) -> u16;

    /// Gets a cell from the line, or undefined if the line index does not exist.
    ///
    /// Note that the result of this function should be used immediately after
    /// calling as when the terminal updates it could lead to unexpected
    /// behavior.
    ///
    /// Takes:
    ///   - `x`: The character index to get.
    ///   - `cell`: Optional cell object to load data into for performance
    ///             reasons. This is mainly useful when every cell in the buffer
    ///             is being looped over to avoid creating new objects for every
    ///             cell.
    #[wasm_bindgen(structural, method, js_name = getCell)]
    pub fn get_cell(this: &BufferLine, x: u16, cell: Option<BufferCell>) -> Option<BufferCell>;

    /// Gets the line as a string. Note that this is gets only the string for the
    /// line, not taking isWrapped into account.
    ///
    /// Takes:
    ///   - `trim_right`: Whether to trim any whitespace at the right of the line.
    ///   - `start_column`: The column to start from (inclusive).
    ///   - `end_column`: The column to end at (exclusive).
    #[wasm_bindgen(structural, method, js_name = translateToString)]
    pub fn translate_to_string(
        this: &BufferLine,
        trim_right: Option<bool>,
        start_column: Option<u16>,
        end_column: Option<u16>,
    ) -> Str;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents a terminal buffer.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type Buffer;

    /// Gets the line within the buffer where the top of the bottom page is
    /// (when fully scrolled down).
    #[wasm_bindgen(structural, method, getter = baseY)]
    pub fn base_y(this: &Buffer) -> u16;

    /// Gets the x position of the cursor. This ranges between 0 (left side) and
    /// [`Terminal::cols()`] (after last cell of the row).
    ///
    /// [`Terminal::cols()`]: Terminal::cols
    #[wasm_bindgen(structural, method, getter = cursorX)]
    pub fn cursor_x(this: &Buffer) -> u16;

    /// Gets the y position of the cursor. This ranges between 0 (when the
    /// cursor is at baseY) and [`Terminal::rows()`] - 1 (when the cursor is on
    /// the last row).
    ///
    /// [`Terminal::rows()`]: Terminal::rows
    #[wasm_bindgen(structural, method, getter = cursorY)]
    pub fn cursor_y(this: &Buffer) -> u16;

    /// Gets a line from the buffer, or undefined if the line index does not
    /// exist.
    ///
    /// Note that the result of this function should be used immediately after
    /// calling as when the terminal updates it could lead to unexpected
    /// behavior.
    ///
    /// Takes:
    ///   - `y`: The line index to get.
    #[wasm_bindgen(structural, method, js_name = getLine)]
    pub fn get_line(this: &Buffer, y: u16) -> Option<BufferLine>;

    /// Creates an empty cell object suitable as a cell reference in
    /// `line.getCell(x, cell)`. Use this to avoid costly recreation of
    /// cell objects when dealing with tons of cells.
    #[wasm_bindgen(structural, method, js_name = getNullCell)]
    pub fn get_null_cell(this: &Buffer) -> BufferCell;

    /// Gets the amount of lines in the buffer.
    #[wasm_bindgen(structural, method, getter = length)]
    pub fn length(this: &Buffer) -> u32;

    /// Gets the type of the buffer.
    #[wasm_bindgen(structural, method, getter = type)]
    pub fn buffer_type(this: &Buffer) -> BufferType;

    /// Gets the line within the buffer where the top of the viewport is.
    #[wasm_bindgen(structural, method, getter = viewportY)]
    pub fn viewport_y(this: &Buffer) -> u16;
}

#[wasm_bindgen(module = "xterm")]
extern "C" {
    /// Represents the terminal's set of buffers.
    ///
    /// (This is a [duck-typed interface]).
    ///
    /// [duck-typed interface]: https://rustwasm.github.io/docs/wasm-bindgen/reference/working-with-duck-typed-interfaces.html
    #[derive(Debug, Clone)]
    pub type BufferNamespace;

    /// Gets the active buffer, this will either be the normal or alternate
    /// buffers.
    #[wasm_bindgen(structural, method, getter = active)]
    pub fn active(this: &BufferNamespace) -> Buffer;

    /// Gets the alternate buffer, this becomes the active buffer when an
    /// application enters this mode via DECSET (`CSI ? 4 7 h`).
    #[wasm_bindgen(structural, method, getter = alternate)]
    pub fn alternate(this: &BufferNamespace) -> Buffer;

    /// Gets the normal buffer.
    #[wasm_bindgen(structural, method, getter = normal)]
    pub fn normal(this: &BufferNamespace) -> Buffer;

    /// Adds an event listener for when the active buffer changes.
    ///
    /// Returns a [`Disposable`] to stop listening.
    ///
    /// See [`attach_buffer_change_event_listener`] (if the `ext` feature is
    /// enabled) for a friendlier version of this function.
    ///
    /// [`attach_buffer_change_event_listener`]: BufferNamespace::attach_buffer_change_event_listener
    #[wasm_bindgen(structural, method, js_name = onBufferChange)]
    pub fn on_buffer_change(
        this: &BufferNamespace,
        listener: &Closure<dyn FnMut(Buffer)>,
    ) -> super::interfaces::Disposable;
}
