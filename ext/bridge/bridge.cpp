/*
  bridge.cpp
*/

#ifndef _UNICODE
#define _UNICODE
#ifdef _UNICODE
#ifndef UNICODE
#define UNICODE
#endif
#endif
#endif

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// #include <stdarg.h>
// #include <varargs.h>

#define _USE_MATH_DEFINES
#include <math.h>

#include <time.h>
#include <sys/utime.h>

// fake for Rust (no mangle) must include ex_c.h before d3dx9.h
#include <ex_c.h>

#include <windows.h>
// #include <d3d9.h>
#include <d3dx9.h>

// fake for Rust (must exist the code at least once to link D3DXMatrixIdentity)
int dummy_use()
{
  D3DXMATRIX m;
  D3DXMatrixIdentity(&m);
  return 0;
}

// (link winmm.lib for timeBeginPeriod / timeEndPeriod) (feature Win32_Media)
#include <timeapi.h> // must be after windows.h

#define SAFE_RELEASE(p) do{ if(p){ (p)->Release(); (p) = NULL; } }while(0)
#define DIR_SHADER L"./shader"
#define DIR_RESOURCE L"C:/prj/test/resource"
#define MAXLIGHT 8
#define MAXFONT 4

// define BRIDGE_FAKE_DISABLE before include bridge.hpp (for Rust bindgen)
#define BRIDGE_FAKE_DISABLE
#include <bridge.hpp>

// Matrix to be transposed both (GL: column major MV) (DirectX: row major VM)
const float ref_mat44_i[][4] = {{1,0,0,0}, {0,1,0,0}, {0,0,1,0}, {0,0,0,1}};
const float ref_mat44_t[][4] = {{1,0,0,7}, {0,1,0,5}, {0,0,1,3}, {0,0,0,1}};
const float ref_mat44_s[][4] = {{7,0,0,0}, {0,5,0,0}, {0,0,3,0}, {0,0,0,1}};
const float ref_mat44_r[][4] = {{1,0,0,0}, {0,0,-1,0}, {0,1,0,0}, {0,0,0,1}};
const float ref_mat44_q[][4] = {{1,0,0,0}, {0,0,1,0}, {0,-1,0,0}, {0,0,0,1}};
const float ref_mat44_d[][4] = {{1,0,0,0}, {0,1,0,0}, {0,0,-1,0}, {0,0,0,1}};
const float ref_mat44_z[][4] = {
  {0.9945219,-0.1045285,0,0}, {0.1045285,0.9945219,0,0}, {0,0,1,0}, {0,0,0,1}};
const float ref_mat44_c[][4] = {
  {          1,          0,          0,          0},
  {          0,  0.6401843,  0.7682213,          0},
  {          0, -0.7682213,  0.6401843,  3.9051247},
  {          0,          0,          0,          1}};
const float ref_mat44_v[][4] = {
  {  0.9945219, -0.1045285,          0,          0},
  {  0.0669175,  0.6366774,  0.7682213,          0},
  { -0.0803010, -0.7640129,  0.6401843,  3.9051247},
  {          0,          0,          0,          1}};
const float ref_mat44_p[][4] = {
  {  0.4330126,          0,          0,          0},
  {          0,  0.5773502,          0,          0},
  {          0,          0,  1.0002000, -0.1000200},
  {          0,          0,          1,          0}};
const float ref_mat_column_major[] = {1,5,9,13,2,6,10,14,3,7,11,15,4,8,12,16};
const float dump_mat_m44[] = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16};
extern const byte *dump_mat_m44_u8s = (const byte *)"\
[[ 3f800000 40000000 40400000 40800000]\n\
 [ 40a00000 40c00000 40e00000 41000000]\n\
 [ 41100000 41200000 41300000 41400000]\n\
 [ 41500000 41600000 41700000 41800000]]";

const wchar_t *LOGFILE = L"./stat.log";

#ifdef VERTEX_2D
// RHW only for 2D
// #define FVF_CVTX (D3DFVF_XYZRHW|D3DFVF_DIFFUSE|D3DFVF_TEX1)
//#define FVF_CVTX (D3DFVF_XYZRHW|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX1)
// #define FVF_CVTX (D3DFVF_XYZRHW|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX0)
const DWORD FVF_CVTX = (
  D3DFVF_XYZRHW|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX1);
#else
// #define FVF_CVTX (D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_TEX1)
//#define FVF_CVTX (D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX1)
// #define FVF_CVTX (D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX0)
const DWORD FVF_CVTX = (
  D3DFVF_XYZ|D3DFVF_DIFFUSE|D3DFVF_SPECULAR|D3DFVF_TEX1);
#endif

// Stream Offset Type Method Usage UsageIndex
const D3DVERTEXELEMENT9 CvtxElem[] = {
  {0, 0, D3DDECLTYPE_FLOAT3, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_POSITION, 0},
  {0, 12, D3DDECLTYPE_D3DCOLOR, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_COLOR, 0},
  {0, 16, D3DDECLTYPE_D3DCOLOR, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_COLOR, 1},
  {0, 20, D3DDECLTYPE_FLOAT2, D3DDECLMETHOD_DEFAULT, D3DDECLUSAGE_TEXCOORD, 0},
  D3DDECL_END()}; // == {0xff, 0, D3DDECLTYPE_UNUSED, 0, 0, 0}

Cxd *createD3D()
{
  Cxd *xd = (Cxd *)malloc(sizeof(Cxd));
  if(xd) ZeroMemory(xd, sizeof(Cxd));
  return xd;
}

void destroyD3D(Cxd **xd)
{
  if(*xd){ free(*xd); *xd = NULL; }
}

void finishD3D(Cxd *xd)
{
  if(!xd) return;
  if(xd->mr.disposer) xd->mr.disposer(xd, xd->mr.owner);
  for(UINT i = 0; i < MAXFONT; ++i) SAFE_RELEASE(xd->font[i]);
  SAFE_RELEASE(xd->lc.shader);
  SAFE_RELEASE(xd->lc.vdecl);
  SAFE_RELEASE(xd->dev);
  SAFE_RELEASE(xd->d3d);
}

