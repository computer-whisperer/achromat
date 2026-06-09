# achromat

Color-managed image decoding for the prism/damascene ecosystem. An
*achromat* is a lens corrected for color error; this crate is the shared
decode stack that keeps color correct from file to framebuffer.

It started life inside [prism-bg], was copied into [damascene-gallery],
and was extracted here when [prism-explorer] became its third consumer.

[prism-bg]: https://github.com/computer-whisperer/prism-bg
[damascene-gallery]: https://github.com/computer-whisperer/damascene-gallery
[prism-explorer]: https://github.com/computer-whisperer/prism-explorer

## What it does

- **`decode`** — format dispatch by magic bytes (not extension) and
  per-format decoders: PNG (cICP/iCCP/sRGB/gAMA+cHRM), JPEG, WebP, AVIF
  (including manual `colr` box parsing), JPEG XL, JPEG XR (scRGB, via
  vendored jxrlib), OpenEXR, Radiance HDR. Output is straight- or
  premultiplied-alpha pixels packed 8-bit for ordinary SDR content and
  fp16 for everything that needs range or precision.
- **`color`** — parametric color encodings: transfer functions (sRGB,
  gamma 2.2, BT.1886, PQ, linear), primary volumes (sRGB, Display-P3,
  BT.2020, custom chromaticities), luminance metadata, EOTF/OETF math.
- **`cms`** — ICC profile resolution to parametric encodings, with a
  four-case ladder: cicp tag → named-TRC matrix-shaper matching → TRC
  rewrite in the same gamut → full LUT conversion (via moxcms).
- **`convert`** (feature `damascene`) — bridge to damascene's `Image`
  type (PQ decoded to linear anchored at the declared reference white),
  linear-light box-filtered thumbnailing, and single-pass pixel stats.

## jpegxr vendoring

jpegxr 0.3.1's pinned bindgen 0.69 generates opaque structs with current
libclang, so `vendor/jpegxr` carries it with bindgen bumped to 0.72.
Cargo `[patch.crates-io]` sections apply only at the consuming workspace
root, so consumers must carry the patch themselves, pointed at this
repo's vendor copy:

```toml
[patch.crates-io]
jpegxr = { path = "../achromat/vendor/jpegxr" }
```

Drop when upstream updates.

## License

MIT or Apache-2.0, at your option.
