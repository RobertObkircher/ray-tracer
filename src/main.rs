use crate::v3::*;
use rayon::prelude::*;
use std::cmp::min;
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Instant;

mod v3;

fn percentage(name: &str, percentage: f64) {
    print!("\r{} {:6.2}%", name, 100.0 * percentage);
    std::io::stdout().flush().unwrap();
}

fn main() -> std::io::Result<()> {
    let image_width = 256;
    let image_height = 256;
    let mut pixels = vec![vec![v3(0.0, 0.0, 0.0); image_width]; image_height];

    render(&mut pixels, |row, col, pixel| {
        let r = col as f64 / (image_width - 1) as f64;
        let g = row as f64 / (image_height - 1) as f64;
        let b = 0.25 as f64;
        *pixel = v3(r, g, b);
    });

    let file = File::create("image.ppm")?;
    write_ppm_file(file, &pixels)?;

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

fn write_ppm_file(mut file: File, pixels: &Vec<Vec<V3>>) -> std::io::Result<()> {
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
    for (i, pixel_row) in pixels.iter().enumerate() {
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
