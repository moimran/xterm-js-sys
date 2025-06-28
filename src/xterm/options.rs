//! Terminal configuration and options for xterm.js bindings.

use super::types::*;
use wasm_bindgen::prelude::*;

macro_rules! wasm_struct {
    (
        $(#[constructor::skip = $const_skip_reason:literal])?
        #[wasm_bindgen $(( $($wb_opts:tt)* ))? ]
        $(#[$metas:meta])*
        pub struct $nom:ident {
            $(
                $(#[doc = $docs_field:literal])*
                $(#[wasm_bindgen($($wasm_opt:ident$( = $wasm_val:tt)?),+)])?
                // $(#[$metas_field:meta])*
                $(#[deprecated($($depr:tt)+)])?
                $(pub $field:ident: $field_ty:ty)?
                $(|
                    clone(
                        set = $set:ident,
                        js_name = $js_name:ident
                        $(, pub = $public:ident)?
                    )
                    $priv_field:ident: $priv_field_ty:ty
                )?
                ,
            )+
        }
    ) => {
        #[wasm_bindgen $(( $($wb_opts)* ))? ]
        $(#[$metas])*
        pub struct $nom {
            $(
                $(#[doc = $docs_field])*
                $(#[wasm_bindgen($($wasm_opt$( = $wasm_val)?),+)])?
                // $(#[$metas_field])*
                $(#[deprecated($($depr)+)])?
                $(pub $field: $field_ty)?
                $(
                    $(
                       #[doc = $public]
                       #[wasm_bindgen(skip)] pub
                    )?
                    pub(in super) $priv_field: $priv_field_ty
                )?
                ,
            )+
        }

        $(
            #[cfg(__never__)]
            #[doc = $const_skip_reason]
        )?
        impl $nom {
            #[doc = "Constructor."]
            #[allow(deprecated, clippy::too_many_arguments)]
            #[must_use]
            pub const fn new(
                $(
                    $($field: $field_ty,)?
                    $($priv_field: $priv_field_ty,)?
                )+
            ) -> Self {
                Self {
                    $(
                        $($field,)?
                        $($priv_field,)?
                    )+
                }
            }
        }

        #[wasm_bindgen]
        impl $nom {
            $(
                $(#[doc = $docs_field])*

                // Some garbage to swallow the doc comment in case we're not
                // dealing with a private field:
                $(
                    #[allow(unused_doc_comments)]
                    #[cfg(__never__)]
                    fn $field() -> () { }
                )?

                $(
                    #[doc = "\n\nGetter."]
                    #[wasm_bindgen(getter = $js_name)]
                    #[must_use]
                    pub fn $priv_field(&self) -> $priv_field_ty {
                        self.$priv_field.clone()
                    }
                )?

                $(#[doc = $docs_field])*

                // Again: garbage to swallow the doc comment.
                $(
                    #[allow(unused_doc_comments)]
                    #[cfg(__never__)]
                    fn $field() -> () { }
                )?

                $(
                    #[doc = "\n\nSetter."]
                    #[wasm_bindgen(setter = $js_name)]
                    pub fn $set(&mut self, $priv_field: $priv_field_ty) {
                        self.$priv_field = $priv_field;
                    }
                )?
            )*
        }
    };
}

wasm_struct! {
#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone)]
/// Data type to register a `CSI`, `DCS`, or `ESC` callback in the parser in the
/// form:
///   - ESC I..I F
///   - CSI Prefix P..P I..I F
///   - DCS Prefix P..P I..I F data_bytes ST
///
/// with these rules/restrictions:
///   - prefix can only be used with `CSI` and `DCS`
///   - only one leading prefix byte is recognized by the parser before any
///     other parameter bytes (P..P)
///   - intermediate bytes are recognized up to 2
///
/// For custom sequences make sure to read ECMA-48 and the resources at
/// vt100.net to not clash with existing sequences or reserved address space.
/// General recommendations:
///   - use private address space (see ECMA-48)
///   - use max one intermediate byte (technically not limited by the spec,
///     in practice there are no sequences with more than one intermediate byte,
///     thus parsers might get confused with more intermediates)
///   - test against other common emulators to check whether they escape/ignore
///     the sequence correctly
///
/// Notes: OSC command registration is handled differently (see addOscHandler)
/// APC, PM or SOS is currently not supported.
pub struct FunctionIdentifier {
    /// Optional prefix byte, must be in range \\x3c .. \\x3f.
    /// Usable in CSI and DCS.
    |clone(set = set_prefix, js_name = prefix)
    prefix: Option<Str>,

    /// Optional intermediate bytes, must be in range \\x20 .. \\x2f.
    /// Usable in CSI, DCS and ESC.
    |clone(set = set_intermediates, js_name = intermediates)
    intermediates: Option<Str>,

    /// Final byte, must be in range \\x40 .. \\x7e for CSI and DCS,
    /// \\x30 .. \\x7e for ESC.
    |clone(set = set_final, js_name = final)
    final_byte: Str,
}}

wasm_struct! {
#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, PartialEq, Default)]
/// Contains colors to theme the terminal with.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct Theme {
    /// The default foreground color.
    |clone(set = set_foreground, js_name = foreground)
    foreground: Option<Str>,

    /// The default background color.
    |clone(set = set_background, js_name = background)
    background: Option<Str>,

    /// The cursor color.
    |clone(set = set_cursor, js_name = cursor)
    cursor: Option<Str>,

    /// The accent color of the cursor (fg color for a block cursor).
    |clone(set = set_cursor_accent, js_name = cursorAccent)
    cursor_accent: Option<Str>,

    /// The selection background color (can be transparent).
    |clone(set = set_selection_background, js_name = selectionBackground)
    selection_background: Option<Str>,

    /// The selection foreground color.
    |clone(set = set_selection_foreground, js_name = selectionForeground)
    selection_foreground: Option<Str>,

    /// The selection background color when the terminal does not have focus (can be transparent).
    |clone(set = set_selection_inactive_background, js_name = selectionInactiveBackground)
    selection_inactive_background: Option<Str>,

    /// ANSI black (eg. `\\x1b[30m`).
    |clone(set = set_black, js_name = black)
    black: Option<Str>,

    /// ANSI red (eg. `\\x1b[31m`).
    |clone(set = set_red, js_name = red)
    red: Option<Str>,

    /// ANSI green (eg. `\\x1b[32m`).
    |clone(set = set_green, js_name = green)
    green: Option<Str>,

    /// ANSI yellow (eg. `\\x1b[33m`).
    |clone(set = set_yellow, js_name = yellow)
    yellow: Option<Str>,

    /// ANSI blue (eg. `\\x1b[34m`).
    |clone(set = set_blue, js_name = blue)
    blue: Option<Str>,

    /// ANSI magenta (eg. `\\x1b[35m`).
    |clone(set = set_magenta, js_name = magenta)
    magenta: Option<Str>,

    /// ANSI cyan (eg. `\\x1b[36m`).
    |clone(set = set_cyan, js_name = cyan)
    cyan: Option<Str>,

    /// ANSI white (eg. `\\x1b[37m`).
    |clone(set = set_white, js_name = white)
    white: Option<Str>,

    /// ANSI bright black (eg. `\\x1b[1;30m`).
    |clone(set = set_bright_black, js_name = brightBlack)
    bright_black: Option<Str>,

    /// ANSI bright red (eg. `\\x1b[1;31m`).
    |clone(set = set_bright_red, js_name = brightRed)
    bright_red: Option<Str>,

    /// ANSI bright green (eg. `\\x1b[1;32m`).
    |clone(set = set_bright_green, js_name = brightGreen)
    bright_green: Option<Str>,

    /// ANSI bright yellow (eg. `\\x1b[1;33m`).
    |clone(set = set_bright_yellow, js_name = brightYellow)
    bright_yellow: Option<Str>,

    /// ANSI bright blue (eg. `\\x1b[1;34m`).
    |clone(set = set_bright_blue, js_name = brightBlue)
    bright_blue: Option<Str>,

    /// ANSI bright magenta (eg. `\\x1b[1;35m`).
    |clone(set = set_bright_magenta, js_name = brightMagenta)
    bright_magenta: Option<Str>,

    /// ANSI bright cyan (eg. `\\x1b[1;36m`).
    |clone(set = set_bright_cyan, js_name = brightCyan)
    bright_cyan: Option<Str>,

    /// ANSI bright white (eg. `\\x1b[1;37m`).
    |clone(set = set_bright_white, js_name = brightWhite)
    bright_white: Option<Str>,

    /// ANSI extended colors (16-255).
    |clone(set = set_extended_ansi, js_name = extendedAnsi)
    extended_ansi: Option<Vec<Str>>,
}}

wasm_struct! {
#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Enable various window manipulation and report features (`CSI Ps ; Ps ; Ps
/// t`).
///
/// Most settings have no default implementation, as they heavily rely on the
/// embedding environment.
///
/// To implement a feature, create a custom CSI hook like this:
///
/// ```ts
/// term.parser.addCsiHandler({final: 't'}, params => {
///   const ps = params[0];
///   switch (ps) {
///     case XY:
///       ...            // your implementation for option XY
///       return true;   // signal Ps=XY was handled
///   }
///   return false;      // any Ps that was not handled
/// });
/// ```
///
/// Note on security: Most features are meant to deal with some information of
/// the host machine where the terminal runs on. This is seen as a security risk
/// possibly leaking sensitive data of the host to the program in the terminal.
/// Therefore all options (even those without a default implementation) are
/// guarded by the boolean flag and disabled by default.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct WindowOptions {
    /// Ps=10 ; 0 Undo full-screen mode. Ps=10 ; 1 Change to full-screen. Ps=10
    /// ; 2 Toggle full-screen.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = fullscreenWin)]
    pub fullscreen_win: Option<bool>,

    /// Ps=16 Report xterm character cell size in pixels. Result is "CSI 6 ;
    /// height ; width t".
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getCellSizePixels)]
    pub get_cell_size_pixels: Option<bool>,

    /// Ps=20 Report xterm window's icon label. Result is "OSC L label ST".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getIconTitle)]
    pub get_icon_title: Option<bool>,

    /// Ps=19 Report the size of the screen in characters. Result is "CSI 9 ;
    /// height ; width t".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getScreenSizeChars)]
    pub get_screen_size_chars: Option<bool>,

    /// Ps=15 Report size of the screen in pixels. Result is "CSI 5 ; height ;
    /// width t".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getScreenSizePixels)]
    pub get_screen_size_pixels: Option<bool>,

    /// Ps=13 Report xterm window position. Result is "CSI 3 ; x ; y t". Ps=13 ;
    /// 2 Report xterm text-area position. Result is "CSI 3 ; x ; y t".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinPosition)]
    pub get_win_position: Option<bool>,

    /// Ps=18 Report the size of the text area in characters. Result is "CSI 8 ;
    /// height ; width t".
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getWinSizeChars)]
    pub get_win_size_chars: Option<bool>,

    /// Ps=14 Report xterm text area size in pixels. Result is "CSI 4 ; height ;
    /// width t". Ps=14 ; 2 Report xterm window size in pixels. Result is "CSI 4
    /// ; height ; width t".
    ///
    /// Has a default implementation.
    #[wasm_bindgen(js_name = getWinSizePixels)]
    pub get_win_size_pixels: Option<bool>,

    /// Ps=11 Report xterm window state. If the xterm window is non-iconified, it
    /// returns "CSI 1 t". If the xterm window is iconified, it returns "CSI 2 t".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinState)]
    pub get_win_state: Option<bool>,

    /// Ps=21 Report xterm window's title. Result is "OSC l label ST".
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = getWinTitle)]
    pub get_win_title: Option<bool>,

    /// Ps=6 Lower the xterm window to the bottom of the stacking order.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = lowerWin)]
    pub lower_win: Option<bool>,

    /// Ps=9 ; 0 Restore maximized window. Ps=9 ; 1 Maximize window (i.e., resize
    /// to screen size). Ps=9 ; 2 Maximize window vertically. Ps=9 ; 3 Maximize
    /// window horizontally.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = maximizeWin)]
    pub maximize_win: Option<bool>,

    /// Ps=2 Iconify window.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = minimizeWin)]
    pub minimize_win: Option<bool>,

    /// Ps=22 ; 0 Save xterm icon and window title on stack. Ps=22 ; 1 Save xterm
    /// icon title on stack. Ps=22 ; 2 Save xterm window title on stack.
    ///
    /// All variants have a default implementation.
    #[wasm_bindgen(js_name = popTitle)]
    pub pop_title: Option<bool>,

    /// Ps=23 ; 0 Restore xterm icon and window title from stack. Ps=23 ; 1
    /// Restore xterm icon title from stack. Ps=23 ; 2 Restore xterm window title
    /// from stack.
    ///
    /// All variants have a default implementation.
    #[wasm_bindgen(js_name = pushTitle)]
    pub push_title: Option<bool>,

    /// Ps=5 Raise the window to the front of the stacking order.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = raiseWin)]
    pub raise_win: Option<bool>,

    /// Ps=7 Refresh the window.
    #[wasm_bindgen(js_name = refreshWin)]
    pub refresh_win: Option<bool>,

    /// Ps=1 De-iconify window.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = restoreWin)]
    pub restore_win: Option<bool>,

    /// Ps>=24 Resize to Ps lines (DECSLPP). DECSLPP is not implemented. This
    /// settings is also used to enable / disable DECCOLM (earlier variant of
    /// DECSLPP).
    #[wasm_bindgen(js_name = setWinLines)]
    pub set_win_lines: Option<bool>,

    /// Ps=3 ; x ; y Move window to [x, y].
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinPosition)]
    pub set_win_position: Option<bool>,

    /// Ps = 8 ; height ; width Resize the text area to given height and width in
    /// characters. Omitted parameters should reuse the current height or width.
    /// Zero parameters use the display's height or width.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinSizeChars)]
    pub set_win_size_chars: Option<bool>,

    /// Ps = 4 ; height ; width Resize the window to given `height` and `width`
    /// in pixels. Omitted parameters should reuse the current height or width.
    /// Zero parameters should use the display's height or width.
    ///
    /// No default implementation.
    #[wasm_bindgen(js_name = setWinSizePixels)]
    pub set_win_size_pixels: Option<bool>,
}}

wasm_struct! {
#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, PartialEq, Default)]
/// Terminal modes as set by SM/DECSET.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct Modes {
    /// Application Cursor Keys (DECCKM): `CSI ? 1 h`
    #[wasm_bindgen(js_name = applicationCursorKeysMode)]
    pub application_cursor_keys_mode: Option<bool>,

    /// Application Keypad Mode (DECNKM): `CSI ? 66 h`
    #[wasm_bindgen(js_name = applicationKeypadMode)]
    pub application_keypad_mode: Option<bool>,

    /// Bracketed Paste Mode: `CSI ? 2004 h`
    #[wasm_bindgen(js_name = bracketedPasteMode)]
    pub bracketed_paste_mode: Option<bool>,

    /// Insert Mode (IRM): `CSI 4 h`
    #[wasm_bindgen(js_name = insertMode)]
    pub insert_mode: Option<bool>,

    /// Mouse Tracking mode. This can be one of the following:
    /// - none: This is the default value and can be reset with DECRST
    /// - x10: Send Mouse X & Y on button press `CSI ? 9 h`
    /// - vt200: Send Mouse X & Y on button press and release `CSI ? 1000 h`
    /// - drag: Use Cell Motion Mouse Tracking `CSI ? 1002 h`
    /// - any: Use All Motion Mouse Tracking `CSI ? 1003 h`
    #[wasm_bindgen(js_name = mouseTrackingMode)]
    pub mouse_tracking_mode: Option<MouseTrackingMode>,

    /// Origin Mode (DECOM): `CSI ? 6 h`
    #[wasm_bindgen(js_name = originMode)]
    pub origin_mode: Option<bool>,

    /// Reverse-wraparound Mode: `CSI ? 45 h`
    #[wasm_bindgen(js_name = reverseWraparoundMode)]
    pub reverse_wraparound_mode: Option<bool>,

    /// Send FocusIn/FocusOut events: `CSI ? 1004 h`
    #[wasm_bindgen(js_name = sendFocusMode)]
    pub send_focus_mode: Option<bool>,

    /// Auto-Wrap Mode (DECAWM): `CSI ? 7 h`
    #[wasm_bindgen(js_name = wraparoundMode)]
    pub wraparound_mode: Option<bool>,
}}

