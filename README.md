# SinkView

SinkView is a real-time log viewer for Unix systems meant to work with [lg_sink](https://github.com/aj-thomas-8/lg_sink).

It is ideal for debugging TUI applications providing a display for debug, info, warn, and error messages during application runtime.

### Features:
- Color coded display of log messages
- Supports app driven clearing of console

## Setup
__Requirements__: `rust/cargo`
- Clone the repository
- Build/run the project:
  ```
  cargo run
  ```
Logs from an application utilzing lg_sink log functions will be displayed on the terminal 
