use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use xterm_js_sys::xterm::{LogLevel, Terminal, TerminalOptions, KeyEventData, ResizeEventData};
use xterm_js_sys::ext::TerminalOptionsExt;

#[path = "../../common.rs"]
mod common;
use common::log;

#[macro_export]
#[doc(hidden)]
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

pub(crate) const ENABLE_MOUSE_MODE_CSI_SEQUENCE: &str = concat!(
    csi!("?1000h"),
    csi!("?1002h"),
    csi!("?1015h"),
    csi!("?1006h")
);

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().ok_or("no global `window` exists")?;
    let document = window.document().ok_or("should have a document on window")?;
    let terminal_div = document
        .get_element_by_id("terminal")
        .ok_or("should have a terminal div")?
        .dyn_into::<web_sys::HtmlElement>()?;

    let options = TerminalOptions::default()
        .with_log_level(LogLevel::Debug)
        .with_font_size(11.0);

    let term_orig = Terminal::new(Some(options));

    term_orig.open(terminal_div);
    term_orig.write(ENABLE_MOUSE_MODE_CSI_SEQUENCE.to_string());

    let term = term_orig.clone();
    let key_listener = Closure::wrap(Box::new(move |e: KeyEventData| {
        // A port of the xterm.js echo demo:
        let key = e.key();
        let ev = e.dom_event();

        let printable = matches!(
            (ev.alt_key(), ev.ctrl_key(), ev.meta_key()),
            (false, false, false)
        );

        const ENTER_ASCII_KEY_CODE: u32 = 13;
        const BACKSPACE_ASCII_KEY_CODE: u32 = 8;

        match ev.key_code() {
            ENTER_ASCII_KEY_CODE => {
                term.write("\n\r\x1B[1;3;31m$ \x1B[0m".to_string())
            }
            BACKSPACE_ASCII_KEY_CODE => {
                term.write("\u{0008} \u{0008}".to_string())
            }
            _ if printable => term.write(key),
            _ => {}
        }

        log!("[key event] got {:?}", e);
    }) as Box<dyn FnMut(KeyEventData)>);
    let l = term_orig.on_key(&key_listener);

    let binary_listener = Closure::wrap(Box::new(move |s: String| {
        log!("[binary event] bin: {:?}", s);
    }) as Box<dyn FnMut(String)>);
    let b = term_orig.on_binary(&binary_listener);

    let data_listener = Closure::wrap(Box::new(move |s: String| {
        log!("[data event] data: {:?}", s);
    }) as Box<dyn FnMut(String)>);
    let d = term_orig.on_data(&data_listener);

    let resize_listener = Closure::wrap(Box::new(move |r: ResizeEventData| {
        log!("[resize event] resize: {:?}", r);
    }) as Box<dyn FnMut(ResizeEventData)>);
    let r = term_orig.on_resize(&resize_listener);

    // Store event listeners by leaking them - this is the simplest approach for WASM
    // where we don't have multiple threads to worry about
    std::mem::forget(key_listener);
    std::mem::forget(binary_listener);
    std::mem::forget(data_listener);
    std::mem::forget(resize_listener);
    std::mem::forget(l);
    std::mem::forget(b);
    std::mem::forget(d);
    std::mem::forget(r);

    let term = term_orig;

    term.focus();

    term.write("\x1B[35;31m hello!\n".to_string());
    term.write("\x1B[1;3;31mxterm.js\x1B[0m with 🦀\n$ ".to_string());

    Ok(())
}
