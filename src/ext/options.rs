//! Simplified helpers for [`TerminalOptions`] and [`Theme`].
//!
//! This module provides builder-style methods for creating and configuring
//! xterm.js options in a more Rust-friendly way.
//!
//! [`TerminalOptions`]: crate::xterm::TerminalOptions
//! [`Theme`]: crate::xterm::Theme

use crate::xterm::{
    BellStyle, CursorStyle, LogLevel, TerminalOptions, Theme,
};

/// Extension trait for [`TerminalOptions`] providing builder-style methods.
pub trait TerminalOptionsExt {
    /// Set the log level and return self for chaining.
    fn with_log_level(self, log_level: LogLevel) -> Self;
    
    /// Set the font size and return self for chaining.
    fn with_font_size(self, font_size: f32) -> Self;
    
    /// Set the number of columns and return self for chaining.
    fn with_cols(self, cols: u16) -> Self;
    
    /// Set the number of rows and return self for chaining.
    fn with_rows(self, rows: u16) -> Self;
    
    /// Set the cursor style and return self for chaining.
    fn with_cursor_style(self, cursor_style: CursorStyle) -> Self;
    
    /// Set whether the cursor blinks and return self for chaining.
    fn with_cursor_blink(self, cursor_blink: bool) -> Self;
    
    /// Set the bell style and return self for chaining.
    fn with_bell_style(self, bell_style: BellStyle) -> Self;
    
    /// Set whether to convert EOL and return self for chaining.
    fn with_convert_eol(self, convert_eol: bool) -> Self;
    
    /// Set the scrollback amount and return self for chaining.
    fn with_scrollback(self, scrollback: u32) -> Self;
    
    /// Set the tab stop width and return self for chaining.
    fn with_tab_stop_width(self, tab_stop_width: u16) -> Self;
}

impl TerminalOptionsExt for TerminalOptions {
    fn with_log_level(mut self, log_level: LogLevel) -> Self {
        self.log_level = Some(log_level);
        self
    }
    
    fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
        self
    }
    
    fn with_cols(mut self, cols: u16) -> Self {
        self.cols = Some(cols);
        self
    }
    
    fn with_rows(mut self, rows: u16) -> Self {
        self.rows = Some(rows);
        self
    }
    
    fn with_cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = Some(cursor_style);
        self
    }
    
    fn with_cursor_blink(mut self, cursor_blink: bool) -> Self {
        self.cursor_blink = Some(cursor_blink);
        self
    }
    
    fn with_bell_style(mut self, bell_style: BellStyle) -> Self {
        self.bell_style = Some(bell_style);
        self
    }
    
    fn with_convert_eol(mut self, convert_eol: bool) -> Self {
        self.convert_eol = Some(convert_eol);
        self
    }
    
    fn with_scrollback(mut self, scrollback: u32) -> Self {
        self.scrollback = Some(scrollback);
        self
    }
    
    fn with_tab_stop_width(mut self, tab_stop_width: u16) -> Self {
        self.tab_stop_width = Some(tab_stop_width);
        self
    }
}

/// Extension trait for [`Theme`] providing builder-style methods.
pub trait ThemeExt {
    /// Set the foreground color and return self for chaining.
    fn with_foreground<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the background color and return self for chaining.
    fn with_background<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the cursor color and return self for chaining.
    fn with_cursor<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the black color and return self for chaining.
    fn with_black<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the red color and return self for chaining.
    fn with_red<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the green color and return self for chaining.
    fn with_green<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the yellow color and return self for chaining.
    fn with_yellow<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the blue color and return self for chaining.
    fn with_blue<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the magenta color and return self for chaining.
    fn with_magenta<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the cyan color and return self for chaining.
    fn with_cyan<S: Into<String>>(self, color: S) -> Self;
    
    /// Set the white color and return self for chaining.
    fn with_white<S: Into<String>>(self, color: S) -> Self;
}

impl ThemeExt for Theme {
    fn with_foreground<S: Into<String>>(mut self, color: S) -> Self {
        self.set_foreground(Some(color.into()));
        self
    }
    
    fn with_background<S: Into<String>>(mut self, color: S) -> Self {
        self.set_background(Some(color.into()));
        self
    }
    
    fn with_cursor<S: Into<String>>(mut self, color: S) -> Self {
        self.set_cursor(Some(color.into()));
        self
    }
    
    fn with_black<S: Into<String>>(mut self, color: S) -> Self {
        self.set_black(Some(color.into()));
        self
    }
    
    fn with_red<S: Into<String>>(mut self, color: S) -> Self {
        self.set_red(Some(color.into()));
        self
    }
    
    fn with_green<S: Into<String>>(mut self, color: S) -> Self {
        self.set_green(Some(color.into()));
        self
    }
    
    fn with_yellow<S: Into<String>>(mut self, color: S) -> Self {
        self.set_yellow(Some(color.into()));
        self
    }
    
    fn with_blue<S: Into<String>>(mut self, color: S) -> Self {
        self.set_blue(Some(color.into()));
        self
    }
    
    fn with_magenta<S: Into<String>>(mut self, color: S) -> Self {
        self.set_magenta(Some(color.into()));
        self
    }
    
    fn with_cyan<S: Into<String>>(mut self, color: S) -> Self {
        self.set_cyan(Some(color.into()));
        self
    }
    
    fn with_white<S: Into<String>>(mut self, color: S) -> Self {
        self.set_white(Some(color.into()));
        self
    }
}
