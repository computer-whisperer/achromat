//! Color-managed image decoding for the prism/damascene ecosystem.
//!
//! An *achromat* is a lens corrected for color error; this crate is the
//! shared decode stack that keeps color correct from file to framebuffer.
//! It grew up in prism-bg, was copied into damascene-gallery, and lives
//! here now so its consumers (prism-bg, damascene-gallery, prism-explorer)
//! stop hand-syncing it.
//!
//! - [`decode`] — format sniffing by magic bytes and per-format decoders
//!   (PNG, JPEG, WebP, AVIF, JPEG XL, JPEG XR, EXR, Radiance HDR), packing
//!   to 8-bit or fp16 wire formats with premultiplied ([`decode::load`])
//!   or straight ([`decode::load_straight`]) alpha.
//! - [`color`] — parametric color encodings: transfer functions,
//!   primaries, luminance metadata, EOTF/OETF math.
//! - [`cms`] — ICC profile resolution to parametric encodings (cicp tag →
//!   TRC matching → curve rewrite → LUT conversion via moxcms).
//! - [`convert`] (feature `damascene`) — bridge to damascene's `Image`
//!   (PQ anchored to declared reference white), linear-light box-filtered
//!   thumbnailing, and single-pass pixel stats.

pub mod cms;
pub mod color;
#[cfg(feature = "damascene")]
pub mod convert;
pub mod decode;
