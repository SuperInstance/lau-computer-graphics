//! Bézier curves and surfaces.

use nalgebra::Vector2;
use nalgebra::Vector3;

/// Evaluate a cubic Bézier curve at parameter t.
pub fn bezier_cubic(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vector2<f64> {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
}

/// Evaluate a quadratic Bézier curve at parameter t.
pub fn bezier_quadratic(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>) -> Vector2<f64> {
    let u = 1.0 - t;
    let uu = u * u;
    let tt = t * t;

    uu * p0 + 2.0 * u * t * p1 + tt * p2
}

/// Tangent of a cubic Bézier curve at parameter t.
pub fn bezier_cubic_tangent(t: f64, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vector2<f64> {
    let u = 1.0 - t;
    3.0 * u * u * (p1 - p0) + 6.0 * u * t * (p2 - p1) + 3.0 * t * t * (p3 - p2)
}

/// Evaluate a bicubic Bézier surface at (u, v).
pub fn bezier_surface(
    u: f64, v: f64,
    control_points: &[[Vector3<f64>; 4]; 4],
) -> Vector3<f64> {
    // De Casteljau in u for each row, then in v
    let mut row_points = [Vector3::zeros(); 4];
    for i in 0..4 {
        row_points[i] = bezier_cubic_3d(u,
            &control_points[i][0],
            &control_points[i][1],
            &control_points[i][2],
            &control_points[i][3],
        );
    }
    bezier_cubic_3d(v, &row_points[0], &row_points[1], &row_points[2], &row_points[3])
}

fn bezier_cubic_3d(t: f64, p0: &Vector3<f64>, p1: &Vector3<f64>, p2: &Vector3<f64>, p3: &Vector3<f64>) -> Vector3<f64> {
    let u = 1.0 - t;
    let uu = u * u;
    let uuu = uu * u;
    let tt = t * t;
    let ttt = tt * t;

    uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
}

/// Sample a Bézier curve at evenly spaced t values.
pub fn sample_bezier_cubic(n: usize, p0: &Vector2<f64>, p1: &Vector2<f64>, p2: &Vector2<f64>, p3: &Vector2<f64>) -> Vec<Vector2<f64>> {
    (0..=n)
        .map(|i| {
            let t = i as f64 / n as f64;
            bezier_cubic(t, p0, p1, p2, p3)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_bezier_cubic_endpoints() {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(1.0, 2.0);
        let p2 = Vector2::new(3.0, 2.0);
        let p3 = Vector2::new(4.0, 0.0);

        let start = bezier_cubic(0.0, &p0, &p1, &p2, &p3);
        assert_relative_eq!(start.x, 0.0);
        assert_relative_eq!(start.y, 0.0);

        let end = bezier_cubic(1.0, &p0, &p1, &p2, &p3);
        assert_relative_eq!(end.x, 4.0);
        assert_relative_eq!(end.y, 0.0);
    }

    #[test]
    fn test_bezier_cubic_midpoint() {
        // Linear control points → straight line
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(1.0, 0.0);
        let p2 = Vector2::new(2.0, 0.0);
        let p3 = Vector2::new(3.0, 0.0);
        let mid = bezier_cubic(0.5, &p0, &p1, &p2, &p3);
        assert_relative_eq!(mid.x, 1.5);
    }

    #[test]
    fn test_bezier_quadratic_endpoints() {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(1.0, 2.0);
        let p2 = Vector2::new(2.0, 0.0);

        let start = bezier_quadratic(0.0, &p0, &p1, &p2);
        assert_relative_eq!(start.x, 0.0);

        let end = bezier_quadratic(1.0, &p0, &p1, &p2);
        assert_relative_eq!(end.x, 2.0);
    }

    #[test]
    fn test_bezier_quadratic_midpoint() {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(1.0, 2.0);
        let p2 = Vector2::new(2.0, 0.0);
        let mid = bezier_quadratic(0.5, &p0, &p1, &p2);
        assert_relative_eq!(mid.x, 1.0);
        assert_relative_eq!(mid.y, 1.0);
    }

    #[test]
    fn test_bezier_cubic_tangent() {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(1.0, 3.0);
        let p2 = Vector2::new(3.0, 3.0);
        let p3 = Vector2::new(4.0, 0.0);
        let tan = bezier_cubic_tangent(0.0, &p0, &p1, &p2, &p3);
        // Tangent at t=0 should be proportional to (p1-p0)
        assert_relative_eq!(tan.x, 3.0);
        assert_relative_eq!(tan.y, 9.0);
    }

    #[test]
    fn test_bezier_surface_corner() {
        let pts = [[Vector3::new(0.0, 0.0, 0.0); 4]; 4];
        // All zeros → surface should be zero everywhere
        let p = bezier_surface(0.5, 0.5, &pts);
        assert_relative_eq!(p.x, 0.0);
        assert_relative_eq!(p.y, 0.0);
        assert_relative_eq!(p.z, 0.0);
    }

    #[test]
    fn test_bezier_surface_flat() {
        // Flat grid control points
        let pts = [
            [Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(3.0, 0.0, 0.0)],
            [Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 1.0, 0.0), Vector3::new(2.0, 1.0, 0.0), Vector3::new(3.0, 1.0, 0.0)],
            [Vector3::new(0.0, 2.0, 0.0), Vector3::new(1.0, 2.0, 0.0), Vector3::new(2.0, 2.0, 0.0), Vector3::new(3.0, 2.0, 0.0)],
            [Vector3::new(0.0, 3.0, 0.0), Vector3::new(1.0, 3.0, 0.0), Vector3::new(2.0, 3.0, 0.0), Vector3::new(3.0, 3.0, 0.0)],
        ];
        let p = bezier_surface(0.5, 0.5, &pts);
        assert_relative_eq!(p.x, 1.5, epsilon = 1e-10);
        assert_relative_eq!(p.y, 1.5, epsilon = 1e-10);
        assert_relative_eq!(p.z, 0.0);
    }

    #[test]
    fn test_sample_bezier_cubic() {
        let p0 = Vector2::new(0.0, 0.0);
        let p1 = Vector2::new(0.0, 0.0);
        let p2 = Vector2::new(1.0, 0.0);
        let p3 = Vector2::new(1.0, 0.0);
        let samples = sample_bezier_cubic(4, &p0, &p1, &p2, &p3);
        assert_eq!(samples.len(), 5);
        assert_relative_eq!(samples[0].x, 0.0);
        assert_relative_eq!(samples[4].x, 1.0);
    }
}
