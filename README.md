# Fraczal

Fraczal is a command-line program that generates images of the [Mandelbrot set]. Its current differentiating feature is its support for user-defined sequential color palettes in the [HCL color space].

The name comes from the terms *[fractal],* a shape or process containing fine recurring structure, and *[quetzal],* a colorful bird found in Mexico and Central America.

## Context

The HCL color space is a color model that has been recommended for data visualization in such papers as [Escaping RGBland], [Somewhere Over the Rainbow] and [colorspace][colorspace paper], in which it’s presented as a more intuitive alternative to the well-known [RGB], [HSL and HSV] models. It is thought to provide an accurate model of human color vision and is a perceptually uniform color space (all neighboring pairs of colors appear equally distinct). We care about this because we want changes in color to represent changes in data accurately.

To remain consistent with the scientific literature, we use the term “HCL color space” to refer to the CIELCh(uv) or polar *L\*, u\*, v\** color space, a cylindrical transformation of the [CIE 1976 *L\*, u\*, v\** (CIELUV) color space][CIELUV].

## Installation

Fraczal is pre-release software and must be built from source.

### Requirements

[Rust][Rust installation] 1.63 or newer

### Instructions

```sh
git clone https://github.com/ok-ryoko/fraczal
cd fraczal
cargo build --release
```

## Usage

After following the installation instructions, running the following command in the root of this repository should generate a 1280-by-720 24-bit [PNG] image of the Mandelbrot set colored using [Fabio Crameri’s Lajolla palette][scientific colour maps].

```sh
./target/release/fraczal \
    -W=1280 \
    -H=720 \
    --upper-left='-2 + 0.5i' \
    --cheight=1 \
    -p=assets/palettes/Lajolla.json
```

![Colorful, low-resolution render of the Mandelbrot set][example image]

The bounding box corresponding to this image extends from −2 + 0.5i to about −0.2 − 0.5i in the complex plane. By default, the aspect ratio of the bounding box is equal to the aspect ratio of the image dimensions (in this example, 1280/720 or 16/9), meaning that we need to provide only the desired height of the bounding box. This ensures that distortion-free images are generated.

### Configuring the program with options

#### Options that take a parameter

##### `--width`, `-W`

The width of the output image in pixels

##### `--height`, `-H`

The height of the output image in pixels

##### `--upper-left`

The location of the upper-left corner of the bounding box in the complex plane in the form `'a + bi'`

##### `--cheight`

The height of the bounding box in the complex plane

##### `--palette`, `-p`

The path to a JSON file containing a palette definition

##### `--aspect-ratio`, `-a`

The aspect ratio of the bounding box in the complex plane (defaults to the ratio of the width and height of the output image)

##### `--max-iter`, `-N`

The maximum number of iterations to allow per point in the complex plane (defaults to `1000`)

##### `--out-file`, `-o`

The path to the output image (defaults to a *.png* file in the current directory named after the Unix epoch when the program started writing the image to disk)

#### Flags

##### `--reverse`, `-r`

Reverse the palette

### Using and defining color palettes

Fraczal supports only sequential color palettes. The lightness in a sequential color palette changes monotonically. Sequential palettes are therefore a [natural choice][seaborn-luminance] for [escape-time coloring strategies], which rely on a monotonically increasing, non-negative quantity (the escape time).

The HCL color space is ideal for sequential palette discovery because it is perceptually uniform. Fraczal includes 70 such sequential palettes. These are available in the [*assets/palettes* directory][palette dir] and have been adapted from the [colorspace R package].

Fraczal implements the palette selection strategy described in the [colorspace paper]. Palettes are defined as JSON files like so:

```json
{
  "name":   "Viridis",
  "start":  { "h": 300, "C": 40, "L": 15 },
  "end":    { "h":  70, "C": 95, "L": 90 },
  "powerC": 1.0,
  "powerL": 1.1
}
```

`start` and `end` are the endpoints of the palette, supplied as CIELCh(uv) coordinates:

- `h` is the *hue*, $h_{uv}$, an angle in degrees;
- `C` is the *chroma*, $C_{uv}^*$, a real number greater than or equal to 0, and
- `L` is the *lightness*, $L^*$, a real number between 0 and 100 (inclusive).

Fraczal doesn’t enforce the constraints on `C` and `L`—they are purely logical. Values that don’t respect the constraints may lead to unexpected results.

`powerC` and `powerL` are real numbers that determine whether `C` and `L`, respectively, change linearly. `C` and `L` change linearly if their respective power is equal to 1; they otherwise change nonlinearly.

