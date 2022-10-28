# CwinUI

Terminal windowing and UI library for rust applications.

## About

The library is inspired by [pancurses](https://github.com/ihalila/pancurses) and [tui](https://github.com/fdehau/tui-rs/).
It is currently based around shared buffers and is focused on efficiency. The implementation however might change in the future, as support for dynamic layouts is planned.

## Features

- [x] Windowing support (based on shared buffers).
- [x] Custom widgets.
- [x] Alignment support (widgets can be aligned to each other in various ways, e.g. center, top-left, etc.
- [x] Color support.
- [ ] Dynamic layouts and resizing.

## Installation

Add this to your project's `Cargo.toml`:

```toml
[dependencies]
cwinui = { git = "https://github.com/ShinyJonny/cwinui" tag = "v0.1.2 "}
```
