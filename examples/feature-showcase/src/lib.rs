//! Comprehensive Feature Showcase for xterm-js-sys v5.5.0
//! 
//! This example demonstrates all the new features and improvements added in v5.5.0:
//! - New methods: input(), writeln(), options property
//! - New events: onBell, onWriteParsed  
//! - Enhanced options with builder patterns
//! - Advanced features: custom wheel handlers, texture atlas management
//! - Marker and character joiner functionality
//! - Theme customization with builder patterns

use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use js_sys::Uint8Array;
use web_sys::{console, HtmlElement, HtmlButtonElement};

use xterm_js_sys::xterm::{
    LogLevel, Terminal, TerminalOptions, KeyEventData,
    CursorStyle, BellStyle, Theme, WindowsPty,
};
use xterm_js_sys::ext::{TerminalOptionsExt, ThemeExt};

macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn setTimeout(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearTimeout(token: f64);
}

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
    
    log!("🚀 xterm-js-sys v5.5.0 Feature Showcase Starting...");
    
    spawn_local(async {
        if let Err(e) = run_showcase().await {
            log!("❌ Error running showcase: {:?}", e);
        }
    });
}

async fn run_showcase() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    // Create the main terminal with enhanced options using builder pattern
    let terminal = create_enhanced_terminal(&document)?;
    
    // Set up all the new event handlers
    setup_event_handlers(&terminal)?;
    
    // Create control panel for interactive features
    create_control_panel(&document, &terminal)?;
    
    // Start the feature demonstration
    start_feature_demo(&terminal).await?;
    
    Ok(())
}

fn create_enhanced_terminal(document: &web_sys::Document) -> Result<Terminal, JsValue> {
    let terminal_div = document
        .get_element_by_id("terminal")
        .ok_or("Terminal div not found")?
        .dyn_into::<HtmlElement>()?;

    // 🆕 NEW: Enhanced TerminalOptions with builder pattern and new v5.5.0 options
    let options = TerminalOptions::default()
        .with_log_level(LogLevel::Info)
        .with_font_size(14.0)
        .with_cols(120)
        .with_rows(30)
        .with_cursor_style(CursorStyle::Block)
        .with_cursor_blink(true)
        .with_bell_style(BellStyle::Sound)
        .with_scrollback(10000)
        .with_tab_stop_width(4);
    
    // Set additional new v5.5.0 options
    let mut enhanced_options = options;
    enhanced_options.allow_proposed_api = Some(true);
    enhanced_options.cursor_inactive_style = Some(xterm_js_sys::xterm::CursorInactiveStyle::Outline);
    enhanced_options.custom_glyphs = Some(true);
    enhanced_options.minimum_contrast_ratio = Some(4.5);
    enhanced_options.windows_pty = Some(WindowsPty::Auto);
    enhanced_options.alt_click_moves_cursor = Some(true);
    enhanced_options.fast_scroll_modifier = Some(xterm_js_sys::xterm::FastScrollModifier::Alt);
    enhanced_options.fast_scroll_sensitivity = Some(5.0);
    enhanced_options.ignore_bracketed_paste_mode = Some(false);
    
    // 🆕 NEW: Enhanced Theme with builder pattern
    let theme = Theme::default()
        .with_foreground("#f8f8f2")
        .with_background("#282a36")
        .with_cursor("#f8f8f0")
        .with_black("#21222c")
        .with_red("#ff5555")
        .with_green("#50fa7b")
        .with_yellow("#f1fa8c")
        .with_blue("#bd93f9")
        .with_magenta("#ff79c6")
        .with_cyan("#8be9fd")
        .with_white("#f8f8f2");
    
    enhanced_options.set_theme(Some(theme));
    
    let terminal = Terminal::new(Some(enhanced_options));
    terminal.open(terminal_div);
    
    log!("✅ Terminal created with enhanced v5.5.0 options");
    
    Ok(terminal)
}

