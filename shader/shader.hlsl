// shader.hlsl

cbuffer cb: register(b2) {
  float4 ext = float4(0.5f, 1.0f, 0.5f, 1.0f); // RGBA
};
// float4 ext: register(c0);
float4x4 world;
float4x4 view;
float4x4 prj;

texture texmap;
sampler smp = sampler_state {
  Texture = <texmap>;
  MinFilter = LINEAR;
  MagFilter = LINEAR;
  MipFilter = NONE;
  AddressU = Clamp;
  AddressV = Clamp;
};

struct VS_INPUT {
  float3 pos: POSITION0; // pos in local
  float4 spos: POSITION1; // sub pos in local
  float3 norm: NORMAL0; // norm in local
  float3 tan: TANGENT0; // tangent in local
  float3 binorm: BINORMAL0; // bi norm in local
  float4 dif: COLOR0; // diffuse
  float4 spc: COLOR1; // specular
  float2 texCoords0: TEXCOORD0; // texture UV
  float2 texCoords1: TEXCOORD1; // sub texture UV
};

struct VS_OUTPUT {
  float4 dif: COLOR0; // through
  float4 spc: COLOR1; // through
  float2 texCoords0: TEXCOORD0; // through
  float3 pos: POSITION1; // through
  float4 ppos: SV_POSITION; // pos in projection
};

struct PS_OUTPUT {
  float4 c: SV_TARGET0;
};

VS_OUTPUT VS(VS_INPUT vsi)
{
  float4x4 wvp = mul(mul(world, view), prj);
  VS_OUTPUT vso;
  vso.dif = vsi.dif;
  vso.spc = vsi.spc;
  vso.texCoords0 = vsi.texCoords0;
  vso.pos = vsi.pos;
  vso.ppos = mul(float4(vsi.pos, 1.0f), wvp);
  return vso;
}

PS_OUTPUT PS(VS_OUTPUT psi)
{
  float4 texdif = tex2D(smp, psi.texCoords0);
  PS_OUTPUT pso;
//  pso.c = ext; // test
//  pso.c = (2.0f + psi.spc) * psi.dif; // test
//  pso.c = psi.spc + psi.dif; // without texture
//  pso.c = texdif; // only texture
  pso.c = psi.spc + psi.dif * texdif; // whole
  return pso;
}

technique Render
{
  pass P0
  {
    vertexShader = compile vs_3_0 VS();
    pixelShader = compile ps_3_0 PS();
  }
}
