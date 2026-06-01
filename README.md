# lau-computer-graphics

> A teaching-quality Rust library covering the core of a computer-graphics course — 2D/3D transforms, projection, rasterization, ray tracing, shading models, Bézier curves, colour science, and agent-state visualisation — all built on `nalgebra` and fully tested.

**67 tests · zero unsafe · serde-serializable · `cargo add lau-computer-graphics`**

---

## What This Does

`lau-computer-graphics` implements the fundamental algorithms taught in a first course on computer graphics, with clean APIs and mathematical rigour:

| Module | What it covers |
|---|---|
| `color` | RGB ↔ HSV conversion, channel arithmetic, clamping, lerp |
| `transform` | 2D & 3D translation, rotation, scale via homogeneous matrices |
| `projection` | Perspective, orthographic, and look-at view matrices |
| `rasterize` | Bresenham line drawing, scanline triangle fill, `Framebuffer` |
| `shading` | Flat / Gouraud / Phong illumination, barycentric coordinates |
| `ray` | Ray–sphere and ray–plane intersection with hit normals |
| `bezier` | Quadratic, cubic, and bicubic Bézier curves & surfaces |
| `agent_viz` | Render agent state-space graphs with interactions to a framebuffer |

---

## The Key Idea

Computer graphics is the art of turning **mathematical descriptions** of scenes into **pixel images**. This crate walks through every stage of that pipeline:

1. **Modelling** — position objects in space with affine transforms (`transform`, `bezier`).
2. **Viewing** — place a virtual camera with the look-at matrix; project the 3D world onto a 2D image plane (`projection`).
3. **Rasterization** — decide which pixels each primitive covers: Bresenham for lines, scanline fill for triangles (`rasterize`).
4. **Shading** — compute the colour of each pixel using lighting equations — flat (per-face), Gouraud (per-vertex, interpolate), or Phong (interpolate normal, light per-pixel) (`shading`).
5. **Ray tracing** (alternative to rasterization) — fire a ray per pixel, find the closest intersection, and shade (`ray`).

The `agent_viz` module applies all of this to visualise abstract agent state-spaces: agents as coloured dots, interactions as lines, projected from an n-dimensional state space onto a 2D framebuffer.

---

## Install

```toml
[dependencies]
lau-computer-graphics = "0.1"
```

```bash
cargo add lau-computer-graphics
```

### Dependencies

| Crate | Purpose |
|---|---|
| `nalgebra` 0.33 | Linear-algebra types (vectors, matrices) |
| `serde` (derive) | Serialisable colour types |

---

## Quick Start

```rust
use lau_computer_graphics::prelude::*;

// --- Transforms ---
let m = translate_3d(1.0, 2.0, 3.0);
let p = Vector3::new(0.0, 0.0, 0.0);
let moved = transform_point_3d(&m, &p);
assert_eq!(moved, Vector3::new(1.0, 2.0, 3.0));

// --- Projection ---
let proj = perspective(std::f64::consts::FRAC_PI_4, 16.0 / 9.0, 0.1, 100.0);
let view = look_at(
    &Vector3::new(0.0, 0.0, 5.0),  // eye
    &Vector3::new(0.0, 0.0, 0.0),  // center
    &Vector3::new(0.0, 1.0, 0.0),  // up
);

// --- Ray tracing ---
let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
let sphere = Sphere { center: Vector3::zeros(), radius: 1.0 };
if let Some(hit) = ray_sphere_intersect(&ray, &sphere) {
    println!("Hit at t={}, point={:?}", hit.t, hit.point);
}

// --- Rasterization ---
let mut fb = Framebuffer::new(640, 480);
fb.draw_line(0, 0, 100, 200, 0xFFFFFF);

// --- Bézier curves ---
let p0 = Vector2::new(0.0, 0.0);
let p1 = Vector2::new(1.0, 3.0);
let p2 = Vector2::new(3.0, 3.0);
let p3 = Vector2::new(4.0, 0.0);
let mid = bezier_cubic(0.5, &p0, &p1, &p2, &p3);
```

---

## API Reference

### `color` — RGB and HSV colour models

```rust
// RGB in [0, 1]
let c = Rgb::new(0.8, 0.3, 0.5);
let c = Rgb::from_u8(255, 128, 0);
let clamped = c.clamp();        // clamp channels to [0, 1]
let mid = Rgb::lerp(&a, &b, 0.5); // linear interpolation

// Arithmetic
let sum = a + b;    // channel-wise addition
let bright = a * 2.0; // scalar multiply

// Convert to/from HSV
let hsv: Hsv = c.to_hsv();    // h ∈ [0, 360), s,v ∈ [0, 1]
let rgb: Rgb = hsv.to_rgb();
```

