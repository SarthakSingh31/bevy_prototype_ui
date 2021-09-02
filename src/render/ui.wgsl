[[block]]
struct View {
    view_proj: mat4x4<f32>;
    world_position: vec3<f32>;
};
[[group(0), binding(0)]]
var view: View;

struct UiVertexInput {
    [[location(0)]] transform_0: vec4<f32>;
    [[location(1)]] transform_1: vec4<f32>;
    [[location(2)]] transform_2: vec4<f32>;
    [[location(3)]] transform_3: vec4<f32>;
    [[location(4)]] size: vec2<f32>;
    [[location(5)]] margin: vec2<f32>;
    [[location(6)]] background_color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vertex(
    [[builtin(vertex_index)]] vertex_index: u32,
    instance: UiVertexInput,
) -> VertexOutput {
    // Re-assemble the object transform matrix
    let object = mat4x4<f32>(
        instance.transform_0,
        instance.transform_1,
        instance.transform_2,
        instance.transform_3,
    );
    // Unit quad (using PrimitiveTopology::TriangleStrip)
    var unit_quad_pos: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
        vec2<f32>( 0.5, -0.5), // bottom right
        vec2<f32>(-0.5, -0.5), // bottom left
        vec2<f32>( 0.5,  0.5), // top right
        vec2<f32>(-0.5,  0.5), // top left
    );
    // var unit_quad_uv: array<vec2<f32>, 4> = array<vec2<f32>, 4>(
    //     vec2<f32>(1.0, 1.0), // bottom right
    //     vec2<f32>(0.0, 1.0), // bottom left
    //     vec2<f32>(1.0, 0.0), // top right
    //     vec2<f32>(0.0, 0.0), // top left
    // );
    // Scale the vertices of the unit square
    let scaled_position = unit_quad_pos[vertex_index] * instance.size;
    // Apply the object and view transformations
    let clip_position = view.view_proj * object * vec4<f32>(scaled_position, 0.0, 1.0);
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = vec4<f32>(0.);
    // out.uv = unit_quad_uv[vertex_index];
    return out;
}

struct FragmentOutput {
    [[location(0)]] color: vec4<f32>;
};

[[stage(fragment)]]
fn fragment(input: VertexOutput) -> FragmentOutput {
    // Flat color in the full quad
    var out: FragmentOutput;
    out.color = input.color;
    return out;
}