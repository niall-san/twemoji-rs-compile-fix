# twemoji-rs
Small Rust crate to provide the twemoji icon .png corresponding to a string. Fixed (bodged) to allow assets to be included at compile time.

```rust
use twemoji_rs::get_twemoji;

if let Some(path_to_icon) = get_twemoji("🚀") {
    // The 72x72 Twemoji image for this emoji
    let img = ImageReader::open(path_to_icon)?.decode()?;
} else {
    println!("Couldn't find an icon file :(");
}
```

All credits to [https://twemoji.twitter.com/](https://twemoji.twitter.com/) for the images !
