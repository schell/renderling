@group(0)
@binding(1)
var environment_texture: texture_cube<f32>;

@group(0)
@binding(2)
var environment_sampler: sampler;

struct Input {
    @builtin(position) position: vec4<f32>,
    @location(0) local_pos: vec3<f32>,
};

@fragment
fn fragment_convolve_diffuse_irradiance(input: Input) -> @location(0) vec4<f32> {
    let pi = 3.1415927;
    let frac_pi_2 = pi / 2.0;

    let normal = normalize(input.local_pos);
    var irradiance = vec3f(0.0, 0.0, 0.0);
    let right = normalize(cross(vec3f(0.0, 1.0, 0.0), normal));
    let up = normalize(cross(normal, right));

    let sample_delta = 0.025;
    var nr_samples = 0.0;
    var phi = 0.0;
    for (var phi = 0.0; phi < 2.0 * pi; phi += sample_delta) {
        for (var theta = 0.0; theta < frac_pi_2; theta += sample_delta) {
            // spherical to cartisian tangent coords
            let tangent_sample = vec3f(
                sin(theta) * cos(phi),
                sin(theta) * sin(phi),
                cos(theta),
            );
            // tangent to world coords
            let sample_vec =
                normalize(tangent_sample.x * right + tangent_sample.y * up + tangent_sample.z * normal);
            let sample = textureSample(environment_texture, environment_sampler, sample_vec)
                * cos(theta)
                * sin(theta);
            irradiance += sample.xyz;
            nr_samples += 1.0;
        }
    }
    let color = irradiance * (pi / nr_samples);
    return vec4<f32>(color.xyz, 1.0);
}
