mod endianness;
mod tiff;
mod header;
mod ifd;
mod tag;
mod tagtype;
mod contants;

#[cfg(test)]
mod tests {
    use crate::tiff::Tiff;
    use std::fs::File;
    use std::time::Instant;

    #[test]
    fn test() {
        let now = Instant::now();
        let mut f = File::open("test/D3200.NEF").expect("Cannot open file.");
//        let mut f = File::open("test/X100.RAF").expect("Cannot open file.");
        // File offset to skip RAF header  - 160 (not implemented)
        let file = Tiff::new_from_file(&mut f, 0);
        let now = now.elapsed();
        println!("{:?}", file);
        println!("Time 0.{:#06}μs", now.as_micros());
    }
}

