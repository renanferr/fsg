use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn init_program(ctx_ptr: &WebGlRenderingContext, vert_src_ptr: &str, frag_src_ptr: &str) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(
        ctx_ptr,
        WebGlRenderingContext::VERTEX_SHADER,
        vert_src_ptr,
    )?;
    let frag_shader = compile_shader(
        ctx_ptr,
        WebGlRenderingContext::FRAGMENT_SHADER,
        frag_src_ptr,
    )?;
    let program = link_program(ctx_ptr, &vert_shader, &frag_shader)?;
    ctx_ptr.use_program(Some(&program));

    return Ok(program)
}

pub fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGlRenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
