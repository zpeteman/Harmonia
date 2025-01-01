use std::fs::{self};                                                                                                
                                                                                                                    
pub fn find_artist_in_dir(dir: &str, album_name: &str) -> Option<String> {                                           
    // Read the contents of the directory                                                                           
    let entries = fs::read_dir(dir).expect("Failed to read the directory");                                         
                                                                                                                    
    for entry in entries {                                                                                          
        match entry {                                                                                               
            Ok(entry) => {                                                                                          
                let path = entry.path();                                                                            
                                                                                                                    
                if path.is_dir() {                                                                                  
                    // Check if the album name matches part of the directory name                                   
                    if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {                       
                        if dir_name.to_lowercase().contains(&album_name.to_lowercase()) {                           
                            return Some(path.to_string_lossy().to_string()); // Return the path if a match is found 
                        }                                                                                           
                    }                                                                                               
                }                                                                                                   
            },                                                                                                      
            Err(_) => continue, // Ignore any errors in reading directory entries                                   
        }                                                                                                           
    }                                                                                                               
                                                                                                                    
    None // Return None if no matching album is found                                                               
}                                                                                                                   
                                                                                                                    
pub fn artist_details(album_path: &str) -> (usize, usize) {
    // Read the contents of the album directory
    let entries = fs::read_dir(album_path).expect("Failed to read artist directory");

    let mut albums = 0;
    let mut songs = 0;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_dir() {
                    albums += 1;

                    let new_entries = fs::read_dir(&path).expect("Failed to read album directory.");
                    for i in new_entries {
                        match i {
                            Ok(i) => {
                                let new_path = i.path();
                                if new_path.is_file() {
                                    if let Some(exp) = new_path.extension().and_then(|ext| ext.to_str()) {
                                        if ["mp3", "wav", "flac", "aac"].contains(&exp) {
                                            songs += 1;
                                        }
                                    }
                                }
                            }
                            Err(_) => continue, // Ignore errors in reading files
                        }
                    }
                }
            }
            Err(_) => continue, // Ignore any errors in reading directory entries
        }
    }

    (albums, songs)
}
