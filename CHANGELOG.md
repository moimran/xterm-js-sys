# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [5.5.0-alpha1] - 2025-06-28

### Added
- Support for xterm.js v5.5.0
- Updated TypeScript definitions to match latest xterm.js API
- Modular code structure for better maintainability
- Support for new xterm.js package structure (@xterm/xterm)

### Changed
- **BREAKING**: Updated to xterm.js v5.5.0 API
- **BREAKING**: Package name changed from `xterm` to `@xterm/xterm` in npm dependencies
- Reorganized internal module structure for better code organization
- Updated examples to work with new API
- Disabled `ext` feature by default due to compatibility issues with new structure

### Fixed
- Compilation issues with wasm32-unknown-unknown target
- Updated event listener APIs to match xterm.js v5.5.0
- Fixed TypeScript definition imports

### Migration Guide
- Update your `package.json` to use `"@xterm/xterm": "^5.5.0"` instead of `"xterm": "^4.6.0"`
- Update your `Cargo.toml` to use `xterm-js-sys = "5.5.0-alpha1"`
- If using the `ext` feature, note that it's currently disabled by default due to compatibility issues
- Event listener methods may have changed - refer to updated examples