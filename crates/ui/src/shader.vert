#version 450

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec4 aColor;
layout(location = 2) in vec2 aUv;

layout(location = 3) in vec4 aModelMatrix0;
layout(location = 4) in vec4 aModelMatrix1;
layout(location = 5) in vec4 aModelMatrix2;
layout(location = 6) in vec4 aModelMatrix3;

layout(location = 0) out vec4 vColor;
layout(location = 1) out vec2 vUv;

layout(set = 0, binding = 0) uniform Camera {
  mat4 uProjection;
  mat4 uView;
};

void main()
{
  mat4 model = mat4(aModelMatrix0, aModelMatrix1, aModelMatrix2, aModelMatrix3);
  vColor = aColor;
  vUv = aUv;
  gl_Position = uProjection * uView * model * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
