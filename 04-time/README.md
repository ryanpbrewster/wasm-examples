# Reaching outside the land of pure computation
Web Assembly cannot interact with the outside world without help.

### How does `Date.now()` or `System.currentTimeMillis()` actually work?
[System calls](https://pzemtsov.github.io/2017/07/23/the-slow-currenttimemillis.html)!

### How does this work in WebAssembly?

It doesn't! Web Assembly can't make system calls unless we give it access.

To see how, let's move on to `05-time2`.
