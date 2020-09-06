use crate::camera::Camera;
use crate::geometry::*;
use crate::v3::*;
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::min;
use std::f64::INFINITY;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

mod camera;
mod geometry;
mod v3;

fn percentage(name: &str, percentage: f64) {
    print!("\r{} {:6.2}%", name, 100.0 * percentage);
    std::io::stdout().flush().expect("failed to flush stdout");
}

fn ray_color(ray: &Ray, world: &HittableList) -> C3 {
    if let Some(hit) = world.hit(ray, 0.0, INFINITY) {
        return (hit.normal + c3(1.0, 1.0, 1.0)).scale(0.5);
    }
    let dir = ray.direction.norm();
    let t = 0.5 * (dir.y + 1.0);
    c3(1.0, 1.0, 1.0).scale(1.0 - t) + c3(0.5, 0.7, 1.0).scale(t)
}

fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random::<f64>()
}

// y up, x right, z back (rhs coordinate system)
fn main() -> std::io::Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f64 / aspect_ratio).round() as usize;
    let samples_per_pixel = 128;

    // world
    let world = HittableList {
        spheres: vec![
            Sphere {
                center: p3(0.0, 0.0, -1.0),
                radius: 0.5,
            },
            Sphere {
                center: p3(0.0, -100.5, -1.0),
                radius: 100.0,
            },
        ],
    };

    // camera
    let camera = Camera::new(aspect_ratio);

    // render
    let mut pixels = vec![vec![v3(0.0, 0.0, 0.0); image_width]; image_height];
    render(&mut pixels, |row, col, pixel| {
        let mut color = C3::zero();

        for s in 0..samples_per_pixel {
            let u = (col as f64 + random::<f64>()) / (image_width - 1) as f64;
            let v = (row as f64 + random::<f64>()) / (image_height - 1) as f64;
            let ray = camera.ray(u, v);
            color += ray_color(&ray, &world);
        }

        *pixel = color.div(samples_per_pixel as f64);
    });

    let file = File::create("image.ppm")?;
    write_ppm_file(BufWriter::new(file), &pixels)?;

    Ok(())
}

fn render<F>(pixels: &mut Vec<Vec<V3>>, f: F)
where
    F: Fn(usize, usize, &mut V3) + Sync,
{
    let start = Instant::now();
    let progress = AtomicUsize::new(0);
    percentage("rendering", 0.0);
    let height = pixels.len();
    pixels
        .par_iter_mut()
        .enumerate()
        .for_each(|(row, scanline)| {
            scanline
                .iter_mut()
                .enumerate()
                .for_each(|(col, pixel)| f(row, col, pixel));
            let old = progress.fetch_add(1, Ordering::SeqCst);
            percentage("rendering", (old + 1) as f64 / height as f64);
        });
    percentage("rendering", 1.0);
    println!("\nRendering took {:.2}s", start.elapsed().as_secs_f64());
}

fn write_ppm_file<W: Write>(mut file: W, pixels: &Vec<Vec<V3>>) -> std::io::Result<()> {
    let start = Instant::now();
    percentage("writing", 0.0);

    writeln!(file, "P3")?;
    writeln!(
        file,
        "{} {}",
        pixels.get(0).map(|row| row.len()).unwrap_or(0),
        pixels.len()
    )?;
    writeln!(file, "255")?;
    // The book iterates the rows backwards, because it assumes
    // the bottom left corner is (0,0). In ppm the first pixel
    // is at the top left.
    for (i, pixel_row) in pixels.iter().rev().enumerate() {
        for pixel in pixel_row {
            let ir = min(255, (256.0 * pixel.x) as u64);
            let ig = min(255, (256.0 * pixel.y) as u64);
            let ib = min(255, (256.0 * pixel.z) as u64);

            writeln!(file, "{} {} {} ", ir, ig, ib)?;
        }
        percentage("writing", (i + 1) as f64 / pixels.len() as f64);
    }

    println!("\nWriting took {:.2}s", start.elapsed().as_secs_f64());
    Ok(())
}
