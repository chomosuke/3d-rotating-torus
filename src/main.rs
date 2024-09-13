#![feature(thread_sleep_until)]
use std::{
    thread,
    time::{Duration, Instant},
};

use linear_alg::{rotate_y, rotate_z, Vector};
use shader::{get_frame, View};

mod linear_alg;
mod shader;

fn main() {
    let height = 2.0;
    let camera = Vector {
        x: 0.0,
        y: 0.0,
        z: -1.0 - height,
    };
    let screen_tl = Vector {
        x: -1.0,
        y: 1.0,
        z: -height,
    };
    let screen_tr = Vector {
        x: 1.0,
        y: 1.0,
        z: -height,
    };
    let screen_bl = Vector {
        x: -1.0,
        y: -1.0,
        z: -height,
    };
    let screen_br = Vector {
        x: 1.0,
        y: -1.0,
        z: -height,
    };
    let light = Vector {
        x: 0.0,
        y: 3.0,
        z: -0.5,
    };
    let coords = [camera, screen_tl, screen_tr, screen_bl, screen_br, light];

    let theta_z_frame = std::f64::consts::PI / 600.0 / 5.0;
    let theta_x_frame = std::f64::consts::PI / 120.0 / 5.0;
    let mut theta_z = 0.0;
    let mut theta_x = 0.0;

    let mut next_frame = Instant::now();
    loop {
        thread::sleep_until(next_frame);
        next_frame += Duration::from_secs_f32(1.0 / 30.0);
        let [camera, top_left, top_right, bottom_left, bottom_right, light] =
            // coords.map(|c| rotate_y(rotate_z(c, theta_z), theta_x));
            coords.map(|c| rotate_y(c, theta_x));

        let frame = get_frame(
            0.8,
            1.5,
            View {
                camera,
                top_left,
                top_right,
                bottom_left,
                bottom_right,
                width: 190,
                height: 90,
            },
            light,
        );

        // let grey_scale =
        //     r##".'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"##.as_bytes();
        let grey_scale = ".:-=+*#%@".as_bytes();
        // let grey_scale = "0123456789".as_bytes();
        for line in frame {
            for char in line {
                let char = char as usize;
                if char > 0 {
                    let i = char * grey_scale.len() / (u8::MAX as usize + 1);
                    print!("{}", grey_scale[i] as char);
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        theta_x += theta_x_frame;
        theta_z += theta_z_frame;
    }
}
