//! Some [`Theme`]s.
//!
//! [`Theme`]: crate::xterm::Theme

use crate::xterm::Theme;

macro_rules! c {
    ($hex:literal) => {{
        // Ideally we'd be able to use const functions for all this, but alas;
        // things are not there yet.
        let _type_assert: u32 = $hex;

        format!("#{:6X}", $hex)
    }};
}

impl Theme {
    /// An xterm.js [`Theme`] based on the [Nord color palette][nord].
    ///
    /// This is loosely based on the [iTerm 2 Nord theme][iterm2].
    ///
    /// [nord]: https://www.nordtheme.com/
    /// [iterm2]: https://github.com/arcticicestudio/nord-iterm2
    #[allow(clippy::unreadable_literal)]
    #[rustfmt::skip]
    #[must_use]
    pub fn nord() -> Self {
        let mut theme = Self::default();

        // Set colors using the setter methods
        theme.set_black(Some(c!(0x343434)));
        theme.set_bright_black(Some(c!(0x434c5e)));
        theme.set_red(Some(c!(0xbf616a)));
        theme.set_bright_red(Some(c!(0xbf616a)));
        theme.set_green(Some(c!(0xa3be8c)));
        theme.set_bright_green(Some(c!(0xa3be8c)));
        theme.set_yellow(Some(c!(0xebcb8b)));
        theme.set_bright_yellow(Some(c!(0xebcb8b)));
        theme.set_blue(Some(c!(0x81a1c1)));
        theme.set_bright_blue(Some(c!(0x81a1c1)));
        theme.set_magenta(Some(c!(0xb48ead)));
        theme.set_bright_magenta(Some(c!(0xb48ead)));
        theme.set_cyan(Some(c!(0x88c0d0)));
        theme.set_bright_cyan(Some(c!(0x8fbcbb)));
        theme.set_white(Some(c!(0xe5e9f0)));
        theme.set_bright_white(Some(c!(0xeceff4)));

        theme.set_background(Some(c!(0x2e3440)));
        theme.set_foreground(Some(c!(0xd8dee9)));

        theme
    }
}
