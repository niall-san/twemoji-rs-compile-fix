# twemoji-rs
⚠️ Right now it seems making this crate as I envisionned it is not feasible  
⚠️ So it is on hold (and non functionning) until this is answered:  
⚠️ https://github.com/rust-lang/cargo/issues/11683

Small Rust crate to provide the twemoji icon corresponding to a string

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
