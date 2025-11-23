use std::path::PathBuf;
use crate::error::ImagixError;
use crate::resize::get_image_files;

pub fn get_stats(src_folder: PathBuf) -> Result<(usize, f64), ImagixError> {
    let image_files = get_image_files(src_folder)?;
    
    let size = image_files
        .iter()
        .map(move |f| f.metadata().unwrap().len())
        .sum::<u64>();
    
    Ok((image_files.len(), (size / 1_000_000) as f64))
}