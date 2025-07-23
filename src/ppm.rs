use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;

// Texture struct for image data
#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<[u8; 3]>,
}

// PPM image writer: outputs pixel data to ASCII .ppm file.
pub fn write_ppm<W: Write>(writer: &mut W, width: usize, height: usize, pixels: &[[u8; 3]]) -> io::Result<()> {
    writeln!(writer, "P3")?;
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?;
    for pixel in pixels {
        writeln!(writer, "{} {} {}", pixel[0], pixel[1], pixel[2])?;
    }
    Ok(())
}

// PPM image reader: loads pixel data from ASCII .ppm file.
pub fn read_ppm(path: &str) -> io::Result<Texture> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line)?; // P3
    if line.trim() != "P3" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a P3 PPM file"));
    }

    // Skip comments
    line.clear();
    loop {
        reader.read_line(&mut line)?;
        if !line.starts_with('#') {
            break;
        }
        line.clear();
    }

    let dims: Vec<usize> = line
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if dims.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid PPM dimensions"));
    }
    let width = dims[0];
    let height = dims[1];

    line.clear();
    reader.read_line(&mut line)?; // max value
    if line.trim() != "255" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "PPM max value must be 255"));
    }

    let mut pixels = Vec::with_capacity(width * height);
    for line in reader.lines() {
        for chunk in line?.split_whitespace().collect::<Vec<_>>().chunks(3) {
            if chunk.len() == 3 {
                let r = chunk[0].parse().unwrap_or(0);
                let g = chunk[1].parse().unwrap_or(0);
                let b = chunk[2].parse().unwrap_or(0);
                pixels.push([r, g, b]);
            }
        }
    }
    Ok(Texture { width, height, pixels })
}
