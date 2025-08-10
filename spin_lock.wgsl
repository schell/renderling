struct type_2 {
    member: array<u32>,
}

struct type_10 {
    member: array<atomic<u32>>,
}

@group(0) @binding(0) 
var<storage, read_write> global: type_10;

fn function() {
    var phi_99_: bool;

    switch bitcast<i32>(0u) {
        default: {
            loop {
                if (0u < arrayLength((&global.member))) {
                } else {
                    phi_99_ = true;
                    break;
                }
                let _e15 = atomicExchange((&global.member[0u]), 1u);
                continue;
                continuing {
                    phi_99_ = false;
                    break if !(select(true, false, (_e15 == 0u)));
                }
            }
            let _e20 = phi_99_;
            if _e20 {
                break;
            }
            let _e22 = atomicLoad((&global.member[1u]));
            atomicStore((&global.member[1u]), bitcast<u32>((bitcast<f32>(_e22) + 1f)));
            break;
        }
    }
    return;
}

@compute @workgroup_size(16, 1, 1) 
fn synctest_spin_lock() {
    function();
}