HRESULT initD3D(Cxd *xd, HWND wnd)
{
  if(!xd) return E_FAIL;
  xd->wnd = wnd;
  RECT rct;
  GetClientRect(xd->wnd, &rct);
  xd->w = rct.right - rct.left;
  xd->h = rct.bottom - rct.top;
  _timespec_now(&xd->fps.utns);
  xd->fps.tick = (double)(xd->fps.fps = 0.0f);
  xd->fps.frames = 0;
  if(xd->d3d) return S_OK;
  if(!(xd->d3d = Direct3DCreate9(D3D_SDK_VERSION))){
    MessageBox(0, L"fail: create Direct3D9", NULL, MB_OK);
    return E_FAIL;
  }
  ZeroMemory(&xd->d3dpp, sizeof(xd->d3dpp));
  xd->d3dpp.BackBufferWidth = xd->w;
  xd->d3dpp.BackBufferHeight = xd->h;
  xd->d3dpp.BackBufferFormat = D3DFMT_UNKNOWN;
  xd->d3dpp.BackBufferCount = 1;
  xd->d3dpp.MultiSampleType = D3DMULTISAMPLE_NONE;
  xd->d3dpp.MultiSampleQuality = 0;
  xd->d3dpp.SwapEffect = D3DSWAPEFFECT_DISCARD;
  xd->d3dpp.hDeviceWindow = wnd;
  xd->d3dpp.Windowed = TRUE;
  xd->d3dpp.EnableAutoDepthStencil = TRUE;
  xd->d3dpp.AutoDepthStencilFormat = D3DFMT_D24S8; // D3DFMT_D16
  xd->d3dpp.Flags = 0;
  xd->d3dpp.FullScreen_RefreshRateInHz = D3DPRESENT_RATE_DEFAULT;
  xd->d3dpp.PresentationInterval = D3DPRESENT_INTERVAL_DEFAULT; // _IMMEDIATE
#if 1
  if(FAILED(xd->d3d->CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, wnd,
    D3DCREATE_HARDWARE_VERTEXPROCESSING, &xd->d3dpp, &xd->dev))){
    MessageBox(0, L"fail: HW mode Direct3D9 device\nretry HEL", NULL, MB_OK);
    if(FAILED(xd->d3d->CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, wnd,
      D3DCREATE_SOFTWARE_VERTEXPROCESSING, &xd->d3dpp, &xd->dev))){
      MessageBox(0, L"fail: SW mode Direct3D9 device", NULL, MB_OK);
#else
  if(FAILED(xd->d3d->CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_HAL, wnd,
    D3DCREATE_MIXED_VERTEXPROCESSING, &xd->d3dpp, &xd->dev))){
    MessageBox(0, L"fail: HAL mode Direct3D9 device\nretry REF", NULL, MB_OK);
    if(FAILED(xd->d3d->CreateDevice(D3DADAPTER_DEFAULT, D3DDEVTYPE_REF, wnd,
      D3DCREATE_MIXED_VERTEXPROCESSING, &xd->d3dpp, &xd->dev))){
      MessageBox(0, L"fail: REF mode Direct3D9 device", NULL, MB_OK);
#endif
      xd->d3d->Release();
      xd->d3d = NULL;
      return E_FAIL;
    }
  }
  ZeroMemory(&xd->lc, sizeof(xd->lc));
  ZeroMemory(&xd->mr, sizeof(xd->mr));
  ZeroMemory(&xd->font, sizeof(xd->font));
  if(xd->dev){
    D3DVIEWPORT9 vp = {0, 0, (DWORD)xd->w, (DWORD)xd->h, 0.0f, 1.0f};
    if(FAILED(xd->dev->SetViewport(&vp))) return E_FAIL;

    if(FAILED(xd->dev->CreateVertexDeclaration(CvtxElem, &xd->lc.vdecl)))
      return E_FAIL;
    if(!xd->lc.vdecl) return E_FAIL;
    xd->dev->SetVertexDeclaration(xd->lc.vdecl); // for SetStreamSource

    wchar_t p[256];
    static const wchar_t *fn[] = {L"shader.hlsl"};
    _snwprintf_s(p, sizeof(p), _countof(p), L"%s/%s", DIR_SHADER, fn[0]);
    LPD3DXBUFFER err = NULL;
    if(FAILED(D3DXCreateEffectFromFile(xd->dev, p, NULL, NULL,
      D3DXSHADER_DEBUG, NULL, &xd->lc.shader, &err))) return E_FAIL;
    if(!xd->lc.shader) return E_FAIL;
    xd->lc.tech = xd->lc.shader->GetTechniqueByName("Render");
    for(int i = 0; i < _countof(Cpname); ++i)
      xd->lc.params[i] = xd->lc.shader->GetParameterByName(NULL, Cpname[i]);
/*
    static const wchar_t *fn[] = {L"shader_VS.vso", L"shader_PS.pso"};
    for(int n = 0; n < _countof(fn); ++n){
      _snwprintf_s(p, sizeof(p), _countof(p), L"%s/%s", DIR_SHADER, fn[n]);
// SetRenderTarget();
// D3DXAssembleShader();
// D3DXAssembleShaderFromFile();
// D3DXCompileShader();
// CreateVertexShader();
// CreatePixelShader();
      if(!n) xd->dev->SetVertexShader(NULL);
      else xd->dev->SetPixelShader(NULL);
    }
*/

    xd->dev->SetTextureStageState(0, D3DTSS_COLORARG1, D3DTA_TEXTURE);
    xd->dev->SetTextureStageState(0, D3DTSS_COLORARG2, D3DTA_DIFFUSE);
    xd->dev->SetTextureStageState(0, D3DTSS_COLOROP, D3DTOP_MODULATE);
    xd->dev->SetTextureStageState(0, D3DTSS_ALPHAARG1, D3DTA_TEXTURE);
    xd->dev->SetTextureStageState(0, D3DTSS_ALPHAARG2, D3DTA_DIFFUSE);
    xd->dev->SetTextureStageState(0, D3DTSS_ALPHAOP, D3DTOP_MODULATE);

    xd->dev->SetRenderState(D3DRS_ALPHABLENDENABLE, TRUE);
    xd->dev->SetRenderState(D3DRS_SRCBLEND, D3DBLEND_SRCALPHA); // or ZERO
    xd->dev->SetRenderState(D3DRS_DESTBLEND, D3DBLEND_INVSRCALPHA); // or ONE

    xd->dev->SetRenderState(D3DRS_CULLMODE, D3DCULL_CCW); // _CW _NONE
    xd->dev->SetRenderState(D3DRS_ZENABLE, TRUE);
//    xd->dev->SetRenderState(D3DRS_LIGHTING, TRUE);
    xd->dev->SetRenderState(D3DRS_LIGHTING, FALSE);
    xd->dev->SetRenderState(D3DRS_SPECULARENABLE, TRUE);
    xd->dev->SetRenderState(D3DRS_AMBIENT, 0x00111111);
  }
  return S_OK;
}

