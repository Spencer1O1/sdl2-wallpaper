# sdl2_wallpaper

Use SDL2 to actively render to the desktop background. Use simple functions to get access to either the desktop background window, or a Rust-SDL2 canvas bound to it. Then, you can render whatever you want for ultimate customization.

## Note

As of now, only Windows is supported. It has only been developed with Windows 10, so that is the only OS that is expected to work.

## Installation

**Prerequisite:** Make sure you have [Rust-SDL2](https://crates.io/crates/sdl2) installed, and that it is working.

There are two easy installation options for sdl2_wallpaper.

1. Use [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) from the terminal

```bash
cargo add sdl2_wallpaper
```

2. Add the dependency to your `Cargo.toml` file

```toml
[dependencies]
sdl2_wallpaper = "0.1.0"
```

## Usage

(Based on the Rust-SDL2 [example](https://rust-sdl2.github.io/rust-sdl2/sdl2/#getting-started))

```rust
use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    
    // The only real difference here from the Rust-SDL2 example is
    // how we get the window/canvas.

    let mut canvas = sdl2_wallpaper::get_canvas(video_subsystem).unwrap();
    
    // If you'd rather get the window and then build the canvas from that:

    // let window = sdl2_wallpaper::get_window(video_subsystem).unwrap();
    // [maybe do something to your window]
    // let mut canvas = window.into_canvas().build().unwrap()

    canvas.set_draw_color(Color::RGB(0, 64, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    } 
}
```

## Contributing

Pull requests are very welcome. Please feel free to make this more rusty! For major updates, like supporting another OS, please open an issue first to discuss what you want, as well as a course of action.

Since this is a new package, there is so much to add to this and I'm sure we'll find bugs. Let's work together!

## License

[MIT](https://choosealicense.com/licenses/mit/)
OR
[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
