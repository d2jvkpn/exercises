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
B = 10                            # batch size
T = 42                            # seq_len
d_model = 768                     # 隐藏维度, embedding
num_heads = 12                    # 头数
head_size = d_model // num_heads  # 单头维度 d_k = d_v
assert num_heads * head_size == d_model
dropout_p = 0.2

# ====== 输入 X: (T, d_model) ======
X = np.random.randn(T, d_model).astype(np.float32)

# ====== 线性映射矩阵（权重）: (d_model, head_size) ======
Wq = np.random.randn(d_model, head_size).astype(np.float32)
Wk = np.random.randn(d_model, head_size).astype(np.float32)
Wv = np.random.randn(d_model, head_size).astype(np.float32)

# ====== 计算 Q, K, V: (T, head_size) ======
Q = X @ Wq # (T, head_size)
K = X @ Wk # (T, head_size)
V = X @ Wv # (T, head_size)

# ====== 缩放点积注意力 scores = Q K^T / sqrt(d_k): (T, T) ======
scores = (Q @ K.transpose(-1, -2)) / np.sqrt(head_size)

# ====== 因果 Mask（上三角置为 -inf，阻止看未来位）: (T, T)======
# mask: (T, T)，主对角线以上为 True
# TODO: block_size >= T
causal_mask = np.triu(np.ones((T, T), dtype=bool), k=1)
# [False,  True,  True...]
# [False,  False, True...]
# [False,  False, False...]
# ...

# 广播到 batch 维: (T, T)
scores = np.where(causal_mask, -np.inf, scores)

# ====== 稳定 softmax ======
def softmax(x, axis=-1):
    x = x - x.max(axis=axis, keepdims=True)
    e = np.exp(x)
    return e / e.sum(axis=axis, keepdims=True)

def dropout(x, dropout_p, training=True):
    if not training or dropout_p == 0.0:
        return x

    keep_prob = 1 - dropout_p
    mask = (np.random.rand(*x.shape) < keep_prob).astype(np.float32)
    return (x * mask) / keep_prob

attn = softmax(scores, axis=-1)  # (T, T)
attn = dropout(attn, dropout_p)

# ====== 输出（该头的上下文表示） O = softmax(scores) V: (T, head_size) ======
O_head = attn @ V # (T, T) @ (T, head_size)

# multi-head
heads = [O_head.copy() for _ in range(num_heads)]

O_concat = np.concatenate(heads, axis=-1)  # (T, d_model)
#O_stack = np.stack(heads, axis=1)         # (T, num_heads, head_size)
#O_reshape = O_stack.reshape(T, d_model)   # (T, d_model)

# projection
W = np.random.randn(num_heads * head_size, d_model) # weights, num_heads * d_v
b = np.random.randn(d_model)                        # bias

O_projected = O_concat @ W + b
O_projected = dropout(O_projected, dropout_p)

# ====== 打印形状核对 ======
print(f"Parameters: T={T}, d_model={d_model}, num_heads={num_heads}, head_size={head_size}")
print(f"X: {X.shape}")                     # (42, 768)
print(f"Wq/Wk/Wv: {Wq.shape}")             # (768, 64)
print(f"Q/K/V: {Q.shape}")                 # (42, 64)
print(f"scores: {scores.shape}")           # (42, 42)
print(f"attention: {attn.shape}")          # (42, 42)
print(f"O_head: {O_head.shape}")           # (42, 64)
print(f"O_concat: {O_concat.shape}")       # (42, 768)
print(f"O_projected: {O_projected.shape}") # (42, 768)
