@fragment
fn fs_main(
    @location(0) color: vec3<f32>,
    @location(1) frag_pos: vec2<f32>
) -> @location(0) vec4<f32> {
    let dist = distance(frag_pos, uniforms.offset);
    let radius = 0.25;
    let edge = 0.01;

    let alpha = smoothstep(radius, radius - edge, dist);
    let circle_color = vec3<f32>(1.0, 0.0, 0.0);

    let final_color = mix(circle_color, color, alpha);
    return vec4<f32>(final_color, 1.0);
}



struct Uniforms {
    offset: vec2<f32>,
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) frag_pos: vec2<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let shifted = input.position.xy + uniforms.offset;
    output.position = vec4<f32>(input.position, 1.0);
    output.frag_pos = input.position.xy;
    output.color = input.color;
    return output;
}