size_t manage_resource_n_pvec(Cxd *xd)
{
  if(!xd) return 0;
  return ((size_t)&xd->mr.disposer - (size_t)&xd->mr) / sizeof(void *);
}

void ***manage_resource_ptr_mut(Cxd *xd)
{
  return (void ***)&xd->mr;
}

HRESULT manage_resource_set_disposer(Cxd *xd,
  void *o, HRESULT (*d)(Cxd *xd, void *o), const Csa *sa)
{
  if(!xd) return E_FAIL;
  if(xd->mr.disposer) xd->mr.disposer(xd, xd->mr.owner);
  xd->mr.disposer = d;
  xd->mr.owner = o;
  xd->mr.sa = sa;
  return S_OK;
}

HRESULT disposeManageResourceElements(Cxd *xd)
{
  if(!xd) return E_FAIL;
  if(!xd->mr.sa) return E_FAIL;
  if(!xd->mr.sa->a) return E_FAIL;
  if(xd->mr.tex){
    for(size_t i = 0; i < xd->mr.sa->a[1]; ++i) SAFE_RELEASE(xd->mr.tex[i]);
  }
  if(xd->mr.vbuf){
    for(size_t i = 0; i < xd->mr.sa->a[6]; ++i) SAFE_RELEASE(xd->mr.vbuf[i]);
  }
  return S_OK;
}

HRESULT disposeManageResource(Cxd *xd, void *o)
{
  if(!xd) return E_FAIL;
  if(FAILED(disposeManageResourceElements(xd))) return E_FAIL;
  if(xd->mr.tex) free(xd->mr.tex); xd->mr.tex = NULL;
  if(xd->mr.vbuf) free(xd->mr.vbuf); xd->mr.vbuf = NULL;
//  xd->mr.sa->a[1] = xd->mr.sa->a[6] = 0;
  return S_OK;
}

HRESULT initManageResource(Cxd *xd,
  void *o, HRESULT (*d)(Cxd *xd, void *o), const Csa *sa)
{
  if(!xd) return E_FAIL;
  if(!sa) return E_FAIL;
  if(!sa->a) return E_FAIL;
  manage_resource_set_disposer(xd, o, d, sa);
  xd->mr.tex = (LPDIRECT3DTEXTURE9 *)malloc(sizeof(void *) * sa->a[1]);
  if(!xd->mr.tex) return E_FAIL;
  xd->mr.vbuf = (LPDIRECT3DVERTEXBUFFER9 *)malloc(sizeof(void *) * sa->a[6]);
  if(!xd->mr.vbuf){ free(xd->mr.tex); xd->mr.tex = NULL; return E_FAIL; }
  for(size_t i = 0; i < sa->a[1]; ++i) xd->mr.tex[i] = NULL;
  for(size_t i = 0; i < sa->a[6]; ++i) xd->mr.vbuf[i] = NULL;
  return S_OK;
}

HRESULT initFont(Cxd *xd)
{
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  static const Cfont fonts[] = { // <= MAXFONT
    {L"Arial", 24, 0, FW_HEAVY, TRUE},
    {L"Verdana", 32, 0, FW_HEAVY, TRUE},
    {L"Consolas", 48, 0, FW_HEAVY, TRUE},
    {L"FixedSys", 16, 0, FW_HEAVY, FALSE}};
  for(int i = 0; i < MAXFONT; ++i){
    if(!xd->font[i]){
      HRESULT hr = D3DXCreateFont(xd->dev,
        fonts[i].h, fonts[i].w, fonts[i].weight, 1, fonts[i].italic,
        DEFAULT_CHARSET, OUT_TT_ONLY_PRECIS, ANTIALIASED_QUALITY, FF_DONTCARE,
        fonts[i].fontface, &xd->font[i]);
      if(FAILED(hr)) return E_FAIL;
    }
  }
  return S_OK;
}

