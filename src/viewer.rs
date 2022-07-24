use crate::body::Body;
use crate::camera::Camera;
use crate::grid::Grid;
use wasm_bindgen::JsCast;
use web_sys::*;

pub struct Viewer {
    context: WebGlRenderingContext,
    transform_location: WebGlUniformLocation,
}

impl Viewer {
    pub fn new() -> Self {
        // grab context
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGlRenderingContext>()
            .unwrap();

        canvas.set_height(800);
        canvas.set_width(800);
        context.viewport(0, 0, 800, 800);
        context.enable(WebGlRenderingContext::DEPTH_TEST);

        // create program
        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
                attribute vec4 position;
                attribute vec3 normal;

                uniform mat4 world_to_camera;

                varying vec3 v_normal;
                varying vec3 v_position;

                void main() {
                    gl_Position = world_to_camera * position;
                    v_normal = normal;
                    v_position = position.xyz;
                }
            "#,
        )
        .unwrap();
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"
                precision mediump float;

                varying vec3 v_normal;
                varying vec3 v_position;

                void main() {
                    vec3 n_normal = normalize(v_normal);
                    vec3 ambient_color = vec3(1.,1.,1.);

                    vec3 light1_position = vec3(1.,-1.,1.);
                    vec3 light1_direction = normalize(light1_position - v_position);
                    float light1_brightness = clamp(dot(n_normal, light1_direction),0.,1.);
                    vec3 light1_color = vec3(0.996, 0.972, 0.866);

                    vec3 light2_position = vec3(-1.,-1.,1.);
                    vec3 light2_direction = normalize(light2_position - v_position);
                    float light2_brightness = clamp(dot(n_normal, light2_direction),0.,1.);
                    vec3 light2_color = vec3(1, 1., 1.);
                    
                    gl_FragColor = vec4(0., 0., 0., 1.0);
                    gl_FragColor.rgb += ambient_color * 0.2; // ambient
                    gl_FragColor.rgb += light1_color * light1_brightness * 0.7; // normal
                    gl_FragColor.rgb += light2_color * light2_brightness * 0.5; // normal
                }
            "#,
        )
        .unwrap();
        let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
        context.use_program(Some(&program));

        // look up uniforms
        let transform_location = context
            .get_uniform_location(&program, "world_to_camera")
            .unwrap();
        Self {
            context: context,
            transform_location: transform_location,
        }
    }

    pub fn render(&self, camera: &Camera, grid: &Grid, body: &Body) {
        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
        self.context.clear(WebGlRenderingContext::DEPTH_BUFFER_BIT); // not sure if it needed

        // grid
        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.transform_location),
            false,
            &camera.get_world_to_camera(),
        );
        let vertices = grid.generate_vertices();
        bind_buffer(&self.context, &vertices, 0);
        bind_buffer(&self.context, &grid.generate_normals(), 1);
        self.context.draw_arrays(
            WebGlRenderingContext::LINES,
            0,
            (vertices.len() as f32 / 3.) as i32,
        );

        // body
        self.context.uniform_matrix4fv_with_f32_array(
            Some(&self.transform_location),
            false,
            &camera.get_world_to_camera(),
        );
        let vertices = body.generate_vertices();
        bind_buffer(&self.context, &vertices, 0);
        bind_buffer(&self.context, &body.generate_normals(), 1);
        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            (vertices.len() as f32 / 3.) as i32,
        );
    }
}

fn bind_buffer(context: &WebGlRenderingContext, array: &[f32], attribute_index: u32) {
    let buffer = context
        .create_buffer()
        .ok_or("failed to create buffer")
        .unwrap();
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let view_array = js_sys::Float32Array::view(&array);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &view_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    context.vertex_attrib_pointer_with_i32(
        attribute_index,
        3,
        WebGlRenderingContext::FLOAT,
        false,
        0,
        0,
    );
    context.enable_vertex_attrib_array(attribute_index);
}

fn compile_shader(
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

fn link_program(
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
