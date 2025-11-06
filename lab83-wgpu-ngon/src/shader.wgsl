struct Params {
    data: vec4f, // [width, height, n_float, radius]
};

@group(0) @binding(0) var<uniform> params: Params;

fn aspect_scale(p: vec2f, width: f32, height: f32) -> vec2f {
    let aspect = width / height;
    return vec2f(p.x / aspect, p.y);
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> @builtin(position) vec4f {
    let width = params.data.x;
    let height = params.data.y;
    let n = u32(round(params.data.z));
    let radius = params.data.w;

    // TriangleList: 3 vertices per triangle
    let tri = vid / 3u;           // which triangle (0..n-1)
    let corner = vid % 3u;        // 0=center, 1=ring[k], 2=ring[k+1]

    if (corner == 0u) {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }

    // WGSL doesn't support inline conditional expressions like Rust.
    // Use select() or an if-else to choose the vertex index.
    let k = select((tri + 1u) % n, tri, corner == 1u);
    let two_pi: f32 = 6.283185307179586;
    let t = two_pi * f32(k) / f32(n);
    let p = vec2f(cos(t), sin(t)) * radius;
    let ps = aspect_scale(p, width, height);
    return vec4f(ps, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    // A simple pleasant color
    return vec4f(0.2, 0.7, 1.0, 1.0);
}
