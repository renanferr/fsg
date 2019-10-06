use super::particle::Particle;
use super::webgl;
use std::cell::{BorrowError, BorrowMutError, RefCell};
use std::string::String;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};

pub enum DrawError {
    BorrowMutErr(BorrowMutError),
    BorrowErr(BorrowError),
    JsErr(JsValue),
    Str(String),
}

impl From<BorrowMutError> for DrawError {
    fn from(e: BorrowMutError) -> Self {
        DrawError::BorrowMutErr(e)
    }
}

impl From<BorrowError> for DrawError {
    fn from(e: BorrowError) -> Self {
        DrawError::BorrowErr(e)
    }
}

impl From<JsValue> for DrawError {
    fn from(e: JsValue) -> Self {
        DrawError::JsErr(e)
    }
}

impl From<String> for DrawError {
    fn from(e: String) -> Self {
        DrawError::Str(e)
    }
}

pub struct Scene {
    particles: RefCell<Vec<Particle>>,
    context: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    program: WebGlProgram,
    gravity: f32,
    width: f32,
    height: f32,
    pub should_spawn_particles: bool,
}

impl Scene {
    // #[wasm_bindgen()]
    pub fn new(
        canvas: HtmlCanvasElement,
        vert_shader_src: std::string::String,
        frag_shader_src: std::string::String,
    ) -> Result<Scene, JsValue> {
        let context = canvas
            .get_context("webgl")?
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()?;

        let program = webgl::init_program(&context, &vert_shader_src, &frag_shader_src)?;
        let canvas_width = canvas.width() as f32;
        let canvas_height = canvas.height() as f32;

        Scene::set_size_uniforms(&context, &program, (canvas_width, canvas_height));

        return Ok(Scene {
            particles: RefCell::new(Vec::new()),
            context: context,
            canvas: canvas,
            program: program,
            gravity: 1.0,
            height: canvas_height,
            width: canvas_width,
            should_spawn_particles: false,
        });
    }

    pub fn add_particle(&self, p: Particle) -> Result<(), BorrowMutError> {
        let mut particles = self.particles.try_borrow_mut()?;
        particles.push(p);
        return Ok(());
    }

    pub fn draw(&self) -> Result<(), DrawError> {
        let buffer = self
            .context
            .create_buffer()
            .ok_or(String::from("failed to create buffer"))?;

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        let mut vertices: Vec<f32> = Vec::new();
        let particles = self.particles.try_borrow()?;
        for p in particles.iter() {
            let pos = p.get_position();
            vertices.push(pos.x);
            vertices.push(pos.y);
        }

        // println!("{:?}", particles);
        web_sys::console::log_1(&JsValue::from(format!("{:?}", vertices)));
        // web_sys::console::log_1(&"oi".into());
        // web_sys::console::log(&"oie".into());
        // println!("{:?}", vertices);
        // println!(vertices);

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);

            self.context.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        let loc = self
            .context
            .get_attrib_location(&self.program, "spritePosition");
        self.context.enable_vertex_attrib_array(loc as u32);
        self.context.vertex_attrib_pointer_with_i32(
            loc as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context
            .draw_arrays(WebGlRenderingContext::POINTS, 0, 1);

        return Ok(());
    }

    pub fn width(&self) -> f32 {
        return self.width;
    }

    pub fn height(&self) -> f32 {
        return self.height;
    }

    pub fn get_context(&self) -> &WebGlRenderingContext {
        return &self.context;
    }

    pub fn get_canvas(&self) -> &HtmlCanvasElement {
        return &self.canvas;
    }

    fn set_size_uniforms(
        ctx: &WebGlRenderingContext,
        program_ptr: &WebGlProgram,
        size: (f32, f32),
    ) {
        let uniform_loc = ctx.get_uniform_location(program_ptr, "screenSize");
        let (width, height) = size;

        ctx.uniform2f(uniform_loc.as_ref(), width, height);
    }
}