### `transform` — 2D & 3D affine transforms

```rust
// 2D (3×3 homogeneous)
translate_2d(tx, ty) -> Matrix3
rotate_2d(angle_rad)  -> Matrix3
scale_2d(sx, sy)      -> Matrix3
transform_point_2d(&m, &point) -> Vector2

// 3D (4×4 homogeneous)
translate_3d(tx, ty, tz) -> Matrix4
rotate_x(angle_rad)      -> Matrix4
rotate_y(angle_rad)      -> Matrix4
rotate_z(angle_rad)      -> Matrix4
scale_3d(sx, sy, sz)     -> Matrix4
transform_point_3d(&m, &point) -> Vector3

// Compose by matrix multiplication: let mvp = projection * view * model;
```

### `projection` — Camera projection

```rust
// Perspective: fov_y in radians, aspect = width/height, near/far > 0
perspective(fov_y, aspect, near, far) -> Matrix4

// Orthographic
orthographic(left, right, bottom, top, near, far) -> Matrix4

// View matrix (camera transform)
look_at(&eye, &center, &up) -> Matrix4
```

### `rasterize` — Drawing primitives

```rust
// Bresenham line: returns all integer (x, y) points
bresenham_line(x0, y0, x1, y1) -> Vec<(i32, i32)>

// Scanline triangle fill: all integer points inside the triangle
scanline_triangle(x0, y0, x1, y1, x2, y2) -> Vec<(i32, i32)>

// Framebuffer: 2D pixel grid (u32 RGB, packed 0xRRGGBB)
let mut fb = Framebuffer::new(width, height);
fb.set_pixel(x, y, color);
fb.get_pixel(x, y) -> Option<u32>
fb.draw_line(x0, y0, x1, y1, color);
```

### `shading` — Illumination models

```rust
// Material
let mat = Material {
    ambient: Rgb::new(0.1, 0.1, 0.1),
    diffuse: Rgb::new(0.7, 0.7, 0.7),
    specular: Rgb::new(1.0, 1.0, 1.0),
    shininess: 32.0,
};

// Light
let light = Light { position: Vector3::new(...), color: Rgb::new(1.0, 1.0, 1.0) };

// Flat shading (one normal per face)
flat_shade(&normal, &position, &light, &material, &view_pos) -> Rgb

// Gouraud shading (interpolate vertex colours via barycentric coords)
gouraud_shade(&[color_v0, color_v1, color_v2], &[u, v, w]) -> Rgb

// Phong shading (same as flat_shade but called per-pixel with interpolated normal)
phong_shade(&normal, &position, &light, &material, &view_pos) -> Rgb

// Utilities
face_normal(&v0, &v1, &v2) -> Vector3       // cross-product normal
barycentric(&p, &a, &b, &c) -> [f64; 3]     // barycentric coordinates
```

### `ray` — Ray tracing primitives

```rust
let ray = Ray::new(origin, direction);  // direction is auto-normalised
ray.at(t) -> Vector3;                   // point along ray

// Sphere intersection
let sphere = Sphere { center, radius };
ray_sphere_intersect(&ray, &sphere) -> Option<Hit>
// Hit { t: f64, point: Vector3, normal: Vector3 }

// Plane intersection (ax + by + cz = d)
let plane = Plane { normal, d };
ray_plane_intersect(&ray, &plane) -> Option<Hit>
```

### `bezier` — Parametric curves & surfaces

```rust
// 2D curves
bezier_quadratic(t, &p0, &p1, &p2) -> Vector2
bezier_cubic(t, &p0, &p1, &p2, &p3) -> Vector2
bezier_cubic_tangent(t, &p0, &p1, &p2, &p3) -> Vector2
sample_bezier_cubic(n, &p0, &p1, &p2, &p3) -> Vec<Vector2>  // n+1 evenly spaced samples

// 3D surface (4×4 control-point grid)
bezier_surface(u, v, &control_points) -> Vector3
```

### `agent_viz` — Agent state-space visualisation

```rust
let mut scene = AgentScene::new(
    Vector2::new(0.0, 0.0),  // min bound
    Vector2::new(10.0, 10.0), // max bound
);
scene.add_agent(Agent { id: 0, position: pos, state: vec![...], color: Rgb::red(), label: "A".into() });
scene.add_interaction(Interaction { from: 0, to: 1, strength: 1.0, color: Rgb::gray() });

let fb: Framebuffer = scene.render(800, 600);  // rasterise to pixel buffer

// State-space utilities
state_distance(&agent_a, &agent_b) -> f64    // Euclidean distance in state space
agents_within_radius(&agents, &center, radius) -> Vec<usize>
```

