//! # lau-computer-graphics
//!
//! Computer graphics fundamentals — rendering, transforms, rasterization, ray tracing, and shading.
//!
//! Provides 2D/3D transformations, projection matrices, line drawing (Bresenham),
//! triangle rasterization (scanline), shading models (flat, Gouraud, Phong),
//! ray tracing basics, color models, Bézier curves/surfaces, and agent visualization.

pub mod color;
pub mod transform;
pub mod projection;
pub mod rasterize;
pub mod shading;
pub mod ray;
pub mod bezier;
pub mod agent_viz;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::transform::*;
    pub use crate::projection::*;
    pub use crate::rasterize::*;
    pub use crate::shading::*;
    pub use crate::ray::*;
    pub use crate::bezier::*;
    pub use crate::agent_viz::*;
}
