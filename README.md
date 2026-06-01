# lau-computer-graphics

> Computer graphics fundamentals — rendering, transforms, rasterization, ray tracing, and shading

## What This Does

Computer graphics fundamentals — rendering, transforms, rasterization, ray tracing, and shading. Part of the PLATO/LAU ecosystem — a mathematically rigorous framework for building educational agents that learn, teach, and evolve.

## The Key Idea

This crate implements the core abstractions needed for its domain, with a focus on correctness, composability, and conservation guarantees. Every public type is serializable (serde), every algorithm is tested, and every invariant is verified.

## Install

```bash
cargo add lau-computer-graphics
```

## Quick Start

See the API Reference below for complete usage. Key entry points:

```rust
use lau_computer_graphics::*;
// See types and methods below for complete usage
```

## API Reference

```rust
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> 
pub fn scanline_triangle(
pub struct Framebuffer 
    pub fn new(width: usize, height: usize) -> Self 
    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) 
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u32> 
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) 
pub fn perspective(fov_y: f64, aspect: f64, near: f64, far: f64) -> Matrix4<f64> 
pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Matrix4<f64> 
pub fn look_at(eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) -> Matrix4<f64> 
pub fn translate_2d(tx: f64, ty: f64) -> Matrix3<f64> 
pub fn rotate_2d(angle_rad: f64) -> Matrix3<f64> 
pub fn scale_2d(sx: f64, sy: f64) -> Matrix3<f64> 
pub fn translate_3d(tx: f64, ty: f64, tz: f64) -> Matrix4<f64> 
pub fn rotate_x(angle_rad: f64) -> Matrix4<f64> 
pub fn rotate_y(angle_rad: f64) -> Matrix4<f64> 
pub fn rotate_z(angle_rad: f64) -> Matrix4<f64> 
pub fn scale_3d(sx: f64, sy: f64, sz: f64) -> Matrix4<f64> 
pub fn transform_point_2d(m: &Matrix3<f64>, p: &Vector2<f64>) -> Vector2<f64> 
pub fn transform_point_3d(m: &Matrix4<f64>, p: &Vector3<f64>) -> Vector3<f64> 
pub struct Light 
pub struct Material 
pub fn flat_shade(
pub fn gouraud_shade(
pub fn phong_shade(
pub fn face_normal(v0: &Vector3<f64>, v1: &Vector3<f64>, v2: &Vector3<f64>) -> Vector3<f64> 
pub fn barycentric(p: &Vector3<f64>, a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> [f64; 3] 
pub struct Ray 
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self 
    pub fn at(&self, t: f64) -> Vector3<f64> 
pub struct Sphere 
pub struct Hit 
pub fn ray_sphere_intersect(ray: &Ray, sphere: &Sphere) -> Option<Hit> 
pub struct Plane 
pub fn ray_plane_intersect(ray: &Ray, plane: &Plane) -> Option<Hit> 
pub fn bezier_cubic(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vector2<f64> 
pub fn bezier_quadratic(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>) -> Vector2<f64> 
pub fn bezier_cubic_tangent(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vector2<f64> 
pub fn bezier_surface(
pub fn sample_bezier_cubic(n: usize, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vec<Vector2<f64>> 
pub struct Rgb 
    pub fn new(r: f64, g: f64, b: f64) -> Self 
    pub fn from_u8(r: u8, g: u8, b: u8) -> Self 
    pub fn clamp(&self) -> Self 
    pub fn lerp(a: &Rgb, b: &Rgb, t: f64) -> Self 
    pub fn to_hsv(&self) -> Hsv 
pub struct Hsv 
    pub fn new(h: f64, s: f64, v: f64) -> Self 
    pub fn to_rgb(&self) -> Rgb 
pub struct Agent 
pub struct Interaction 
pub struct AgentScene 
    pub fn new(min_bound: Vector2<f64>, max_bound: Vector2<f64>) -> Self 
    pub fn add_agent(&mut self, agent: Agent) 
    pub fn add_interaction(&mut self, interaction: Interaction) 
    pub fn render(&self, width: usize, height: usize) -> Framebuffer 
pub fn state_distance(a: &Agent, b: &Agent) -> f64 
pub fn agents_within_radius(agents: &[Agent], center: &Agent, radius: f64) -> Vec<usize> 
```

## How It Works

Read the source in `src/` for full implementation details. All algorithms are documented with inline comments explaining the mathematical foundations.

## The Math

This crate implements formal mathematical constructs. See the source documentation for theorem statements and proofs of correctness.

## Testing

**67 tests** covering construction, serialization, correctness properties, edge cases, and composability with other lau-* crates.

## License

MIT
