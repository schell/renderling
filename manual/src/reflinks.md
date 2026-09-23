# docs

[`Camera`]: {{DOCS_URL}}/renderling/camera/struct.Camera.html
[`Camera::with_default_perspective`]: {{DOCS_URL}}/renderling/camera/struct.Camera.html#method.with_default_perspective

[`Context`]: {{DOCS_URL}}/renderling/context/struct.Context.html
[`Context::new_stage`]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.new_stage
[`Context::headless`]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.headless
[`Context::try_headless`]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.try_headless
[`Context::get_next_frame`]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.get_next_frame

[`Frame`]: {{DOCS_URL}}/renderling/context/struct.Frame.html
[`Frame::present`]: {{DOCS_URL}}/renderling/context/struct.Frame.html#method.present

[`Primitive`]: {{DOCS_URL}}/renderling/primitive/struct.Primitive.html

[`Material`]: {{DOCS_URL}}/renderling/material/struct.Material.html
[`Material::set_albedo_texture`]: {{DOCS_URL}}/renderling/material/struct.Material.html#method.set_albedo_texture
[`Material::with_normal_texture`]: {{DOCS_URL}}/renderling/material/struct.Material.html#method.with_normal_texture

[`AtlasImage`]: {{DOCS_URL}}/renderling/atlas/struct.AtlasImage.html
[`AtlasTexture`]: {{DOCS_URL}}/renderling/atlas/struct.AtlasTexture.html
[`Stage::set_images`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_images

[`Stage::tonemapping`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.tonemapping
[`Tonemapping`]: {{DOCS_URL}}/renderling/tonemapping/struct.Tonemapping.html
[`TonemapConstants`]: {{DOCS_URL}}/renderling/tonemapping/struct.TonemapConstants.html
[`Tonemap`]: {{DOCS_URL}}/renderling/tonemapping/struct.Tonemap.html

[`DebugChannel`]: {{DOCS_URL}}/renderling/pbr/debug/enum.DebugChannel.html
[`Stage::set_debug_mode`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_debug_mode
[`Stage::set_use_debug_overlay`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_use_debug_overlay

[`NestedTransform`]: {{DOCS_URL}}/renderling/transform/struct.NestedTransform.html
[`NestedTransform::add_child`]: {{DOCS_URL}}/renderling/transform/struct.NestedTransform.html#method.add_child
[`Stage::new_nested_transform`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_nested_transform

[`Animator`]: {{DOCS_URL}}/renderling/gltf/anime/struct.Animator.html
[`Stage::new_morph_targets`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_morph_targets
[`Stage::new_morph_target_weights`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_morph_target_weights
[`Primitive::set_morph_targets`]: {{DOCS_URL}}/renderling/primitive/struct.Primitive.html#method.set_morph_targets

[`Stage::set_use_frustum_culling`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_use_frustum_culling
[`Stage::set_use_occlusion_culling`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_use_occlusion_culling
[`Stage::new_light_tiling`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_light_tiling
[`LightTilingConfig`]: {{DOCS_URL}}/renderling/light/struct.LightTilingConfig.html
[`Stage::set_msaa_sample_count`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_msaa_sample_count

[`Stage::set_ambient_color`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.set_ambient_color
[`Stage::with_ambient_color`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.with_ambient_color
[`Stage::ambient_color`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.ambient_color

[`Mat4`]: https://docs.rs/glam/latest/glam/f32/struct.Mat4.html

[`RenderTarget`]: {{DOCS_URL}}/renderling/context/struct.RenderTarget.html

[`Skybox`]: {{DOCS_URL}}/renderling/skybox/struct.Skybox.html

[`ShadowMap`]: {{DOCS_URL}}/renderling/light/struct.ShadowMap.html
[`ShadowMap::update`]: {{DOCS_URL}}/renderling/light/struct.ShadowMap.html#method.update
[`ShadowMap::descriptor_lock`]: {{DOCS_URL}}/renderling/light/struct.ShadowMap.html#method.descriptor_lock
[`Stage::new_shadow_map`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_shadow_map
[`Context::with_shadow_mapping_atlas_texture_size`]: {{DOCS_URL}}/renderling/context/struct.Context.html#method.with_shadow_mapping_atlas_texture_size

[`Stage`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html
[`Stage::new_camera`]: {{DOCS_URL}}/renderling/stage/struct.Stage.html#method.new_camera

[`Vertices`]: {{DOCS_URL}}/renderling/geometry/struct.Vertices.html
[`Vertices::get_vertex`]: {{DOCS_URL}}/renderling/geometry/struct.Vertices.html#method.get_vertex
[`Vertices::modify_vertex`]: {{DOCS_URL}}/renderling/geometry/struct.Vertices.html#method.modify_vertex
[`Vertices::set_vertex`]: {{DOCS_URL}}/renderling/geometry/struct.Vertices.html#method.set_vertex

[`Vertex`]: {{DOCS_URL}}/renderling/geometry/struct.Vertex.html

# friends

[glam]: https://crates.io/crates/glam
[image]: https://crates.io/crates/image
[wgpu]: https://crates.io/crates/wgpu
[winit]: https://crates.io/crates/winit
[web-sys]: https://crates.io/crates/web-sys

# other

[builder-pattern]: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html
