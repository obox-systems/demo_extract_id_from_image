# Text extraction from an image demo

This program uses tesseract for computer vision to extract a numerical id from an image 
and creates a folder with that id to store the image.

## Try it out!
1. Install [Rust](https://rustup.rs/)
2. Run the app
```bash
$ cargo run --release
```
It will search for all files with a `.png` extension in the current folder and move them to a folder with an id written on that image.

It will format the file like this: `{id}-{created_date}.png`.
