use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, HtmlCanvasElement, WebGlProgram};
mod webgl;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()?;

    let vert_shader_src = document
        .get_element_by_id("vertex-shader")
        .unwrap()
        .inner_html();

    let frag_shader_src = document
        .get_element_by_id("fragment-shader")
        .unwrap()
        .inner_html();

    let context = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let program = webgl::init_program(&context, &vert_shader_src, &frag_shader_src)?;
    
    let canvas_width = canvas.width() as f32;
    let canvas_height = canvas.height() as f32;

    set_size_uniforms(&context, &program, (canvas_width, canvas_height));

    let vertices: [f32; 2] = [
        (canvas_width / 2.0) as f32,
        (canvas_height / 2.0) as f32,
    ];

    let buffer = context.create_buffer().ok_or("failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    unsafe {
        let vert_array = js_sys::Float32Array::view(&vertices);

        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }

    let loc = context.get_attrib_location(&program, "spritePosition");
    context.enable_vertex_attrib_array(loc as u32);
    context.vertex_attrib_pointer_with_i32(
        loc as u32,
        2,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    
    context.draw_arrays(
        WebGlRenderingContext::POINTS,
        0,
        1,
    );
    Ok(())
}

fn set_size_uniforms(ctx: &WebGlRenderingContext, program_ptr: &WebGlProgram, size: (f32, f32)) {
    let uniform_loc = ctx.get_uniform_location(program_ptr, "screenSize");
    let (width, height) = size;
    ctx.uniform2f(
        uniform_loc.as_ref(),
        width,
        height,
    );
}