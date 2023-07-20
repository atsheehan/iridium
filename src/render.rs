use std::ffi::CString;

use gl::types::{GLenum, GLuint};
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
    window::{Window, WindowBuilder, WindowId},
};

const CUBE_VERTEX_SHADER_SRC: &str = include_str!("../shaders/cube.vert");
const CUBE_FRAGMENT_SHADER_SRC: &str = include_str!("../shaders/cube.frag");

pub(crate) struct Renderer {
    window: Window,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
    cube_program: Program,
    cube_vertex_array_id: GLuint,
}

impl Renderer {
    pub(crate) fn new(event_loop: &EventLoop<()>) -> Self {
        let window_builder = WindowBuilder::new().with_title("iridium");

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

        let cube_program = Program::build(CUBE_VERTEX_SHADER_SRC, CUBE_FRAGMENT_SHADER_SRC);

        unsafe {
            gl::UseProgram(cube_program.gl_id());
        }

        let cube_vertex_array_id = unsafe {
            let mut cube_vertex_array_id = 0;
            gl::GenVertexArrays(1, &mut cube_vertex_array_id);
            gl::BindVertexArray(cube_vertex_array_id);
            cube_vertex_array_id
        };

        unsafe {
            gl::ClearColor(0.6, 0.4, 0.8, 1.0);
        }

        Self {
            window,
            surface,
            context,
            cube_program,
            cube_vertex_array_id,
        }
    }

    pub(crate) fn window_id(&self) -> WindowId {
        self.window.id()
    }

    pub(crate) fn clear(&mut self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub(crate) fn draw_triangle(&mut self) {
        unsafe {
            gl::UseProgram(self.cube_program.gl_id());
            gl::BindVertexArray(self.cube_vertex_array_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub(crate) fn present(&mut self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }
}

struct ProgramId(GLuint);

struct Program {
    id: ProgramId,
}

impl Program {
    fn build(vertex_shader_src: &str, fragment_shader_src: &str) -> Self {
        let vertex_shader = Shader::compile(vertex_shader_src, ShaderType::Vertex);
        let fragment_shader = Shader::compile(fragment_shader_src, ShaderType::Fragment);

        let program_id = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(program_id, vertex_shader.gl_id());
            gl::AttachShader(program_id, fragment_shader.gl_id());
            gl::LinkProgram(program_id);
            gl::DetachShader(program_id, vertex_shader.gl_id());
            gl::DetachShader(program_id, fragment_shader.gl_id());
        }

        Self {
            id: ProgramId(program_id),
        }
    }

    fn gl_id(&self) -> GLuint {
        self.id.0
    }
}

struct ShaderId(GLuint);

struct Shader {
    id: ShaderId,
}

impl Shader {
    fn compile(source: &str, shader_type: ShaderType) -> Self {
        let source = CString::new(source).unwrap();
        let id = unsafe { gl::CreateShader(shader_type.gl_shader_type()) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        Self { id: ShaderId(id) }
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
