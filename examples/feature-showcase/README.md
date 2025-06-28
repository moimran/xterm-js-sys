# xterm-js-sys v5.5.0 Feature Showcase

A comprehensive demonstration of all the new features and improvements added in xterm-js-sys v5.5.0.

## 🚀 What's New in v5.5.0

This showcase demonstrates the complete set of new features and improvements:

### 🆕 New Methods
- **`input()`** - Programmatic input simulation for automated testing and scripting
- **`writeln()`** - Write with automatic newline for cleaner output
- **`options` property** - Direct options access with getter/setter support
- **`clearTextureAtlas()`** - Canvas renderer optimization for performance
- **`attachCustomWheelEventHandler()`** - Custom wheel event processing

### 🔔 New Events
- **`onBell`** - Bell notification events for audio/visual feedback
- **`onWriteParsed`** - Write parsing completion events for synchronization
- Enhanced event data structures with better type safety

### ⚙️ Enhanced Options
- **`allowProposedApi`** - Enable experimental features safely
- **`cursorInactiveStyle`** - Customize inactive cursor appearance
- **`customGlyphs`** - Enable custom character rendering
- **`minimumContrastRatio`** - Accessibility compliance support
- **`windowsPty`** - Windows PTY backend selection (ConPTY/WinPTY)
- **`altClickMovesCursor`** - Alt+click cursor positioning
- **`fastScrollModifier`** - Configurable fast scroll key
- **`fastScrollSensitivity`** - Adjustable scroll speed multiplier
- **`ignoreBracketedPasteMode`** - Bracketed paste mode control

### 🏗️ Builder Patterns
- **Fluent TerminalOptions** - Chainable `.with_*()` methods for clean configuration
- **Fluent Theme** - Type-safe theme building with method chaining
- **Improved Developer Experience** - More intuitive and discoverable API

### 🔧 Advanced Features
- **Marker Management** - Register and track specific terminal lines
- **Character Joiners** - Custom character rendering for ligatures
- **Terminal Modes Access** - Read current terminal state and modes
- **Enhanced Write Methods** - Support for both String and Uint8Array data

### 🏛️ Architecture Improvements
- **Modular Structure** - Better organized codebase for maintainability
- **Clean Separation** - Organized type definitions and interfaces
- **Zero Warnings** - Clean compilation with comprehensive documentation

## 🎮 Interactive Demo

The showcase includes an interactive terminal with:

1. **Live Feature Testing** - Click buttons to test each new method
2. **Real-time Event Handling** - See new events in action
3. **Builder Pattern Examples** - Observe fluent API usage
4. **Performance Demonstrations** - Experience optimization features

## 🛠️ Building and Running

### Prerequisites
- Rust with `wasm32-unknown-unknown` target
- Node.js and npm (for development server)

### Build Steps

1. **Build the WebAssembly module:**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Generate bindings (optional - for development):**
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

3. **Serve the demo:**
   ```bash
   # Using a simple HTTP server
   python -m http.server 8000
   # or
   npx serve .
   ```

4. **Open in browser:**
   Navigate to `http://localhost:8000`

## 📋 Code Examples

### Builder Pattern Usage
```rust
use xterm_js_sys::xterm::{TerminalOptions, Theme};
use xterm_js_sys::ext::{TerminalOptionsExt, ThemeExt};

// Fluent TerminalOptions configuration
let options = TerminalOptions::default()
    .with_log_level(LogLevel::Info)
    .with_font_size(14.0)
    .with_cursor_style(CursorStyle::Block)
    .with_cursor_blink(true)
    .with_scrollback(10000);

// Fluent Theme customization
let theme = Theme::default()
    .with_foreground("#f8f8f2")
    .with_background("#282a36")
    .with_cursor("#f8f8f0")
    .with_red("#ff5555")
    .with_green("#50fa7b");
```

### New Methods Usage
```rust
// Programmatic input simulation
terminal.input("echo 'Hello World!'\n".to_string());

// Write with automatic newline
terminal.writeln("Status: Ready".to_string());

// Direct options access
let mut current_options = terminal.options();
current_options.font_size = Some(16.0);
terminal.set_options(current_options);

// Canvas optimization
terminal.clear_texture_atlas();
```

### Enhanced Event Handling
```rust
// Bell event handler
let bell_listener = Closure::wrap(Box::new(move || {
    console::log_1(&"Bell rang!".into());
}) as Box<dyn FnMut()>);
terminal.on_bell(&bell_listener);

// Write parsed event handler
let write_parsed_listener = Closure::wrap(Box::new(move || {
    console::log_1(&"Content parsed!".into());
}) as Box<dyn FnMut()>);
terminal.on_write_parsed(&write_parsed_listener);
```

## 🔍 What to Look For

When running the showcase, pay attention to:

1. **Smooth Performance** - Notice the optimized rendering
2. **Rich Theming** - Observe the custom color scheme
3. **Interactive Buttons** - Test each new method
4. **Event Logging** - Watch the browser console for event notifications
5. **Builder Patterns** - See the clean, readable configuration code

## 📚 Documentation

For complete API documentation, see:
- [xterm-js-sys Documentation](https://docs.rs/xterm-js-sys)
- [xterm.js Official Docs](https://xtermjs.org/docs/)

## 🤝 Contributing

Found an issue or want to improve the showcase? Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This showcase is part of xterm-js-sys and is licensed under the same terms.
