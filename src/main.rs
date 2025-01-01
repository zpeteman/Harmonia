use clap::{Command, Arg};
mod music;
mod list;

fn main() {
    let matches = Command::new("Harmonia")
        .version("1.0")
        .author("Ilyas Zanan")
        .about("A Command-Line Music Player")
        .subcommand(
            Command::new("-pl")
                .about("Play something i don't know.")
                .subcommand(
                    Command::new("-s")
                        .about("Play song")
                        .arg(Arg::new("song").help("The song to play").required(true).index(1)),
                )
                .subcommand(
                    Command::new("-ar")
                        .about("Playing the artist's songs randomly")
                        .arg(Arg::new("artist").help("The artist name").required(true).index(1)),
                )
                .subcommand(
                    Command::new("-al")
                        .about("Playing an album")
                        .arg(Arg::new("album").help("The album name").required(true).index(1)),
                ),
        )
        .subcommand(Command::new("-pa").about("Pause the current song"))
        .subcommand(Command::new("-s").about("Stop the music"))
        .subcommand(
            Command::new("-l")
                .about("List music by album or artist.")
                .arg(
                    Arg::new("name")
                        .help("The album or artist name to list songs for")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if let Some(play_matches) = matches.subcommand_matches("-pl") {
        if let Some(song_matches) = play_matches.subcommand_matches("-s") {
            if let Some(song) = song_matches.get_one::<String>("song") {
                music::play_song(song);
            }
        } else if let Some(artist_matches) = play_matches.subcommand_matches("-ar") {
            if let Some(artist) = artist_matches.get_one::<String>("artist") {
                music::play_artist(artist);
            }
        } else if let Some(album_matches) = play_matches.subcommand_matches("-al") {
            if let Some(album) = album_matches.get_one::<String>("album") {
                music::play_album(album);
            }
        }
    } else if let Some(list_matches) = matches.subcommand_matches("-l") {
        if let Some(name) = list_matches.get_one::<String>("name") {
            list::list(name);
        }
    } else if matches.subcommand_matches("-pa").is_some() {
        music::pause_music();
    } else if matches.subcommand_matches("-s").is_some() {
        music::stop_music();
    }
}







































































































































