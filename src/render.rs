use std::{
    ffi::{c_void, CString},
    fmt::Display,
};

use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event_loop::EventLoop,
    window::{Fullscreen, Window, WindowBuilder, WindowId},
};

use crate::{
    math::{RandomNumberGenerator, Vec3},
    world::Camera,
};

const CUBE_VERTEX_SHADER_SRC: &str = include_str!("../shaders/cube.vert");
const CUBE_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/cube.frag");

pub(crate) struct Renderer {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    cube_program: Program,
    cube_vertex_array_id: GLuint,
    cube_texture_id: GLuint,
}

impl Renderer {
    pub(crate) fn new(event_loop: &EventLoop<()>, windowed: bool) -> Self {
        let fullscreen_option = if windowed {
            None
        } else {
            Some(Fullscreen::Borderless(None))
        };

        let window_builder = WindowBuilder::new()
            .with_title("iridium")
            .with_fullscreen(fullscreen_option);

        let config_template = ConfigTemplateBuilder::default();
        let (window, config) = DisplayBuilder::new()
            .with_window_builder(Some(window_builder))
            .build(event_loop, config_template, |mut configs| {
                configs.next().unwrap()
            })
            .unwrap();
        let window = window.unwrap();
        let display = config.display();

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 0))))
            .build(Some(window.raw_window_handle()));

        let surface_attributes =
            window.build_surface_attributes(SurfaceAttributesBuilder::default());
        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attributes)
                .unwrap()
        };

        let context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .unwrap()
                .make_current(&surface)
                .unwrap()
        };

        gl::load_with(|s| display.get_proc_address(&CString::new(s).unwrap()));

        let cube_program =
            Program::build(CUBE_VERTEX_SHADER_SRC, CUBE_FRAGMENT_SHADER_SRC).unwrap();

        unsafe {
            gl::UseProgram(cube_program.gl_id());
        }

        let cube_vertex_array_id = unsafe {
            let mut cube_vertex_array_id = 0;
            gl::GenVertexArrays(1, &mut cube_vertex_array_id);
            gl::BindVertexArray(cube_vertex_array_id);
            cube_vertex_array_id
        };

        let cube_texture_id = unsafe {
            let mut cube_texture_id = 0;
            gl::GenTextures(1, &mut cube_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture_id);

            const TEXTURE_WIDTH: usize = 8;
            const TEXTURE_HEIGHT: usize = 8;
            const VALUES_PER_PIXEL: usize = 3;
            const TEXTURE_SIZE: usize = TEXTURE_WIDTH * TEXTURE_HEIGHT * VALUES_PER_PIXEL;

            let mut rng = RandomNumberGenerator::with_seed(42);
            let mut data: Vec<u8> = Vec::with_capacity(TEXTURE_SIZE);

            for _ in 0..TEXTURE_SIZE {
                data.push(rng.gen_range(120, 180) as u8);
            }

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB8 as GLint,
                TEXTURE_WIDTH as GLsizei,
                TEXTURE_HEIGHT as GLsizei,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST_MIPMAP_NEAREST as GLint,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            cube_texture_id
        };

        unsafe {
            gl::ClearColor(0.6, 0.4, 0.8, 1.0);
            gl::Enable(gl::DEPTH_TEST);
        }

        Self {
            window,
            surface,
            context,
            cube_program,
            cube_vertex_array_id,
            cube_texture_id,
        }
    }

    pub(crate) fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub(crate) fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub(crate) fn draw_cube(&mut self, position: &Vec3) {
        self.cube_program.set_uniform_vec3("position", position);

        unsafe {
            gl::UseProgram(self.cube_program.gl_id());
            gl::BindVertexArray(self.cube_vertex_array_id);
            gl::BindTexture(gl::TEXTURE_2D, self.cube_texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    pub(crate) fn present(&mut self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    pub(crate) fn set_viewport(&mut self) {
        let window_size = self.window.inner_size();
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;
        self.cube_program
            .set_uniform_f32("aspect_ratio", &aspect_ratio);

        unsafe {
            gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
        }
    }

    pub(crate) fn set_camera(&mut self, camera: &Camera) {
        self.cube_program
            .set_uniform_vec3("camera_position", camera.position());
        self.cube_program
            .set_uniform_f32("camera_heading", &camera.heading());
    }
}

struct ProgramId(GLuint);

struct Program {
    id: ProgramId,
}

impl Program {
    fn build(vertex_shader_src: &str, fragment_shader_src: &str) -> Result<Self, ShaderError> {
        let vertex_shader = Shader::compile(vertex_shader_src, ShaderType::Vertex)?;
        let fragment_shader = Shader::compile(fragment_shader_src, ShaderType::Fragment)?;

        let program_id = unsafe { gl::CreateProgram() };

        let linking_was_successful: bool = unsafe {
            gl::AttachShader(program_id, vertex_shader.gl_id());
            gl::AttachShader(program_id, fragment_shader.gl_id());
            gl::LinkProgram(program_id);
            gl::DetachShader(program_id, vertex_shader.gl_id());
            gl::DetachShader(program_id, fragment_shader.gl_id());

            let mut linking_was_successful: GLint = gl::FALSE as GLint;
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut linking_was_successful);

            linking_was_successful == gl::TRUE as GLint
        };

        if linking_was_successful {
            Ok(Self {
                id: ProgramId(program_id),
            })
        } else {
            let error_message_len: usize = unsafe {
                let mut error_message_len = 0;
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut error_message_len);
                error_message_len as usize
            };

            let mut error_message_buffer: Vec<u8> = vec![b' '; error_message_len];
            let mut bytes_read = 0;

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    error_message_len as GLint,
                    &mut bytes_read,
                    error_message_buffer.as_mut_ptr() as *mut GLchar,
                );
            }

            let filled_buffer = &error_message_buffer[..(bytes_read as usize)];
            let error_message = CString::new(filled_buffer).unwrap().into_string().unwrap();

            Err(ShaderError::Linking(error_message))
        }
    }

    fn gl_id(&self) -> GLuint {
        self.id.0
    }

    fn set_uniform_vec3(&mut self, name: &str, value: &Vec3) {
        let Vec3(x, y, z) = *value;

        unsafe {
            gl::Uniform3f(self.uniform_location(name), x, y, z);
        }
    }

    fn set_uniform_f32(&mut self, name: &str, value: &f32) {
        unsafe {
            gl::Uniform1f(self.uniform_location(name), *value);
        }
    }

    fn uniform_location(&self, name: &str) -> GLint {
        let cstr_name = CString::new(name).unwrap();
        unsafe { gl::GetUniformLocation(self.gl_id(), cstr_name.as_ptr()) }
    }
}

