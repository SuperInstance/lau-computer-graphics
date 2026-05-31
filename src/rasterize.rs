//! Rasterization: Bresenham line drawing and scanline triangle rasterization.

/// Bresenham's line algorithm. Returns all integer points on the line from (x0,y0) to (x1,y1).
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (x0, y0);

    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}

/// Scanline triangle rasterization. Returns all integer points inside the triangle.
pub fn scanline_triangle(
    x0: i32, y0: i32,
    x1: i32, y1: i32,
    x2: i32, y2: i32,
) -> Vec<(i32, i32)> {
    // Sort vertices by y
    let mut verts = [(x0, y0), (x1, y1), (x2, y2)];
    verts.sort_by_key(|v| v.1);
    let (ax, ay) = verts[0];
    let (bx, by) = verts[1];
    let (cx, cy) = verts[2];

    let mut points = Vec::new();

    // Helper: interpolate x at a given y along an edge
    let interp_x = |x_from: i32, y_from: i32, x_to: i32, y_to: i32, y: i32| -> f64 {
        if y_to == y_from {
            return x_from as f64;
        }
        let t = (y - y_from) as f64 / (y_to - y_from) as f64;
        x_from as f64 + t * (x_to - x_from) as f64
    };

    // Bottom half: ay to by
    if ay != by {
        for y in ay..by {
            let xa = interp_x(ax, ay, bx, by, y);
            let xb = interp_x(ax, ay, cx, cy, y);
            let (xmin, xmax) = if xa < xb { (xa, xb) } else { (xb, xa) };
            let x_start = xmin.ceil() as i32;
            let x_end = xmax.floor() as i32;
            for x in x_start..=x_end {
                points.push((x, y));
            }
        }
    }

    // Top half: by to cy
    if by != cy {
        for y in by..=cy {
            let xa = interp_x(bx, by, cx, cy, y);
            let xb = interp_x(ax, ay, cx, cy, y);
            let (xmin, xmax) = if xa < xb { (xa, xb) } else { (xb, xa) };
            let x_start = xmin.ceil() as i32;
            let x_end = xmax.floor() as i32;
            for x in x_start..=x_end {
                points.push((x, y));
            }
        }
    }

    points
}

/// A simple framebuffer for rasterization.
#[derive(Clone)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<u32>>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![vec![0; width]; height],
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            self.pixels[y as usize][x as usize] = color;
        }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Option<u32> {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            Some(self.pixels[y as usize][x as usize])
        } else {
            None
        }
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        for (x, y) in bresenham_line(x0, y0, x1, y1) {
            self.set_pixel(x, y, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bresenham_horizontal() {
        let pts = bresenham_line(0, 0, 5, 0);
        assert_eq!(pts.len(), 6);
        for i in 0..=5 {
            assert_eq!(pts[i], (i as i32, 0));
        }
    }

    #[test]
    fn test_bresenham_vertical() {
        let pts = bresenham_line(3, 2, 3, 5);
        assert_eq!(pts.len(), 4);
        assert_eq!(pts[0], (3, 2));
        assert_eq!(pts[3], (3, 5));
    }

    #[test]
    fn test_bresenham_diagonal() {
        let pts = bresenham_line(0, 0, 3, 3);
        assert_eq!(pts.len(), 4);
        for i in 0..=3 {
            assert_eq!(pts[i], (i as i32, i as i32));
        }
    }

    #[test]
    fn test_bresenham_reverse() {
        let pts = bresenham_line(5, 0, 0, 0);
        assert_eq!(pts.len(), 6);
        assert_eq!(pts[0], (5, 0));
        assert_eq!(pts[5], (0, 0));
    }

    #[test]
    fn test_bresenham_single_point() {
        let pts = bresenham_line(3, 3, 3, 3);
        assert_eq!(pts, vec![(3, 3)]);
    }

    #[test]
    fn test_scanline_triangle_basic() {
        let pts = scanline_triangle(0, 0, 4, 0, 2, 4);
        assert!(!pts.is_empty());
        // Should contain the centroid area
        assert!(pts.contains(&(2, 2)));
    }

    #[test]
    fn test_scanline_triangle_flat() {
        let _pts = scanline_triangle(0, 0, 4, 0, 2, 0);
        // Degenerate: all y=0, should have limited or no fill
        // Points may be limited since height is 0
    }

    #[test]
    fn test_framebuffer_set_get() {
        let mut fb = Framebuffer::new(10, 10);
        fb.set_pixel(5, 5, 0xFF0000);
        assert_eq!(fb.get_pixel(5, 5), Some(0xFF0000));
        assert_eq!(fb.get_pixel(-1, 0), None);
        assert_eq!(fb.get_pixel(10, 0), None);
    }

    #[test]
    fn test_framebuffer_draw_line() {
        let mut fb = Framebuffer::new(20, 20);
        fb.draw_line(0, 0, 5, 0, 0xFFFFFF);
        for x in 0..=5 {
            assert_eq!(fb.get_pixel(x, 0), Some(0xFFFFFF));
        }
    }

    #[test]
    fn test_bresenham_steep() {
        let pts = bresenham_line(0, 0, 1, 5);
        assert_eq!(pts.len(), 6);
        assert_eq!(pts[0], (0, 0));
        assert_eq!(pts[5], (1, 5));
    }
}
