extern crate glob;

use std::error::Error;
use std::path::Path;
use std::env;
use std::time::{Duration, Instant};
use std::fmt;
use std::fs;

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
        let p = entry.expect("Failed xxxx");
        let p = p.as_path();

        println!("{}: {}", n, p.display());

        let timer = Instant::now();
        let img = image::open(p)?;
        let resized = img.thumbnail(size, size);
        let out = Path::new(thumb_dir).join(Path::new(p).file_name().expect("invalid filename"));
        let _ = resized.save(out).ok().expect("Saving image failed");
        println!("Thumbnailed to {} in {}", size, Elapsed::from(&timer));
    }

    println!("count: {}", n);
    println!("total took: {}", Elapsed::from(&timer_total));
    Ok(())
}