---

## How It Works

### Homogeneous Coordinates

2D points `(x, y)` are embedded in 3D as `(x, y, 1)` and 3D points `(x, y, z)` as `(x, y, z, 1)`. This lets us represent **translation** as a matrix multiply — which in turn means *any* sequence of translates, rotates, and scales can be composed into a single matrix by multiplication. The `transform_point_*` functions divide by the homogeneous `w` component after the multiply.

### Bresenham's Line Algorithm

A classic integer-only algorithm. It maintains an error term `err = dx + dy` (where `dy` is negative). At each step it increments `x` if `2·err ≥ dy` and increments `y` if `2·err ≤ dx`. No floating-point arithmetic, no division — just additions and comparisons. This is why it was invented for 1960s plotters and still used in GPUs.

### Scanline Triangle Rasterization

Sort the three vertices by Y coordinate. Walk horizontal scanlines from the bottom vertex to the top, interpolating the left and right edge X coordinates. Fill all pixels between the edges on each scanline.

### Ray–Sphere Intersection

Given ray **P**(**t**) = **O** + **D**t and sphere |**P** − **C**|² = r², substitute to get a quadratic in t:

  at² + bt + c = 0

where a = **D**·**D**, b = 2(**O**−**C**)·**D**, c = (**O**−**C**)·(**O**−**C**) − r². The discriminant determines hit (positive), miss (negative), or tangent (zero). This crate uses the half-b optimisation for numerical stability.

### Phong Reflection Model

The colour at a surface point is the sum of three terms:

**I** = **I**_ambient + **I**_diffuse + **I**_specular

- **Ambient:** constant approximation of indirect light. `k_a · L_a`
- **Diffuse (Lambertian):** proportional to `max(0, N·L)` — the cosine of the angle between the surface normal **N** and the light direction **L**.
- **Specular (Phong):** proportional to `max(0, R·V)^α` — the reflection of **L** about **N**, compared to the view direction **V**, raised to the shininess exponent α.

### Bézier Curves

A cubic Bézier curve is a weighted sum of four control points:

**B**(t) = (1−t)³**P**₀ + 3(1−t)²t**P**₁ + 3(1−t)t²**P**₂ + t³**P**₃

The Bernstein polynomials `(1−t)³, 3(1−t)²t, 3(1−t)t², t³` form a partition of unity (they sum to 1 for all t), so the curve is always inside the convex hull of its control points. Bicubic surfaces extend this to two parameters (u, v) over a 4×4 control-point grid.

---

## The Math

### Perspective Projection

The perspective matrix maps a view frustum to the normalised device cube [−1, 1]³:

$$M_{persp} = \begin{pmatrix} f/a & 0 & 0 & 0 \\ 0 & f & 0 & 0 \\ 0 & 0 & \frac{n+f}{n-f} & \frac{2nf}{n-f} \\ 0 & 0 & -1 & 0 \end{pmatrix}$$

where $f = 1/\tan(\text{fov}_y/2)$ and $a = \text{aspect}$. The $-1$ in the last row implements the perspective divide (w = −z), which makes farther objects appear smaller.

### Barycentric Coordinates

Any point **P** inside a triangle (**A**, **B**, **C**) can be written as **P** = u**A** + v**B** + w**C** where u + v + w = 1. These coordinates are computed by solving a 2×2 linear system derived from dot products. They are the key to Gouraud shading (interpolate vertex colours) and Phong shading (interpolate vertex normals).

### Reflection Vector

The reflection of incident vector **I** about normal **N** is:

**R** = **I** − 2(**I**·**N**)**N**

This is the standard formula used in the Phong specular term.

---

## Testing

**67 unit tests** covering:

- **Transforms:** identity, translation, rotation at 90°, composition, scaling
- **Projection:** perspective near/far plane NDC mapping, orthographic bounds, look-at orthonormality
- **Rasterization:** horizontal, vertical, diagonal, steep, and reverse Bresenham lines; triangle fill; framebuffer bounds
- **Ray tracing:** hits, misses, tangent hits, inside-sphere rays, parallel-plane misses, behind-ray planes, normal direction
- **Shading:** direct light, no light (behind surface), specular highlights, Gouraud interpolation, barycentric at vertices and centroid
- **Colour:** RGB↔HSV round-trip, clamping, lerp, channel arithmetic, edge cases (black, pure colours)
- **Bézier:** endpoints, midpoints, tangent direction, flat surfaces, sampling

```bash
cargo test
```

---

## License

MIT
