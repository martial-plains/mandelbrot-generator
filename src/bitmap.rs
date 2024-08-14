use std::{fs::File, io::Write, mem, path::Path};

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct FileHeader {
    pub header: [u8; 2],
    pub file_size: i32,
    pub reserved: i32,
    pub data_offset: i32,
}

#[repr(C, packed(2))]
#[derive(Debug)]
pub struct InfoHeader {
    pub header_size: i32,
    pub width: i32,
    pub height: i32,
    pub planes: i16,
    pub bits_per_pixel: i16,
    pub compression: i32,
    pub data_size: i32,
    pub horizontal_resolution: i32,
    pub vertical_resolution: i32,
    pub colors: i32,
    pub important_colors: i32,
}

impl Default for InfoHeader {
    fn default() -> Self {
        Self {
            header_size: 40,
            width: Default::default(),
            height: Default::default(),
            planes: 1,
            bits_per_pixel: 24,
            compression: 0,
            data_size: Default::default(),
            horizontal_resolution: Default::default(),
            vertical_resolution: Default::default(),
            colors: 0,
            important_colors: 0,
        }
    }
}

impl Default for FileHeader {
    fn default() -> Self {
        Self {
            header: [b'B', b'M'],
            file_size: Default::default(),
            reserved: 0,
            data_offset: Default::default(),
        }
    }
}

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl Image {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![0; width * height * 3];

        Self {
            width,
            height,
            pixels,
        }
    }

    /// Writes the image data to a file.
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The file size, data offset, width, or height cannot be converted to `i32`.
    /// - The file cannot be created at the specified path.
    /// - Writing the file header, info header, or pixel data to the file fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let image = Image::new(100, 100, vec![255; 30000]);
    /// image.write("output.bmp").expect("Failed to write image");
    /// ```
    pub fn write(&self, file_name: &str) -> anyhow::Result<bool> {
        let mut file_header = FileHeader::default();
        let mut info_header = InfoHeader::default();

        file_header.file_size = i32::try_from(
            size_of::<InfoHeader>() + size_of::<FileHeader>() + (self.width * self.height * 3),
        )?;
        file_header.data_offset = i32::try_from(size_of::<InfoHeader>() + size_of::<FileHeader>())?;

        info_header.width = i32::try_from(self.width)?;
        info_header.height = i32::try_from(self.height)?;

        let path = Path::new(file_name);
        let mut file = File::create(path)?;

        file.write_all(unsafe {
            &mem::transmute::<FileHeader, [u8; mem::size_of::<FileHeader>()]>(file_header)
        })?;
        file.write_all(unsafe {
            &mem::transmute::<InfoHeader, [u8; mem::size_of::<InfoHeader>()]>(info_header)
        })?;
        file.write_all(&self.pixels)?;

        Ok(true)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, red: u8, green: u8, blue: u8) {
        let pixel = self.pixels.as_mut_ptr();

        unsafe {
            let p_pixel = pixel.add((y * 3 * self.width) + (x * 3));
            *p_pixel = blue;
            *p_pixel.add(1) = green;
            *p_pixel.add(2) = red;
        }
    }
}
