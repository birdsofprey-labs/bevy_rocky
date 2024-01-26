use image::{RgbImage, ImageBuffer, Rgb, Rgb32FImage};
use noise::{NoiseFn, Perlin, Seedable, RidgedMulti, Fbm, MultiFractal};
use rocky::{equirectangular_to_xyz, spherical_mapping_to_xyz, convert_cube_uv_to_xyz, bilinear_sample2, normalized};

fn main() {
    let freq = 4.0*2.0;
    let px_normal_radius = 0.999;
    let dim =  8*1024u32;

    let dimf = dim as f32;

    //let mut buffer: Rgb32FImage = ImageBuffer::new(dim as u32, dim as u32);
    //let perlin = Perlin::new(1);
   // let ridged_multi = Fbm::<Perlin>::default();
    //let ridged_multi = Fbm::<Perlin>::default();
    let ridged_multi = RidgedMulti::<Perlin>::default();
    
    let ridged_multi = ridged_multi.set_octaves(10);

    let ridged_multi2 = Fbm::<Perlin>::default();
    let ridged_multi2 = ridged_multi2.set_octaves(9);
    
    for face in 0..6 {
        let mut face_image = ImageBuffer::new(dim, dim);

    for (x, y, pixel) in face_image.enumerate_pixels_mut(){
       
        // Convert pixel coordinates to longitude and latitude
       let (u,v) = (x as f32 / dimf, y as f32 / dimf);

       let p = convert_cube_uv_to_xyz(face as usize, u, v);

        let p = [p.0 as f32, p.1 as f32 ,p.2 as f32 ];
        let p = normalized(p);
        let p = p.map(|x| x  as f64 * freq as f64);

        let v =  ridged_multi.get(p)*0.5 + ridged_multi2.get(p)*0.5 ;
        let v = (v + 1.0) / 2.0;
        //let v = v * 255.0;
        //let v = 1.0;
        let ir = v as f32;
        let ig = v as f32;
        let ib = v as f32;

        *pixel = Rgb::<f32>([ir, ig, ib]);
    }

    println!("saving face {}", face);
    face_image.save(format!("image{}.exr", face)).unwrap();
    
    
    let mut norm_image = ImageBuffer::<Rgb<u8>, Vec<u8>>::new(dim, dim);
    for (x, y, pixel) in norm_image.enumerate_pixels_mut(){
        let x = x as f32;
        let y = y as f32;

        let col = bilinear_sample2(&face_image, x, y).0[0];
        let col_right = bilinear_sample2(&face_image, x + px_normal_radius, y).0[0];
        let col_bottom = bilinear_sample2(&face_image, x, y + px_normal_radius).0[0];

        let normal = [
            col_right - col,
            0.002,
            col_bottom - col
        ];

        let normal = normalized(normal);
        let normal = normal.map(|x| (x+1.0) / 2.0); 
        let normal = normal.map(|x| (x * 255.0) as u8); 
       

        *pixel = Rgb::<u8>(normal);
    }
    norm_image.save(format!("norm_image{}.png", face)).unwrap();
    break;
}

}
