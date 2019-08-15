// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
*resvg* is an [SVG](https://en.wikipedia.org/wiki/Scalable_Vector_Graphics) rendering library.

*resvg* can be used to render SVG files based on a
[static](http://www.w3.org/TR/SVG11/feature#SVG-static)
[SVG Full 1.1](https://www.w3.org/TR/SVG/Overview.html) subset.
In simple terms: no animations and scripting.

It can be used as a simple SVG to PNG converted.
And as an embeddable library to paint SVG on an application native canvas.
*/

#![doc(html_root_url = "https://docs.rs/resvg/0.7.0")]

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "cairo-backend")]
pub use cairo;

#[cfg(feature = "qt-backend")]
pub use resvg_qt as qt;

#[cfg(feature = "skia-backend")]
pub use resvg_skia as skia;

#[cfg(feature = "raqote-backend")]
pub use raqote;

pub use usvg::{self, Error};


#[cfg(feature = "cairo-backend")]
pub mod backend_cairo;

#[cfg(feature = "qt-backend")]
pub mod backend_qt;

#[cfg(feature = "skia-backend")]
pub mod backend_skia;

#[cfg(feature = "raqote-backend")]
pub mod backend_raqote;

pub mod utils;
mod filter;
mod geom;
mod image;
mod layers;
mod options;

/// Commonly used types and traits.
pub mod prelude {
    pub use usvg::{self, prelude::*};
    pub use crate::{geom::*, options::*, utils, OutputImage, Render};
}

pub use crate::geom::*;
pub use crate::options::*;


/// A generic interface for image rendering.
///
/// Instead of using backend implementation directly, you can
/// use this trait to write backend-independent code.
pub trait Render {
    /// Renders SVG to image.
    ///
    /// Returns `None` if an image allocation failed.
    fn render_to_image(
        &self,
        tree: &usvg::Tree,
        opt: &Options,
    ) -> Option<Box<dyn OutputImage>>;

    /// Renders SVG node to image.
    ///
    /// Returns `None` if an image allocation failed.
    fn render_node_to_image(
        &self,
        node: &usvg::Node,
        opt: &Options,
    ) -> Option<Box<dyn OutputImage>>;
}

/// A generic interface for output image.
pub trait OutputImage {
    /// Saves rendered image to the selected path.
    fn save_png(
        &mut self,
        path: &std::path::Path,
    ) -> bool;
}


/// Returns a default backend.
///
/// - If both backends are enabled - cairo backend will be returned.
/// - If no backends are enabled - will panic.
/// - Otherwise will return a corresponding backend.
#[allow(unreachable_code)]
pub fn default_backend() -> Box<dyn Render> {
    #[cfg(feature = "cairo-backend")]
    {
        return Box::new(backend_cairo::Backend);
    }

    #[cfg(feature = "qt-backend")]
    {
        return Box::new(backend_qt::Backend);
    }

    #[cfg(feature = "skia-backend")]
    {
        return Box::new(backend_skia::Backend);
    }

    #[cfg(feature = "raqote-backend")]
    {
        return Box::new(backend_raqote::Backend);
    }

    unreachable!("at least one backend must be enabled")
}

pub(crate) fn use_shape_antialiasing(
    mode: usvg::ShapeRendering,
) -> bool {
    match mode {
        usvg::ShapeRendering::OptimizeSpeed         => false,
        usvg::ShapeRendering::CrispEdges            => false,
        usvg::ShapeRendering::GeometricPrecision    => true,
    }
}

/// Converts an image to an alpha mask.
pub(crate) fn image_to_mask(
    data: &mut [u8],
    img_size: ScreenSize,
) {
    let width = img_size.width();
    let height = img_size.height();
    let stride = width * 4;

    let coeff_r = 0.2125 / 255.0;
    let coeff_g = 0.7154 / 255.0;
    let coeff_b = 0.0721 / 255.0;

    for y in 0..height {
        for x in 0..width {
            let idx = (y * stride + x * 4) as usize;

            let r = data[idx + 2] as f64;
            let g = data[idx + 1] as f64;
            let b = data[idx + 0] as f64;

            let luma = r * coeff_r + g * coeff_g + b * coeff_b;

            data[idx + 0] = 0;
            data[idx + 1] = 0;
            data[idx + 2] = 0;
            data[idx + 3] = f64_bound(0.0, luma * 255.0, 255.0) as u8;
        }
    }
}

pub(crate) trait ConvTransform<T> {
    fn to_native(&self) -> T;
    fn from_native(_: &T) -> Self;
}
