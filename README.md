# rxscreen
A simple wrapper over libx11 to take screenshots

# Dependencies
- libx11

# Features
* save (save screenshots directly to file)

# Similar projects
* [x11cap (Linux)](https://github.com/bryal/X11Cap)
* [captrs (Windows, Linux)](https://github.com/bryal/captrs)
* [dxgcap (Windows)](https://github.com/bryal/dxgcap-rs)


# Performance

This implementation has some flaws and potential to be optimized, but i'd consider it well enough.

Do mind though, that saving screenshots with the `save` feature may take multiple seconds without optimizations enabled.
With opt-level 3, the process takes a fraction of the time (on my system 12 secs without optimizations, 0.2 seconds with opt-level 3)