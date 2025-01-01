use std::fs;                         
use std::fs::File;                         
use rodio::{Decoder, OutputStream, Sink};  
use rodio::Source;                         



pub fn list(list: &str) {                                                                                                  
    // Define the root directory where albums are stored (assuming a structure where albums are in subdirectories)                
    let root_dir = dir(); // You should replace this with the actual path to your Harmonia directory                        
                                                                                                                                  
    // Call the function to find the album                                                                                        
    if let Some(list_path) = find_album_in_dir(&root_dir, list) {                                                       
        println!("found: {}", list);                                                                                       
                                                                                                                                  
        // Now print the songs in the album directory                                                                             
        print_songs_in_album(&list_path);                                                                                
    } else {                                                                                                                      
        println!("this '{}' is not found.", list);                                                                                 
    }                                                                                                                             
}                                                                                                                                 

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
                        println!("to quite use Cntrl C.");                                                                                                                    
                        if ["mp3", "wav", "flac", "acc"].contains(&extension) {                                     
                            if let Some(song_name) = path.file_stem().and_then(|name| name.to_str()) {              
                                println!("- playing {}.", song_name);                                                        
                            }
                            // Open the audio file                                                                                
                            let file = File::open(&path).expect("Failed to open the audio file");                                 
                                                                                                                                  
                            // Create an output stream to play audio                                                              
                            let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream");  
                                                                                                                                  
                            // Decode the audio file into a source (this automatically supports .wav, .mp3, etc.)                 
                            let source = Decoder::new(file).expect("Failed to decode the audio file");                            
                                                                                                                                  
                            // Create a Sink to play the decoded source                                                           
                            let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");                       
                                                                                                                                  
                            // Append the source (audio data) to the sink for playback                                            
                            sink.append(source.amplify(0.5));                                                                                  
                                                                                                                                  
                            // This will automatically play and block the thread until the audio finishes                         
                            sink.sleep_until_end(); // Wait until the audio finishes playing                                      
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

pub fn dir() -> String {                                                    
    loop {                                                                   
        let username = whoami::username();                                   
        let path = format!("/home/{}/Music/Harmonia",username);              
        return path;                                                         
    }     
}
