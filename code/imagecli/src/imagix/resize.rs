use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::error::ImagixError;

#[derive(Debug, Clone, Copy)]
pub enum SizeOption {
    Small,
    Medium,
    Large,
}

impl Into<u32> for SizeOption {
    fn into(self) -> u32 {
        match self {
            SizeOption::Small => 640,
            SizeOption::Medium => 1024,
            SizeOption::Large => 1920,
        }
    }
}

impl FromStr for SizeOption {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "small" => Ok(SizeOption::Small),
            "medium" => Ok(SizeOption::Medium),
            "large" => Ok(SizeOption::Large),
            _ => Err(ImagixError::UserInputError("Invalid option for size".to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Single,
    All
}

impl FromStr for Mode {
    type Err = ImagixError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Mode::Single),
            "all" => Ok(Mode::All),
            _ => Err(ImagixError::UserInputError("Invalid option for mode".to_string()))
        }
    }
}

pub fn get_image_files(src_folder: PathBuf) -> Result<Vec<PathBuf>, ImagixError> {
    let entries = fs::read_dir(src_folder)
        .map_err(|_| ImagixError::UserInputError("Invalid source folder".to_string()))?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, std::io::Error>>()?
        .into_iter()
        .filter(|p| {
            matches!(
                p.extension()
                .and_then(|s| Some(s.to_str().unwrap())),
                Some("png") | Some("jpg") | Some("jpeg") | 
                Some("PNG") | Some("JPG") | Some("JPEG")
            )
        })
        .collect();

    Ok(entries)
}

fn prepare_destination_path(src_path: PathBuf) -> Result<PathBuf, ImagixError> {
    // construct destination filename with .png extension
    let new_file_name = src_path
        .file_stem()
        .and_then(|s| Some(s.to_str().unwrap().to_string() + "_resized.png"))
        .ok_or(ImagixError::UserInputError("Invalid source file".to_string()))?;

    // construct destination path
    let mut dest_folder = src_path.clone();
    
    dest_folder.pop();
    dest_folder.push("resized");

    if !dest_folder.exists() {
        fs::create_dir(&dest_folder)?;
    }

    dest_folder.push(new_file_name);
    Ok(dest_folder)
}

pub fn resize_image(size: u32, src_path: PathBuf) -> Result<(), ImagixError> {
    let destination_path = prepare_destination_path(src_path.clone())?;
    
    let timer = std::time::Instant::now();
    
    let img = image::open(&src_path)?;
    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let (target_width, target_height) = (size, (size as f32 / aspect_ratio) as u32);
    let resized_img = img.thumbnail(target_width, target_height);

    let mut output_file = fs::File::create(&destination_path)?;
    resized_img.write_to(&mut output_file, image::ImageFormat::Png)?;

    println!(
        "Resized to ({}, {}) image saved to {} in {:?}",
        target_width,
        target_height,
        destination_path.display(),
        timer.elapsed()
    );

    Ok(())
}

pub fn resize_image_request(size: SizeOption, path: PathBuf) -> Result<(), ImagixError> {
    resize_image(size.into(), path)
}

pub fn resize_many_images_request(size: SizeOption, path: PathBuf) -> Result<(), ImagixError> {
    let img_path = path.clone();
    
    let image_files = get_image_files(img_path)?;

    for img in image_files {
        resize_image_request(size, img)?;
    }

    Ok(())
}

pub fn process_resize_request(size: SizeOption, mode: Mode, path: PathBuf) -> Result<(), ImagixError> {
    match mode {
        Mode::Single => {
            resize_image_request(size, path)?;
        },
        Mode::All => {
            resize_many_images_request(size, path)?;
        }
    }

    Ok(())
}

#[cfg(test)]

mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn resize_single_image() {
        let path = PathBuf::from("./tmp/images/one.jpg");
        let destination_path = PathBuf::from("./tmp/images/resized/one_resized.png");

        resize_image_request(SizeOption::Large, path).unwrap();

        assert_eq!(true, destination_path.exists());
    }
    
    #[rstest]
    fn resize_many_images() {
        let path = PathBuf::from("./tmp/images");

        let destination_path_1 = PathBuf::from("./tmp/images/resized/one_resized.png");
        let destination_path_2 = PathBuf::from("./tmp/images/resized/two_resized.png");
        let destination_path_3 = PathBuf::from("./tmp/images/resized/three_resized.png");
        
        resize_many_images_request(SizeOption::Medium, path).unwrap();
        
        assert_eq!(true, destination_path_1.exists());
        assert_eq!(true, destination_path_2.exists());
        assert_eq!(true, destination_path_3.exists());
    }
}