`h` always changes linearly.

Some palettes have a `Cmax` field that imposes an upper limit on `C`. We would set this field when we want `C` to take a “triangular” trajectory, increasing to `Cmax` before decreasing.

## Community

### Understanding the code of conduct

Please take time to read the [code of conduct] before reaching out for support or making a contribution.

### Getting support

If you’re encountering unexpected or undesirable program behavior, check the [issue tracker] to see whether your problem has already been reported. If not, please consider taking time to create a bug report and make the community aware of the problem.

If you have questions about using the program or participating in the community around the program, consider [starting a discussion][discussions].

### Contributing to Fraczal

[Ryoko] is accepting contributions in the form of bug reports and feature suggestions. You can contribute by [opening an issue][issue tracker] and filling out the appropriate template.

You will soon be able to work on Fraczal and open pull requests.

## License

Fraczal is free and open source software [licensed under the MIT license][license].

## Acknowledgements

The `color` module draws on information from the following sources:

- C. Poynton, *Digital Video and HDTV: Algorithms and Interfaces.* San Francisco, CA, USA: Morgan Kaufmann, 2003, ch. 21–23, pp. 219, 225–227, 230, 251 and 267, doi: [10.1016/B978-1-55860-792-7.X5061-7][digital video book]
- J. Schanda, *[Colorimetry: Understanding the CIE System][colorimetry book],* J. Schanda, Ed. Hoboken, NJ, USA: John Wiley & Sons, 2007, ch. 3, p 61

The `palettes` submodule is based on foundational work by the developers ([Ross Ihaka] *et al.*) of the [colorspace R package]:

- A. Zeileis, J. C. Fisher, K. Hornik, R. Ihaka, C. D. McWhite, P. Murrell, R. Stauffer and C. O. Wilke, “colorspace: A Toolbox for Manipulating and Assessing Colors and Palettes,” Journal of Statistical Software, vol. 96, no. 1, Nov. 2020, doi: [10.18637/jss.v096.i01][colorspace paper]

[Open Source Guides], the [GitHub documentation] and the [github/docs repository][github/docs] have been instrumental in preparing this repository for community contributions.

[CIELUV]: https://en.wikipedia.org/wiki/CIELUV
[code of conduct]: ./CODE_OF_CONDUCT.md
[colorimetry book]: https://www.wiley.com/en-us/Colorimetry%3A+Understanding+the+CIE+System-p-9780470049044
[colorspace paper]: https://doi.org/10.18637/jss.v096.i01
[colorspace R package]: https://colorspace.r-forge.r-project.org/articles/colorspace.html
[digital video book]: https://doi.org/10.1016/B978-1-55860-792-7.X5061-7
[discussions]: https://github.com/ok-ryoko/fraczal/discussions
[escape-time coloring strategies]: https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set#Escape_time_algorithm
[Escaping RGBland]: https://doi.org/10.1016/j.csda.2008.11.033
[example image]: ./docs/img/Mandelbrot-Lajolla.png
[fractal]: https://en.wikipedia.org/wiki/Fractal
[GitHub documentation]: https://docs.github.com/en
[github/docs]: https://github.com/github/docs
[HCL color space]: https://en.wikipedia.org/wiki/HCL_color_space
[HSL and HSV]:https://en.wikipedia.org/wiki/HSL_and_HSV 
[issue tracker]: https://github.com/ok-ryoko/fraczal/issues
[license]: ./LICENSE.txt
[Mandelbrot set]: https://en.wikipedia.org/wiki/Mandelbrot_set
[Open Source Guides]: https://opensource.guide/
[palette dir]: ./assets/palettes/
[PNG]: https://en.wikipedia.org/wiki/Portable_Network_Graphics
[quetzal]: https://en.wikipedia.org/wiki/Quetzal
[RGB]: https://en.wikipedia.org/wiki/RGB_color_model
[Ross Ihaka]: https://www.stat.auckland.ac.nz/~ihaka/
[Rust installation]: https://www.rust-lang.org/tools/install
[Ryoko]: https://github.com/ok-ryoko
[scientific colour maps]: https://www.fabiocrameri.ch/colourmaps/
[seaborn-luminance]: https://seaborn.pydata.org/tutorial/color_palettes.html#vary-luminance-to-represent-numbers
[SemVer2]: https://semver.org/spec/v2.0.0.html
[Somewhere Over the Rainbow]: https://doi.org/10.1175/BAMS-D-13-00155.1
