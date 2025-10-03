#!/usr/bin/env python3
import numpy as np

class LayerNorm:
    def __init__(self, normalized_shape, eps=1e-5):
        self.normalized_shape = normalized_shape
        self.eps = eps
        # 可学习参数
        self.gamma = np.ones(normalized_shape)  # 缩放参数
        self.beta = np.zeros(normalized_shape)  # 平移参数
        # 存储中间结果用于反向传播
        self.cache = None

    def forward(self, x):
        """
        x: [batch_size, seq_len, d_model] 或 [batch_size, d_model]
        """
        # 计算均值和方差
        mean = np.mean(x, axis=-1, keepdims=True)
        variance = np.var(x, axis=-1, keepdims=True)

        # 归一化
        x_normalized = (x - mean) / np.sqrt(variance + self.eps)
        
        # 缩放和平移
        output = self.gamma * x_normalized + self.beta

        # 存储中间结果用于反向传播
        self.cache = (x, mean, variance, x_normalized)
        
        return output

    def backward(self, dout):
        """
        dout: 上游梯度
        """
        x, mean, variance, x_normalized = self.cache
        batch_size = x.shape[0]

        # 计算gamma和beta的梯度
        dgamma = np.sum(dout * x_normalized, axis=0)
        dbeta = np.sum(dout, axis=0)

        # 计算输入x的梯度
        dx_normalized = dout * self.gamma

        dvariance = np.sum(
            dx_normalized * (x - mean) * - 0.5 * (variance + self.eps)**(-1.5),
            axis=-1,
            keepdims=True,
        )

        a = np.sum(
            dx_normalized * -1 / np.sqrt(variance + self.eps),
            axis=-1,
            keepdims=True,
        )

        b = dvariance * np.sum(-2 * (x - mean), axis=-1, keepdims=True) / x.shape[-1]

        dmean =  a + b

        dx = dx_normalized / np.sqrt(variance + self.eps) + \
             dvariance * 2 * (x - mean) / x.shape[-1] + \
             dmean / x.shape[-1]

        return dx, dgamma, dbeta
