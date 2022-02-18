#version 450

layout(location = 0) in vec3 vPos;
layout(location = 1) in vec2 vUv;
layout(location = 2) in vec3 vNorm;

layout(set = 1, binding = 0) uniform texture2D uDiffuseTexture;
layout(set = 1, binding = 1) uniform sampler uDiffuseSampler;
layout(set = 1, binding = 2) uniform texture2D uSpecularTexture;
layout(set = 1, binding = 3) uniform sampler uSpecularSampler;
layout(set = 1, binding = 4) uniform MaterialShininess {
  float uShininess;
};

layout(set = 0, binding = 0) uniform Camera {
  mat4 uProjection;
  mat4 uView;
};

struct PointLight {
  vec3 position;
  uint should_continue;

  vec3 attenuation;
  uint _padding;

  vec4 ambient_color;
  vec4 diffuse_color;
  vec4 specular_color;
};

struct SpotLight {
  vec3 position;
  uint should_continue;

  vec3 direction;
  float inner_cutoff;

  vec3 attenuation;
  float outer_cutoff;

  vec4 ambient_color;
  vec4 diffuse_color;
  vec4 specular_color;
};

struct DirectionalLight {
  vec3 direction;
  uint should_continue;

  vec4 ambient_color;
  vec4 diffuse_color;
  vec4 specular_color;
};

#define MAX_POINT_LIGHTS 64
#define MAX_SPOT_LIGHTS 32
#define MAX_DIRECTIONAL_LIGHTS 8

layout(set = 2, binding = 0) uniform PointLights {
  PointLight uPointLights[MAX_POINT_LIGHTS];
};
layout(set = 2, binding = 1) uniform SpotLights {
  SpotLight uSpotLights[MAX_SPOT_LIGHTS];
};
layout(set = 2, binding = 2) uniform DirectionalLights {
  DirectionalLight uDirectionalLights[MAX_DIRECTIONAL_LIGHTS];
};

layout(location = 0) out vec4 FragColor;

// Calculate attenuation
// attenuation.x: constant
// attenuation.y: linear
// attenuation.z: quadratic
float attenuate(vec3 attenuation, float distance) {
  return (1.0 / (attenuation.x + attenuation.y * distance + attenuation.z * (distance * distance)));
}

// Calculate a point light's color contribution to a fragment.
vec3 color_point(PointLight light, vec3 normal, vec3 cameraToFragDir, vec4 diffuseColor, vec4 specularColor) {
  vec3 lightPos = (uView * vec4(light.position, 1.0)).xyz;
  vec3 lightDir = normalize(lightPos - vPos);
  // diffuse shading
  float diff = max(dot(normal, lightDir), 0.0);
  // specular shading
  vec3 halfwayDir = normalize(lightDir + cameraToFragDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
  // attenuation
  float distance = length(lightPos - vPos);
  float attenuation = attenuate(light.attenuation, distance);
  // combine results
  vec3 ambient  = light.ambient_color.rgb * diffuseColor.rgb;
  vec3 diffuse  = light.diffuse_color.rgb * diff * diffuseColor.rgb;
  vec3 specular = light.specular_color.rgb * spec * specularColor.rgb;
  ambient  *= attenuation;
  diffuse  *= attenuation;
  specular *= attenuation;
  return (ambient + diffuse + specular);
}

// Calculate a spotlight's color contribution to a fragment.
vec3 color_spot(SpotLight light, vec3 normal, vec3 cameraToFragDir, vec4 diffuseColor, vec4 specularColor) {
  vec3 lightPos = (uView * vec4(light.position, 1.0)).xyz;
  vec3 lightDir = normalize(lightPos - vPos);
  // diffuse shading
  float diff = max(dot(normal, lightDir), 0.0);
  // specular shading
  vec3 halfwayDir = normalize(lightDir + cameraToFragDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
  // attenuation
  float distance = length(lightPos - vPos);
  float attenuation = attenuate(light.attenuation, distance);
  // spotlight intensity
  vec3 direction = normalize(-(uView * vec4(light.direction, 0.0)).xyz);
  float theta = dot(lightDir, direction);
  float epsilon = light.inner_cutoff - light.outer_cutoff;
  float intensity = clamp((theta - light.outer_cutoff) / epsilon, 0.0, 1.0);
  // combine results
  vec3 ambient = light.ambient_color.rgb * diffuseColor.rgb;
  vec3 diffuse = light.diffuse_color.rgb * diff * diffuseColor.rgb;
  vec3 specular = light.specular_color.rgb * spec * specularColor.rgb;
  ambient *= attenuation * intensity;
  diffuse *= attenuation * intensity;
  specular *= attenuation * intensity;
  return (ambient + diffuse + specular);
}

// Calculate a directional light's color contribution to a fragment.
vec3 color_directional(DirectionalLight light, vec3 normal, vec3 cameraToFragDir, vec4 diffuseColor, vec4 specularColor) {
  vec3 lightDir = normalize(-(uView * vec4(light.direction, 0.0)).xyz);
  // diffuse shading
  float diff = max(dot(normal, lightDir), 0.0);
  // specular shading
  vec3 halfwayDir = normalize(lightDir + cameraToFragDir);
  float spec = pow(max(dot(normal, halfwayDir), 0.0), uShininess);
  // combine results
  vec3 ambient  = light.ambient_color.rgb * diffuseColor.rgb;
  vec3 diffuse  = light.diffuse_color.rgb * diff * diffuseColor.rgb;
  vec3 specular = light.specular_color.rgb * spec * specularColor.rgb;
  return (ambient + diffuse + specular);
}

void main()
{
  vec3 norm = normalize(vNorm);
  vec3 cameraToFragDir = normalize(-vPos);
  vec4 diffuseColor = texture(sampler2D(uDiffuseTexture, uDiffuseSampler), vUv);
  vec4 specularColor = texture(sampler2D(uSpecularTexture, uSpecularSampler), vUv);

  vec3 color = vec3(0.0);

  for (int i = 0; i < MAX_DIRECTIONAL_LIGHTS; i++) {
    DirectionalLight light = uDirectionalLights[i];
    color += color_directional(light, norm, cameraToFragDir, diffuseColor, specularColor);
    if (light.should_continue == 0) {
      break;
    }
  }

  for (int i = 0; i < MAX_POINT_LIGHTS; i++) {
    PointLight light = uPointLights[i];
    color += color_point(light, norm, cameraToFragDir, diffuseColor, specularColor);
    if (light.should_continue == 0) {
      break;
    }
  }

  for (int i = 0; i < MAX_SPOT_LIGHTS; i++) {
    SpotLight light = uSpotLights[i];
    color += color_spot(light, norm, cameraToFragDir, diffuseColor, specularColor);
    if (light.should_continue == 0) {
      break;
    }
  }

  FragColor = vec4(1.0, 0.0, 0.0, 1.0);//vec4(color, 1.0);
}
