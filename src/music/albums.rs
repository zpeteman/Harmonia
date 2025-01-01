use std::fs::{self};

pub fn find_album_in_dir(dir: &str, album_name: &str) -> Option<String> { 
    // Read the contents of the directory
    let entries = fs::read_dir(dir).expect("Failed to read the directory");

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                // If it's a directory, check if it matches the album name
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
                        if dir_name.to_lowercase().contains(&album_name.to_lowercase()) {
                            return Some(path.to_string_lossy().to_string()); // Return the path if a match is found
                        } else {
                            // If the album name is not found here, check subdirectories recursively
                            if let Some(found) = find_album_in_dir(path.to_str().unwrap(), album_name) {
                                return Some(found);
                            }
                        }
                    }
                }
            },
            Err(_) => continue, // Ignore errors in reading directory entries
        }
    }

    None // Return None if no matching album is found
}

pub fn print_songs_in_album(album_path: &str) {
    // Read the contents of the album directory
    let entries = fs::read_dir(album_path).expect("Failed to read album directory");
    for entry in entries {
        
        match entry {
            
            Ok(entry) => {
                let path = entry.path();
                // If it's a file, check its extension and print the song name
                if path.is_file() {
                    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
                        
                        if ["mp3", "wav", "flac", "acc"].contains(&extension) {
                            if let Some(song_name) = path.file_stem().and_then(|name| name.to_str()) {
                                println!("- {}", song_name);
                            }
                        }
                    }
                } else if path.is_dir() {
                    // If it's a directory, recursively check for songs within it
                    print_songs_in_album(path.to_str().unwrap());
                }
            },
            Err(_) => continue, // Ignore errors in reading directory entries
        }
    }
}
