#!/usr/bin/env python3
import numpy as np

class MultiHeadAttention:
    def __init__(self, d_model, num_heads, dropout=0.1):
        self.d_model = d_model
        self.num_heads = num_heads
        self.d_k = d_model // num_heads  # 每个头的维度
        self.d_v = d_model // num_heads
        self.dropout_rate = dropout

        # 初始化权重矩阵 (Xavier初始化)
        self.W_Q = np.random.randn(d_model, d_model) * np.sqrt(1.0 / (d_model + d_model))
        self.W_K = np.random.randn(d_model, d_model) * np.sqrt(1.0 / (d_model + d_model))
        self.W_V = np.random.randn(d_model, d_model) * np.sqrt(1.0 / (d_model + d_model))
        self.W_O = np.random.randn(d_model, d_model) * np.sqrt(1.0 / (d_model + d_model))

        # 偏置项
        #self.b_Q = np.zeros(d_model)
        #self.b_K = np.zeros(d_model)
        #self.b_V = np.zeros(d_model)
        self.b_O = np.zeros(d_model)

        # 缓存用于反向传播
        self.cache = None

    def softmax(self, x, axis=-1):
        """稳定的softmax实现"""
        x_max = np.max(x, axis=axis, keepdims=True)
        e_x = np.exp(x - x_max)
        return e_x / np.sum(e_x, axis=axis, keepdims=True)

    def dropout(self, x, training=True):
        """Dropout实现"""
        if training and self.dropout_rate > 0:
            mask = (np.random.rand(*x.shape) > self.dropout_rate)
            return x * mask / (1 - self.dropout_rate)

        return x

    def split_heads(self, x, batch_size, seq_len):
        """将输入分割成多个头"""
        # x: [batch_size, seq_len, d_model]
        # 重塑为: [batch_size, seq_len, num_heads, d_k]
        x = x.reshape(batch_size, seq_len, self.num_heads, self.d_k)
        # 转置为: [batch_size, num_heads, seq_len, d_k]
        return x.transpose(0, 2, 1, 3)

    def combine_heads(self, x, batch_size, seq_len):
        """将多个头合并"""
        # x: [batch_size, num_heads, seq_len, d_v]
        # 转置回: [batch_size, seq_len, num_heads, d_v]
        x = x.transpose(0, 2, 1, 3)
        # 重塑为: [batch_size, seq_len, d_model]
        return x.reshape(batch_size, seq_len, self.d_model)

    def scaled_dot_product_attention(self, Q, K, V, mask=None):
        """缩放点积注意力机制"""
        # Q, K, V: [batch_size, num_heads, seq_len, d_k]

        # 计算注意力分数: Q * K^T / sqrt(d_k)
        scores = np.matmul(Q, K.transpose(0, 1, 3, 2)) / np.sqrt(self.d_k)
        # scores: [batch_size, num_heads, seq_len, seq_len]

        # 应用mask（如果有）
        if mask is not None:
            scores = scores + (mask * -1e9)  # 将mask位置设为负无穷

        # 计算注意力权重
        attention_weights = self.softmax(scores, axis=-1)
        # attention_weights: [batch_size, num_heads, seq_len, seq_len]

        # 应用Dropout到注意力权重
        attention_weights_dropout = self.dropout(attention_weights, training=True)

        # 计算输出: 注意力权重 * V
        output = np.matmul(attention_weights_dropout, V)
        # output: [batch_size, num_heads, seq_len, d_v]

        return output, attention_weights

    def forward(self, X, mask=None, training=True):
        """
         前向传播
        Q, K, V: [batch_size, seq_len, d_model]
        mask: [batch_size, 1, seq_len, seq_len] 或 None
        """
        batch_size, seq_len, _ = X.shape

        # 线性变换生成Q, K, V
        Q_linear = np.dot(X, self.W_Q) # + self.b_Q
        K_linear = np.dot(X, self.W_K) # + self.b_K
        V_linear = np.dot(X, self.W_V) # + self.b_V

        # 分割成多个头
        Q_heads = self.split_heads(Q_linear, batch_size, seq_len)
        K_heads = self.split_heads(K_linear, batch_size, seq_len)
        V_heads = self.split_heads(V_linear, batch_size, seq_len)

        # 计算缩放点积注意力
        attn_output, attention_weights = self.scaled_dot_product_attention(
            Q_heads, K_heads, V_heads, mask,
        )

        # 合并多头输出
        combined = self.combine_heads(attn_output, batch_size, seq_len)

        # 输出投影
        output = np.dot(combined, self.W_O) # + self.b_O

        # 应用Dropout到最终输出
        output = self.dropout(output, training)

        # 缓存中间结果用于反向传播
        self.cache = {
            'X': X,
            #'Q': Q,
            #'K': K,
            #'V': V,
            'Q_linear': Q_linear,
            'K_linear': K_linear,
            'V_linear': V_linear,
            'Q_heads': Q_heads,
            'K_heads': K_heads,
            'V_heads': V_heads,
            'attention_weights': attention_weights,
            'combined': combined,
            'batch_size': batch_size,
            'seq_len': seq_len,
            'mask': mask,
        }

        #return output, attention_weights
        return output

    def backward(self, d_output):
        """反向传播"""
        cache = self.cache
        batch_size = cache['batch_size']
        seq_len = cache['seq_len']

        # 输出投影层的梯度
        d_combined = np.dot(d_output, self.W_O.T)

        d_W_O = np.dot(
            cache['combined'].transpose(0, 1, 2).reshape(-1, self.d_model).T, 
            d_output.reshape(-1, self.d_model),
        )

        d_b_O = np.sum(d_output, axis=(0, 1))

        # 分割d_combined为多头梯度
        d_attn_output = d_combined.reshape(batch_size, seq_len, self.num_heads, self.d_v)
        d_attn_output = d_attn_output.transpose(0, 2, 1, 3)

        # 注意力机制的反向传播（简化版）
        # 这里需要实现完整的注意力反向传播，但比较复杂
        # 简化处理：直接传递梯度
        d_V_heads = d_attn_output
        d_attention_weights = np.zeros_like(cache['attention_weights'])

        # 合并头的反向传播
        d_V_linear = self.combine_heads(d_V_heads, batch_size, seq_len)

        # 线性变换的反向传播
        d_V = np.dot(d_V_linear, self.W_V.T)
        d_W_V = np.dot(cache['V'].transpose(0, 1, 2).reshape(-1, self.d_model).T, 
                      d_V_linear.reshape(-1, self.d_model))
        d_b_V = np.sum(d_V_linear, axis=(0, 1))

        # 类似处理Q和K（这里简化）
        d_Q = np.zeros_like(cache['Q'])
        d_K = np.zeros_like(cache['K'])
        d_W_Q = np.zeros_like(self.W_Q)
        d_W_K = np.zeros_like(self.W_K)
        d_b_Q = np.zeros_like(self.b_Q)
        d_b_K = np.zeros_like(self.b_K)

        return d_Q, d_K, d_V, d_W_Q, d_W_K, d_W_V, d_W_O, d_b_Q, d_b_K, d_b_V, d_b_O
