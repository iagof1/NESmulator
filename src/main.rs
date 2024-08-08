use nes::cpu::CPU;
use nes::rom::NesFile;

use sdl2::event::Event;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::error::Error;
use std::time::Duration;

fn initialize_sdl() -> (Sdl, Canvas<Window>) {
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context
        .video()
        .expect("Failed to initialize video subsystem");

    let window = video_subsystem
        .window("NES Emulator", 256, 240)
        .position_centered()
        .build()
        .expect("Failed to create window");

    let canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .expect("Failed to create canvas");

    (sdl_context, canvas)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize SDL2
    let (sdl_context, mut canvas) = initialize_sdl();
    let texture_creator = canvas.texture_creator();

    // Load the NES file
    let nes_file = NesFile::load("src/samples/Super Mario Bros. (World).nes")?;
    // Initialize CPU and PPU
    let mut cpu = CPU::new(
        nes_file.prg_rom.clone(),
        nes_file.chr_rom.clone(),
        nes_file.mirroring,
    );

    // Main loop
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, 256, 240)
        .expect("Failed to create texture");

    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        cpu.run();
        let frame = cpu.memory_bus.ppu.render_frame();

        texture
            .update(None, &frame, 256 * 4)
            .expect("Failed to update texture");
        canvas
            .copy(&texture, None, Some(Rect::new(0, 0, 256, 240)))
            .expect("Failed to copy texture to canvas");
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
