use image::{RgbImage, ImageBuffer, Rgb, Rgb32FImage};
use noise::{NoiseFn, Perlin, Seedable, RidgedMulti, Fbm};
use rocky::{equirectangular_to_xyz, spherical_mapping_to_xyz};

fn main() {

    let dim_mult =  1;
    let width = 1024*dim_mult;
    let height = 512*dim_mult;

    let width = 1024*dim_mult;
    let height = 1024*dim_mult;


    let width64 = width as f64;
    let height64 = height as f64;

    let mut buffer: Rgb32FImage = ImageBuffer::new(width as u32, height as u32);
    //let perlin = Perlin::new(1);
    let ridged_multi = Fbm::<Perlin>::default();
    
    for (x, y, pixel) in buffer.enumerate_pixels_mut(){
        let freq = 20.0;
        // Convert pixel coordinates to longitude and latitude
        let longitude = (x as f64) / width64 * 360.0 - 180.0;
        let latitude = 90.0 - (y as f64) / height64 * 180.0;

        // Convert longitude and latitude to XYZ coordinates
        let x = latitude.to_radians().cos() * longitude.to_radians().sin();
        let y = latitude.to_radians().sin();
        let z = latitude.to_radians().cos() * longitude.to_radians().cos();

        let p = [x,y,z];
        let p = p.map(|x| x * freq);

        let v =  ridged_multi.get(p) ;
        let v = (v + 1.0) / 2.0;
        //let v = v * 255.0;
        let ir = v as f32;
        let ig = v as f32;
        let ib = v as f32;

        *pixel = Rgb::<f32>([ir, ig, ib]);
    }

    buffer.save("image.exr").unwrap();
}