HRESULT initTexture(Cxd *xd)
{
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  static const wchar_t *fn[] = { // <= xd->mr.sa->a[1]
    L"f0.png", L"f1.png", L"f2.png", L"f3.png", L"f4.png", L"f5.png",
    L"72dpi.png", L"72dpi_ascii_reigasou_16x16.png"};
  wchar_t p[256];
  for(int n = 0; n < _countof(fn); ++n){
    if(!xd->mr.tex[n]){
      _snwprintf_s(p, sizeof(p), _countof(p), L"%s/%s", DIR_RESOURCE, fn[n]);
      if(FAILED(D3DXCreateTextureFromFile(xd->dev, p, &xd->mr.tex[n])))
        return E_FAIL;
      if(!xd->mr.tex[n]) return E_FAIL;
    }
  }
  DWORD m[256][256];
  UINT w = _countof(m[0]), h = _countof(m);
  DWORD c[] = {
    0xffffffff, 0xffff0000, 0xff00ff00, 0xffffff00,
    0xff0000ff, 0xffff00ff, 0xff00ffff, 0xff7f7f7f};
  DWORD mask = 0x00ffffff;
  int o = _countof(fn);
  int e = o - 2, f = o - 1, g = _countof(c);
  for(int k = 0; k < g; ++k){
    int n = o + k;
    if(!xd->mr.tex[n]){
      if(FAILED(readTexture(xd, !k ? f : e, w, h, (DWORD *)m))) return E_FAIL;
      for(UINT j = 0; j < h; ++j){
        DWORD *p = (DWORD *)m + w * j; // sizeof(DWORD)
        if(!k){
          for(UINT i = 0; i < w; ++i) p[i] = (p[i] & ~mask) | (~p[i] & mask);
        }else{
          for(UINT i = 0; i < w; ++i) if((p[i] & mask) == mask) p[i] &= c[k];
        }
      }
      if(FAILED(initTextureIndirect(xd, n, w, h, (DWORD *)m))) return E_FAIL;
      if(!xd->mr.tex[n]) return E_FAIL;
    }
  }
  LPDIRECT3DTEXTURE9 t = xd->mr.tex[o];
  xd->mr.tex[o] = xd->mr.tex[o + g - 1];
  xd->mr.tex[o + g - 1] = xd->mr.tex[e];
  xd->mr.tex[e] = t;
  for(int n = 0; n < o + g; ++n)
    if(FAILED(alphaTexture(xd, n, w, h, 0x00ffffff))) return E_FAIL;
  for(int k = 31; k >= 30; --k){
    DWORD m[16][16];
    UINT w = _countof(m[0]), h = _countof(m);
    for(UINT j = 0; j < h; ++j)
      for(UINT i = 0; i < w; ++i)
        m[j][i] = k == 31 ? 0xfff0c020 : 0xffc020f0;
    if(FAILED(initTextureIndirect(xd, k, w, h, (DWORD *)m))) return E_FAIL;
    if(!xd->mr.tex[k]) return E_FAIL;
  }
//  if(xd->mr.tex[31]) xd->dev->UpdateTexture(xd->mr.tex[31], xd->mr.tex[30]);
  return S_OK;
}

HRESULT initTextureIndirect(Cxd *xd, size_t n, UINT w, UINT h, DWORD *q)
{
  if(!xd) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  if(!xd->mr.tex[n]){
    if(FAILED(D3DXCreateTexture(xd->dev, w, h, 1, D3DUSAGE_DYNAMIC,
      D3DFMT_A8R8G8B8, D3DPOOL_DEFAULT, &xd->mr.tex[n]))) return E_FAIL;
    if(!xd->mr.tex[n]) return E_FAIL;
    D3DLOCKED_RECT rct; // must create with D3DUSAGE_DYNAMIC to LockRect
    if(xd->mr.tex[n]->LockRect(0, &rct, NULL,
      D3DLOCK_DISCARD | D3DLOCK_NOOVERWRITE) == D3D_OK){
      for(UINT j = 0; j < h; ++j){
        DWORD *o = q + w * j; // sizeof(DWORD)
        DWORD *p = (DWORD *)((BYTE *)rct.pBits + rct.Pitch * j);
        for(UINT i = 0; i < w; ++i) p[i] = o[i];
      }
      xd->mr.tex[n]->UnlockRect(0);
    }
  }
  return S_OK;
}

HRESULT readTexture(Cxd *xd, size_t n, UINT w, UINT h, DWORD *q)
{
  if(!xd) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  if(!xd->mr.tex[n]) return E_FAIL;
  else{
    D3DLOCKED_RECT rct; // must create with D3DUSAGE_DYNAMIC to LockRect
    if(xd->mr.tex[n]->LockRect(0, &rct, NULL,
      D3DLOCK_DISCARD | D3DLOCK_NOOVERWRITE) == D3D_OK){
      for(UINT j = 0; j < h; ++j){
        DWORD *o = q + w * j; // sizeof(DWORD)
        DWORD *p = (DWORD *)((BYTE *)rct.pBits + rct.Pitch * j);
        for(UINT i = 0; i < w; ++i) o[i] = p[i];
      }
      xd->mr.tex[n]->UnlockRect(0);
    }
  }
  return S_OK;
}

HRESULT alphaTexture(Cxd *xd, size_t n, UINT w, UINT h, DWORD mask)
{
  if(!xd) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  if(!xd->mr.tex[n]) return E_FAIL;
  else{
//    DWORD *q = (DWORD *)malloc(sizeof(DWORD) * w * h);
//    if(!q) return E_FAIL;
    D3DLOCKED_RECT rct; // must create with D3DUSAGE_DYNAMIC to LockRect
    if(xd->mr.tex[n]->LockRect(0, &rct, NULL,
      D3DLOCK_DISCARD | D3DLOCK_NOOVERWRITE) == D3D_OK){
      for(UINT j = 0; j < h; ++j){
//        DWORD *o = q + w * j; // sizeof(DWORD)
        DWORD *p = (DWORD *)((BYTE *)rct.pBits + rct.Pitch * j);
        for(UINT i = 0; i < w; ++i) if(!(p[i] & mask)) p[i] = 0; // o[i];
      }
      xd->mr.tex[n]->UnlockRect(0);
    }
//    free(q);
  }
  return S_OK;
}

