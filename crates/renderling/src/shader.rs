//! WGSL shader helpers.

#[wgsl_rs::wgsl]
pub mod constants {
    //! Common constants.

    use wgsl_rs::std::*;

    pub const TOP_LEFT_2D: Vec2f = vec2f(0.0, 0.0);
    pub const TOP_RIGHT_2D: Vec2f = vec2f(1.0, 0.0);
    pub const BOTTOM_LEFT_2D: Vec2f = vec2f(0.0, 1.0);
    pub const BOTTOM_RIGHT_2D: Vec2f = vec2f(1.0, 1.0);

    pub const UV_COORD_QUAD_CCW: [Vec2f; 6] = [
        BOTTOM_LEFT_2D,
        BOTTOM_RIGHT_2D,
        TOP_RIGHT_2D,
        TOP_RIGHT_2D,
        TOP_LEFT_2D,
        BOTTOM_LEFT_2D,
    ];

    pub const CLIP_SPACE_COORD_QUAD_CCW_TL: Vec4f = vec4f(-1.0, 1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_BL: Vec4f = vec4f(-1.0, -1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_TR: Vec4f = vec4f(1.0, 1.0, 0.5, 1.0);
    pub const CLIP_SPACE_COORD_QUAD_CCW_BR: Vec4f = vec4f(1.0, -1.0, 0.5, 1.0);

    pub const CLIP_SPACE_COORD_QUAD_CCW: [Vec4f; 6] = [
        CLIP_SPACE_COORD_QUAD_CCW_BL,
        CLIP_SPACE_COORD_QUAD_CCW_BR,
        CLIP_SPACE_COORD_QUAD_CCW_TR,
        CLIP_SPACE_COORD_QUAD_CCW_TR,
        CLIP_SPACE_COORD_QUAD_CCW_TL,
        CLIP_SPACE_COORD_QUAD_CCW_BL,
    ];
}
