# random_color
:gear: :art: Rust crate for generating random attractive colors.

Inspired by [RandomColor](https://github.com/davidmerfield/randomColor).

## Usage 

Example:
```rust
let color = RandomColor::new()
  .hue("blue") // Optional
  .luminosity("light") // Optional
  .seed(42) // Optional
  .alpha(1.0) // Optional
  .to_hsl(); // 

// color => "hsl(179, 99%, 10%)"
```

Avaible outputs:
```rust
  // As HSV Array
  let color = RandomColor::new().to_hsv_array(); // color => [179, 20, 100]

  // As RGB
  let color = RandomColor::new().to_rgb(); // color => "rgb(204, 255, 254)"

  // As RGBA
  let color = RandomColor::new().to_rgba(); // color => "rgba(204, 255, 254, 1)"

  // As RGB Array
  let color = RandomColor::new().to_rgb_array(); // color => [204, 255, 254]

  // As HSL
  let color = RandomColor::new().to_hsl(); // color => "hsl(179, 99%, 10%)"

  // As HSLA
  let color = RandomColor::new().to_hsla(); // color => "hsl(179, 99%, 10%, 1)"

  // As HSL Array
  let color = RandomColor::new().to_hsl_array(); // color => [179, 99, 10]
  
  // As Hex
  let color = RandomColor::new().to_hex(); // color => "#b31464"
```
## Roadmap

+ Seed from string
+ Iteartor
+ Documentation

## License

The MIT License (MIT)

Copyright (c) 2017 <a href="http://lucasmarino.me">Lucas Maximiliano Marino</a>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
