use super::particle::Particle;
use super::webgl;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlRenderingContext};
use std::borrow::BorrowMut;
use std::cell::RefCell;

pub struct Scene {
    particles: Vec<Particle>,
    context: WebGlRenderingContext,
    canvas: HtmlCanvasElement,
    program: WebGlProgram,
    gravity: f32,
    width: f32,
    height: f32,
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
            particles: Vec::new(),
            context: context,
            canvas: canvas,
            program: program,
            gravity: 1.0,
            height: canvas_height,
            width: canvas_width,
        });
    }

    pub fn add_particle(&self, p: Particle) {
        self.particles.borrow_mut().push(p)
    }

    fn draw(&self) -> Result<(), JsValue> {
        let buffer = self
            .context
            .create_buffer()
            .ok_or("failed to create buffer")?;
        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

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
