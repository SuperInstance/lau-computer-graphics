//! 2D/3D transformations with homogeneous coordinates.

use nalgebra::{Matrix3, Matrix4, Vector2, Vector3, Vector4};
// Transform module uses nalgebra; serde available via crate-level re-export

/// Build a 2D translation matrix.
pub fn translate_2d(tx: f64, ty: f64) -> Matrix3<f64> {
    let mut m = Matrix3::identity();
    m[(0, 2)] = tx;
    m[(1, 2)] = ty;
    m
}

/// Build a 2D rotation matrix (angle in radians).
pub fn rotate_2d(angle_rad: f64) -> Matrix3<f64> {
    let c = angle_rad.cos();
    let s = angle_rad.sin();
    Matrix3::new(c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0)
}

/// Build a 2D scale matrix.
pub fn scale_2d(sx: f64, sy: f64) -> Matrix3<f64> {
    Matrix3::new(sx, 0.0, 0.0, 0.0, sy, 0.0, 0.0, 0.0, 1.0)
}

/// Build a 3D translation matrix.
pub fn translate_3d(tx: f64, ty: f64, tz: f64) -> Matrix4<f64> {
    let mut m = Matrix4::identity();
    m[(0, 3)] = tx;
    m[(1, 3)] = ty;
    m[(2, 3)] = tz;
    m
}

/// Build a 3D rotation matrix around the X axis.
pub fn rotate_x(angle_rad: f64) -> Matrix4<f64> {
    let c = angle_rad.cos();
    let s = angle_rad.sin();
    Matrix4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, c, -s, 0.0,
        0.0, s, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Build a 3D rotation matrix around the Y axis.
pub fn rotate_y(angle_rad: f64) -> Matrix4<f64> {
    let c = angle_rad.cos();
    let s = angle_rad.sin();
    Matrix4::new(
        c, 0.0, s, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -s, 0.0, c, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Build a 3D rotation matrix around the Z axis.
pub fn rotate_z(angle_rad: f64) -> Matrix4<f64> {
    let c = angle_rad.cos();
    let s = angle_rad.sin();
    Matrix4::new(
        c, -s, 0.0, 0.0,
        s, c, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Build a 3D scale matrix.
pub fn scale_3d(sx: f64, sy: f64, sz: f64) -> Matrix4<f64> {
    Matrix4::new(
        sx, 0.0, 0.0, 0.0,
        0.0, sy, 0.0, 0.0,
        0.0, 0.0, sz, 0.0,
        0.0, 0.0, 0.0, 1.0,
    )
}

/// Transform a 2D point by a 3x3 homogeneous matrix.
pub fn transform_point_2d(m: &Matrix3<f64>, p: &Vector2<f64>) -> Vector2<f64> {
    let homo = m * Vector3::new(p.x, p.y, 1.0);
    Vector2::new(homo.x / homo.z, homo.y / homo.z)
}

/// Transform a 3D point by a 4x4 homogeneous matrix.
pub fn transform_point_3d(m: &Matrix4<f64>, p: &Vector3<f64>) -> Vector3<f64> {
    let homo = m * Vector4::new(p.x, p.y, p.z, 1.0);
    Vector3::new(homo.x / homo.w, homo.y / homo.w, homo.z / homo.w)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_translate_2d() {
        let m = translate_2d(3.0, 4.0);
        let p = Vector2::new(1.0, 2.0);
        let result = transform_point_2d(&m, &p);
        assert_relative_eq!(result.x, 4.0);
        assert_relative_eq!(result.y, 6.0);
    }

    #[test]
    fn test_rotate_2d_90() {
        let m = rotate_2d(std::f64::consts::FRAC_PI_2);
        let p = Vector2::new(1.0, 0.0);
        let result = transform_point_2d(&m, &p);
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_scale_2d() {
        let m = scale_2d(2.0, 3.0);
        let p = Vector2::new(1.0, 1.0);
        let result = transform_point_2d(&m, &p);
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 3.0);
    }

    #[test]
    fn test_compose_2d() {
        // Translate then rotate: translate (1,0) then rotate 90° → (0,1)
        let t = translate_2d(1.0, 0.0);
        let r = rotate_2d(std::f64::consts::FRAC_PI_2);
        let composed = r * t;
        let p = Vector2::new(0.0, 0.0);
        let result = transform_point_2d(&composed, &p);
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_translate_3d() {
        let m = translate_3d(1.0, 2.0, 3.0);
        let p = Vector3::new(0.0, 0.0, 0.0);
        let result = transform_point_3d(&m, &p);
        assert_relative_eq!(result.x, 1.0);
        assert_relative_eq!(result.y, 2.0);
        assert_relative_eq!(result.z, 3.0);
    }

    #[test]
    fn test_rotate_x_90() {
        let m = rotate_x(std::f64::consts::FRAC_PI_2);
        let p = Vector3::new(0.0, 1.0, 0.0);
        let result = transform_point_3d(&m, &p);
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.z, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rotate_y_90() {
        let m = rotate_y(std::f64::consts::FRAC_PI_2);
        let p = Vector3::new(1.0, 0.0, 0.0);
        let result = transform_point_3d(&m, &p);
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.z, -1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_rotate_z_90() {
        let m = rotate_z(std::f64::consts::FRAC_PI_2);
        let p = Vector3::new(1.0, 0.0, 0.0);
        let result = transform_point_3d(&m, &p);
        assert_relative_eq!(result.x, 0.0, epsilon = 1e-10);
        assert_relative_eq!(result.y, 1.0, epsilon = 1e-10);
        assert_relative_eq!(result.z, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_scale_3d() {
        let m = scale_3d(2.0, 3.0, 4.0);
        let p = Vector3::new(1.0, 1.0, 1.0);
        let result = transform_point_3d(&m, &p);
        assert_relative_eq!(result.x, 2.0);
        assert_relative_eq!(result.y, 3.0);
        assert_relative_eq!(result.z, 4.0);
    }

    #[test]
    fn test_compose_3d() {
        let t = translate_3d(5.0, 0.0, 0.0);
        let s = scale_3d(2.0, 2.0, 2.0);
        let composed = s * t; // scale then translate
        let p = Vector3::new(1.0, 0.0, 0.0);
        let result = transform_point_3d(&composed, &p);
        // translate: (6,0,0), then scale: (12,0,0)
        assert_relative_eq!(result.x, 12.0);
        assert_relative_eq!(result.y, 0.0);
        assert_relative_eq!(result.z, 0.0);
    }
}
