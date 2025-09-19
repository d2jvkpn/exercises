#!/usr/bin/env python3

import numpy as np
np.random.seed(0)

# ====== 超参数 ======
B = 10          # batch size
T = 42          # seq_len
d_model = 768   # 隐藏维度
n_heads = 12    # 头数

head_size = d_model // n_heads  # 每头维度（这里整除）
assert d_model == n_heads * head_size

# ====== 输入 ======
X = np.random.randn(B, T, d_model).astype(np.float32)  # [B, T, d_model]

# ====== 参数（一次性投影 QKV，再输出投影 Wo）======
# Wqkv: [d_model, 3 * n_heads * head_size]
Wqkv = np.random.randn(d_model, 3 * n_heads * head_size).astype(np.float32)
bqkv = np.random.randn(3 * n_heads * head_size).astype(np.float32)  # 可选偏置

# Wo: [n_heads * head_size, d_model]
Wo = np.random.randn(n_heads * head_size, d_model).astype(np.float32)
bo = np.random.randn(d_model).astype(np.float32)  # 可选偏置

def softmax(x, axis=-1):
    x = x - x.max(axis=axis, keepdims=True)  # 数值稳定
    e = np.exp(x)
    return e / e.sum(axis=axis, keepdims=True)

# ====== 因果 Mask: 阻止看未来 token ======
# [T, T]：上三角（不含主对角）为 True
causal_mask = np.triu(np.ones((T, T), dtype=bool), k=1)

# ====== 前向：MHA ======
# 1) 一次性线性映射到 QKV
qkv = X @ Wqkv + bqkv  # [B, T, 3*h*hs]

# 2) 拆分成 Q,K,V，并分头
# 先 reshape 为 [B, T, 3, h, hs]，再分离第三维
qkv = qkv.reshape(B, T, 3, n_heads, head_size)
Q = qkv[:, :, 0]  # [B, T, h, hs]
K = qkv[:, :, 1]
V = qkv[:, :, 2]

# 3) 置换到 [B, h, T, hs] 便于做注意力
Q = np.transpose(Q, (0, 2, 1, 3))  # [B, h, T, hs]
K = np.transpose(K, (0, 2, 1, 3))
V = np.transpose(V, (0, 2, 1, 3))

# 4) scores = Q @ K^T / sqrt(d_k)  -> [B, h, T, T]
scores = (Q @ np.transpose(K, (0, 1, 3, 2))) / np.sqrt(head_size)

# 5) 应用因果 Mask（广播到 batch/head）
scores = np.where(causal_mask[None, None, :, :], -1e9, scores)

# 6) 注意力权重
attn = softmax(scores, axis=-1)  # [B, h, T, T]

# 7) 上下文：attn @ V -> [B, h, T, hs]
context = attn @ V

# 8) 合并各头：先到 [B, T, h, hs]，再 reshape 到 [B, T, h*hs]
context = np.transpose(context, (0, 2, 1, 3)).reshape(B, T, n_heads * head_size)

# 9) 输出线性映射回 d_model
Y = context @ Wo + bo  # [B, T, d_model]

# ====== 打印检查 ======
print("X:", X.shape)
print("Q/K/V per-head:", Q.shape, K.shape, V.shape)   # [B, h, T, hs]
print("scores/attn:", scores.shape, attn.shape)       # [B, h, T, T]
print("context:", context.shape)                      # [B, T, h*hs]
print("Y (MHA输出):", Y.shape)                         # [B, T, d_model]
