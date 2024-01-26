use image::{Rgba, GenericImageView, Pixel, ImageBuffer, Rgb};

pub fn magnitude(vector: [f32; 3]) -> f32 {
    let sum_of_squares = vector.iter().map(|&x| x * x).sum::<f32>();
    sum_of_squares.sqrt()
}

pub fn normalized(vector: [f32; 3]) -> [f32; 3] {
    let mag = magnitude(vector);
    vector.iter().map(|&x| x / mag).collect::<Vec<_>>().try_into().unwrap()
}

pub fn equirectangular_to_xyz(equirectangular: [f64;2]) -> [f64;3] {
    let longitude = equirectangular[0].to_radians();
    let latitude = equirectangular[1].to_radians() + 90.0;

    let x = latitude.cos() * longitude.sin();
    let y = latitude.sin();
    let z = latitude.cos() * longitude.cos();

    [x, y, z]
}

pub fn xyz_to_equirectangular(xyz: [f64;3]) -> [f64;2] {
    let phi = xyz[1].atan2(xyz[0]);
    let theta = xyz[2].acos();

    let longitude = phi.to_degrees();
    let latitude = theta.to_degrees() - 90.0;

    [longitude, latitude]
}

pub fn spherical_mapping_to_xyz(point: [f64;2], radius: f64) -> [f64;3] {
    let theta = point[1] * std::f64::consts::PI / 180.0;
    let phi = (90.0 - point[0]) * std::f64::consts::PI / 180.0;

    let x = radius * phi.sin() * theta.cos();
    let y = radius * phi.sin() * theta.sin();
    let z = radius * phi.cos();
    [x, y, z]
}

pub fn convert_cube_uv_to_xyz(index: usize, u: f32, v: f32) -> (f32, f32, f32) {
    // Convert range 0 to 1 to -1 to 1
    let uc = 2.0 * u - 1.0;
    let vc = 2.0 * v - 1.0;

    match index {
        0 => (1.0, vc, -uc),  // POSITIVE X
        1 => (-1.0, vc, uc),  // NEGATIVE X
        2 => (uc, 1.0, -vc),  // POSITIVE Y
        3 => (uc, -1.0, vc),  // NEGATIVE Y
        4 => (uc, vc, 1.0),   // POSITIVE Z
        5 => (-uc, vc, -1.0), // NEGATIVE Z
        _ => panic!("Invalid index"),
    }
}


// Perform bilinear interpolation to sample a pixel at a floating-point position (x, y)
pub fn bilinear_sample(image: &image::DynamicImage, x: f32, y: f32) -> Rgba<u8> {
    let (width, height) = image.dimensions();
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    if x0 >= width - 1 || y0 >= height - 1 {
        return image.get_pixel(x0, y0).to_rgba();
    }

    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    let image = image.as_rgba32f().unwrap();
    let p00 = image.get_pixel(x0, y0).to_rgba();
    let p10 = image.get_pixel(x1, y0).to_rgba();
    let p01 = image.get_pixel(x0, y1).to_rgba();
    let p11 = image.get_pixel(x1, y1).to_rgba();

    let interpolated_color = Rgba([
        lerp(lerp(p00[0], p10[0], dx), lerp(p01[0], p11[0], dx), dy) as u8,
        lerp(lerp(p00[1], p10[1], dx), lerp(p01[1], p11[1], dx), dy) as u8,
        lerp(lerp(p00[2], p10[2], dx), lerp(p01[2], p11[2], dx), dy) as u8,
        lerp(lerp(p00[3], p10[3], dx), lerp(p01[3], p11[3], dx), dy) as u8,
    ]);

    interpolated_color
}



// Perform bilinear interpolation to sample a pixel at a floating-point position (x, y)
pub fn bilinear_sample2(image: &ImageBuffer<Rgb<f32>, Vec<f32>>, x: f32, y: f32) -> Rgb<f32> {
    let (width, height) = image.dimensions();
    let x0 = x.floor() as u32;
    let y0 = y.floor() as u32;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    if x0 >= width - 1 || y0 >= height - 1 {
        return image.get_pixel(x0, y0).to_rgb();
    }

    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    let p00 = image.get_pixel(x0, y0).to_rgba();
    let p10 = image.get_pixel(x1, y0).to_rgba();
    let p01 = image.get_pixel(x0, y1).to_rgba();
    let p11 = image.get_pixel(x1, y1).to_rgba();

    let interpolated_color = Rgb([
        lerp(lerp(p00[0], p10[0], dx), lerp(p01[0], p11[0], dx), dy),
        lerp(lerp(p00[1], p10[1], dx), lerp(p01[1], p11[1], dx), dy),
        lerp(lerp(p00[2], p10[2], dx), lerp(p01[2], p11[2], dx), dy),
        //lerp(lerp(p00[3], p10[3], dx), lerp(p01[3], p11[3], dx), dy),
    ]);

    interpolated_color
}

// Linear interpolation function
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
