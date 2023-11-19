use mandelbrot::{parse_complex, parse_pair, pixel_to_point, render, write_image};
use std::io::Write;

/// Examples
/// cargo run m.png 4000x3000 -1.20,0.35 -1,0.20
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "Err").unwrap();
        writeln!(std::io::stderr(), "Err {}", args[0]).unwrap();
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Err");
    let upper_left = parse_complex(&args[3]).expect("Err");
    let lower_right = parse_complex(&args[4]).expect("Err");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let thread = 8;
    let rows_per_band = bounds.1 / thread + 1;
    {
        let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();
        let _ = crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                    pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        });
    }

    write_image(&args[1], &pixels, bounds).expect("err");
}
