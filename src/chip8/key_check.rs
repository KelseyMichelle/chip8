use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn keycheck(sdl_context: &sdl2::Sdl, keypress: u8) -> u8 {
     let mut out = keypress;
     let mut event_pump = sdl_context.event_pump().unwrap();
     for event in event_pump.poll_iter() {
          let key = match event {
               Event::Quit { .. }
               | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
               } => std::process::exit(0x0100),

               Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
               } => 0,
               Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
               } => 1,
               Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
               } => 2,
               Event::KeyDown {
                    keycode: Some(Keycode::X),
                    ..
               } => 3,
               Event::KeyDown {
                    keycode: Some(Keycode::Z),
                    ..
               } => 4,
               Event::KeyDown {
                    keycode: Some(Keycode::C),
                    ..
               } => 5,
               Event::KeyDown {
                    keycode: Some(Keycode::U),
                    ..
               } => 6,
               Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
               } => 7,
               Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
               } => 8,
               Event::KeyDown {
                    keycode: Some(Keycode::E),
                    ..
               } => 9,
               Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
               } => 10,
               Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
               } => 11,
               Event::KeyDown {
                    keycode: Some(Keycode::K),
                    ..
               } => 12,
               Event::KeyDown {
                    keycode: Some(Keycode::M),
                    ..
               } => 13,
               Event::KeyDown {
                    keycode: Some(Keycode::H),
                    ..
               } => 14,
               Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
               } => 15,
               Event::KeyUp { .. } => {
                    out = 17;
                    17
               }
               _ => 17,
          };
          if key != 17 {
               out = key;
          }
     }
     if out != keypress && keypress != 17 {
          out = 17;
     }
     out
}
