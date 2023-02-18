use std::fs::{read_dir, rename, metadata, create_dir};

use tesseract::Tesseract;

fn main() {
  let mut tesseract = Tesseract::new(Some("./tessdata"), Some("digits")).unwrap();

  let files = read_dir("./").unwrap()
    .filter_map(|file| {
      if file.is_ok() {
        let file = file.unwrap();
        if file.path().extension().is_some() {
          return Some(file)
        }
      }
      None
    })
    .filter(|file| {
      file.path().extension().unwrap().eq("png")
    });
  
  for file in files {
    tesseract = tesseract.set_image(file.path().to_str().unwrap()).unwrap();
    let text = tesseract.get_text().unwrap();
    let id: String = text.lines().last().unwrap().split(' ').collect();
    println!("{id}");
    create_dir(&id).unwrap();
    let metadata = metadata(file.path()).unwrap();
    let created_date = metadata.created().unwrap().elapsed().unwrap().as_millis();
    rename(file.path(), format!("./{id}/{id}-{created_date}.png")).unwrap();
  }
}
