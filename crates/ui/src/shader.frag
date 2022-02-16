#version 450
layout(location = 0) in vec4 vColor;
layout(location = 1) in vec2 vUv;

const uint ColorOnly = 0u;
const uint TextureOnly = 1u;
const uint ReplaceUvRedWithColor = 2u;

layout(set = 1, binding = 0) uniform BlendStyle {
  uint uBlendStyle;
};
layout(set = 1, binding = 1) uniform texture2D uTexture;
layout(set = 1, binding = 2) uniform sampler uSampler;

layout(location = 0) out vec4 FragColor;

void main()
{
  vec4 uvColor = texture(sampler2D(uTexture, uSampler), vUv);

  switch(uBlendStyle) {
  case ColorOnly:
    FragColor = vColor;
    break;
  case TextureOnly:
    FragColor = uvColor;
    break;
  case ReplaceUvRedWithColor:
    FragColor = vec4(vColor.rgb, vColor.a * uvColor.r);
    break;
  default:
    discard;
  }
}
