// Vertex

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@location(0) position: vec4<f32>) -> VertexOutput {
  var out: VertexOutput;
  out.position = position;
  return out;
}

// Fragment

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.97, 0.88, 0.21, 1.0);
}
