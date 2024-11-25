var<private> global: vec2<f32>;
var<private> global_1: vec4<f32>;
@group(0) @binding(1) 
var global_2: sampler;
@group(0) @binding(0) 
var global_3: texture_2d<f32>;

fn function() {
    let _e4 = global;
    let _e5 = textureSample(global_3, global_2, _e4);
    global_1 = _e5;
    return;
}

@fragment 
fn convolutiongenerate_mipmap_fragment(@location(0) param: vec2<f32>) -> @location(0) vec4<f32> {
    global = param;
    function();
    let _e3 = global_1;
    return _e3;
}
