# Harmonia

Harmonia is a simple CLI-based music player built in Rust. It allows you to organize your music library and play songs in `.wav` format.

## Features

- Play `.wav` files with customizable volume control.
- Lightweight and easy to use.
- Having a simple yet good search engine.

## Directory Structure

All your music files should be stored in the following structure under the `Music` directory:

```
Harmonia/
├── Singer1/
│   ├── Album1/
│   │   ├── song1.wav
│   │   ├── song2.wav
│   ├── Album2/
│       ├── song3.wav
├── Singer2/
    ├── Album3/
        ├── song4.wav
```

## Installation

1. Clone the repository:
    ```bash
    git clone https://github.com/your-username/Harmonia.git
    ```

2. Navigate to the project directory:
    ```bash
    cd Harmonia
    ```

3. Build the project:
    ```bash
    cargo build --release
    ```

4. Add the binary to your system's PATH:
    ```bash
    export PATH=$PATH:/path/to/Harmonia/target/release
    ```

## Usage

```bash
harmonia help
```
u will find everything here.

## Requirements

- Rust installed on your machine.
- All audio files must be in `.wav` format.
- The names of the files should be too close to each other.
- You must make create the file tree manually.

## License

this is an open-source project, feel free to make changes.
