extern crate glob;

use std::error::Error;
use std::path::Path;
use std::env;

use glob::{glob_with, MatchOptions};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    args.next();
    let src :String = args.next().unwrap().parse().unwrap();
    let width = args.next().unwrap().parse().unwrap();
    let height = args.next().unwrap().parse().unwrap();

    let options = &MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let mut n = 0;
    for entry in glob_with(format!("{}*.jpg", src).as_str(), *options)? {
        n = n + 1;
        // Do the job
        let p = entry.expect("Failed xxxx");
        let p = p.as_path();

        println!("{}: {}", n, p.display());

        let img = image::open(p)?;
        let resized = img.thumbnail(width, height);
        let out = Path::new("/home/hacklog/Videos/壁纸wallpaper/thumb").join(Path::new(p).file_name().expect("invalid filename"));
        let _ = resized.save(out).ok().expect("Saving image failed");
    }

    println!("count: {}", n);
    Ok(())
}