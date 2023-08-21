use std::{collections::HashMap, fs::File, error::Error};
use std::io::Read;
#[derive(Default)]
pub struct ImagesCache {
  cache: HashMap<String, Vec<u8>>,
}

impl ImagesCache {
  fn new() -> Self {
    Self {
      cache: HashMap::new(),
    }
  }

  pub fn load_image(&mut self, image_path: &str) -> Result<&Vec<u8>, std::io::Error> {
    if !self.cache.contains_key(image_path) {
      let image_data = read_image_from_file(image_path.to_string());
      match image_data {
        Ok(_) => {}
        Err(e) => {
          println!("Error: {:?}", e);
          return Err(e);
        }
      }
          
      self.cache.insert(image_path.to_string(), image_data.unwrap());
    }

    // Return a reference to the cached image data
    Ok(self.cache.get(image_path).unwrap())
  }
}

fn read_image_from_file(url: String) -> Result<Vec<u8>, std::io::Error> {
  let mut file = File::open(url);
  match file {
    Ok(_) => {}
    Err(e) => {
      println!("Error: {:?}", e);
      return Err(e);
    }
  }

  // Read the image file into a Vec<u8>
  let mut image_data = Vec::new();
  let was_readed = file.unwrap().read_to_end(&mut image_data);
  // Convert the Vec<u8> into a &[_] slice
  match was_readed {
    Ok(_) => Ok(image_data),
    Err(e) => {
      println!("Error: {:?}", e);
      Err(e)
    }
  }

  // Now you can use image_data_slice as needed
}
