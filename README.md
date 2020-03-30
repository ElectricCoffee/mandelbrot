# Mandelbrot Illustrator
A Mandelbrot illustrator written in Rust.

It uses the `num` crate to provide complex numbers and the `png` crate to create the output.

The output it generates looks a little something like this:

![img](https://github.com/ElectricCoffee/mandelbrot/blob/master/mandelbrot_4800x4800.png)

The hideous colours were chosen because they lie within a value if 127 of each other, making it relatively easy to just "generate" them by hand.