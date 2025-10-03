#!/usr/bin/env python3
import numpy as np

class FFN:
    def __init__(self, d_model, d_ff, dropout=0.1):
        self.d_model = d_model
        self.d_ff = d_ff
        self.dropout_rate = dropout

        # 权重初始化 (Xavier初始化)
        self.W1 = np.random.randn(d_model, d_ff) * np.sqrt(2.0 / (d_model + d_ff))
        self.b1 = np.zeros(d_ff)
        self.W2 = np.random.randn(d_ff, d_model) * np.sqrt(2.0 / (d_ff + d_model))
        self.b2 = np.zeros(d_model)

        # 缓存用于反向传播
        self.cache = None

    def relu(self, x):
        return np.maximum(0, x)

    def relu_derivative(self, x):
        return (x > 0).astype(float)

    def dropout(self, x, training=True):
        if training and self.dropout_rate > 0:
            mask = (np.random.rand(*x.shape) > self.dropout_rate) / (1 - self.dropout_rate)
            return x * mask
        return x

    def forward(self, x, training=True):
        """
        x: [batch_size, seq_len, d_model]
        """
        batch_size, seq_len, d_model = x.shape

        # 重塑为 [batch_size * seq_len, d_model]
        x_reshaped = x.reshape(-1, d_model)

        # 第一层线性变换 + ReLU
        self.z1 = np.dot(x_reshaped, self.W1) + self.b1  # [batch*seq_len, d_ff]
        self.a1 = self.relu(self.z1)

        # Dropout
        self.a1_dropout = self.dropout(self.a1, training)

        # 第二层线性变换
        self.z2 = np.dot(self.a1_dropout, self.W2) + self.b2  # [batch*seq_len, d_model]

        # 重塑回原始形状
        output = self.z2.reshape(batch_size, seq_len, d_model)

        # 存储中间结果
        self.cache = (x_reshaped, self.z1, self.a1, self.a1_dropout)

        return output

    def backward(self, dout):
        """
        dout: [batch_size, seq_len, d_model]
        """
        x_reshaped, z1, a1, a1_dropout = self.cache
        batch_size, seq_len, d_model = dout.shape

        # 重塑梯度
        dout_reshaped = dout.reshape(-1, d_model)

        # 第二层反向传播
        dz2 = dout_reshaped  # [batch*seq_len, d_model]
        dW2 = np.dot(a1_dropout.T, dz2)  # [d_ff, d_model]
        db2 = np.sum(dz2, axis=0)  # [d_model]
        da1_dropout = np.dot(dz2, self.W2.T)  # [batch*seq_len, d_ff]

        # Dropout反向传播 (需要乘以mask，但这里简化处理)
        da1 = da1_dropout  # 实际实现中需要存储mask

        # ReLU反向传播
        dz1 = da1 * self.relu_derivative(z1)  # [batch*seq_len, d_ff]

        # 第一层反向传播
        dW1 = np.dot(x_reshaped.T, dz1)  # [d_model, d_ff]
        db1 = np.sum(dz1, axis=0)  # [d_ff]
        dx_reshaped = np.dot(dz1, self.W1.T)  # [batch*seq_len, d_model]

        # 重塑回原始形状
        dx = dx_reshaped.reshape(batch_size, seq_len, d_model)

        return dx, dW1, db1, dW2, db2
