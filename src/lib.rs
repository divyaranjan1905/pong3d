use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
extern crate js_sys;

pub fn webgl_context(canvas_id: &str) -> Result<WebGlRenderingContext, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let gl: WebGlRenderingContext = canvas
        .get_context("webgl")?
	.unwrap()
	.dyn_into::<WebGlRenderingContext>() // Much of this dyn_into and unwrap() is for error checking and type safety purposes.
	.unwrap();

    gl.viewport(
    	0,
    	0,
    	canvas.width().try_into().unwrap(),
    	    canvas.height().try_into().unwrap(),);

    Ok(gl)
}

pub fn create_shader (gl: &WebGlRenderingContext, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
    let shader = gl.create_shader(shader_type).ok_or_else(|| JsValue::from_str("Unable to create shader object"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
	Ok(shader)
    } else {
	Err(JsValue::from_str(&gl.get_shader_info_log(&shader).unwrap_or_else(|| "Unknown error creating shader".into()),
	))
    }

}


// Read the .glsl files in ../shaders/ into strings
pub fn load_shaders(gl: &WebGlRenderingContext) -> Result<WebGlProgram, JsValue> {


}