HRESULT prepareVertexBuffer(Cxd *xd, UINT n, Cvtx *vtx, UINT sz, DWORD fvf)
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!xd->mr.vbuf) return E_FAIL;
#endif
  if(!xd->mr.vbuf[n]){
    if(FAILED(xd->dev->CreateVertexBuffer(sz, D3DUSAGE_WRITEONLY,
      fvf, D3DPOOL_MANAGED, &xd->mr.vbuf[n], NULL))) return E_FAIL;
  }
  if(xd->mr.vbuf[n]){
    void *p;
    if(xd->mr.vbuf[n]->Lock(0, sz, (void**)&p, 0) == D3D_OK){
      memcpy(p, vtx, sz);
      xd->mr.vbuf[n]->Unlock();
    }
  }
  return S_OK;
}

HRESULT prepRectFAN(Cxd *xd, Cvtx *vtx, DWORD *c, DWORD *s,
  float u, float v, float w, float h,
  float x, float y, float z, float a, float b, D3DXVECTOR3 *cg) // Cvtx[4]
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!cg) return E_FAIL;
#endif
  // right turn for TRIANGLEFAN
#ifdef VERTEX_2D
  float rhw = 1.0f;
  *vtx++ = {x, y, z, rhw, *c++, *s++, u, v};
  *vtx++ = {x + a, y, z, rhw, *c++, *s++, u + w, v};
  *vtx++ = {x + a, y + b, z, rhw, *c++, *s++, u + w, v + h};
  *vtx++ = {x, y + b, z, rhw, *c++, *s++, u, v + h};
  cg->x = x + a / 2, cg->y = y + b / 2, cg->z = z;
#else
  float fh = (float)xd->h;
  *vtx++ = {x, fh - y, z, *c++, *s++, u, v};
  *vtx++ = {x + a, fh - y, z, *c++, *s++, u + w, v};
  *vtx++ = {x + a, fh - (y + b), z, *c++, *s++, u + w, v + h};
  *vtx++ = {x, fh - (y + b), z, *c++, *s++, u, v + h};
  cg->x = x + a / 2, cg->y = fh - (y + b / 2), cg->z = z;
#endif
  return S_OK;
}

HRESULT prepRectSTRIP(Cxd *xd, Cvtx *vtx, DWORD *c, DWORD *s,
  float u, float v, float w, float h,
  float x, float y, float z, float a, float b, D3DXVECTOR3 *cg) // Cvtx[4]
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!cg) return E_FAIL;
#endif
  // Z (right turn left turn for TRIANGLESTRIP
#ifdef VERTEX_2D
  float rhw = 1.0f;
  *vtx++ = {x, y, z, rhw, *c++, *s++, u, v};
  *vtx++ = {x + a, y, z, rhw, *c++, *s++, u + w, v};
  *vtx++ = {x, y + b, z, rhw, *c++, *s++, u, v + h};
  *vtx++ = {x + a, y + b, z, rhw, *c++, *s++, u + w, v + h};
  cg->x = x + a / 2, cg->y = y + b / 2, cg->z = z;
#else
  float fh = (float)xd->h;
  *vtx++ = {x, fh - y, z, *c++, *s++, u, v};
  *vtx++ = {x + a, fh - y, z, *c++, *s++, u + w, v};
  *vtx++ = {x, fh - (y + b), z, *c++, *s++, u, v + h};
  *vtx++ = {x + a, fh - (y + b), z, *c++, *s++, u + w, v + h};
  cg->x = x + a / 2, cg->y = fh - (y + b / 2), cg->z = z;
#endif
  return S_OK;
}

HRESULT drawVT(Cxd *xd, UINT t, UINT v, Cvtx *vtx, UINT sz, DWORD fvf,
  D3DPRIMITIVETYPE ptype, UINT st, UINT pc) // st: start, pc: primitive count
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  if(!xd->mr.tex[t]) return E_FAIL;
  if(!xd->mr.vbuf) return E_FAIL;
  if(!xd->mr.vbuf[v]) return E_FAIL;
  if(!xd->lc.vdecl) return E_FAIL;
  if(!xd->lc.shader) return E_FAIL;
  if(!xd->lc.tech) return E_FAIL;
#endif
  if(FAILED(prepareVertexBuffer(xd, v, vtx, sz, fvf))) return E_FAIL;
  xd->dev->SetStreamSource(0, xd->mr.vbuf[v], 0, sizeof(Cvtx));
  xd->dev->SetTexture(0, xd->mr.tex[t]);
  xd->dev->SetFVF(fvf);
#ifdef USE_VERTEX_SHADER
  xd->lc.shader->SetTechnique(xd->lc.tech);
  xd->lc.shader->SetTexture("texmap", xd->mr.tex[t]);
  xd->lc.shader->SetMatrix(xd->lc.params[0], &xd->lc.world);
  xd->lc.shader->SetMatrix(xd->lc.params[1], &xd->lc.view);
  xd->lc.shader->SetMatrix(xd->lc.params[2], &xd->lc.prj);
/**/
  D3DXVECTOR4 ext = {0.9f, 0.6f, 0.3f, 1.0f}; // RGBA
  xd->lc.shader->SetVector(xd->lc.params[3], &ext);
/**/
  xd->lc.shader->CommitChanges();
  xd->lc.shader->Begin(NULL, 0);
  xd->lc.shader->BeginPass(0);
  xd->dev->DrawPrimitive(ptype, st, pc);
  xd->lc.shader->EndPass();
  xd->lc.shader->End();
#else
  xd->dev->DrawPrimitive(ptype, st, pc);
#endif
  return S_OK;
}

HRESULT drawChars(Cxd *xd, DWORD *c, DWORD *s,
  UINT t, int cw, int ch, int sw, int sh, float x, float y, float z,
  const wchar_t *w, UINT l) // texture cell w/h scale w/h
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!xd->mr.tex) return E_FAIL;
  if(!xd->mr.tex[t]) return E_FAIL;
  if(!xd->mr.sa) return E_FAIL;
  if(!xd->mr.sa->a) return E_FAIL;
