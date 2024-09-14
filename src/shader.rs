use roots::find_roots_quartic;

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

pub struct Frame {
    height: usize,
    width: usize,
    inner: Vec<u8>,
}

pub struct FrameIter<'a> {
    iter: &'a Frame,
    row: usize,
}

impl <'a> Iterator for FrameIter<'a> {
    type Item = FrameRow<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        //println!("height: {}", self.iter.height);
        if self.row >= self.iter.height {
            return None;
        }

        let out = FrameRow { bytes: self.iter, row: self.row, idx: 0 };
        self.row += 1;
        //println!("{}", self.row);

        Some(out)
    }
}

impl <'a> IntoIterator for &'a Frame {
    type IntoIter= FrameIter<'a>;
    type Item = FrameRow<'a>;
    fn into_iter(self) -> Self::IntoIter {
        FrameIter {
            iter: self,
            row: 0
        }
    }
}

pub struct FrameRow<'a> {
    bytes: &'a Frame,
    row: usize,
    idx: usize,
}

impl <'a> FrameRow<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        let start = self.row * self.bytes.height;
        let end = start + self.bytes.width;
        &self.bytes.inner[start..end]
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn chars(self) -> FrameChars<'a> {
        let Self {
            bytes,
            row,
            idx
        } = self;
        FrameChars {bytes, row, idx}
    }
}

pub struct FrameChars<'a> {
    bytes: &'a Frame,
    row: usize,
    idx: usize,
}

// let grey_scale =
//     r##".'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"##.as_bytes();
const GREY_SCALE: &[u8] = b".......::::::-----====+++**#%@";

impl <'a> Iterator for FrameChars<'a> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.bytes.width {
            return None;
        }
        let start = self.row * self.bytes.width;
        let idx = start + self.idx;
        self.idx += 1;

        let char = self.bytes.inner[idx] as usize;

        let c = if char > 0 {
            let i = char * GREY_SCALE.len() / (u8::MAX as usize + 1);
            GREY_SCALE[i] as char
        } else {
            ' '
        };
        Some(c)
    }
}




impl <'a> Iterator for FrameRow<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.bytes.width {
            return None;
        }
        let start = self.row * self.bytes.width;
        let idx = start + self.idx;
        self.idx += 1;
        Some(self.bytes.inner[idx])
    }
}

impl Frame {
    pub fn with_capacity(width: usize, height: usize) -> Self {
        let inner = Vec::with_capacity(width * height);
        Self {
            inner,
            width,
            height,
        }
    }

    pub fn push(&mut self, val: u8) {
        self.inner.push(val)
    }
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
) -> Frame {

    let mut frame = Frame::with_capacity(width, height);

    //let mut frame = Vec::with_capacity(height);
    for i in 0..height {
        //let mut row = Vec::with_capacity(width);
        for j in 0..width {
            let bottom = i as f64 / (height as f64 - 1.0);
            let top = 1.0 - bottom;
            let right = j as f64 / (width as f64 - 1.0);
            let left = 1.0 - right;
            frame.push(get_pixel(
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
        //frame.push(row);
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
    let mut ray_v = ray_eye - ray_screen;
    if ray_v.z == 0.0 {
        ray_v.z = f32::MIN_POSITIVE as f64;
    }
    let r_path = (inner_radius + outer_radius) / 2.0;
    let r_circ = r_path - inner_radius;

    let ax = ray_v.x / ray_v.z;
    let bx = -(ray_v.x * ray_eye.z) / ray_v.z + ray_eye.x;
    let ay = ray_v.y / ray_v.z;
    let by = -(ray_v.y * ray_eye.z) / ray_v.z + ray_eye.y;

    // coefficient of lhs in side ^2
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

    let roots = find_roots_quartic(a, b, c, d, e);
    let zs = match roots {
        roots::Roots::No(z) => z.to_vec(),
        roots::Roots::One(z) => z.to_vec(),
        roots::Roots::Two(z) => z.to_vec(),
        roots::Roots::Three(z) => z.to_vec(),
        roots::Roots::Four(z) => z.to_vec(),
    };

    let mut z = None;
    for zi in zs {
        let d = ray_eye.z - zi;
        if d.is_sign_positive() == ray_v.z.is_sign_positive() {
            if let Some(z) = &mut z {
                let d2: f64 = ray_eye.z - *z;
                if d.abs() < d2.abs() {
                    *z = zi
                }
            } else {
                z = Some(zi);
            }
        }
    }

    let Some(z) = z else {
        return 0;
    };

    let x = ax * z + bx;
    let y = ay * z + by;

    let surface = Vector { x, y, z };

    let projection = Vector { x, y, z: 0.0 };
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

// #[test]
// fn test() {}