struct ShaderId(GLuint);

struct Shader {
    id: ShaderId,
}

impl Shader {
    fn compile(source: &str, shader_type: ShaderType) -> Result<Self, ShaderError> {
        let source = CString::new(source).unwrap();
        let id = unsafe { gl::CreateShader(shader_type.gl_shader_type()) };

        let compile_was_successful: bool = unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);

            let mut compile_was_successful: GLint = gl::FALSE as GLint;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut compile_was_successful);

            compile_was_successful == gl::TRUE as GLint
        };

        if compile_was_successful {
            Ok(Self { id: ShaderId(id) })
        } else {
            let error_message_len: usize = unsafe {
                let mut error_message_len = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut error_message_len);
                error_message_len as usize
            };

            let mut error_message_buffer: Vec<u8> = vec![b' '; error_message_len];
            let mut bytes_read = 0;

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    error_message_len as GLint,
                    &mut bytes_read,
                    error_message_buffer.as_mut_ptr() as *mut GLchar,
                );
            }

            let filled_buffer = &error_message_buffer[..(bytes_read as usize)];
            let error_message = CString::new(filled_buffer).unwrap().into_string().unwrap();

            Err(ShaderError::Compilation(error_message))
        }
    }

    fn gl_id(&self) -> GLuint {
        self.id.0
    }
}

enum ShaderType {
    Vertex,
    Fragment,
}

impl ShaderType {
    fn gl_shader_type(&self) -> GLenum {
        match self {
            Self::Vertex => gl::VERTEX_SHADER,
            Self::Fragment => gl::FRAGMENT_SHADER,
        }
    }
}

#[derive(Debug)]
enum ShaderError {
    Compilation(String),
    Linking(String),
}

impl Display for ShaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compilation(error) => write!(f, "shader compilation error: {}", error),
            Self::Linking(error) => write!(f, "shader linking error: {}", error),
        }
    }
}