fn setup_event_handlers(terminal: &Terminal) -> Result<(), JsValue> {
    let term = terminal.clone();
    
    // Standard event handlers
    let key_listener = Closure::wrap(Box::new(move |e: KeyEventData| {
        let key = e.key();
        let dom_event = e.dom_event();
        
        if dom_event.ctrl_key() && key == "c" {
            log!("🔑 Ctrl+C detected");
            return;
        }
        
        // Echo the key
        term.write(key);
    }) as Box<dyn FnMut(KeyEventData)>);
    
    let key_disposable = terminal.on_key(&key_listener);
    
    // 🆕 NEW: onBell event handler (v5.5.0)
    let bell_listener = Closure::wrap(Box::new(move || {
        log!("🔔 Bell event triggered!");
        // You could play a custom sound here
    }) as Box<dyn FnMut()>);
    
    let bell_disposable = terminal.on_bell(&bell_listener);
    
    // 🆕 NEW: onWriteParsed event handler (v5.5.0)
    let write_parsed_listener = Closure::wrap(Box::new(move || {
        log!("📝 Write parsed event - content was written and parsed");
    }) as Box<dyn FnMut()>);
    
    let write_parsed_disposable = terminal.on_write_parsed(&write_parsed_listener);
    
    // Store listeners to prevent them from being dropped
    std::mem::forget(key_listener);
    std::mem::forget(bell_listener);
    std::mem::forget(write_parsed_listener);
    std::mem::forget(key_disposable);
    std::mem::forget(bell_disposable);
    std::mem::forget(write_parsed_disposable);
    
    log!("✅ Event handlers set up (including new v5.5.0 events)");
    
    Ok(())
}

fn create_control_panel(document: &web_sys::Document, terminal: &Terminal) -> Result<(), JsValue> {
    let controls_div = document
        .get_element_by_id("controls")
        .ok_or("Controls div not found")?;
    
    // 🆕 NEW: input() method demo button
    let input_button = document.create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    input_button.set_text_content(Some("Test input() method"));
    input_button.set_class_name("btn");
    
    let term_for_input = terminal.clone();
    let input_callback = Closure::wrap(Box::new(move || {
        // 🆕 NEW: input() method for programmatic input simulation
        term_for_input.input("echo 'Hello from input() method!'\n".to_string());
        log!("✅ Used new input() method");
    }) as Box<dyn FnMut()>);
    
    input_button.set_onclick(Some(input_callback.as_ref().unchecked_ref()));
    controls_div.append_child(&input_button)?;
    
    // 🆕 NEW: writeln() method demo button
    let writeln_button = document.create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    writeln_button.set_text_content(Some("Test writeln() method"));
    writeln_button.set_class_name("btn");
    
    let term_for_writeln = terminal.clone();
    let writeln_callback = Closure::wrap(Box::new(move || {
        // 🆕 NEW: writeln() method (write + newline)
        term_for_writeln.writeln("This is written with writeln() method! 📝".to_string());
        log!("✅ Used new writeln() method");
    }) as Box<dyn FnMut()>);
    
    writeln_button.set_onclick(Some(writeln_callback.as_ref().unchecked_ref()));
    controls_div.append_child(&writeln_button)?;
    
    // Store callbacks
    std::mem::forget(input_callback);
    std::mem::forget(writeln_callback);
    
    // 🆕 NEW: options property demo button
    let options_button = document.create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    options_button.set_text_content(Some("Test options property"));
    options_button.set_class_name("btn");

    let term_for_options = terminal.clone();
    let options_callback = Closure::wrap(Box::new(move || {
        // 🆕 NEW: options property getter/setter
        let current_options = term_for_options.options();
        log!("📋 Current font size: {:?}", current_options.font_size);

        // Modify options
        let mut new_options = current_options;
        new_options.font_size = Some(16.0);
        term_for_options.set_options(new_options);

        log!("✅ Used new options property getter/setter");
    }) as Box<dyn FnMut()>);

    options_button.set_onclick(Some(options_callback.as_ref().unchecked_ref()));
    controls_div.append_child(&options_button)?;

    // 🆕 NEW: clearTextureAtlas() method demo button
    let clear_atlas_button = document.create_element("button")?
        .dyn_into::<HtmlButtonElement>()?;
    clear_atlas_button.set_text_content(Some("Clear Texture Atlas"));
    clear_atlas_button.set_class_name("btn");

    let term_for_atlas = terminal.clone();
    let atlas_callback = Closure::wrap(Box::new(move || {
        // 🆕 NEW: clearTextureAtlas() method for canvas renderer optimization
        term_for_atlas.clear_texture_atlas();
        log!("✅ Used new clearTextureAtlas() method");
    }) as Box<dyn FnMut()>);

    clear_atlas_button.set_onclick(Some(atlas_callback.as_ref().unchecked_ref()));
    controls_div.append_child(&clear_atlas_button)?;

    // Store callbacks
    std::mem::forget(options_callback);
    std::mem::forget(atlas_callback);

    log!("✅ Control panel created with new method demos");

    Ok(())
}