wasm_struct! {
#[wasm_bindgen(inspectable)]
#[derive(Debug, Clone, PartialEq, Default)]
/// An object containing start up options for the terminal.
///
/// (This is really an interface, but we just go and define our own type that
/// satisfies the interface.)
pub struct TerminalOptions {
    /// Whether to allow the use of proposed API. When false, any usage of APIs
    /// marked as experimental/proposed will throw an error. This defaults to
    /// true currently, but will change to false in v5.0.
    #[wasm_bindgen(js_name = allowProposedApi)]
    pub allow_proposed_api: Option<bool>,

    /// Whether background should support non-opaque color. It must be set
    /// before executing the [`Terminal::open`] method and can't be changed
    /// later without executing it again. Note that enabling this can negatively
    /// impact performance.
    ///
    /// [`Terminal::open()`]: Terminal::open
    #[wasm_bindgen(js_name = allowTransparency)]
    pub allow_transparency: Option<bool>,

    /// If enabled, alt + click will move the prompt cursor to position
    /// underneath the mouse. The default is true.
    #[wasm_bindgen(js_name = altClickMovesCursor)]
    pub alt_click_moves_cursor: Option<bool>,

    /// A data uri of the sound to use for the bell when
    /// [`TerminalOptions.bell_style`] = 'sound'.
    |clone(set = set_bell_sound, js_name = bellSound)
    bell_sound: Option<Str>,

    /// The type of the bell notification the terminal will use.
    #[wasm_bindgen(js_name = bellStyle)]
    pub bell_style: Option<BellStyle>,

    /// The number of columns in the terminal.
    #[wasm_bindgen(js_name = cols)]
    pub cols: Option<u16>,

    /// When enabled the cursor will be set to the beginning of the next line
    /// with every new line. This is equivalent to sending '\r\n' for each '\n'.
    /// Normally the termios settings of the underlying PTY deals with the
    /// translation of '\n' to '\r\n' and this setting should not be used. If
    /// you deal with data from a non-PTY related source, this settings might be
    /// useful.
    #[wasm_bindgen(js_name = convertEol)]
    pub convert_eol: Option<bool>,

    /// Whether the cursor blinks.
    #[wasm_bindgen(js_name = cursorBlink)]
    pub cursor_blink: Option<bool>,

    /// The style of the cursor.
    #[wasm_bindgen(js_name = cursorStyle)]
    pub cursor_style: Option<CursorStyle>,

    /// The width of the cursor in CSS pixels when [`cursor_style`] is set to
    /// ['bar'].
    ///
    /// [`cursor_style`]: TerminalOptions.cursor_style
    /// ['bar']: CursorStyle::Bar
    #[wasm_bindgen(js_name = cursorWidth)]
    pub cursor_width: Option<f32>,

    /// The style of the cursor when the terminal is not focused.
    #[wasm_bindgen(js_name = cursorInactiveStyle)]
    pub cursor_inactive_style: Option<CursorInactiveStyle>,

    /// Whether to enable custom glyphs for box drawing characters.
    #[wasm_bindgen(js_name = customGlyphs)]
    pub custom_glyphs: Option<bool>,

    /// Whether input should be disabled.
    #[wasm_bindgen(js_name = disableStdin)]
    pub disable_stdin: Option<bool>,

    /// Whether to draw bold text in bright colors. The default is true.
    #[wasm_bindgen(js_name = drawBoldTextInBrightColors)]
    pub draw_bold_text_in_bright_colors: Option<bool>,

    /// The modifier key hold to multiply scroll speed.
    #[wasm_bindgen(js_name = fastScrollModifier)]
    pub fast_scroll_modifier: Option<FastScrollModifier>,

    /// The scroll speed multiplier used for fast scrolling.
    #[wasm_bindgen(js_name = fastScrollSensitivity)]
    pub fast_scroll_sensitivity: Option<f32>,

    /// The font family used to render text.
    |clone(set = set_font_family, js_name = fontFamily)
    font_family: Option<Str>,

    /// The font size used to render text.
    #[wasm_bindgen(js_name = fontSize)]
    pub font_size: Option<f32>,

    /// The font weight used to render non-bold text.
    #[wasm_bindgen(js_name = fontWeight)]
    pub font_weight: Option<f32>,

    /// The font weight used to render bold text.
    #[wasm_bindgen(js_name = fontWeightBold)]
    pub font_weight_bold: Option<FontWeight>,

    /// Whether to ignore the bracketed paste mode. When true, this will always
    /// paste without the `\x1b[200~` and `\x1b[201~` sequences, even when the
    /// shell enables bracketed mode.
    #[wasm_bindgen(js_name = ignoreBracketedPasteMode)]
    pub ignore_bracketed_paste_mode: Option<bool>,

    /// The spacing in whole pixels between characters.
    #[wasm_bindgen(js_name = letterSpacing)]
    pub letter_spacing: Option<f32>,

    /// The line height used to render text.
    #[wasm_bindgen(js_name = lineHeight)]
    pub line_height: Option<f32>,

    /// The duration in milliseconds to hover over a link before showing the
    /// tooltip.
    #[wasm_bindgen(js_name = linkTooltipHoverDuration)]
    pub link_tooltip_hover_duration: Option<u16>,

    /// What log level to use, this will log for all levels below and including
    /// what is set:
    ///  1) debug
    ///  2) info __(default)__
    ///  3) warn
    ///  4) error
    ///  5) off
    #[wasm_bindgen(js_name = logLevel)]
    pub log_level: Option<LogLevel>,

    /// Whether holding a modifier key will force normal selection behavior,
    /// regardless of whether the terminal is in mouse events mode. This will
    /// also prevent mouse events from being emitted by the terminal. For
    /// example, this allows you to use xterm.js' regular selection inside tmux
    /// with mouse mode enabled.
    #[wasm_bindgen(js_name = macOptionClickForcesSelection)]
    pub mac_option_click_forces_selection: Option<bool>,

    /// Whether to treat option as the meta key.
    #[wasm_bindgen(js_name = macOptionIsMeta)]
    pub mac_option_is_meta: Option<bool>,

    /// The minimum contrast ratio for text in the terminal, setting this will
    /// change the foreground color dynamically depending on whether the
    /// contrast ratio is met. Example values:
    ///   - 1: The default, do nothing.
    ///   - 4.5: Minimum for WCAG AA compliance.
    ///   - 7: Minimum for WCAG AAA compliance.
    ///   - 21: White on black or black on white.
    #[wasm_bindgen(js_name = minimumContrastRatio)]
    pub minimum_contrast_ratio: Option<f32>,

    /// The type of renderer to use, this allows using the fallback DOM renderer
    /// when canvas is too slow for the environment. The following features do
    /// not work when the DOM renderer is used:
    ///
    ///   - Letter spacing
    ///   - Cursor blinking
    #[wasm_bindgen(js_name = rendererType)]
    pub renderer_type: Option<RendererType>,

    /// Whether to select the word under the cursor on right click, this is
    /// standard behavior in a lot of macOS applications.
    #[wasm_bindgen(js_name = rightClickSelectsWord)]
    pub right_click_selects_word: Option<bool>,

    /// The number of rows in the terminal.
    #[wasm_bindgen(js_name = rows)]
    pub rows: Option<u16>,

    /// Whether screen reader support is enabled. When on this will expose
    /// supporting elements in the DOM to support NVDA on Windows and VoiceOver
    /// on macOS.
    #[wasm_bindgen(js_name = screenReaderMode)]
    pub screen_reader_mode: Option<bool>,

    /// The scrolling speed multiplier used for adjusting normal scrolling
    /// speed.
    #[wasm_bindgen(js_name = scrollSensitivity)]
    pub scroll_sensitivity: Option<f32>,

    /// The amount of scrollback in the terminal. Scrollback is the amount of
    /// rows that are retained when lines are scrolled beyond the initial
    /// viewport.
    #[wasm_bindgen(js_name = scrollback)]
    pub scrollback: Option<u32>,

    /// The size of tab stops in the terminal.
    #[wasm_bindgen(js_name = tabStopWidth)]
    pub tab_stop_width: Option<u16>,

    /// The color theme of the terminal.
    |clone(set = set_theme, js_name = theme)
    theme: Option<Theme>,

    /// Enable various window manipulation and report features. All features are
    /// disabled by default for security reasons.
    |clone(set = set_window_options, js_name = windowOptions)
    window_options: Option<WindowOptions>,

    /// Whether "Windows mode" is enabled. Because Windows backends winpty and
    /// conpty operate by doing line wrapping on their side, xterm.js does not
    /// have access to wrapped lines. When Windows mode is enabled the following
    /// changes will be in effect:
    ///
    ///   - Reflow is disabled.
    ///   - Lines are assumed to be wrapped if the last character of the line is
    ///     not whitespace.
    ///
    /// **Deprecated**: Use windowsPty instead.
    #[wasm_bindgen(js_name = windowsMode)]
    pub windows_mode: Option<bool>,

    /// The Windows PTY backend to use. This will override the `windows_mode` option.
    /// When set to 'auto', the backend will be automatically detected based on the
    /// environment. When set to 'conpty', the ConPTY backend will be used. When set
    /// to 'winpty', the WinPTY backend will be used.
    #[wasm_bindgen(js_name = windowsPty)]
    pub windows_pty: Option<WindowsPty>,

    /// A string containing all characters that are considered word separated by
    /// the double click to select work logic.
    |clone(set = set_word_separator, js_name = wordSeparator)
    word_separator: Option<Str>,
}}
