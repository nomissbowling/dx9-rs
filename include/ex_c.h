// fake for Rust (no mangle) must include ex_c.h before d3dx9.h

extern "C" {

struct D3DXVECTOR3; // forward ref (unknown size 0)
struct D3DXMATRIX; // forward ref (unknown size 0)
struct D3DXQUATERNION; // forward ref (unknown size 0)

// fake
D3DXQUATERNION *D3DXQuaternionNormalize(D3DXQUATERNION *q,
  const D3DXQUATERNION *a);
D3DXQUATERNION *D3DXQuaternionInverse(D3DXQUATERNION *q,
  const D3DXQUATERNION *a);
D3DXQUATERNION *D3DXQuaternionMultiply(D3DXQUATERNION *q,
  const D3DXQUATERNION *a, const D3DXQUATERNION *b);
D3DXQUATERNION *D3DXQuaternionRotationAxis(D3DXQUATERNION *q,
  const D3DXVECTOR3 *axis, float th);
D3DXQUATERNION *D3DXQuaternionRotationMatrix(D3DXQUATERNION *q,
  const D3DXMATRIX *m);
D3DXMATRIX *D3DXMatrixRotationQuaternion(D3DXMATRIX *m,
  const D3DXQUATERNION *q);
D3DXMATRIX *D3DXMatrixIdentity(D3DXMATRIX *m);
D3DXMATRIX *D3DXMatrixTranspose(D3DXMATRIX *m,
  const D3DXMATRIX *a);
float D3DXMatrixDeterminant(const D3DXMATRIX *m);
D3DXMATRIX *D3DXMatrixInverse(D3DXMATRIX *m,
  float *det, const D3DXMATRIX *a);
D3DXMATRIX *D3DXMatrixMultiply(D3DXMATRIX *m,
  const D3DXMATRIX *a, const D3DXMATRIX *b);
D3DXMATRIX *D3DXMatrixTranslation(D3DXMATRIX *m,
  float x, float y, float z);
D3DXMATRIX *D3DXMatrixScaling(D3DXMATRIX *m,
  float x, float y, float z);
D3DXMATRIX *D3DXMatrixRotationAxis(D3DXMATRIX *m,
  const D3DXVECTOR3 *axis, float rad);
D3DXMATRIX *D3DXMatrixLookAtLH(D3DXMATRIX *m,
  const D3DXVECTOR3 *eye, const D3DXVECTOR3 *lookat, const D3DXVECTOR3 *up);
D3DXMATRIX *D3DXMatrixPerspectiveFovLH(D3DXMATRIX *m,
  float rad, float ratio, float z0, float z1);

}