async fn start_feature_demo(terminal: &Terminal) -> Result<(), JsValue> {
    // Welcome message
    terminal.writeln("🚀 Welcome to xterm-js-sys v5.5.0 Feature Showcase!".to_string());
    terminal.writeln("".to_string());
    terminal.writeln("This demo showcases all the new features added in v5.5.0:".to_string());
    terminal.writeln("".to_string());

    // Demonstrate new methods
    terminal.writeln("📝 NEW METHODS:".to_string());
    terminal.writeln("• input() - Programmatic input simulation".to_string());
    terminal.writeln("• writeln() - Write with automatic newline".to_string());
    terminal.writeln("• options property - Direct options access".to_string());
    terminal.writeln("• clearTextureAtlas() - Canvas optimization".to_string());
    terminal.writeln("".to_string());

    // Demonstrate new options
    terminal.writeln("⚙️ NEW OPTIONS:".to_string());
    terminal.writeln("• allowProposedApi, cursorInactiveStyle, customGlyphs".to_string());
    terminal.writeln("• minimumContrastRatio, windowsPty, altClickMovesCursor".to_string());
    terminal.writeln("• fastScrollModifier, fastScrollSensitivity".to_string());
    terminal.writeln("".to_string());

    // Demonstrate new events
    terminal.writeln("🔔 NEW EVENTS:".to_string());
    terminal.writeln("• onBell - Bell notification events".to_string());
    terminal.writeln("• onWriteParsed - Write parsing completion events".to_string());
    terminal.writeln("".to_string());

    // Demonstrate builder patterns
    terminal.writeln("🏗️ BUILDER PATTERNS:".to_string());
    terminal.writeln("• TerminalOptions with fluent .with_*() methods".to_string());
    terminal.writeln("• Theme with fluent .with_*() methods".to_string());
    terminal.writeln("".to_string());

    // Interactive instructions
    terminal.writeln("🎮 INTERACTIVE FEATURES:".to_string());
    terminal.writeln("• Use the buttons above to test new methods".to_string());
    terminal.writeln("• Type to see enhanced event handling".to_string());
    terminal.writeln("• Press Ctrl+C to see key detection".to_string());
    terminal.writeln("".to_string());

    // Demonstrate enhanced write method with Uint8Array
    terminal.writeln("🔢 ENHANCED WRITE METHOD:".to_string());
    let binary_data = Uint8Array::new_with_length(5);
    binary_data.set_index(0, 72);  // 'H'
    binary_data.set_index(1, 101); // 'e'
    binary_data.set_index(2, 108); // 'l'
    binary_data.set_index(3, 108); // 'l'
    binary_data.set_index(4, 111); // 'o'

    terminal.write_uint8_array(&binary_data);
    terminal.writeln(" <- Written using Uint8Array!".to_string());
    terminal.writeln("".to_string());

    // Demonstrate marker functionality
    terminal.writeln("📍 MARKER FUNCTIONALITY:".to_string());
    let marker = terminal.register_marker(None);
    if let Some(marker) = marker {
        terminal.writeln(format!("• Marker created at line: {}", marker.line()));
        terminal.writeln("• Markers help track specific lines".to_string());
    }
    terminal.writeln("".to_string());

    // Show current modes
    terminal.writeln("🔧 TERMINAL MODES:".to_string());
    let modes = terminal.modes();
    terminal.writeln(format!("• Application cursor keys: {:?}", modes.application_cursor_keys_mode));
    terminal.writeln(format!("• Application keypad: {:?}", modes.application_keypad_mode));
    terminal.writeln(format!("• Bracketed paste: {:?}", modes.bracketed_paste_mode));
    terminal.writeln("".to_string());

    terminal.writeln("✨ Ready for interaction! Try the buttons above or start typing...".to_string());
    terminal.write("$ ".to_string());

    log!("✅ Feature demonstration completed");

    Ok(())
}
