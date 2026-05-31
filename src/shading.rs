//! Shading models: flat, Gouraud, and Phong.

use nalgebra::Vector3;
use crate::color::Rgb;

/// A light source at a position with a color.
#[derive(Debug, Clone)]
pub struct Light {
    pub position: Vector3<f64>,
    pub color: Rgb,
}

/// Material properties for shading.
#[derive(Debug, Clone)]
pub struct Material {
    pub ambient: Rgb,
    pub diffuse: Rgb,
    pub specular: Rgb,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Rgb::new(0.1, 0.1, 0.1),
            diffuse: Rgb::new(0.7, 0.7, 0.7),
            specular: Rgb::new(1.0, 1.0, 1.0),
            shininess: 32.0,
        }
    }
}

/// Flat shading: uses a single normal for the entire face.
pub fn flat_shade(
    normal: &Vector3<f64>,
    position: &Vector3<f64>,
    light: &Light,
    material: &Material,
    view_pos: &Vector3<f64>,
) -> Rgb {
    let n = normal.normalize();
    let light_dir = (light.position - position).normalize();
    let view_dir = (view_pos - position).normalize();
    let reflect_dir = reflect(&(-light_dir), &n);

    let ambient = Rgb::new(
        material.ambient.r * light.color.r,
        material.ambient.g * light.color.g,
        material.ambient.b * light.color.b,
    );

    let diff = n.dot(&light_dir).max(0.0);
    let diffuse = Rgb::new(
        material.diffuse.r * light.color.r * diff,
        material.diffuse.g * light.color.g * diff,
        material.diffuse.b * light.color.b * diff,
    );

    let spec = reflect_dir.dot(&view_dir).max(0.0).powf(material.shininess);
    let specular = Rgb::new(
        material.specular.r * light.color.r * spec,
        material.specular.g * light.color.g * spec,
        material.specular.b * light.color.b * spec,
    );

    (ambient + diffuse + specular).clamp()
}

/// Gouraud shading: compute lighting at vertices and interpolate.
pub fn gouraud_shade(
    vertex_colors: &[Rgb; 3],
    barycentric: &[f64; 3],
) -> Rgb {
    Rgb::new(
        vertex_colors[0].r * barycentric[0] + vertex_colors[1].r * barycentric[1] + vertex_colors[2].r * barycentric[2],
        vertex_colors[0].g * barycentric[0] + vertex_colors[1].g * barycentric[1] + vertex_colors[2].g * barycentric[2],
        vertex_colors[0].b * barycentric[0] + vertex_colors[1].b * barycentric[1] + vertex_colors[2].b * barycentric[2],
    )
}

/// Phong shading: interpolate normal and compute lighting per-pixel.
pub fn phong_shade(
    normal: &Vector3<f64>,
    position: &Vector3<f64>,
    light: &Light,
    material: &Material,
    view_pos: &Vector3<f64>,
) -> Rgb {
    // Same as flat_shade but called per-pixel with interpolated normal
    flat_shade(normal, position, light, material, view_pos)
}

/// Compute reflection vector.
fn reflect(incident: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
    incident - 2.0 * incident.dot(normal) * normal
}

/// Compute a face normal from three vertices (counter-clockwise winding).
pub fn face_normal(v0: &Vector3<f64>, v1: &Vector3<f64>, v2: &Vector3<f64>) -> Vector3<f64> {
    let edge1 = v1 - v0;
    let edge2 = v2 - v0;
    edge1.cross(&edge2).normalize()
}

/// Compute barycentric coordinates of point p in triangle (a, b, c).
pub fn barycentric(p: &Vector3<f64>, a: &Vector3<f64>, b: &Vector3<f64>, c: &Vector3<f64>) -> [f64; 3] {
    let v0 = b - a;
    let v1 = c - a;
    let v2 = p - a;
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denom = d00 * d11 - d01 * d01;
    if denom.abs() < 1e-10 {
        return [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
    }
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;
    [u, v, w]
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_face_normal() {
        let v0 = Vector3::new(0.0, 0.0, 0.0);
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let n = face_normal(&v0, &v1, &v2);
        assert_relative_eq!(n.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(n.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(n.z, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_flat_shade_direct_light() {
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let position = Vector3::new(0.0, 0.0, 0.0);
        let light = Light {
            position: Vector3::new(0.0, 0.0, 5.0),
            color: Rgb::new(1.0, 1.0, 1.0),
        };
        let material = Material::default();
        let view = Vector3::new(0.0, 0.0, 5.0);
        let color = flat_shade(&normal, &position, &light, &material, &view);
        // Should be bright (direct light from front)
        assert!(color.r > 0.5);
        assert!(color.g > 0.5);
        assert!(color.b > 0.5);
    }

    #[test]
    fn test_flat_shade_no_light() {
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let position = Vector3::new(0.0, 0.0, 0.0);
        let light = Light {
            position: Vector3::new(0.0, 0.0, -5.0), // behind surface
            color: Rgb::new(1.0, 1.0, 1.0),
        };
        let material = Material::default();
        let view = Vector3::new(0.0, 0.0, 5.0);
        let color = flat_shade(&normal, &position, &light, &material, &view);
        // Only ambient should contribute
        assert!(color.r < 0.2);
    }

    #[test]
    fn test_gouraud_shade() {
        let colors = [
            Rgb::new(1.0, 0.0, 0.0),
            Rgb::new(0.0, 1.0, 0.0),
            Rgb::new(0.0, 0.0, 1.0),
        ];
        let bary = [1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0];
        let result = gouraud_shade(&colors, &bary);
        assert_relative_eq!(result.r, 1.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(result.g, 1.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(result.b, 1.0 / 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_barycentric_center() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(1.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 1.0, 0.0);
        let p = Vector3::new(1.0 / 3.0, 1.0 / 3.0, 0.0);
        let bary = barycentric(&p, &a, &b, &c);
        assert_relative_eq!(bary[0], 1.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(bary[1], 1.0 / 3.0, epsilon = 1e-10);
        assert_relative_eq!(bary[2], 1.0 / 3.0, epsilon = 1e-10);
    }

    #[test]
    fn test_barycentric_vertex() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(1.0, 0.0, 0.0);
        let c = Vector3::new(0.0, 1.0, 0.0);
        let bary = barycentric(&a, &a, &b, &c);
        assert_relative_eq!(bary[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(bary[1], 0.0, epsilon = 1e-10);
        assert_relative_eq!(bary[2], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_specular_highlight() {
        let normal = Vector3::new(0.0, 0.0, 1.0);
        let position = Vector3::new(0.0, 0.0, 0.0);
        let light = Light {
            position: Vector3::new(0.0, 0.0, 5.0),
            color: Rgb::new(1.0, 1.0, 1.0),
        };
        let material = Material {
            shininess: 64.0,
            ..Material::default()
        };
        let view = Vector3::new(0.0, 0.0, 5.0);
        let color = flat_shade(&normal, &position, &light, &material, &view);
        // Direct specular should be strong
        assert!(color.r > 0.9);
    }
}
