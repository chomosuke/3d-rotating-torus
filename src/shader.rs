use roots::{find_roots_quartic, Roots};

use crate::linear_alg::Vector;

pub struct View {
    pub camera: Vector,
    pub top_left: Vector,
    pub top_right: Vector,
    pub bottom_left: Vector,
    pub bottom_right: Vector,
    pub width: usize,
    pub height: usize,
}

pub fn get_frame(
    inner_radius: f64,
    outer_radius: f64,
    View {
        camera,
        top_left,
        top_right,
        bottom_left,
        bottom_right,
        width,
        height,
    }: View,
    light: Vector,
) -> Vec<Vec<u8>> {
    let mut frame = Vec::with_capacity(height);
    for i in 0..height {
        let mut row = Vec::with_capacity(width);
        for j in 0..width {
            let bottom = i as f64 / (height as f64 - 1.0);
            let top = 1.0 - bottom;
            let right = j as f64 / (width as f64 - 1.0);
            let left = 1.0 - right;
            row.push(get_pixel(
                inner_radius,
                outer_radius,
                camera,
                top_left * top * left
                    + top_right * top * right
                    + bottom_left * bottom * left
                    + bottom_right * bottom * right,
                light,
            ));
        }
        frame.push(row);
    }
    frame
}

/// The donut lay flat on the x-y plane centered around (0, 0, 0)
pub fn get_pixel(
    inner_radius: f64,
    outer_radius: f64,
    ray_eye: Vector,
    ray_screen: Vector,
    light: Vector,
) -> u8 {
    let ray_v = ray_eye - ray_screen;
    let r_path = (inner_radius + outer_radius) / 2.0;
    let r_circ = r_path - inner_radius;

    let Some(surface) = get_surface_intersection(r_path, r_circ, ray_eye, ray_v) else {
        return 0;
    };

    let projection = Vector {
        x: surface.x,
        y: surface.y,
        z: 0.0,
    };
    let center = projection * (r_path / projection.length());

    let normal = surface - center;
    let normal = normal / normal.length();
    let light_v = light - surface;
    let light_v = light_v / light_v.length();

    let light_angle = normal.x * light_v.x + normal.y * light_v.y + normal.z * light_v.z;
    let light_angle = light_angle.max(0.0);

    let brightness = light_angle * 255.99;
    (brightness as u8).max(1)
}

fn get_surface_intersection(
    r_path: f64,
    r_circ: f64,
    ray_eye: Vector,
    ray_v: Vector,
) -> Option<Vector> {
    let surfaces = [
        get_surface_intersection_via_z(r_path, r_circ, ray_eye, ray_v),
        get_surface_intersection_via_x(r_path, r_circ, ray_eye, ray_v),
        {
            let ray_eye = ray_eye.swap_x_y();
            let ray_v = ray_v.swap_x_y();
            get_surface_intersection_via_x(r_path, r_circ, ray_eye, ray_v).map(|s| s.swap_x_y())
        },
    ];

    let mut surface = None;
    let mut epsilon = r_circ / 20.0;
    for s in surfaces {
        // d1 = |z|
        // d2 = |R - sqrt(x^2 + y^2)|
        // d1^2 + d2^2 = r^2

        let Some(s) = s else {
            continue;
        };

        let d1 = s.z;
        let d2 = r_path - (s.x.powi(2) + s.y.powi(2)).sqrt();
        let d = (d1.powi(2) + d2.powi(2)).sqrt();
        let d = (d - r_circ).abs();

        if d <= epsilon {
            epsilon = d;
            surface = Some(s);
        }
    }

    surface
}

fn find_closest_root(roots: Roots<f64>, x_eye: f64, x_v: f64) -> Option<f64> {
    let xs = match roots {
        roots::Roots::No(x) => x.to_vec(),
        roots::Roots::One(x) => x.to_vec(),
        roots::Roots::Two(x) => x.to_vec(),
        roots::Roots::Three(x) => x.to_vec(),
        roots::Roots::Four(x) => x.to_vec(),
    };

    let mut x = None;
    for xi in xs {
        let d = x_eye - xi;
        if d.is_sign_positive() == x_v.is_sign_positive() {
            if let Some(x) = &mut x {
                let d2: f64 = x_eye - *x;
                if d.abs() < d2.abs() {
                    *x = xi
                }
            } else {
                x = Some(xi);
            }
        }
    }

    x
}

fn get_surface_intersection_via_x(
    r_path: f64,
    r_circ: f64,
    ray_eye: Vector,
    ray_v: Vector,
) -> Option<Vector> {
    let az = ray_v.z / ray_v.x;
    let bz = -(ray_v.z * ray_eye.x) / ray_v.x + ray_eye.z;
    let ay = ray_v.y / ray_v.x;
    let by = -(ray_v.y * ray_eye.x) / ray_v.x + ray_eye.y;

    // coefficient of lhs inside ^2
    let c2l = az.powi(2) + 1.0 + ay.powi(2);
    let c1l = 2.0 * az * bz + 2.0 * ay * by;
    let c0l = bz.powi(2) + by.powi(2) + r_path.powi(2) - r_circ.powi(2);

    // coefficient of rhs
    let c2r = 4.0 * r_path.powi(2) * (1.0 + ay.powi(2));
    let c1r = 4.0 * r_path.powi(2) * 2.0 * ay * by;
    let c0r = 4.0 * r_path.powi(2) * by.powi(2);

    // final coefficient
    let a = c2l.powi(2);
    let b = 2.0 * c2l * c1l;
    let c = 2.0 * c2l * c0l + c1l.powi(2) - c2r;
    let d = 2.0 * c1l * c0l - c1r;
    let e = c0l.powi(2) - c0r;

    let x = find_closest_root(find_roots_quartic(a, b, c, d, e), ray_eye.x, ray_v.x)?;

    let z = az * x + bz;
    let y = ay * x + by;

    Some(Vector { x, y, z })
}

fn get_surface_intersection_via_z(
    r_path: f64,
    r_circ: f64,
    ray_eye: Vector,
    ray_v: Vector,
) -> Option<Vector> {
    let ax = ray_v.x / ray_v.z;
    let bx = -(ray_v.x * ray_eye.z) / ray_v.z + ray_eye.x;
    let ay = ray_v.y / ray_v.z;
    let by = -(ray_v.y * ray_eye.z) / ray_v.z + ray_eye.y;

    // coefficient of lhs inside ^2
    let c2l = ax.powi(2) + ay.powi(2) + 1.0;
    let c1l = 2.0 * (ax * bx + ay * by);
    let c0l = bx.powi(2) + by.powi(2) + r_path.powi(2) - r_circ.powi(2);

    // coefficient of rhs
    let c2r = 4.0 * r_path.powi(2) * (ax.powi(2) + ay.powi(2));
    let c1r = 4.0 * r_path.powi(2) * 2.0 * (ax * bx + ay * by);
    let c0r = 4.0 * r_path.powi(2) * (bx.powi(2) + by.powi(2));

    // final coefficient
    let a = c2l.powi(2);
    let b = 2.0 * c2l * c1l;
    let c = 2.0 * c2l * c0l + c1l.powi(2) - c2r;
    let d = 2.0 * c1l * c0l - c1r;
    let e = c0l.powi(2) - c0r;

    let z = find_closest_root(find_roots_quartic(a, b, c, d, e), ray_eye.z, ray_v.z)?;

    let x = ax * z + bx;
    let y = ay * z + by;

    Some(Vector { x, y, z })
}
