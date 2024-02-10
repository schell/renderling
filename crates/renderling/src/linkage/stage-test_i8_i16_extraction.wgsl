struct type_2 {
    member: array<u32>,
}

@group(0) @binding(0) 
var<storage, read_write> global: type_2;
var<private> global_1: vec3<u32>;

fn function() {
    let _e14 = global_1;
    let _e16 = (_e14.x < arrayLength((&global.member)));
    if _e16 {
        let _e19 = global.member[_e14.x];
        let _e23 = bitcast<i32>(((_e19 >> bitcast<u32>(16u)) & 255u));
        let _e28 = ((_e23 & 255i) - ((_e23 & 128i) << bitcast<u32>(1i)));
        if (_e28 > 0i) {
            if _e16 {
                global.member[_e14.x] = bitcast<u32>(_e28);
            }
        }
        if _e16 {
            let _e31 = global.member[_e14.x];
            let _e35 = bitcast<i32>(((_e31 >> bitcast<u32>(16u)) & 65535u));
            let _e40 = ((_e35 & 65535i) - ((_e35 & 32768i) << bitcast<u32>(1i)));
            if (_e40 > 0i) {
                if _e16 {
                    global.member[_e14.x] = bitcast<u32>(_e40);
                }
            }
        }
    }
    return;
}

@compute @workgroup_size(32, 1, 1) 
fn stagetest_i8_i16_extraction(@builtin(global_invocation_id) param: vec3<u32>) {
    global_1 = param;
    function();
}
