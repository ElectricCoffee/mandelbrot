# Mandelbrot & Julia Set Illustrator

A Mandelbrot & Julia Set illustrator written in Rust.

It uses the [num_complex](https://github.com/rust-num/num-complex) crate to provide complex numbers and the [png](https://github.com/image-rs/image-png) crate to create the output.

The outputs it generates look a little something like this:

![Mandelbrot Example](https://raw.githubusercontent.com/ElectricCoffee/mandelbrot/master/mandelbrot_8000x8000.png)
![Julia Example](https://raw.githubusercontent.com/ElectricCoffee/mandelbrot/master/julia_-0.8%2B0.156i_8000x8000.png)

The hideous colours were chosen because they lie within a value if 127 of each other, making it relatively easy to just "generate" them by hand.

# When Building

Due to a bug present in `png` the 8000×8000 px image rendering only works in release mode.
Highest I've made work in debug is 4800×4800 px.

# Configuring the Output
The output is configured via the `config.ron` file, which is used by the program.
The file uses three parameters:
* `scale_factor`, the number the pixel in the image is divided by to get the real and imaginary components of the complex number representing that point.
From the scale factor, the following is derived:
    - image width = 4 × scale factor
    - image height = 4 × scale factor
* `mode`, has two possible values `Mandelbrot` and `Julia(re, im)`. The Mandelbrot mode will draw the Mandelbrot set, while the Julia mode will draw a Julia set with `c` set to a complex number defined by `re` and `im`.
* `iteration_depth`, the number of repeated applications of `fc(z) = z² + c` to determine whether or not that point grows past `|fc(z)| = 2`.
