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
    [[location(5)]] _padding: vec2<f32>;
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
    // No need to use an index buffer only to save on two f32, the first vertex is repeated
    // (we use PrimitiveTopology::TriangleStrip)
    var unit_quad: array<vec2<f32>, 5> = array<vec2<f32>, 5>(
        vec2<f32>(-0.5, -0.5), // SW
        vec2<f32>(-0.5,  0.5), // NW
        vec2<f32>( 0.5,  0.5), // NE
        vec2<f32>( 0.5, -0.5), // SE
        vec2<f32>(-0.5, -0.5), // SW
    );
    // Scale the vertices of the unit square
    let scaled_position = unit_quad[vertex_index] * instance.size;
    // Apply the object and view transformations
    let clip_position = view.view_proj * object * vec4<f32>(scaled_position, 0.0, 1.0);
    var out: VertexOutput;
    out.clip_position = clip_position;
    out.color = vec4<f32>(0.);
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