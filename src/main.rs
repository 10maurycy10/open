extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use std::time::Duration;
use std::time::Instant;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use size_format::{SizeFormatterBinary, SizeFormatterSI};

use std::cmp::{min,max};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("[KEEP OPEN]", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let ttf_context = match sdl2::ttf::init().map_err(|e| e.to_string()) {
        Ok(x) => x,
        Err(x) => panic!("Cant get tff context E:{}",x)
    };
        
    let mut canvas = match window.into_canvas().present_vsync().build().map_err(|e| e.to_string()) {
        Ok(x) => x,
        Err(x) => panic!("Cant open window E:{}",x)
    };
    
    let texture_creator = canvas.texture_creator();
    
    // Load a font
    let mut font = ttf_context.load_font("font.tff", 128)?;
    
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let start = Instant::now();
    
    let mut last = Instant::now();
    
    // the amount of time spent without running
    let mut lag_time: f64 = 0.0;
    
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{
                    ..
                } => break 'running,
                _ => {}
            }
        }
    
        canvas.clear();
        
        let seconds_after_start:f64 = (Instant::now().checked_duration_since(start).unwrap().as_micros() as f64)/1000_000.0;
        
        let frame_micros = Instant::now().checked_duration_since(last).unwrap().as_micros() as f64;
        
        if (frame_micros) > 1000_000.0 {
            println!("frame was {} micros!",frame_micros);
            lag_time += frame_micros / 1000_000.0;
            println!("lag time is now {}",lag_time);
        }
        last = Instant::now();
    
        let window = canvas.window_mut();
        let (window_width,window_height) = window.size();
        drop(window);
        
        let text = &*format!("{:.20}s",
            SizeFormatterSI::new((seconds_after_start-lag_time) as u64)
        );
    
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(Color::RGBA(255, 255, 255, 255))
            .map_err(|e| e.to_string())?;
            
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
    
        //let TextureQuery { width, height, .. } = texture.query();
        
        let padding = 64;
        let target = rect!(
            0,
            0,
            min(surface.size().0,window_width),
            min(surface.size().1,window_height)
        );

        canvas.copy(&texture, None, Some(target))?;
    
        canvas.present();
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
       
    }

    Ok(())
}
