# Console raytracer
This is a raytracer written in Rust for displaying raytraced images in the terminal with support for lighting, shadows, reflections and multi-threaded rendering.

- Based on the book [Computer Graphics from Scratch](https://www.gabrielgambetta.com/computer-graphics-from-scratch/) by Gabriel Gambetta.
- The engine used for the canvas is [gemini-engine](https://github.com/redpenguinyt/gemini-rust) by me - a monospaced 2d/3d rendering engine for the terminal, though for this project i only really used the 2D half of it

This project can also be added as a cargo package with `cargo add --git https://github.com/`, if you wanna put together your own scene.

The result looks like this:
![The render result](https://cdn.discordapp.com/attachments/887416381933486110/1151517584110387291/image.png)