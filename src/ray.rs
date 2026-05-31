//! Ray tracing basics: ray-sphere and ray-plane intersection.

use nalgebra::Vector3;

/// A ray with origin and direction.
#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Point at parameter t.
    pub fn at(&self, t: f64) -> Vector3<f64> {
        self.origin + self.direction * t
    }
}

/// A sphere defined by center and radius.
#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Vector3<f64>,
    pub radius: f64,
}

/// Result of a ray-sphere intersection.
#[derive(Debug, Clone)]
pub struct Hit {
    pub t: f64,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
}

/// Test intersection of ray with sphere. Returns Hit with smallest positive t, or None.
pub fn ray_sphere_intersect(ray: &Ray, sphere: &Sphere) -> Option<Hit> {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.dot(&ray.direction); // should be 1 if normalized
    let half_b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius * sphere.radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return None;
    }

    let sqrt_d = discriminant.sqrt();
    let mut t = (-half_b - sqrt_d) / a;
    if t < 1e-6 {
        t = (-half_b + sqrt_d) / a;
    }
    if t < 1e-6 {
        return None;
    }

    let point = ray.at(t);
    let normal = (point - sphere.center) / sphere.radius;

    Some(Hit { t, point, normal })
}

/// A plane defined by normal and distance from origin (ax + by + cz = d).
#[derive(Debug, Clone)]
pub struct Plane {
    pub normal: Vector3<f64>,
    pub d: f64,
}

/// Test intersection of ray with plane. Returns Hit or None.
pub fn ray_plane_intersect(ray: &Ray, plane: &Plane) -> Option<Hit> {
    let denom = plane.normal.dot(&ray.direction);
    if denom.abs() < 1e-10 {
        return None; // parallel
    }
    let t = (plane.d - plane.normal.dot(&ray.origin)) / denom;
    if t < 1e-6 {
        return None;
    }
    let point = ray.at(t);
    let normal = plane.normal.normalize();
    Some(Hit { t, point, normal })
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_ray_at() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let p = ray.at(5.0);
        assert_relative_eq!(p.x, 5.0);
        assert_relative_eq!(p.y, 0.0);
    }

    #[test]
    fn test_ray_sphere_hit() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let hit = ray_sphere_intersect(&ray, &sphere).unwrap();
        assert!(hit.t > 0.0);
        assert_relative_eq!(hit.t, 4.0, epsilon = 1e-6);
        assert_relative_eq!(hit.point.z, -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ray_sphere_miss() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            center: Vector3::new(5.0, 0.0, 0.0),
            radius: 1.0,
        };
        assert!(ray_sphere_intersect(&ray, &sphere).is_none());
    }

    #[test]
    fn test_ray_sphere_inside() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 2.0,
        };
        let hit = ray_sphere_intersect(&ray, &sphere).unwrap();
        assert_relative_eq!(hit.t, 2.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ray_sphere_tangent() {
        let ray = Ray::new(Vector3::new(1.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let hit = ray_sphere_intersect(&ray, &sphere);
        // Tangent: discriminant ≈ 0
        assert!(hit.is_some());
    }

    #[test]
    fn test_ray_plane_hit() {
        let ray = Ray::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let plane = Plane {
            normal: Vector3::new(0.0, 1.0, 0.0),
            d: 0.0, // y = 0
        };
        let hit = ray_plane_intersect(&ray, &plane).unwrap();
        assert_relative_eq!(hit.t, 1.0, epsilon = 1e-6);
        assert_relative_eq!(hit.point.y, 0.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ray_plane_miss_parallel() {
        let ray = Ray::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let plane = Plane {
            normal: Vector3::new(0.0, 1.0, 0.0),
            d: 0.0,
        };
        assert!(ray_plane_intersect(&ray, &plane).is_none());
    }

    #[test]
    fn test_ray_plane_behind() {
        let ray = Ray::new(Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 1.0, 0.0));
        let plane = Plane {
            normal: Vector3::new(0.0, 1.0, 0.0),
            d: 0.0,
        };
        assert!(ray_plane_intersect(&ray, &plane).is_none());
    }

    #[test]
    fn test_ray_sphere_normal_direction() {
        let ray = Ray::new(Vector3::new(0.0, 0.0, -5.0), Vector3::new(0.0, 0.0, 1.0));
        let sphere = Sphere {
            center: Vector3::new(0.0, 0.0, 0.0),
            radius: 1.0,
        };
        let hit = ray_sphere_intersect(&ray, &sphere).unwrap();
        // Normal should point away from center at hit point
        assert_relative_eq!(hit.normal.z, -1.0, epsilon = 1e-6);
    }

    #[test]
    fn test_ray_plane_offset() {
        let ray = Ray::new(Vector3::new(0.0, 5.0, 0.0), Vector3::new(0.0, -1.0, 0.0));
        let plane = Plane {
            normal: Vector3::new(0.0, 1.0, 0.0),
            d: 2.0, // y = 2
        };
        let hit = ray_plane_intersect(&ray, &plane).unwrap();
        assert_relative_eq!(hit.t, 3.0, epsilon = 1e-6);
        assert_relative_eq!(hit.point.y, 2.0, epsilon = 1e-6);
    }
}
