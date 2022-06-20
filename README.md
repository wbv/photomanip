# Photo Manipulation
An exercise in Rust, parallel programming, test-driven development, and self.

This project involves working with a handful of image types: portable pixmap
(PPM) and portable graymap (PGM) images. We consider a stricter subset of those
images which would otherwise be valid PPM or PGM images and attempt to define
them below. For simplicity, we ignore unicode and only consider single-byte
characters interpreted as ASCII.



## Image Types
Note that we don't include PBM files, and we only consider comments valid if
they are the second line of the file, even though regular PPM/PGM files may
contain comments anywhere between the magic number and the _raster_.

### PPM - Color Images
- Starts with `P3` or `P6` and a newline (`\n`, `\r`, or `\n\r`)
- Next line is a comment if-and-only-if it starts with `#`
- (Comment lines are ignored)
- Next line is width and height of an image in ascii (e.g. `3 2` means width=3
  and height=2), separated by whitespace, then a newline
- Next line is the maximum value a pixel can be, in ascii (upper bound)
  followed by a single whitespace character (one byte)
- Maximum value will always be less than or equal to `255`
- (0 is our implicit lower bound for a pixel)
- The rest of the file is data (also called the _raster_)
  - `P3` - expect ascii numbers representing data, separated by whitespaces
    - Each whitespace-separated ascii-represented decimal number is a value
  - `P6` - expect binary values, one byte after another
    - Each byte is a value
- Data appears in row-major order, top-to-bottom
  - For each row, all columns' pixels are given in left-to-right order
  - For each pixel, each color channel is given with a distinct value for each
    channel (red, green, blue; in that order)

### PGM - Grayscale Images
- Starts with `P2` or `P5` and a newline (`\n`)
- Is otherwise *identical* to a PPM image, except:
  - Where there would be three values for each channel of RGB, we will only
    find one value (i.e. the grayscale channel)



## Program Usage

The program will be called with three arguments, optionally four.
In [POSIX utility argument syntax](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap12.html#tag_12_01):

```
photomanip [option] outputmode basename image.ppm
```
Where:
- `[option]` is at most *one* of:
  ```
   -b amount   Brighten
   -c          Contrast
   -g          Grayscale
   -n          Negate
   -p          Sharpen
   -s          Smooth
  ```
  `amount` is guaranteed to be an integer (represented as ascii).

- `outputmode` is exactly one of:
  ```
   -oa         Output in ASCII mode (P3 PPM)
   -ob         Output in binary mode (P6 PPM)
  ```

- `basename` is the name of the output file to be produced and saved, without
  any extension.

- `image.ppm` is the input file.


## Options

### Brighten (-b)
The argument to brighten must be in the inclusive range `-255` to `255`. This
value will be added to every channel value for every pixel, individually.
```
new_value = value + amount
```


### Contrast (-c)
The image will be first converted to grayscale as if `-g` was given, then:
- Compute a scale factor:
- `scale = 255 / (max - min)` where:
  - `max` is the highest pixel value that appears in the entire image
  - `min` is the lowest pixel value that appears in the entire image
- Subtract `min` from each pixel and then multiply by the scale factor `scale`:
```
new_value = scale * (value - min)
          = 255 * (value - min) / (max - min)
```

Implicitly, this will result in a PGM (grayscale) file.


### Grayscale (-g)
For each pixel, form a single grayscale channel as weighted average of each
color's channel using the following coefficients:
```
   0.3 * Red
   0.6 * Green
 + 0.1 * Blue
--------------
     New Value
```

The output will be a PGM (grayscale) file.


### Negate (-n)
For each value, we use the maximum pixel value possible (given in the file
header, usually 255) and subtract the pixel from this value:
```
new_value = MAX - value
```


### Sharpen (-p)
For each value, we use four cardinal neighbors of that value to determine a new
value by subtracting the neighbor values from 5 times the original value:
```
   N
 W v E
   S

new value (at v) = 5*v - N - S - E - W
```

We will assume that a neighbor which falls off an edge has the same value as the original value (v).

### Smooth (-s)
For each value, we use all eight neighbor values and the original value and form a new value from the unweighted average of those values:
```
NW N NE
W  v  E
SW S SE

new value (at v) = (NW + N + NE + W + v + E + SW + S + SE) / 9
```

We will assume that a neighbor which falls of an edge has the same value as the original value (v).
