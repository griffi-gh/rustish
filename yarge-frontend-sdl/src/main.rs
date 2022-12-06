use yarge_core::{
  Gameboy,
  Key as GbKey,
  RomHeader,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use sdl2::{
  pixels::PixelFormatEnum, 
  event::Event, 
  keyboard::Keycode, 
};
use std::time::Duration;
use clap::Parser;

const GB_PALETTE: [u32; 4] = [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x0000000];
const WIN_WIDTH: usize = GB_WIDTH;
const WIN_HEIGHT: usize = GB_HEIGHT;

#[derive(Parser, Debug)]
#[command()]
struct Args {
  rom_path: String,
  #[arg(long)] skip_bootrom: bool,
  #[arg(long, default_value_t = 2)] scale: u32,
  #[arg(long)] fullscreen: bool,
  #[arg(long)] fullscreen_native: bool,
}

fn main() {
  //Parse arguments
  let args = Args::parse();
  let scale = args.scale;

  //Create a Gameboy struct
  let mut gb = Gameboy::new();

  //Load the ROM file
  let rom = std::fs::read(args.rom_path).expect("Failed to load the ROM file");
  gb.load_rom(&rom).expect("Invalid ROM file");

  //Skip bootrom
  if args.skip_bootrom {
    gb.skip_bootrom();
  }

  //Initialize SDL2 Context, VideoSubsystem, Window, EventPump and Canvas
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = {
    let mut builder = video_subsystem.window(
      "YargeSDL", 
      GB_WIDTH as u32 * scale,
      GB_HEIGHT as u32 * scale
    );
    builder.position_centered();
    if args.fullscreen {
      match args.fullscreen_native {
        true  => builder.fullscreen(),
        false => builder.fullscreen_desktop(),
      };
    }
    builder.build().unwrap()
  };
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut canvas = window.into_canvas().present_vsync().build().unwrap();
  
  //Create SDL2 texture
  let texture_creator = canvas.texture_creator();
  let mut gb_texture = texture_creator.create_texture_streaming(
    PixelFormatEnum::RGB24,
    GB_WIDTH as u32, 
    GB_HEIGHT as u32
  ).unwrap();

  //Main loop
  'run: loop {
    //Process SDL2 events
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          break 'run
        },
        _ => {}
      }
    }
    //Run emulation for one frame
    gb.run_for_frame().unwrap();
    //Copy data to texture
    let gb_data = gb.get_display_data();
    gb_texture.with_lock(None, |tex_data: &mut [u8], _pitch: usize| {
      for (index, color) in gb_data.iter().enumerate() {
        let mapped_color = GB_PALETTE[*color as usize];
        tex_data[3 * index] = mapped_color as u8;
        tex_data[(3 * index) + 1] = (mapped_color >> 8) as u8;
        tex_data[(3 * index) + 2] = (mapped_color >> 16) as u8;
      }
    }).unwrap();
    //Copy texture to the canvas
    canvas.copy(&gb_texture, None, None).unwrap();
    //Draw canvas
    canvas.present();
  }

  //   gb.set_key_state_all(
  //     ((window.is_key_down(Key::Right) as u8) * (GbKey::Right as u8)) |
  //     ((window.is_key_down(Key::Left) as u8) * (GbKey::Left as u8)) |
  //     ((window.is_key_down(Key::Up) as u8) * (GbKey::Up as u8)) |
  //     ((window.is_key_down(Key::Down) as u8) * (GbKey::Down as u8)) |
  //     (((window.is_key_down(Key::Z) || window.is_key_down(Key::NumPad0)) as u8) * (GbKey::A as u8)) |
  //     (((window.is_key_down(Key::X) || window.is_key_down(Key::NumPad1)) as u8) * (GbKey::B as u8)) |
  //     ((window.is_key_down(Key::RightShift) as u8) * (GbKey::Select as u8)) |
  //     ((window.is_key_down(Key::Enter) as u8) * (GbKey::Start as u8))
  //   );
}
