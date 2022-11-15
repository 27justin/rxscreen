# rxscreen
A simple wrapper over libx11 to take screenshots.

# Dependencies
- libx11
- libxext (with the `shm` feature)
- libxrandr (with the `xrandr` feature)

# Features
* save (save screenshots directly to file)
* MIT-SHM (use the MIT-SHM extension to rapidly take screenshots, used for screenrecording and similar)
* Xrandr (use Xrandr to query monitors connected to the X11 server)

# Planned features
* Graphics, to allow creating own displays and drawing on them.
* Extend Xrandr support to change display configuration

# Similar projects
* [x11cap (Linux)](https://github.com/bryal/X11Cap)
* [captrs (Windows, Linux)](https://github.com/bryal/captrs)
* [dxgcap (Windows)](https://github.com/bryal/dxgcap-rs)