#endif
  for(UINT i = 0; i < l; ++i){
    char j = (char)(w[i] & 0x0ff);
    float a = (float)sw, b = (float)sh, rw = 1.0f / cw, rh = 1.0f / ch;
    float p = (float)(j % cw) * rw, q = (float)(j / ch) * rh;
    float e = x + (float)i * a;
    D3DXVECTOR3 cg;
    Cvtx vtx[4];
    prepRectFAN(xd, vtx, c, s, p, q, rw, rh, e, y, z, a, b, &cg);
    drawVT(xd, t, (UINT)xd->mr.sa->a[6] - 1, vtx, sizeof(vtx), FVF_CVTX,
      D3DPT_TRIANGLEFAN, 0, _countof(vtx) - 2);
  }
  return S_OK;
}

HRESULT draw2DText(Cxd *xd, DWORD c, UINT f, int x, int y, const wchar_t *t)
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->font[f]) return E_FAIL;
#endif
  RECT rct = {x, y, 0, 0};
  xd->font[f]->DrawText(NULL, t, -1, &rct, DT_LEFT | DT_NOCLIP, c);
  return S_OK;
}

HRESULT setLight(Cxd *xd)
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
#endif
  DWORD n = 0;
  xd->lc.dir = D3DXVECTOR3(1.0f, -1.0f, 1.0f); // (1.0f, -1.0f, -1.0f);
  ZeroMemory(&xd->lc.light[n], sizeof(D3DLIGHT9));
  xd->lc.light[n].Type = D3DLIGHT_DIRECTIONAL;
  xd->lc.light[n].Diffuse.r = 1.0f;
  xd->lc.light[n].Diffuse.g = 1.0f;
  xd->lc.light[n].Diffuse.b = 1.0f;
  xd->lc.light[n].Specular.r = 1.0f;
  xd->lc.light[n].Specular.g = 1.0f;
  xd->lc.light[n].Specular.b = 1.0f;
  D3DXVec3Normalize((D3DXVECTOR3 *)&xd->lc.light[n].Direction, &xd->lc.dir);
  xd->lc.light[n].Range = 200.0f;
  xd->dev->SetLight(n, &xd->lc.light[n]);
  xd->dev->LightEnable(n, TRUE);
  return S_OK;
}

HRESULT setCamera(Cxd *xd, TransScreen *ptss)
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!ptss) return E_FAIL;
#endif
  D3DXVECTOR3 axisZ = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
  xd->lc.ep = D3DXVECTOR3(ptss->ep[0], ptss->ep[1], ptss->ep[2]);
  xd->lc.la = D3DXVECTOR3(ptss->la[0], ptss->la[1], ptss->la[2]);
  xd->lc.top = D3DXVECTOR3(ptss->top[0], ptss->top[1], ptss->top[2]);
  D3DXMATRIX rot, cam;
  D3DXMatrixRotationAxis(&rot, &axisZ, D3DXToRadian(xd->lc.angle));
  D3DXMatrixLookAtLH(&cam, &xd->lc.ep, &xd->lc.la, &xd->lc.top);
  D3DXMatrixMultiply(&xd->lc.view, &rot, &cam);
  // D3DXMatrixIdentity(&xd->lc.view);
  xd->dev->SetTransform(D3DTS_VIEW, &xd->lc.view);
  // D3DXVec3TransformCoord(&p->vec_view, &p->vec_pos, &xd->lc.view);
  D3DXMatrixPerspectiveFovLH(&xd->lc.prj,
    D3DXToRadian(120.0f), 4.0f / 3.0f, 0.1f, 500.0f);
  xd->dev->SetTransform(D3DTS_PROJECTION, &xd->lc.prj);
  return S_OK;
}

