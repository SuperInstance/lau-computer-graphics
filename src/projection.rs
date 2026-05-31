//! Projection matrices: perspective, orthographic, and view matrix (look-at).

use nalgebra::{Matrix4, Vector3};

/// Perspective projection matrix.
///
/// * `fov_y` — vertical field of view in radians
/// * `aspect` — width / height
/// * `near` — distance to near plane (positive)
/// * `far` — distance to far plane (positive)
pub fn perspective(fov_y: f64, aspect: f64, near: f64, far: f64) -> Matrix4<f64> {
    let f = 1.0 / (fov_y * 0.5).tan();
    let range_inv = 1.0 / (near - far);

    Matrix4::new(
        f / aspect, 0.0, 0.0, 0.0,
        0.0, f, 0.0, 0.0,
        0.0, 0.0, (near + far) * range_inv, 2.0 * near * far * range_inv,
        0.0, 0.0, -1.0, 0.0,
    )
}

/// Orthographic projection matrix.
pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Matrix4<f64> {
    let rml = right - left;
    let tmb = top - bottom;
    let fmn = far - near;

    Matrix4::new(
        2.0 / rml, 0.0, 0.0, -(right + left) / rml,
        0.0, 2.0 / tmb, 0.0, -(top + bottom) / tmb,
        0.0, 0.0, -2.0 / fmn, -(far + near) / fmn,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Look-at view matrix.
pub fn look_at(eye: &Vector3<f64>, center: &Vector3<f64>, up: &Vector3<f64>) -> Matrix4<f64> {
    let f = (center - eye).normalize();
    let s = f.cross(up).normalize();
    let u = s.cross(&f);

    Matrix4::new(
        s.x, s.y, s.z, -s.dot(eye),
        u.x, u.y, u.z, -u.dot(eye),
        -f.x, -f.y, -f.z, f.dot(eye),
        0.0, 0.0, 0.0, 1.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_perspective_identity_corners() {
        // A 90° FOV, square aspect should map predictably
        let p = perspective(std::f64::consts::FRAC_PI_2, 1.0, 1.0, 100.0);
        // Near plane center should map to z = -1 in NDC
        let v = nalgebra::Vector4::new(0.0, 0.0, -1.0, 1.0);
        let result = p * v;
        assert_relative_eq!(result.z / result.w, -1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_perspective_far_plane() {
        let p = perspective(std::f64::consts::FRAC_PI_2, 1.0, 1.0, 100.0);
        let v = nalgebra::Vector4::new(0.0, 0.0, -100.0, 1.0);
        let result = p * v;
        assert_relative_eq!(result.z / result.w, 1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_orthographic_center() {
        let o = orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
        let v = nalgebra::Vector4::new(0.0, 0.0, -50.0, 1.0);
        let result = o * v;
        assert_relative_eq!(result.x / result.w, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y / result.w, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_orthographic_bounds() {
        let o = orthographic(-1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
        let v = nalgebra::Vector4::new(-1.0, 1.0, -0.1, 1.0);
        let result = o * v;
        assert_relative_eq!(result.x / result.w, -1.0, epsilon = 1e-10);
        assert_relative_eq!(result.y / result.w, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_look_at_forward() {
        let eye = Vector3::new(0.0, 0.0, 5.0);
        let center = Vector3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let v = look_at(&eye, &center, &up);
        // Eye position should map to origin in view space
        let p = nalgebra::Vector4::new(0.0, 0.0, 5.0, 1.0);
        let result = v * p;
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_look_at_orthonormal() {
        let eye = Vector3::new(5.0, 3.0, 10.0);
        let center = Vector3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let v = look_at(&eye, &center, &up);
        // Extract rotation part (3x3 upper-left) and check orthogonality
        let rot = v.fixed_view::<3, 3>(0, 0);
        let i = rot * rot.transpose();
        assert_relative_eq!(i[(0, 0)], 1.0, epsilon = 1e-10);
        assert_relative_eq!(i[(1, 1)], 1.0, epsilon = 1e-10);
        assert_relative_eq!(i[(2, 2)], 1.0, epsilon = 1e-10);
        assert_relative_eq!(i[(0, 1)], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_perspective_aspect_ratio() {
        let p = perspective(std::f64::consts::FRAC_PI_4, 2.0, 1.0, 100.0);
        // For FOV=45°, f = 1/tan(22.5°) ≈ 2.414. f/aspect = f/2 ≈ 1.207.
        // So p[(1,1)] = f ≈ 2.414, p[(0,0)] = f/2 ≈ 1.207
        assert_relative_eq!(p[(0, 0)] * 2.0, p[(1, 1)], epsilon = 1e-10);
    }
}
