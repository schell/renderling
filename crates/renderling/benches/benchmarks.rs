use criterion::{black_box, criterion_group, criterion_main, Criterion};
use renderling::Context;
#[cfg(feature = "gltf")]
use renderling::GltfLoader;

#[cfg(feature = "gltf")]
fn load_damaged_helmet(c: &mut Criterion) {
    //let _ = env_logger::builder()
    //    .is_test(true)
    //    .filter_level(log::LevelFilter::Trace)
    //    .filter_module("renderling", log::LevelFilter::Trace)
    //    .filter_module("dagga", log::LevelFilter::Warn)
    //    .filter_module("broomdog", log::LevelFilter::Warn)
    //    .filter_module("naga", log::LevelFilter::Warn)
    //    .filter_module("wgpu", log::LevelFilter::Warn)
    //    .filter_module("wgpu_hal", log::LevelFilter::Warn)
    //    .try_init();

    let (document, buffers, images) = gltf::import("../../gltf/DamagedHelmet.glb").unwrap();

    let mut group = c.benchmark_group("load_damaged_helmet");
    group.sample_size(20);

    println!("{}", std::env::current_dir().unwrap().display());

    let r = Context::headless(100, 100);
    group.bench_function("legacy", |b| {
        b.iter(|| {
            let mut builder = r.new_scene();
            let loader = GltfLoader::load(
                &mut builder,
                document.clone(),
                buffers.clone(),
                images.clone(),
            );
            let scene = builder.build().unwrap();
            black_box((loader, scene))
        })
    });

    let r = Context::headless(100, 100);
    group.bench_function("gltf", |b| {
        b.iter(|| {
            let stage = r.new_stage();
            let gpu_doc = stage
                .load_gltf_document(&document, buffers.clone(), images.clone())
                .unwrap();
            black_box(gpu_doc)
        })
    });
}

fn noop(_c: &mut Criterion) {}

#[cfg(feature = "gltf")]
criterion_group!(benches, load_damaged_helmet);

#[cfg(not(feature = "gltf"))]
criterion_group!(benches, noop);

criterion_main!(benches);
