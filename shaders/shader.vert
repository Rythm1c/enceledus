#version 460

// Attribute layouts - only enabled attributes will have data
layout(location = 0) in vec3 pos;

#ifdef HAS_NORMALS
layout(location = 1) in vec3 norm;
#endif

#ifdef HAS_TEXCOORDS
layout(location = 2) in vec2 tc;
#endif

#ifdef HAS_SKINNING
layout(location = 3) in vec4 weights;
layout(location = 4) in ivec4 boneIds;
#endif

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 lightSpace;

out vs_Out {
#ifdef HAS_NORMALS
    vec3 normal;
#endif
    vec3 fragPos;
#ifdef HAS_TEXCOORDS
    vec2 texCoords;
#endif
    vec4 lightSpacePos;
} vs_out;

#ifdef HAS_SKINNING
const int MAX_BONES = 300;
const int MAX_BONE_INFLUENCE = 4;
uniform mat4 boneMats[MAX_BONES];
#endif

void main() {
    mat4 final_mat = transform;

#ifdef HAS_SKINNING
    mat4 skin = mat4(0.0);
    for(int i = 0; i < MAX_BONE_INFLUENCE; i++) {
        skin += boneMats[boneIds[i]] * weights[i];
    }
    final_mat = transform * skin;
#endif

    gl_Position = projection * view * final_mat * vec4(pos, 1.0);

#ifdef HAS_NORMALS
    vs_out.normal = mat3(transpose(inverse(final_mat))) * norm;
#endif

#ifdef HAS_TEXCOORDS
    vs_out.texCoords = tc;
#endif

    vs_out.fragPos = vec3(final_mat * vec4(pos, 1.0));
    vs_out.lightSpacePos = lightSpace * final_mat * vec4(pos, 1.0);
}

