struct type_11 {
    member: array<u32>,
}

struct type_18 {
    member: vec4<f32>,
    member_1: vec4<f32>,
    member_2: vec4<f32>,
    member_3: vec4<f32>,
}

struct type_19 {
    member: array<vec3<f32>, 8>,
    member_1: array<vec4<f32>, 6>,
}

struct type_20 {
    member: type_18,
    member_1: type_18,
    member_2: type_19,
    member_3: vec3<f32>,
}

struct type_22 {
    member: u32,
    member_1: u32,
}

struct type_25 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
}

struct type_28 {
    member: vec3<f32>,
    member_1: vec4<f32>,
    member_2: vec3<f32>,
    member_3: vec4<f32>,
    member_4: array<u32, 4>,
    member_5: array<f32, 4>,
    member_6: vec2<f32>,
    member_7: vec2<f32>,
}

struct VertexOutput {
    @location(0) member: vec4<f32>,
    @builtin(position) member_1: vec4<f32>,
}

var<private> global: u32;
@group(0) @binding(0) 
var<storage> global_1: type_11;
var<private> global_2: vec4<f32> = vec4<f32>(0f, 0f, 0f, 1f);
var<private> global_3: vec4<f32>;
var<private> global_4: u32;

fn function() {
    var local: array<vec3<f32>, 8>;
    var local_1: array<vec4<f32>, 6>;
    var local_2: array<f32, 4>;
    var local_3: array<u32, 4>;
    var phi_613_: u32;
    var phi_1413_: bool;
    var phi_699_: type_22;
    var phi_700_: type_22;
    var phi_723_: type_22;
    var phi_736_: bool;
    var phi_742_: type_22;
    var phi_743_: type_22;
    var phi_766_: type_22;
    var phi_780_: bool;
    var phi_784_: type_28;
    var phi_1443_: bool;
    var phi_835_: type_25;
    var phi_1513_: bool;
    var phi_996_: type_22;
    var phi_997_: type_22;
    var phi_1020_: type_22;
    var phi_1047_: bool;
    var phi_1053_: type_22;
    var phi_1054_: type_22;
    var phi_1077_: type_22;
    var phi_1100_: bool;
    var phi_1108_: type_20;

    let _e76 = global_4;
    let _e77 = global;
    let _e79 = arrayLength((&global_1.member));
    let _e83 = global_1.member[(_e76 + 1u)];
    let _e87 = global_1.member[(_e76 + 2u)];
    let _e91 = global_1.member[(_e76 + 9u)];
    let _e95 = global_1.member[(_e76 + 10u)];
    if (_e77 >= _e87) {
        phi_613_ = 4294967295u;
    } else {
        phi_613_ = (_e83 + (26u * _e77));
    }
    let _e100 = phi_613_;
    if (_e79 >= 26u) {
        phi_1413_ = (_e100 <= (_e79 - 26u));
    } else {
        phi_1413_ = false;
    }
    let _e105 = phi_1413_;
    if _e105 {
        let _e108 = global_1.member[_e100];
        let _e113 = global_1.member[(_e100 + 1u)];
        let _e118 = global_1.member[(_e100 + 2u)];
        let _e124 = global_1.member[(_e100 + 3u)];
        let _e129 = global_1.member[(_e100 + 4u)];
        let _e134 = global_1.member[(_e100 + 5u)];
        let _e139 = global_1.member[(_e100 + 6u)];
        let _e145 = global_1.member[(_e100 + 7u)];
        let _e150 = global_1.member[(_e100 + 8u)];
        let _e156 = global_1.member[(_e100 + 9u)];
        let _e161 = global_1.member[(_e100 + 10u)];
        let _e167 = global_1.member[(_e100 + 11u)];
        let _e172 = global_1.member[(_e100 + 12u)];
        let _e177 = global_1.member[(_e100 + 13u)];
        let _e183 = global_1.member[(_e100 + 14u)];
        let _e188 = global_1.member[(_e100 + 15u)];
        let _e193 = global_1.member[(_e100 + 16u)];
        let _e198 = global_1.member[(_e100 + 17u)];
        local_3 = array<u32, 4>(0u, 0u, 0u, 0u);
        phi_699_ = type_22(0u, 4u);
        loop {
            let _e203 = phi_699_;
            if (_e203.member < _e203.member_1) {
                phi_700_ = type_22((_e203.member + 1u), _e203.member_1);
                phi_723_ = type_22(1u, _e203.member);
            } else {
                phi_700_ = _e203;
                phi_723_ = type_22(0u, type_22().member_1);
            }
            let _e216 = phi_700_;
            let _e218 = phi_723_;
            switch bitcast<i32>(_e218.member) {
                case 0: {
                    phi_736_ = false;
                    break;
                }
                case 1: {
                    let _e225 = global_1.member[((_e100 + 18u) + _e218.member_1)];
                    local_3[_e218.member_1] = _e225;
                    phi_736_ = true;
                    break;
                }
                default: {
                    phi_736_ = bool();
                    break;
                }
            }
            let _e228 = phi_736_;
            continue;
            continuing {
                phi_699_ = _e216;
                break if !(_e228);
            }
        }
        let _e230 = local_3;
        local_2 = array<f32, 4>(0f, 0f, 0f, 0f);
        phi_742_ = type_22(0u, 4u);
        loop {
            let _e233 = phi_742_;
            if (_e233.member < _e233.member_1) {
                phi_743_ = type_22((_e233.member + 1u), _e233.member_1);
                phi_766_ = type_22(1u, _e233.member);
            } else {
                phi_743_ = _e233;
                phi_766_ = type_22(0u, type_22().member_1);
            }
            let _e246 = phi_743_;
            let _e248 = phi_766_;
            switch bitcast<i32>(_e248.member) {
                case 0: {
                    phi_780_ = false;
                    break;
                }
                case 1: {
                    let _e255 = global_1.member[((_e100 + 22u) + _e248.member_1)];
                    local_2[_e248.member_1] = bitcast<f32>(_e255);
                    phi_780_ = true;
                    break;
                }
                default: {
                    phi_780_ = bool();
                    break;
                }
            }
            let _e259 = phi_780_;
            continue;
            continuing {
                phi_742_ = _e246;
                break if !(_e259);
            }
        }
        let _e261 = local_2;
        phi_784_ = type_28(vec3<f32>(bitcast<f32>(_e108), bitcast<f32>(_e113), bitcast<f32>(_e118)), vec4<f32>(bitcast<f32>(_e124), bitcast<f32>(_e129), bitcast<f32>(_e134), bitcast<f32>(_e139)), vec3<f32>(bitcast<f32>(_e167), bitcast<f32>(_e172), bitcast<f32>(_e177)), vec4<f32>(bitcast<f32>(_e183), bitcast<f32>(_e188), bitcast<f32>(_e193), bitcast<f32>(_e198)), _e230, _e261, vec2<f32>(bitcast<f32>(_e145), bitcast<f32>(_e150)), vec2<f32>(bitcast<f32>(_e156), bitcast<f32>(_e161)));
    } else {
        phi_784_ = type_28(vec3<f32>(0f, 0f, 0f), vec4<f32>(1f, 1f, 1f, 1f), vec3<f32>(0f, 0f, 1f), vec4<f32>(0f, 1f, 0f, 0f), array<u32, 4>(0u, 0u, 0u, 0u), array<f32, 4>(0f, 0f, 0f, 0f), vec2<f32>(0f, 0f), vec2<f32>(0f, 0f));
    }
    let _e264 = phi_784_;
    global_3 = _e264.member_1;
    if (_e79 >= 10u) {
        phi_1443_ = (_e95 <= (_e79 - 10u));
    } else {
        phi_1443_ = false;
    }
    let _e270 = phi_1443_;
    if _e270 {
        let _e273 = global_1.member[_e95];
        let _e278 = global_1.member[(_e95 + 1u)];
        let _e283 = global_1.member[(_e95 + 2u)];
        let _e289 = global_1.member[(_e95 + 3u)];
        let _e294 = global_1.member[(_e95 + 4u)];
        let _e299 = global_1.member[(_e95 + 5u)];
        let _e304 = global_1.member[(_e95 + 6u)];
        let _e310 = global_1.member[(_e95 + 7u)];
        let _e315 = global_1.member[(_e95 + 8u)];
        let _e320 = global_1.member[(_e95 + 9u)];
        phi_835_ = type_25(vec3<f32>(bitcast<f32>(_e273), bitcast<f32>(_e278), bitcast<f32>(_e283)), vec4<f32>(bitcast<f32>(_e289), bitcast<f32>(_e294), bitcast<f32>(_e299), bitcast<f32>(_e304)), vec3<f32>(bitcast<f32>(_e310), bitcast<f32>(_e315), bitcast<f32>(_e320)));
    } else {
        phi_835_ = type_25(vec3<f32>(0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 1f), vec3<f32>(1f, 1f, 1f));
    }
    let _e325 = phi_835_;
    let _e333 = (_e325.member_1.x + _e325.member_1.x);
    let _e334 = (_e325.member_1.y + _e325.member_1.y);
    let _e335 = (_e325.member_1.z + _e325.member_1.z);
    let _e337 = (_e325.member_1.z * _e335);
    let _e338 = (_e325.member_1.w * _e333);
    let _e339 = (_e325.member_1.w * _e334);
    let _e340 = (_e325.member_1.w * _e335);
    let _e360 = (vec4<f32>((1f - fma(_e325.member_1.y, _e334, _e337)), fma(_e325.member_1.x, _e334, _e340), fma(_e325.member_1.x, _e335, -(_e339)), 0f) * _e325.member_2.x);
    let _e362 = (vec4<f32>(fma(_e325.member_1.x, _e334, -(_e340)), (1f - fma(_e325.member_1.x, _e333, _e337)), fma(_e325.member_1.y, _e335, _e338), 0f) * _e325.member_2.y);
    let _e364 = (vec4<f32>(fma(_e325.member_1.x, _e335, _e339), fma(_e325.member_1.y, _e335, -(_e338)), (1f - fma(_e325.member_1.x, _e333, (_e325.member_1.y * _e334))), 0f) * _e325.member_2.z);
    if (_e79 >= 83u) {
        phi_1513_ = (_e91 <= (_e79 - 83u));
    } else {
        phi_1513_ = false;
    }
    let _e372 = phi_1513_;
    if _e372 {
        let _e375 = global_1.member[_e91];
        let _e380 = global_1.member[(_e91 + 1u)];
        let _e385 = global_1.member[(_e91 + 2u)];
        let _e390 = global_1.member[(_e91 + 3u)];
        let _e396 = global_1.member[(_e91 + 4u)];
        let _e401 = global_1.member[(_e91 + 5u)];
        let _e406 = global_1.member[(_e91 + 6u)];
        let _e411 = global_1.member[(_e91 + 7u)];
        let _e417 = global_1.member[(_e91 + 8u)];
        let _e422 = global_1.member[(_e91 + 9u)];
        let _e427 = global_1.member[(_e91 + 10u)];
        let _e432 = global_1.member[(_e91 + 11u)];
        let _e438 = global_1.member[(_e91 + 12u)];
        let _e443 = global_1.member[(_e91 + 13u)];
        let _e448 = global_1.member[(_e91 + 14u)];
        let _e453 = global_1.member[(_e91 + 15u)];
        let _e460 = global_1.member[(_e91 + 16u)];
        let _e465 = global_1.member[(_e91 + 17u)];
        let _e470 = global_1.member[(_e91 + 18u)];
        let _e475 = global_1.member[(_e91 + 19u)];
        let _e481 = global_1.member[(_e91 + 20u)];
        let _e486 = global_1.member[(_e91 + 21u)];
        let _e491 = global_1.member[(_e91 + 22u)];
        let _e496 = global_1.member[(_e91 + 23u)];
        let _e502 = global_1.member[(_e91 + 24u)];
        let _e507 = global_1.member[(_e91 + 25u)];
        let _e512 = global_1.member[(_e91 + 26u)];
        let _e517 = global_1.member[(_e91 + 27u)];
        let _e523 = global_1.member[(_e91 + 28u)];
        let _e528 = global_1.member[(_e91 + 29u)];
        let _e533 = global_1.member[(_e91 + 30u)];
        let _e538 = global_1.member[(_e91 + 31u)];
        let _e545 = global_1.member[(_e91 + 32u)];
        let _e550 = global_1.member[(_e91 + 33u)];
        let _e555 = global_1.member[(_e91 + 34u)];
        local_1 = array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f));
        phi_996_ = type_22(0u, 6u);
        loop {
            let _e560 = phi_996_;
            if (_e560.member < _e560.member_1) {
                phi_997_ = type_22((_e560.member + 1u), _e560.member_1);
                phi_1020_ = type_22(1u, _e560.member);
            } else {
                phi_997_ = _e560;
                phi_1020_ = type_22(0u, type_22().member_1);
            }
            let _e573 = phi_997_;
            let _e575 = phi_1020_;
            switch bitcast<i32>(_e575.member) {
                case 0: {
                    phi_1047_ = false;
                    break;
                }
                case 1: {
                    let _e580 = ((_e91 + 35u) + (_e575.member_1 * 4u));
                    let _e583 = global_1.member[_e580];
                    let _e588 = global_1.member[(_e580 + 1u)];
                    let _e593 = global_1.member[(_e580 + 2u)];
                    let _e598 = global_1.member[(_e580 + 3u)];
                    local_1[_e575.member_1] = vec4<f32>(bitcast<f32>(_e583), bitcast<f32>(_e588), bitcast<f32>(_e593), bitcast<f32>(_e598));
                    phi_1047_ = true;
                    break;
                }
                default: {
                    phi_1047_ = bool();
                    break;
                }
            }
            let _e603 = phi_1047_;
            continue;
            continuing {
                phi_996_ = _e573;
                break if !(_e603);
            }
        }
        let _e605 = local_1;
        local = array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f));
        phi_1053_ = type_22(0u, 8u);
        loop {
            let _e608 = phi_1053_;
            if (_e608.member < _e608.member_1) {
                phi_1054_ = type_22((_e608.member + 1u), _e608.member_1);
                phi_1077_ = type_22(1u, _e608.member);
            } else {
                phi_1054_ = _e608;
                phi_1077_ = type_22(0u, type_22().member_1);
            }
            let _e621 = phi_1054_;
            let _e623 = phi_1077_;
            switch bitcast<i32>(_e623.member) {
                case 0: {
                    phi_1100_ = false;
                    break;
                }
                case 1: {
                    let _e628 = ((_e91 + 59u) + (_e623.member_1 * 3u));
                    let _e631 = global_1.member[_e628];
                    let _e636 = global_1.member[(_e628 + 1u)];
                    let _e641 = global_1.member[(_e628 + 2u)];
                    local[_e623.member_1] = vec3<f32>(bitcast<f32>(_e631), bitcast<f32>(_e636), bitcast<f32>(_e641));
                    phi_1100_ = true;
                    break;
                }
                default: {
                    phi_1100_ = bool();
                    break;
                }
            }
            let _e646 = phi_1100_;
            continue;
            continuing {
                phi_1053_ = _e621;
                break if !(_e646);
            }
        }
        let _e648 = local;
        phi_1108_ = type_20(type_18(vec4<f32>(bitcast<f32>(_e375), bitcast<f32>(_e380), bitcast<f32>(_e385), bitcast<f32>(_e390)), vec4<f32>(bitcast<f32>(_e396), bitcast<f32>(_e401), bitcast<f32>(_e406), bitcast<f32>(_e411)), vec4<f32>(bitcast<f32>(_e417), bitcast<f32>(_e422), bitcast<f32>(_e427), bitcast<f32>(_e432)), vec4<f32>(bitcast<f32>(_e438), bitcast<f32>(_e443), bitcast<f32>(_e448), bitcast<f32>(_e453))), type_18(vec4<f32>(bitcast<f32>(_e460), bitcast<f32>(_e465), bitcast<f32>(_e470), bitcast<f32>(_e475)), vec4<f32>(bitcast<f32>(_e481), bitcast<f32>(_e486), bitcast<f32>(_e491), bitcast<f32>(_e496)), vec4<f32>(bitcast<f32>(_e502), bitcast<f32>(_e507), bitcast<f32>(_e512), bitcast<f32>(_e517)), vec4<f32>(bitcast<f32>(_e523), bitcast<f32>(_e528), bitcast<f32>(_e533), bitcast<f32>(_e538))), type_19(_e648, _e605), vec3<f32>(bitcast<f32>(_e545), bitcast<f32>(_e550), bitcast<f32>(_e555)));
    } else {
        phi_1108_ = type_20(type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_18(vec4<f32>(1f, 0f, 0f, 0f), vec4<f32>(0f, 1f, 0f, 0f), vec4<f32>(0f, 0f, 1f, 0f), vec4<f32>(0f, 0f, 0f, 1f)), type_19(array<vec3<f32>, 8>(vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f), vec3<f32>(0f, 0f, 0f)), array<vec4<f32>, 6>(vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f), vec4<f32>(0f, 0f, 0f, 0f))), vec3<f32>(0f, 0f, 0f));
    }
    let _e652 = phi_1108_;
    let _e692 = fma(_e652.member.member_3.x, _e652.member_1.member.w, fma(_e652.member.member_2.x, _e652.member_1.member.z, fma(_e652.member.member.x, _e652.member_1.member.x, (_e652.member.member_1.x * _e652.member_1.member.y))));
    let _e693 = fma(_e652.member.member_3.y, _e652.member_1.member.w, fma(_e652.member.member_2.y, _e652.member_1.member.z, fma(_e652.member.member.y, _e652.member_1.member.x, (_e652.member.member_1.y * _e652.member_1.member.y))));
    let _e694 = fma(_e652.member.member_3.z, _e652.member_1.member.w, fma(_e652.member.member_2.z, _e652.member_1.member.z, fma(_e652.member.member.z, _e652.member_1.member.x, (_e652.member.member_1.z * _e652.member_1.member.y))));
    let _e695 = fma(_e652.member.member_3.w, _e652.member_1.member.w, fma(_e652.member.member_2.w, _e652.member_1.member.z, fma(_e652.member.member.w, _e652.member_1.member.x, (_e652.member.member_1.w * _e652.member_1.member.y))));
    let _e713 = fma(_e652.member.member_3.x, _e652.member_1.member_1.w, fma(_e652.member.member_2.x, _e652.member_1.member_1.z, fma(_e652.member.member.x, _e652.member_1.member_1.x, (_e652.member.member_1.x * _e652.member_1.member_1.y))));
    let _e714 = fma(_e652.member.member_3.y, _e652.member_1.member_1.w, fma(_e652.member.member_2.y, _e652.member_1.member_1.z, fma(_e652.member.member.y, _e652.member_1.member_1.x, (_e652.member.member_1.y * _e652.member_1.member_1.y))));
    let _e715 = fma(_e652.member.member_3.z, _e652.member_1.member_1.w, fma(_e652.member.member_2.z, _e652.member_1.member_1.z, fma(_e652.member.member.z, _e652.member_1.member_1.x, (_e652.member.member_1.z * _e652.member_1.member_1.y))));
    let _e716 = fma(_e652.member.member_3.w, _e652.member_1.member_1.w, fma(_e652.member.member_2.w, _e652.member_1.member_1.z, fma(_e652.member.member.w, _e652.member_1.member_1.x, (_e652.member.member_1.w * _e652.member_1.member_1.y))));
    let _e734 = fma(_e652.member.member_3.x, _e652.member_1.member_2.w, fma(_e652.member.member_2.x, _e652.member_1.member_2.z, fma(_e652.member.member.x, _e652.member_1.member_2.x, (_e652.member.member_1.x * _e652.member_1.member_2.y))));
    let _e735 = fma(_e652.member.member_3.y, _e652.member_1.member_2.w, fma(_e652.member.member_2.y, _e652.member_1.member_2.z, fma(_e652.member.member.y, _e652.member_1.member_2.x, (_e652.member.member_1.y * _e652.member_1.member_2.y))));
    let _e736 = fma(_e652.member.member_3.z, _e652.member_1.member_2.w, fma(_e652.member.member_2.z, _e652.member_1.member_2.z, fma(_e652.member.member.z, _e652.member_1.member_2.x, (_e652.member.member_1.z * _e652.member_1.member_2.y))));
    let _e737 = fma(_e652.member.member_3.w, _e652.member_1.member_2.w, fma(_e652.member.member_2.w, _e652.member_1.member_2.z, fma(_e652.member.member.w, _e652.member_1.member_2.x, (_e652.member.member_1.w * _e652.member_1.member_2.y))));
    let _e755 = fma(_e652.member.member_3.x, _e652.member_1.member_3.w, fma(_e652.member.member_2.x, _e652.member_1.member_3.z, fma(_e652.member.member.x, _e652.member_1.member_3.x, (_e652.member.member_1.x * _e652.member_1.member_3.y))));
    let _e756 = fma(_e652.member.member_3.y, _e652.member_1.member_3.w, fma(_e652.member.member_2.y, _e652.member_1.member_3.z, fma(_e652.member.member.y, _e652.member_1.member_3.x, (_e652.member.member_1.y * _e652.member_1.member_3.y))));
    let _e757 = fma(_e652.member.member_3.z, _e652.member_1.member_3.w, fma(_e652.member.member_2.z, _e652.member_1.member_3.z, fma(_e652.member.member.z, _e652.member_1.member_3.x, (_e652.member.member_1.z * _e652.member_1.member_3.y))));
    let _e758 = fma(_e652.member.member_3.w, _e652.member_1.member_3.w, fma(_e652.member.member_2.w, _e652.member_1.member_3.z, fma(_e652.member.member.w, _e652.member_1.member_3.x, (_e652.member.member_1.w * _e652.member_1.member_3.y))));
    global_2 = vec4<f32>((fma(fma(_e755, _e364.w, fma(_e734, _e364.z, fma(_e692, _e364.x, (_e713 * _e364.y)))), _e264.member.z, fma(fma(_e755, _e360.w, fma(_e734, _e360.z, fma(_e692, _e360.x, (_e713 * _e360.y)))), _e264.member.x, (fma(_e755, _e362.w, fma(_e734, _e362.z, fma(_e692, _e362.x, (_e713 * _e362.y)))) * _e264.member.y))) + (fma(_e734, _e325.member.z, fma(_e692, _e325.member.x, (_e713 * _e325.member.y))) + _e755)), (fma(fma(_e756, _e364.w, fma(_e735, _e364.z, fma(_e693, _e364.x, (_e714 * _e364.y)))), _e264.member.z, fma(fma(_e756, _e360.w, fma(_e735, _e360.z, fma(_e693, _e360.x, (_e714 * _e360.y)))), _e264.member.x, (fma(_e756, _e362.w, fma(_e735, _e362.z, fma(_e693, _e362.x, (_e714 * _e362.y)))) * _e264.member.y))) + (fma(_e735, _e325.member.z, fma(_e693, _e325.member.x, (_e714 * _e325.member.y))) + _e756)), (fma(fma(_e757, _e364.w, fma(_e736, _e364.z, fma(_e694, _e364.x, (_e715 * _e364.y)))), _e264.member.z, fma(fma(_e757, _e360.w, fma(_e736, _e360.z, fma(_e694, _e360.x, (_e715 * _e360.y)))), _e264.member.x, (fma(_e757, _e362.w, fma(_e736, _e362.z, fma(_e694, _e362.x, (_e715 * _e362.y)))) * _e264.member.y))) + (fma(_e736, _e325.member.z, fma(_e694, _e325.member.x, (_e715 * _e325.member.y))) + _e757)), (fma(fma(_e758, _e364.w, fma(_e737, _e364.z, fma(_e695, _e364.x, (_e716 * _e364.y)))), _e264.member.z, fma(fma(_e758, _e360.w, fma(_e737, _e360.z, fma(_e695, _e360.x, (_e716 * _e360.y)))), _e264.member.x, (fma(_e758, _e362.w, fma(_e737, _e362.z, fma(_e695, _e362.x, (_e716 * _e362.y)))) * _e264.member.y))) + (fma(_e737, _e325.member.z, fma(_e695, _e325.member.x, (_e716 * _e325.member.y))) + _e758)));
    return;
}

@vertex 
fn tutorialtutorial_slabbed_renderlet(@builtin(instance_index) param: u32, @builtin(vertex_index) param_1: u32) -> VertexOutput {
    global_4 = param;
    global = param_1;
    function();
    let _e7 = global_2.y;
    global_2.y = -(_e7);
    let _e9 = global_3;
    let _e10 = global_2;
    return VertexOutput(_e9, _e10);
}
