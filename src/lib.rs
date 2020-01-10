mod endianness;
mod file;
mod header;
mod ifd;
mod tag;
mod tagtype;
mod tiff;

#[cfg(test)]
mod tests {
    use crate::file::Tiff;
    use std::fs::File;
    use std::time::Instant;

    #[test]
    fn test() {
        let now = Instant::now();
        let mut f = File::open("test/D3200.NEF").expect("Cannot open file.");
        let file = Tiff::new_from_file(&mut f, 0);
        let now = now.elapsed();
        println!("{:?}", file);
        println!("Time 0.{:#06}μs", now.as_micros());
    }
}