HRESULT drawD3D(Cxd *xd, TransScreen *ptss)
{
#ifdef _DEBUG
  if(!xd) return E_FAIL;
  if(!xd->dev) return E_FAIL;
  if(!ptss) return E_FAIL;
#endif
  RECT rct;
  GetClientRect(xd->wnd, &rct);
  xd->w = rct.right - rct.left;
  xd->h = rct.bottom - rct.top;
  struct timespec utns;
  _timespec_now(&utns);
  struct tm lt;
  _localtime64_s(&lt, &utns.tv_sec); // _localtime64_r(&utns.tv_sec, &lt);
  wchar_t ts[32];
  int m = _snwprintf_s(ts, sizeof(ts), _countof(ts), L"%02d:%02d:%02d.%03d",
    lt.tm_hour, lt.tm_min, lt.tm_sec, utns.tv_nsec / 1000000);
  unsigned char hex = (unsigned char)(xd->fps.tick * 4) & 0xff;
  xd->dev->Clear(0, NULL,
    D3DCLEAR_TARGET | D3DCLEAR_ZBUFFER | D3DCLEAR_STENCIL,
    D3DCOLOR_ARGB(255, hex, hex, hex), 1.0f, 0);
  if(SUCCEEDED(xd->dev->BeginScene())){
    setLight(xd);
    setCamera(xd, ptss);

    D3DXVECTOR3 axisZ = D3DXVECTOR3(0.0f, 0.0f, 1.0f);
    D3DXMATRIX trans, scale, rot;
    D3DXMatrixTranslation(&trans, -(xd->w / 2.0f), -(xd->h / 2.0f), 0.0f);
    D3DXMatrixScaling(&scale, 0.01f, 0.01f, 1.0f);
    D3DXMatrixMultiply(&xd->lc.world, &trans, &scale);
    D3DXMatrixRotationAxis(&rot, &axisZ, D3DXToRadian(0.0f));
    D3DXMatrixMultiply(&xd->lc.world, &xd->lc.world, &rot);
    // D3DXMatrixIdentity(&xd->lc.world);
    xd->dev->SetTransform(D3DTS_WORLD, &xd->lc.world);

    DWORD w = xd->w, h = xd->h;
    float a = 512.0f, b = 256.0f; // 320.0f, 240.0f
    float e = ((float)w - a) / 2, f = ((float)h - b) / 2, g = 3.25f;
    DWORD c[] = {0xffcc9933, 0xff33cc99, 0xff9933cc, 0xffcc3399}; // ARGB
    DWORD s[] = {0x00202020, 0x00202020, 0x00202020, 0x00202020}; // ARGB
    D3DXVECTOR3 cg;
    Cvtx vtx[4];
    prepRectSTRIP(xd, vtx, c, s, 0.0f, 0.0f, 1.0f, 1.0f, e, f, g, a, b, &cg);
    drawVT(xd, 7, 0, vtx, sizeof(vtx), FVF_CVTX,
      D3DPT_TRIANGLESTRIP, 0, _countof(vtx) - 2);

    for(int k = 0; k < 8; ++k){
      float a = 128.0f, b = 128.0f;
      float e = (k % 4) * 256.0f + 64.0f, f = (k / 4) * 384.0f + 128.0f;
      float g = (k + 3) / 2.0f;
      DWORD c[] = {0xffff8080, 0xff80ff80, 0xff8080ff, 0xffffff80}; // ARGB
      DWORD s[] = {0x00404040, 0x00404040, 0x00404040, 0x00404040}; // ARGB
      prepRectSTRIP(xd, vtx, c, s, 0.0f, 0.0f, 1.0f, 1.0f, e, f, g, a, b, &cg);

      D3DXMATRIX wtmp = xd->lc.world;
      rotCG(&rot, &axisZ, D3DXToRadian(-xd->lc.angle), &cg);
      D3DXMatrixMultiply(&xd->lc.world, &rot, &xd->lc.world);
      xd->dev->SetTransform(D3DTS_WORLD, &xd->lc.world);

      drawVT(xd, 8 + k, 8 + k, vtx, sizeof(vtx), FVF_CVTX,
        D3DPT_TRIANGLESTRIP, 0, _countof(vtx) - 2);

      xd->lc.world = wtmp;
    }
    xd->dev->SetTransform(D3DTS_WORLD, &xd->lc.world);

    wchar_t fps[32];
    int n = _snwprintf_s(fps, sizeof(fps), _countof(fps),
      L"%7.3f", xd->fps.fps);
    wchar_t inf[128];
    int o = _snwprintf_s(inf, sizeof(inf), _countof(inf),
      L"hex: %02x angle: %7.3f", hex, xd->lc.angle);
/**/
    DWORD fcs[] = {
      D3DCOLOR_ARGB(255, 240, 192, 32), D3DCOLOR_ARGB(255, 128, 112, 16)};
    DWORD fc = fcs[hex & 0x80 ? 1 : 0];
    draw2DText(xd, fc, 0, w / 2, h / 2, L"Arial"); // center
    draw2DText(xd, fc, 1, (int)e, (int)f, L"Verdana"); // lt
    draw2DText(xd, fc, 2, w - (int)e, (int)f, L"Consolas"); // rt
    draw2DText(xd, fc, 3, (int)e, h - (int)f, L"FixedSys"); // lb
    draw2DText(xd, fc, 3, w - (int)e, h - (int)f, fps); // rb
    draw2DText(xd, fc, 3, w / 2, h - (int)f, inf); // cb
/**/
    DWORD dif[] = {
      D3DCOLOR_ARGB(255, 32, 240, 192), D3DCOLOR_ARGB(255, 192, 32, 240),
      D3DCOLOR_ARGB(255, 240, 32, 192), D3DCOLOR_ARGB(255, 240, 192, 32)};
    DWORD spc[] = {
      D3DCOLOR_ARGB(0, 2, 2, 2), D3DCOLOR_ARGB(0, 8, 8, 8),
      D3DCOLOR_ARGB(0, 16, 16, 16), D3DCOLOR_ARGB(0, 8, 8, 8)};
//    float th = ((UINT)(xd->fps.tick * 6) % 360) * (float)M_PI / 180.0f;
    float th = (((UINT)xd->fps.tick * 6) % 360) * (float)M_PI / 180.0f;
    float x = (w + a * cosf(th)) / 2, y = (h + b * sinf(th)) / 2, z = 1.0f;
    drawChars(xd, dif, spc, 6, 16, 16, 24, 32, x, y, z, ts, m);
    {
      DWORD dif[] = {
        D3DCOLOR_ARGB(255, 32, 192, 240), D3DCOLOR_ARGB(255, 192, 240, 32),
        D3DCOLOR_ARGB(255, 192, 32, 240), D3DCOLOR_ARGB(255, 240, 192, 32)};
      DWORD spc[] = {
        D3DCOLOR_ARGB(0, 2, 2, 2), D3DCOLOR_ARGB(0, 4, 4, 4),
        D3DCOLOR_ARGB(0, 8, 8, 8), D3DCOLOR_ARGB(0, 4, 4, 4)};
      float x = w - e, y = h - f, z = 0.5f;
      drawChars(xd, dif, spc, 7, 16, 16, 16, 16, x, y, z, fps, n);
    }
    xd->dev->EndScene();
  }
  if(xd->dev->Present(NULL, NULL, NULL, NULL) == D3DERR_DEVICELOST){
    // code for backup and free resources of D3DPOOL_DEFAULT
    // iter;
    if(xd->dev->TestCooperativeLevel() != D3DERR_DEVICENOTRESET) return E_FAIL;
    if(xd->dev->Reset(&xd->d3dpp) != D3D_OK){
      PostQuitMessage(0);
      return E_FAIL;
    }
    // code for recover resources of D3DPOOL_DEFAULT
    // iter;
  }
  return S_OK;
}

