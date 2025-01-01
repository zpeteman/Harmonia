mod dir;
mod albums;
mod artists;

use std::fs;
use std::fs::File;
use rodio::{Decoder, OutputStream, Sink};
use rodio::Source;

pub fn play_song(song: &str) {
    println!("Playing song: {}", song);

    // Specify the directory where your songs are stored
    let music_dir = dir::main(); // Change this path as needed

    // Search for the song in the directory
    let song_path = find_song_in_dir(&music_dir, song);

    match song_path {
        Some(path) => {

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
            println!("Playing. Press Ctrl+C to stop.");
            sink.sleep_until_end(); // Wait until the audio finishes playing

            println!("Finished playing '{}'.", song);
        },
        None => {
            println!("Song '{}' not found.", song);
        }
    }
}

fn find_song_in_dir(dir: &str, search_term: &str) -> Option<std::path::PathBuf> {
    // Read the contents of the directory
    let entries = fs::read_dir(dir).expect("Failed to read the directory");

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    // Get the file name as a string
                    if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                        // Check if the search term is contained in the file name (case insensitive)
                        if file_name.to_lowercase().contains(&search_term.to_lowercase()) {
                            return Some(path); // Return the path of the found song
                        }
                    }
                } else if path.is_dir() {
                    // If it's a directory, search recursively
                    if let Some(found) = find_song_in_dir(path.to_str().unwrap(), search_term) {
                        return Some(found);
                    }
                }
            }
            Err(_) => continue, // Ignore any errors in reading directory entries
        }
    }

    None // Return None if no matching song is found
}

pub fn stop_music(song: &str) {
    println!("Stopping music.");
    // Add stop functionality here
     println!("Playing song: {}", song);                                                                             
                                                                                                                     
     // Specify the directory where your songs are stored                                                            
     let music_dir = dir::main(); // Change this path as needed                                                      
                                                                                                                     
     // Search for the song in the directory                                                                         
     let song_path = find_song_in_dir(&music_dir, song);                                                             
                                                                                                                     
     match song_path {                                                                                               
         Some(path) => {                                                                                             
             println!("Found song at: {}", path.display());                                                          
                                                                                                                     
             // Open the audio file                                                                                  
             let file = File::open(&path).expect("Failed to open the audio file");                                   
                                                                                                                     
             // Create an output stream to play audio                                                                
             let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to create output stream"); 
                                                                                                                  
             // Decode the audio file into a source (this automatically supports .wav, .mp3, etc.)                
             let source = Decoder::new(file).expect("Failed to decode the audio file");                           
                                                                                                                  
             // Create a Sink to play the decoded source                                                          
             let sink = Sink::try_new(&stream_handle).expect("Failed to create audio sink");                      
            
            println!("Playing. Press Ctrl+C to stop.");                                                          

            // Append the source (audio data) to the sink for playback                                            
            sink.append(source.repeat_infinite());                                                                                  
                                                                                                                  
             // This will automatically play and block the thread until the audio finishes                        
             sink.sleep_until_end(); // Wait until the audio finishes playing                                     
                                                                                                                  
             println!("Finished playing '{}'.", song);                                                            
         },                                                                                                       
         None => {                                                                                                
             println!("Song '{}' not found.", song);                                                              
         }                                                                                                        
     }                                                                                                            
}

pub fn play_album(album: &str) {                                                                                   
    // Define the root directory where albums are stored (assuming a structure where albums are in subdirectories) 
    let root_dir = dir::main(); // You should replace this with the actual path to your Harmonia directory              
                                                                                                                   
    // Call the function to find the album                                                                         
    if let Some(album_path) = albums::find_album_in_dir(&root_dir, album) {                                                 
        println!("Album found: {}", album);                                                                        
                                                                                                                   
        // Now print the songs in the album directory                                                              
        albums::print_songs_in_album(&album_path);                                                                         
    } else {                                                                                                       
        println!("Album '{}' not found.", album);                                                                  
    }                                                                                                              
}

pub fn artist(artist: &str) {
    let root_dir = dir::main();
    if let Some(artist_path) = artists::find_artist_in_dir(&root_dir, artist) {
        
        let (albums, songs) = artists::artist_details(&artist_path);

        println!("- {} with: {} albums and {} songs.", artist, albums, songs);
    } else {
        print!("artist not found.");
    }
}
