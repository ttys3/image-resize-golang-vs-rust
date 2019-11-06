extern crate glob;

use std::error::Error;
use std::path::Path;
use std::env;
use std::time::{Duration, Instant};
use std::fmt;
use std::fs;

use image::{DynamicImage, GenericImageView};

struct Elapsed(Duration);

impl Elapsed {
    fn from(start: &Instant) -> Self {
        Elapsed(start.elapsed())
    }
}

impl fmt::Display for Elapsed {
    fn fmt(&self, out: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match (self.0.as_secs(), self.0.subsec_nanos()) {
            (0, n) if n < 1000 => write!(out, "{} ns", n),
            (0, n) if n < 1000_000 => write!(out, "{} µs", n / 1000),
            (0, n) => write!(out, "{} ms", n / 1000_000),
            (s, n) if s < 10 => write!(out, "{}.{:02} s", s, n / 10_000_000),
            (s, _) => write!(out, "{} s", s),
        }
    }
}

use glob::{glob_with, MatchOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let src :String = args.next().expect("src param needed").parse().unwrap();
    let size = args.next().expect("size param needed").parse().unwrap();

    let options = &MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let thumb_dir = "/tmp/thumb-rust";
    fs::remove_dir_all(&thumb_dir);
    fs::create_dir_all(&thumb_dir);

    let timer_total = Instant::now();
    let mut n = 0;
    for entry in glob_with(format!("{}/*.jpg", src).as_str(), *options)? {
        n = n + 1;
        // Do the job
        let p = entry.unwrap();
        let p = p.as_path();

        println!("{}: {}", n, p.display());

        let timer = Instant::now();
        let img = image::open(p)?;
        let resized = thumbnail(&img,size, size);
        let out = Path::new(thumb_dir).join(Path::new(p).file_name().expect("invalid filename"));
        let _ = resized.save(out).ok().expect("Saving image failed");
        println!("Thumbnailed to {} in {}", size, Elapsed::from(&timer));
    }

    println!("count: {}", n);
    println!("total took: {}", Elapsed::from(&timer_total));
    Ok(())
}

/// copy from image-0.22.3/src/dynimage.rs
/// we need call resize_dimensions() with fill param set to true here
fn thumbnail(img: &DynamicImage, nwidth: u32, nheight: u32) -> image::DynamicImage {
    // use nwidth as result thumbnail width
    let (width2, height2) =
        resize_dimensions(img.width(), img.height(), nwidth, nheight, true);
    img.thumbnail_exact(width2, height2)
}

/// copy from image-0.22.3/src/dynimage.rs
/// Calculates the width and height an image should be resized to.
/// This preserves aspect ratio, and based on the `fill` parameter
/// will either fill the dimensions to fit inside the smaller constraint
/// (will overflow the specified bounds on one axis to preserve
/// aspect ratio), or will shrink so that both dimensions are
/// completely contained with in the given `width` and `height`,
/// with empty space on one axis.
fn resize_dimensions(width: u32, height: u32, nwidth: u32, nheight: u32, fill: bool) -> (u32, u32) {
    let ratio = u64::from(width) * u64::from(nheight);
    let nratio = u64::from(nwidth) * u64::from(height);

    let use_width = if fill {
        nratio > ratio
    } else {
        nratio <= ratio
    };
    let intermediate = if use_width {
        u64::from(height) * u64::from(nwidth) / u64::from(width)
    } else {
        u64::from(width) * u64::from(nheight) / u64::from(height)
    };
    if use_width {
        if intermediate <= u64::from(::std::u32::MAX) {
            (nwidth, intermediate as u32)
        } else {
            (
                (u64::from(nwidth) * u64::from(::std::u32::MAX) / intermediate) as u32,
                ::std::u32::MAX,
            )
        }
    } else if intermediate <= u64::from(::std::u32::MAX) {
        (intermediate as u32, nheight)
    } else {
        (
            ::std::u32::MAX,
            (u64::from(nheight) * u64::from(::std::u32::MAX) / intermediate) as u32,
        )
    }
}
