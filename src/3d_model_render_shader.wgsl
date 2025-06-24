// struct VertexOut {
//     @builtin(position) pos: vec4<f32>,
//     @location(0) color: vec3<f32>,
// };

// @vertex
// fn vs_main(
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>
// ) -> VertexOut {
//     var out: VertexOut;
//     out.pos = vec4(position, 1.0);
//     out.color = color;
//     return out;
// }

// @fragment
// fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
//     return vec4(color, 1.0);
// }

// Vertex shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec3<f32>, @location(1) color: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(pos, 1.0);
    out.color = color;
    return out;
}

// Fragment shader
@fragment
fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}