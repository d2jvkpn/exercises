#!/usr/bin/env python3
import numpy as np
import scipy


matrix = np.array([
    [ 1., 2., 3., 4.],
    [ 5., 6., 7., 8.],
    [ 9., 10., 11., 12.],
    [13., 14., 15., 16.],
])

u, s, vt = scipy.sparse.linalg.svds(matrix, k=1)
svd_matrix = u * s * vt

print(f"==> matrix={matrix.size}:\n{matrix}")
print(f"==> u={u.size}:\n{u}")
print(f"==> s={s.size}:\n{s}")
print(f"==> vt={vt.size}:\n{vt}")
print(f"==> svd_matrix={svd_matrix.size}:\n{svd_matrix}")

#### LoRA
# W = (d_in, d_out)                  # y = W x, d_in = 4096, d_out = 4096, parameters = 4096 * 4096
# A = (rank, d_in), B=(d_out, rank)  # y = (W + BA) x, rank = 4, parameters = 4096 * 4 + 4 * 4096
