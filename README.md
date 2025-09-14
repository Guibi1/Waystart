# WayStart

A fast start menu for Wayland-based window managers.

## Features

- Single purpose
- Keyboard first
- Simple configuration
- Power menu and other goodies

## Usage

WayStart can be run in several modes:

### Standalone Mode

Run WayStart as a standalone application. This will open the WayStart window and exit once its closed.

```bash
waystart
```

### Daemon Mode

This mode is the preferred way to run WayStart. It will start the WayStart daemon in the background and allow you to open the WayStart window when needed.

Start the WayStart daemon:

```bash
waystart daemon
```

When running in daemon mode, you can control the WayStart window using the `show` and `hide` commands.

```bash
waystart show
# or
waystart hide
```
