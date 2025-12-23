#version 460
// Attribute layouts - only enabled attributes will have data
layout(location = 0) in vec3 pos;

layout(location = 1) in vec3 norm;

layout(location = 2) in vec2 uv;

layout(location = 3) in vec4 weights;

layout(location = 4) in vec4 boneIds;

uniform mat4 world;

uniform mat4 view;

uniform mat4 projection;

//uniform mat4 lightSpace;

uniform bool animated;

out vs_Out {
    vec3 normal;
    vec3 fragPos;
    vec2 texCoords;
    //vec4 lightSpacePos;
} vs_out;

const int MAX_BONES = 300;
const int MAX_BONE_INFLUENCE = 4;

uniform mat4 boneMats[MAX_BONES];

void main() {
    mat4 final_mat = world;

    if (animated){

        mat4 skin = mat4(0.0);

        for(int i = 0; i < MAX_BONE_INFLUENCE; i++) {
            skin += boneMats[int(boneIds[i])] * weights[i];
        }

        final_mat = world * skin;
    }

    gl_Position = projection * view * final_mat * vec4(pos, 1.0);

    vs_out.normal = mat3(transpose(inverse(final_mat))) * norm;


    vs_out.texCoords = uv;

    vs_out.fragPos = vec3(final_mat * vec4(pos, 1.0));
    //vs_out.lightSpacePos = lightSpace * final_mat * vec4(pos, 1.0);
}

