#!/usr/bin/env python3

import numpy as np
np.random.seed(0)

# Multi-Head Attention

# Focus Areas for Groups of Attention Heads
# Head 1-12 (Layer 1): Focuse on basic syntax and grammar
# Head 12-24(Layer 2): Address lexical relationships
# Head 25-36(Layer 3): Enhance contextual understanding
# Head 37-48(Layer 4): Process semantic roles and dependencies
# Head 49-60(Layer 5): Manager discourse and narrative flow
# ...

# ====== 超参数 ======
#B = 10           # batch size
T = 42           # seq_len
d_model = 768    # 隐藏维度
n_heads = 12     # 头数
head_size = d_model // n_heads  # 单头维度 d_k = d_v
assert n_heads * head_size == d_model

# ====== 输入 X: [T, d_model] ======
X = np.random.randn(T, d_model).astype(np.float32)

# ====== 线性映射矩阵（权重） ======
Wq = np.random.randn(d_model, head_size).astype(np.float32)
Wk = np.random.randn(d_model, head_size).astype(np.float32)
Wv = np.random.randn(d_model, head_size).astype(np.float32)

# ====== 计算 Q, K, V: [T, head_size] ======
Q = X @ Wq 
K = X @ Wk
V = X @ Wv

# ====== 缩放点积注意力 scores = Q K^T / sqrt(d_k): [T, T] ======
scores = (Q @ K.transpose(-1, -2)) / np.sqrt(head_size)

# ====== 因果 Mask（上三角置为 -inf，阻止看未来位）======
# mask: [T, T]，主对角线以上为 True
# TODO: block_size >= T
causal_mask = np.triu(np.ones((T, T), dtype=bool), k=1)
# [False,  True,  True...]
# [False,  False, True...]
# [False,  False, False...]
# ...

# 广播到 batch 维
scores = np.where(causal_mask, -np.inf, scores) # (T, T)

# ====== 稳定 softmax ======
def softmax(x, axis=-1):
    x = x - x.max(axis=axis, keepdims=True)
    e = np.exp(x)
    return e / e.sum(axis=axis, keepdims=True)

attn = softmax(scores, axis=-1)  # [T, T]

# ====== 输出（该头的上下文表示） O = softmax(scores) V: [T, head_size] ======
O_head = attn @ V

# multi-head
heads = [O_head.copy() for _ in range(n_heads)]

O_concat = np.concatenate(heads, axis=-1)  # [T, d_model]

#O_stack = np.stack(heads, axis=1)        # [T, n_heads, head_size]
#O_reshape = O_stack.reshape(T, d_model)  # [T, d_model]

# ====== 打印形状核对 ======
print("X:", X.shape)                        # (42, 768)
print("Q/K/V:", Q.shape, K.shape, V.shape)  # (42, 64)
print("scores:", scores.shape)              # (42, 42)
print("attn:", attn.shape)                  # (42, 42)
print("O_head:", O_head.shape)              # (42, 64)
print("O_concat:", O_concat.shape)          # (42, 768)
