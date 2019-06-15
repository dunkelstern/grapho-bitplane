# grapho-bitplane

This is the pixel buffer library of the `grapho`-set of crates.

There is also:

- `grapho-color`: describes the color primitives used by the `grapho` crates and does color conversion
- `grapho-2d`: describes all kinds of 2D vector graphics and math (like polygon clipping)
- `grapho-rasterize-2d`: 2D rasterizer for the vectors described in `grapho-2d`
- `grapho-filters`: pixel based effects and filters for pixel buffers
- `grapho-cv`: computer vision library for grapho stack
- `grapho-3d`: 3D vector math

## What does it do

`grapho-bitplane` describes the pixel buffer primitives that are used for creating and modifying images.
It will contain color conversion functionality and can work on arbitrary interleaved or non-interleaved
graphics data.

Conversion from all color-plane types into all others will be implemented.
Currently the following bitplane types are implemented:

### RGB interleaved `RGBPixelBuffer`

- `RGB`, `BGR` 24 bit without alpha
- `RGBA`, `ARGB`, `BGRA`, `ABGR` 32 bit with alpha

### Grayscale `GrayscalePixelBuffer`

- `Y` Simple, single Y plane for monochrome images.
- `Yxx` 3 bytes, ignore the last two (interpret a YUV444 image as grayscale)
- `Yx` and `xY`, 2 bytes, ignore the x (interpret a YUV422 image as grayscale)

### YUV  4:2:2 interleaved `YUV422iPixelBuffer`

- `UYVY` YUV 4:2:2 (Y sample at every pixel, U and V sampled at every second pixel horizontally on each line). A macropixel contains 2 pixels in 1 `u32`.
- `YUY2`/`YUV422` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
- `YVYU` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
- `VYUY` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.

### YUV 4:4:4 interleaved `YUV444iPixelBuffer`

- `YUV` / `YUV444` YUV 4:4:4, 3 bytes per pixel, full resolution U and V planes.
- `VUY` like `YUV` but with different component ordering
- `UVY` like `YUV` but with different component ordering
- `YVU` like `YUV` but with different component ordering

### YUV 4:2:2 planar `YUV422pPixelBuffer`

- `YV21` / `I420` 8 bit Y plane followed by 8 bit 2x2 subsampled U and V planes.
- `YV12` 8 bit Y plane followed by 8 bit 2x2 subsampled V and U planes.

## TODO

### YUV 4:2:2 planar/interleaved `YUV422piPixelBuffer`

- `NV12` 8-bit Y plane followed by an interleaved U/V plane with 2x2 subsampling
- `NV21` As NV12 with U and V reversed in the interleaved plane

### YCoCg

- `YCoCg444i` interleaved with stride
- `YCoCg444p` 3 planes no subsampling
- `YCoCg422p` 3 planes, Co and Cg half size subsampled
- `YCoCg422i` interleaved, Co and Cg half size subsampled
