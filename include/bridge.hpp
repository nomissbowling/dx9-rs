/*
  bridge.hpp
*/

#ifndef __BRIDGE_HPP__
#define __BRIDGE_HPP__

#ifndef _UNICODE
#define _UNICODE
#ifdef _UNICODE
#ifndef UNICODE
#define UNICODE
#endif
#endif
#endif

// #define VERTEX_2D
#ifndef VERTEX_2D
#define USE_VERTEX_SHADER
#endif

#ifdef __cplusplus
extern "C" {
#endif

typedef unsigned long long ulonglong;
typedef unsigned long ulong;
typedef unsigned int uint;
typedef unsigned char uchar;
typedef unsigned char byte;

extern const float ref_mat44_i[][4];
extern const float ref_mat44_t[][4];
extern const float ref_mat44_s[][4];
extern const float ref_mat44_r[][4];
extern const float ref_mat44_q[][4];
extern const float ref_mat44_d[][4];
extern const float ref_mat44_z[][4];
extern const float ref_mat44_c[][4];
extern const float ref_mat44_v[][4];
extern const float ref_mat44_p[][4];
extern const float ref_mat_column_major[];
extern const float dump_mat_m44[];
extern const byte *dump_mat_m44_u8s;

extern const wchar_t *LOGFILE;

// This instance must be here to use _countof(Cpname)
extern const char *Cpname[] = {"world", "view", "prj", "ext"}; // 4x4 4x4 4x4 4

struct Cxd_; // forward ref (unknown size 0)
typedef struct Cxd_ Cxd;

// bindgen BUG ? (sometimes be f: *mut usize) place dummy to set Csa::a: *const
struct Fuga {
  const size_t *f;
};

typedef struct Csa_ {
  const size_t *a; // to be a[n]
  size_t n;
} Csa;

typedef struct Cvtx_ {
  float x, y, z; // D3DXVECTOR3 p;
#ifdef VERTEX_2D
  float rhw; // 1.0f only for 2D 1/w D3DFVF_XYZRHW
#endif
  uint c; // DWORD D3DFVF_DIFFUSE
  uint s; // DWORD D3DFVF_SPECULAR
  float u, v; // D3DFVF_TEX1 or D3DFVF_TEX0
} Cvtx;
extern const ulong FVF_CVTX; // DWORD

#ifndef BRIDGE_FAKE_DISABLE // (for Rust bindgen)

// fake
typedef void *HWND;
typedef long HRESULT; // struct exists on Win32::core:: (windows_result::)
//static const long E_FAIL = 0x80004005; // exists on Win32::Foundation::
//static const long S_OK = 0x00000000; // exists on Win32::Foundation::
typedef ulong DWORD;
typedef uint UINT;

// fake
typedef DWORD D3DPRIMITIVETYPE;
struct D3DVERTEXELEMENT9; // unknown size 0
// fake for Rust (no mangle) must include ex_c.h (must use quotes to include)
#include "ex_c.h"

// fake (lost size original in bridge.cpp)
//struct CLightCam;
//struct CManageRsource;
//struct Cfont;
//struct Cfps;

// fake (struct std::timespec same size as struct _timespec64)
//namespace std {
struct timespec {
  long long tv_sec; // i64
  long tv_nsec; // i32
};
//}

#else // BRIDGE_FAKE_DISABLE (skipped when Rust bindgen)

typedef struct CLightCam_ {
  D3DXMATRIX world, view, prj;
  LPDIRECT3DVERTEXDECLARATION9 vdecl;
  LPD3DXEFFECT shader;
  D3DXHANDLE tech, params[_countof(Cpname)];
  D3DLIGHT9 light[MAXLIGHT];
  D3DXVECTOR3 dir;
  D3DXVECTOR3 ep, la, top;
  float angle;
} CLightCam;

typedef struct CManageResource_ {
  LPDIRECT3DVOLUMETEXTURE9 *vt; // volume map texture IDirect3DVolumeTexture9
  LPDIRECT3DTEXTURE9 *tex; // texture IDirect3DTexture9
  LPDIRECT3DCUBETEXTURE9 *ct; // cube map texture IDirect3DCubeTexture9
  LPDIRECT3DSURFACE9 *sf; // surface IDirect3DSurface9
  LPDIRECT3DSURFACE9 *ds; // depth stencil resource without D3DPOOL_DEFAULT
  LPDIRECT3DSURFACE9 *rt; // rendering target surface without D3DPOOL_DEFAULT
  LPDIRECT3DVERTEXBUFFER9 *vbuf; // vertex buffer IDirect3DVertexBuffer9
  LPDIRECT3DINDEXBUFFER9 *ibuf; // index buffer IDirect3DIndexBuffer9
  HRESULT (*disposer)(Cxd *xd, void *o);
  void *owner;
  const Csa *sa;
} ManageResource;

typedef struct Cfont_ {
  const wchar_t *fontface;
  INT h;
  UINT w;
  UINT weight;
  BOOL italic;
} Cfont;

typedef struct Cfps_ {
  struct timespec utns; // struct _timespec64 utns;
  double tick;
  float fps;
  UINT frames;
} Cfps;

typedef struct Cxd_ {
  HWND wnd;
  LONG w, h;
  LPDIRECT3D9 d3d;
  LPDIRECT3DDEVICE9 dev;
  D3DPRESENT_PARAMETERS d3dpp;
  CLightCam lc;
  ManageResource mr;
  LPD3DXFONT font[MAXFONT];
  Cfps fps;
} Cxd;

#include <ex_rust.h>

#endif // BRIDGE_FAKE_DISABLE

extern const D3DVERTEXELEMENT9 CvtxElem[];

Cxd *createD3D();
void destroyD3D(Cxd **xd);
void finishD3D(Cxd *xd);
HRESULT initD3D(Cxd *xd, HWND wnd);
size_t manage_resource_n_pvec(Cxd *xd);
void ***manage_resource_ptr_mut(Cxd *xd);
HRESULT manage_resource_set_disposer(Cxd *xd,
  void *o, HRESULT (*d)(Cxd *xd, void *o), const Csa *sa);
HRESULT disposeManageResourceElements(Cxd *xd);
HRESULT disposeManageResource(Cxd *xd, void *o);
HRESULT initManageResource(Cxd *xd,
  void *o, HRESULT (*d)(Cxd *xd, void *o), const Csa *sa);
HRESULT initFont(Cxd *xd);
HRESULT initTexture(Cxd *xd);
HRESULT initTextureIndirect(Cxd *xd, size_t n, UINT w, UINT h, DWORD *q);
HRESULT readTexture(Cxd *xd, size_t n, UINT w, UINT h, DWORD *q);
HRESULT alphaTexture(Cxd *xd, size_t n, UINT w, UINT h, DWORD mask);
HRESULT prepareVertexBuffer(Cxd *xd, UINT n, Cvtx *vtx, UINT sz, DWORD fvf);
HRESULT prepRectFAN(Cxd *xd, Cvtx *vtx, DWORD *c, DWORD *s,
  float u, float v, float w, float h,
  float x, float y, float z, float a, float b, D3DXVECTOR3 *cg); // Cvtx[4]
HRESULT prepRectSTRIP(Cxd *xd, Cvtx *vtx, DWORD *c, DWORD *s,
  float u, float v, float w, float h,
  float x, float y, float z, float a, float b, D3DXVECTOR3 *cg); // Cvtx[4]
HRESULT drawVT(Cxd *xd, UINT t, UINT v, Cvtx *vtx, UINT sz, DWORD fvf,
  D3DPRIMITIVETYPE ptype, UINT st, UINT pc); // st: start, pc: primitive count
HRESULT drawChars(Cxd *xd, DWORD *c, DWORD *s,
  UINT t, int cw, int ch, int sw, int sh, float x, float y, float z,
  const wchar_t *w, UINT l); // texture cell w/h scale w/h
HRESULT draw2DText(Cxd *xd, DWORD c, UINT f, int x, int y, const wchar_t *t);
HRESULT setLight(Cxd *xd);
HRESULT setCamera(Cxd *xd);
HRESULT drawD3D(Cxd *xd);
HRESULT updateD3D(Cxd *xd);
HRESULT rotCG(D3DXMATRIX *rot,
  const D3DXVECTOR3 *axis, float a, const D3DXVECTOR3 *cg);

void initLog(const wchar_t *f);
void outLog(const wchar_t *f, const wchar_t *fmt, ...);

void _timespec_now(struct timespec *a);
double _timespec_to_double(struct timespec a);
void _timespec_clear(struct timespec *a);
bool _timespec_iszero(struct timespec a);
bool _timespec_isset(struct timespec a);
int _timespec_cmp(struct timespec a, struct timespec b);
void _timespec_sub(struct timespec *r, struct timespec a, struct timespec b);
void _timespec_add(struct timespec *r, struct timespec a, struct timespec b);

typedef struct DispMatParam_ {
  const void *m;
  size_t w, rows, cols;
  const uchar *fmt;
  size_t di, df;
  size_t r, c;
} DispMatParam;

size_t dump_mat(uchar *u8s, size_t l, const float *m, size_t r, size_t c);
size_t cb_xll(uchar *u, size_t l, DispMatParam *p);
size_t cb_x(uchar *u, size_t l, DispMatParam *p);
size_t cb_ull(uchar *u, size_t l, DispMatParam *p);
size_t cb_u(uchar *u, size_t l, DispMatParam *p);
size_t cb_ll(uchar *u, size_t l, DispMatParam *p);
size_t cb_i(uchar *u, size_t l, DispMatParam *p);
size_t cb_f(uchar *u, size_t l, DispMatParam *p);
size_t disp_mat(uchar *u, size_t l, DispMatParam *p,
  size_t (*cb_element)(uchar *u, size_t l, DispMatParam *p));

#ifdef __cplusplus
}
#endif

class CppBridge; // need for bridge_bindings

// new and CLS_CLS_destructor are auto created when in header
class CppBridge {
// friend unsigned long long gget(CppBridge *b);
protected:
public:
  void *p;
public:
  CppBridge(void *q);
  virtual ~CppBridge();
  void *get();
};

#ifdef __cplusplus
extern "C" {
#endif

/*
CppBridge *CppBridge_new(void *q);
void *CppBridge_get(CppBridge *b);
void CppBridge_CppBridge_destructor(CppBridge *b);
*/

ulonglong gget(CppBridge *b);

#ifdef __cplusplus
}
#endif

#endif // __BRIDGE_HPP__
