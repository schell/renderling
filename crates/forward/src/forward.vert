#version 450

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aUv;
layout(location = 2) in vec3 aNorm;

layout(location = 3) in vec4 aModelMatrix0;
layout(location = 4) in vec4 aModelMatrix1;
layout(location = 5) in vec4 aModelMatrix2;
layout(location = 6) in vec4 aModelMatrix3;
layout(location = 7) in vec3 aNormMatrix1;
layout(location = 8) in vec3 aNormMatrix2;
layout(location = 9) in vec3 aNormMatrix3;

layout(location = 0) out vec3 vPos;
layout(location = 1) out vec2 vUv;
layout(location = 2) out vec3 vNorm;

layout(set = 0, binding = 0) uniform Camera {
  mat4 uProjection;
  mat4 uView;
};

void main()
{
  mat4 model = mat4(aModelMatrix0, aModelMatrix1, aModelMatrix2, aModelMatrix3);
  mat3 norm = mat3(aNormMatrix1, aNormMatrix2, aNormMatrix3);
  mat3 view = mat3(uView);
  vec4 viewPosition = uView * model * vec4(aPos, 1.0);

  vPos = vec3(viewPosition);
  vUv = aUv;
  vNorm = view * norm * aNorm;

  gl_Position = uProjection * viewPosition;
}
