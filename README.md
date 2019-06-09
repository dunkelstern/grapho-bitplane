# grapho-bitplane

This is the pixel buffer library of the `grapho`-set of crates.

There is also:

- `grapho-color`: describes the color primitives used by the `grapho` crates and does color conversion
- `grapho-2d`: describes all kinds of 2D vector graphics and math (like polygon clipping)
- `grapho-rasterize-2d`: 2D rasterizer for the vectors described in `grapho-2d`
- `grapho-filters`: pixel based effects and filters for pixel buffers
- `grapho-cv`: computer vision library for grapho stack
- `grapho-3d`: 3D vector math

## What does it do?

`grapho-bitplane` describes the pixel buffer primitives that are used for creating and modifying images.
It will contain color conversion functionality and can work on arbitrary interleaved or non-interleaved
graphics data.

Currently the following bitplane types are implemented:

- `RGB` interleaved with stride

Conversion from all color-plane types into all others will be implemented.

## TODO

### RGB

- `BGR` interleaved with stride
- `RGBA` interleaved with stride
- `BGRA` interleaved with stride

### YCbCr/YUV

- `Y800` Simple, single Y plane for monochrome images.
- `UYVY` YUV 4:2:2 (Y sample at every pixel, U and V sampled at every second pixel horizontally on each line). A macropixel contains 2 pixels in 1 `u32`.
- `YUY2` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
- `YVYU` YUV 4:2:2 as for `UYVY` but with different component ordering within the `u32` macropixel.
- `I420` 8 bit Y plane followed by 8 bit 2x2 subsampled U and V planes.
- `NV12` 8-bit Y plane followed by an interleaved U/V plane with 2x2 subsampling
- `NV21` As NV12 with U and V reversed in the interleaved plane
- `YV12` 8 bit Y plane followed by 8 bit 2x2 subsampled V and U planes.

### YCoCg

- `YCoCg` interleaved with stride
- `YCoCgPlane` 3 planes no subsampling
- `YCoCg422` 3 planes, Co and Cg half size subsampled
