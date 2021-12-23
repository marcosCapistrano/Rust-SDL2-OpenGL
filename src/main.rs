extern crate gl;
extern crate sdl2;

use gl::types::*;
use sdl2::{TimerSubsystem, libc::CS};
use specs::prelude::*;
use specs_derive::Component;
use std::ffi::CString;
use utils::opengl::{Program, Shader};

struct Game {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    timer_subsystem: sdl2::TimerSubsystem,

    window: sdl2::video::Window,
    gl_context: sdl2::video::GLContext,
    program: Program,
    vao: GLuint,
}

impl Game {
    fn new(width: u32, height: u32) -> Game {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let timer_subsystem = sdl_context
            .timer()
            .expect("Failed to initialize timer subsystem");

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 5);

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let gl_context = window.gl_create_context().unwrap();
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

        let vertex_shader =
            Shader::from_vert_source(&CString::new(include_str!("shaders/triangle.vert")).unwrap())
                .unwrap();
        let frag_shader =
            Shader::from_frag_source(&CString::new(include_str!("shaders/triangle.frag")).unwrap())
                .unwrap();

        let tcs_shader = Shader::from_tcs_source(&CString::new(include_str!("shaders/triangle.tcs")).unwrap()).unwrap();
        let tes_shader = Shader::from_tes_source(&CString::new(include_str!("shaders/triangle.tes")).unwrap()).unwrap();
        let geom_shader = Shader::from_geom_source(&CString::new(include_str!("shaders/triangle.geom")).unwrap()).unwrap();

        let program = Program::from_shaders(&[vertex_shader,  /*tcs_shader, tes_shader, geom_shader,*/frag_shader]).unwrap();
        program.set_used();

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }

        let mut vao: GLuint = 0;
        unsafe {
            gl::CreateVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }

        Game {
            sdl_context,
            video_subsystem,
            timer_subsystem,

            window,
            gl_context,
            program,
            vao,
        }
    }

    fn run(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. }
                    | sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.render((self.timer_subsystem.ticks() as f32) / 1000.0);
            self.window.gl_swap_window();
        }
    }

    fn render(&self, current_time: f32) {
        let black: *const f32 = [0.0, 0.0, 0.0, 1.0].as_ptr();
        let red: *const f32 = [
            current_time.sin() * 0.5 + 0.5,
            current_time.cos() * 0.5 + 0.5,
            0.0,
            1.0,
        ]
        .as_ptr();

        let red2: *const f32 = [
            current_time.cos() * 0.5 + 0.5,
            current_time.sin() * 0.5 + 0.5,
            0.0,
            1.0,
        ]
        .as_ptr();

        unsafe { gl::ClearBufferfv(gl::COLOR, 0, black) }

        self.program.set_used();
        let attrib: [GLfloat; 4] = [current_time.sin() * 0.5, current_time.cos() * 0.5, 0.0, 0.0];
        unsafe {
            gl::VertexAttrib4fv(0, attrib.as_ptr());
            gl::VertexAttrib4fv(1, red2);
        }

        unsafe {
            // gl::PatchParameteri(gl::PATCH_VERTICES, 3);
            // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub fn main() {
    let game = Game::new(800, 600);

    game.run();
}
