# Julia Set Illustrator

A Julia illustrator written in Rust.

It uses the [num_complex](https://github.com/rust-num/num-complex) crate to provide complex numbers and the [png](https://github.com/image-rs/image-png) crate to create the output.

The output it generates looks a little something like this:

![img](https://github.com/ElectricCoffee/mandelbrot/blob/julia/julia_-0.8%2B0.156i_8000x8000.png)

The hideous colours were chosen because they lie within a value if 127 of each other, making it relatively easy to just "generate" them by hand.

# When Building

Due to a bug present in `png` the 8000×8000 px image rendering only works in release mode.
Highest I've made work in debug is 4800×4800 px.
