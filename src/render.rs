use std::{
    collections::HashMap,
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
const SKYBOX_VERTEX_SHADER_SRC: &str = include_str!("../shaders/skybox.vert");
const SKYBOX_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/skybox.frag");

pub(crate) struct Renderer {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    cube_program: Program,
    cube_vertex_array_id: GLuint,
    cube_texture_id: GLuint,
    position_array_buffer_id: GLuint,
    cube_count: usize,
    skybox_program: Program,
    skybox_vertex_array_id: GLuint,
    skybox_texture_id: GLuint,
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
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);
        }

        let cube_vertex_array_id = unsafe {
            let mut cube_vertex_array_id = 0;
            gl::GenVertexArrays(1, &mut cube_vertex_array_id);
            gl::BindVertexArray(cube_vertex_array_id);
            cube_vertex_array_id
        };

        let position_array_buffer_id = unsafe {
            let mut position_array_id = 0;
            gl::GenBuffers(1, &mut position_array_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, position_array_id);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
            gl::VertexAttribDivisor(0, 1);
            position_array_id
        };

        let cube_texture_id = unsafe {
            let mut cube_texture_id = 0;
            gl::GenTextures(1, &mut cube_texture_id);
            gl::BindTexture(gl::TEXTURE_2D, cube_texture_id);

            const TEXTURE_WIDTH: usize = 8;
            const TEXTURE_HEIGHT: usize = 8;
            const NUM_PIXELS: usize = TEXTURE_WIDTH * TEXTURE_HEIGHT;
            const VALUES_PER_PIXEL: usize = 3;
            const TEXTURE_SIZE: usize = NUM_PIXELS * VALUES_PER_PIXEL;

            let mut rng = RandomNumberGenerator::with_seed(42);
            let mut texture: Vec<u8> = Vec::with_capacity(TEXTURE_SIZE);

            for _ in 0..NUM_PIXELS {
                let value = rng.gen_range(50, 200) as u8;
                texture.push(value);
                texture.push(value);
                texture.push(value);
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
                texture.as_ptr() as *const c_void,
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

        let skybox_program =
            Program::build(SKYBOX_VERTEX_SHADER_SRC, SKYBOX_FRAGMENT_SHADER_SRC).unwrap();

        let skybox_vertex_array_id = unsafe {
            let mut skybox_vertex_array_id = 0;
            gl::GenVertexArrays(1, &mut skybox_vertex_array_id);
            gl::BindVertexArray(skybox_vertex_array_id);
            skybox_vertex_array_id
        };

        let skybox_texture_id = unsafe {
            let mut skybox_texture_id = 0;
            gl::GenTextures(1, &mut skybox_texture_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, skybox_texture_id);

            const CUBEMAP_LENGTH: usize = 512;
            const NUMBER_OF_STARS: u32 = 200;

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 2);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_POSITIVE_X, CUBEMAP_LENGTH, &image_data);

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 3);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_POSITIVE_Y, CUBEMAP_LENGTH, &image_data);

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 4);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_POSITIVE_Z, CUBEMAP_LENGTH, &image_data);

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 5);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_NEGATIVE_X, CUBEMAP_LENGTH, &image_data);

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 6);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_NEGATIVE_Y, CUBEMAP_LENGTH, &image_data);

            let image_data =
                Self::generate_image_with_random_stars(CUBEMAP_LENGTH, NUMBER_OF_STARS, 7);
            Self::upload_texture(gl::TEXTURE_CUBE_MAP_NEGATIVE_Z, CUBEMAP_LENGTH, &image_data);

            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MIN_FILTER,
                gl::NEAREST as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_CUBE_MAP,
                gl::TEXTURE_MAG_FILTER,
                gl::NEAREST as i32,
            );

            skybox_texture_id
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
            position_array_buffer_id,
            cube_count: 0,
            skybox_program,
            skybox_vertex_array_id,
            skybox_texture_id,
        }
    }

    fn upload_texture(target: GLenum, length: usize, image_data: &[u8]) {
        unsafe {
            gl::TexImage2D(
                target,
                0,
                gl::RGB8 as GLint,
                length as i32,
                length as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                image_data.as_ptr() as *const c_void,
            );
        }
    }

    fn generate_image_with_random_stars(
        image_length: usize,
        number_of_stars: u32,
        seed: u32,
    ) -> Vec<u8> {
        let image_size = image_length * image_length;
        let mut grayscale_pixels = vec![0; image_size];
        let mut rng = RandomNumberGenerator::with_seed(seed);

        for _ in 0..number_of_stars {
            let x = rng.gen_range(0, image_length as u32) as usize;
            let y = rng.gen_range(0, image_length as u32) as usize;
            let intensity = rng.gen_range(10, 256) as u8;

            let index = y * image_length + x;
            grayscale_pixels[index] = intensity;
        }

        grayscale_pixels.iter().flat_map(|p| [*p, *p, *p]).collect()
    }

    pub(crate) fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub(crate) fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub(crate) fn draw_cubes(&mut self) {
        self.activate_cube_program();

        unsafe {
            gl::BindVertexArray(self.cube_vertex_array_id);
            gl::BindTexture(gl::TEXTURE_2D, self.cube_texture_id);
            gl::DrawArraysInstanced(gl::TRIANGLES, 0, 36, self.cube_count as GLint);
        }
    }

    pub(crate) fn draw_skybox(&mut self) {
        self.activate_skybox_program();

        unsafe {
            gl::BindVertexArray(self.skybox_vertex_array_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.skybox_texture_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }

    pub(crate) fn update_block_cache(&mut self, positions: impl Iterator<Item = Vec3>) {
        let position_buffer: Vec<f32> = positions
            .flat_map(|position| [position.x(), position.y(), position.z()])
            .collect();

        self.cube_count = position_buffer.len() / 3;

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.position_array_buffer_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * 3 * self.cube_count) as isize,
                position_buffer.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );
        }
    }

    pub(crate) fn present(&mut self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }

    pub(crate) fn set_viewport(&mut self) {
        let window_size = self.window.inner_size();
        let aspect_ratio = window_size.width as f32 / window_size.height as f32;

        self.activate_cube_program()
            .set_uniform_f32("aspect_ratio", &aspect_ratio);
        self.activate_skybox_program()
            .set_uniform_f32("aspect_ratio", &aspect_ratio);

        unsafe {
            gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
        }
    }

    pub(crate) fn set_camera(&mut self, camera: &Camera) {
        let mut program = self.activate_cube_program();
        program.set_uniform_vec3("camera_position", camera.position());
        program.set_uniform_f32("camera_heading", &camera.heading());
        program.set_uniform_f32("camera_pitch", &camera.pitch());

        let mut program = self.activate_skybox_program();
        program.set_uniform_f32("camera_heading", &camera.heading());
        program.set_uniform_f32("camera_pitch", &camera.pitch());
    }

    fn activate_cube_program(&mut self) -> ActiveProgram<'_> {
        unsafe {
            gl::UseProgram(self.cube_program.gl_id());
            gl::BindVertexArray(self.cube_vertex_array_id);
            gl::BindTexture(gl::TEXTURE_2D, self.cube_texture_id);
        }

        ActiveProgram {
            program: &mut self.cube_program,
        }
    }

    fn activate_skybox_program(&mut self) -> ActiveProgram<'_> {
        unsafe {
            gl::UseProgram(self.skybox_program.gl_id());
            gl::BindVertexArray(self.skybox_vertex_array_id);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, self.skybox_texture_id);
        }

        ActiveProgram {
            program: &mut self.skybox_program,
        }
    }
}

struct ActiveProgram<'a> {
    program: &'a mut Program,
}

impl<'a> ActiveProgram<'a> {
    fn set_uniform_vec3(&mut self, name: &'static str, value: &Vec3) {
        let Vec3(x, y, z) = *value;

        unsafe {
            gl::Uniform3f(self.program.uniform_location(name), x, y, z);
        }
    }

    fn set_uniform_f32(&mut self, name: &'static str, value: &f32) {
        unsafe {
            gl::Uniform1f(self.program.uniform_location(name), *value);
        }
    }
}

struct ProgramId(GLuint);

struct Program {
    id: ProgramId,
    cached_uniform_locations: HashMap<&'static str, GLint>,
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
                cached_uniform_locations: HashMap::new(),
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

    fn uniform_location(&mut self, name: &'static str) -> GLint {
        match self.cached_uniform_locations.get(name) {
            Some(location) => *location,
            None => {
                let cstr_name = CString::new(name).unwrap();
                let location = unsafe { gl::GetUniformLocation(self.gl_id(), cstr_name.as_ptr()) };
                self.cached_uniform_locations.insert(name, location);
                location
            }
        }
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
