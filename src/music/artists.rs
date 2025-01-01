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
                                                                                                                    
pub fn print_albums_in_artist(album_path: &str) {                                                                     
    // Read the contents of the album directory                                                                     
    let entries = fs::read_dir(album_path).expect("Failed to read artist directory");                                
                                                                                                                    
    for entry in entries {                                                                                          
        match entry {                                                                                               
            Ok(entry) => {                                                                                          
                let path = entry.path();                                                                            
                if path.is_dir() {
                    if let Some(album_name) = path.file_stem().and_then(|name| name.to_str()) { 
                        println!("- {}", album_name);                                           
                    }                                                                          
                }                                                                                                   
            },                                                                                                      
            Err(_) => continue, // Ignore any errors in reading directory entries                                   
        }                                                                                                           
    }                                                                                                               
}   
