use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};
mod webgl;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    // let contents = fs::read_to_string("./shaders/frag_shader.frag")
    //     .expect("Something went wrong reading the file");

    // println!("With text:\n{}", contents);

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
    let uniform_loc = context.get_uniform_location(&program, "screenSize");
    context.uniform2f(
        uniform_loc.as_ref(),
        canvas.width() as f32,
        canvas.height() as f32,
    );

    // let vertices: [f32; 18] = [
    //     // ESQUERDA
    //     -0.25, // X
    //     -0.25, // Y
    //     0.0, // Z

    //     // DIREITA
    //     0.25,
    //     -0.25,
    //     0.0,

    //     // TOPO
    //     0.25,
    //     0.5,
    //     0.0,

    //     // ESQUERDA
    //     -0.25,
    //     -0.25,
    //     0.0,

    //     //
    //     -0.25,
    //     0.5,
    //     0.0,

    //     0.25,
    //     0.5,
    //     0.0
    // ];

    let vertices: [f32; 3] = [
        128.0,
        128.0,
        128.0,
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
    // context.enable_vertex_attrib_array(0);

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(
        WebGlRenderingContext::POINTS,
        0,
        (vertices.len() / 3) as i32,
        // 1,
    );
    Ok(())
}