HRESULT updateD3D(Cxd *xd)
{
#ifdef _DEBUG
  if(!xd) E_FAIL;
//  if(!xd->dev) E_FAIL;
#endif
  ++xd->fps.frames;
  struct timespec utns, dt;
  _timespec_now(&utns);
  _timespec_sub(&dt, utns, xd->fps.utns);
  double delta = _timespec_to_double(dt);
  if(delta >= 1 / 60.0f){ // dt.tv_sec >= 1
    xd->fps.utns = utns;
    xd->fps.tick += delta;
    xd->fps.fps = (float)(xd->fps.frames / xd->fps.tick);
    outLog(LOGFILE, L"%llu.%09lu, %09d, %9.4lf, %9.4f, %7.3f\n",
      (unsigned long long)dt.tv_sec, dt.tv_nsec,
      xd->fps.frames, xd->fps.tick, xd->fps.fps, xd->lc.angle);

    xd->lc.angle += (float)(delta * 6);
    if(xd->lc.angle >= 360.0f) xd->lc.angle -= 360.0f;
  }
  return S_OK;
}

HRESULT rotCG(D3DXMATRIX *rot,
  const D3DXVECTOR3 *axis, float a, const D3DXVECTOR3 *cg)
{
#ifdef _DEBUG
  if(!rot) return E_FAIL;
  if(!axis) return E_FAIL;
  if(!cg) return E_FAIL;
#endif
  D3DXMATRIX trans;
  D3DXMatrixTranslation(&trans, -cg->x, -cg->y, -cg->z);
  D3DXMatrixRotationAxis(rot, axis, a);
  D3DXMatrixMultiply(rot, &trans, rot);
  D3DXMatrixTranslation(&trans, cg->x, cg->y, cg->z);
  D3DXMatrixMultiply(rot, rot, &trans);
  return S_OK;
}

void initLog(const wchar_t *f)
{
  FILE *fp;
  if(!FAILED(_wfopen_s(&fp, f, L"wb"))) fclose(fp);
}

void outLog(const wchar_t *f, const wchar_t *fmt, ...)
{
  FILE *fp;
  if(!FAILED(_wfopen_s(&fp, f, L"ab+"))){
    va_list args;
    va_start(args, fmt);
    vfwprintf(fp, fmt, args);
    va_end(args);
    fclose(fp);
  }
}

void _timespec_now(struct timespec *a){
  timespec_get(a, TIME_UTC); // if(_timespec64_get(a, TIME_UTC) == TIME_UTC)
}

double _timespec_to_double(struct timespec a)
{
  return (double)a.tv_sec + (double)a.tv_nsec / 1000000000;
}

void _timespec_clear(struct timespec *a){
  a->tv_sec = a->tv_nsec = 0;
}

bool _timespec_iszero(struct timespec a){
  return !a.tv_sec && !a.tv_nsec;
}

bool _timespec_isset(struct timespec a){
  return !_timespec_iszero(a);
}

int _timespec_cmp(struct timespec a, struct timespec b)
{
  return a.tv_sec == b.tv_sec ?
    (a.tv_nsec == b.tv_nsec ? 0 : (a.tv_nsec < b.tv_nsec ? -1 : 1)):
    (a.tv_sec < b.tv_sec ? -1 : 1);
}

void _timespec_sub(struct timespec *r, struct timespec a, struct timespec b)
{
  r->tv_sec = a.tv_sec - b.tv_sec;
  if((r->tv_nsec = a.tv_nsec - b.tv_nsec) < 0){
    --r->tv_sec;
    r->tv_nsec += 1000000000;
  }
}

void _timespec_add(struct timespec *r, struct timespec a, struct timespec b)
{
  r->tv_sec = a.tv_sec + b.tv_sec;
  if((r->tv_nsec = a.tv_nsec + b.tv_nsec) >= 1000000000){
    ++r->tv_sec;
    r->tv_nsec -= 1000000000;
  }
}

size_t dump_mat(uchar *u8s, size_t l, const float *m, size_t r, size_t c)
{
  DispMatParam p = {m, sizeof(float), r, c,
    (const uchar *)"%08x", 18, 7, 0, 0};
  return disp_mat(u8s, l, &p, cb_x);
}

size_t cb_xll(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(ulonglong *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_x(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(ulong *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_ull(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(ulonglong *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_ll(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(long long *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_u(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(ulong *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_i(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(long *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t cb_f(uchar *u, size_t l, DispMatParam *p)
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  return sprintf_s(s, l, (char *)p->fmt,
    *(float *)&((const byte *)p->m)[p->w * (p->cols * p->r + p->c)]);
}

size_t disp_mat(uchar *u, size_t l, DispMatParam *p,
  size_t (*cb_element)(uchar *u, size_t l, DispMatParam *p))
{
  char *s = (char *)u; // utf8
  s[0] = '\0'; // safety
  size_t t = 0;
  t += sprintf_s(s + t, l - t, "[");
  for(size_t r = 0; r < p->rows; ++r){
    t += sprintf_s(s + t, l - t, "%s[", !r ? "" : " ");
    for(size_t c = 0; c < p->cols; ++c){
      p->r = r, p->c = c;
      uchar elem[32];
      cb_element(elem, sizeof(elem), p);
      t += sprintf_s(s + t, l - t, " %s", elem);
    }
    t += sprintf_s(s + t, l - t, "]%s", r < p->rows - 1 ? "\n" : "");
  }
  t += sprintf_s(s + t, l - t, "]");
  return t;
}

CppBridge::CppBridge(void *q) : p(q)
{
  if(!p) fprintf(stderr, "\nconstructor: %p\n", p);
}

CppBridge::~CppBridge()
{
  if(!p) fprintf(stderr, "\ndestructor: %p\n", p);
}

void *CppBridge::get()
{
  return this->p;
}

/*
CppBridge *CppBridge_new(void *q) {return new CppBridge(q);}
void *CppBridge_get(CppBridge *b) {return b->get();}
void CppBridge_CppBridge_destructor(CppBridge *b) {delete b;}
*/

ulonglong gget(CppBridge *b)
{
  return (ulonglong)b->p;
}
