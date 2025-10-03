#!/usr/bin/env python3
import numpy as np

from layer_norm import LayerNorm
from feed_forward_network import FFN
from transformer_encoder_layer import TransformerEncoderLayer

# 参数设置
d_model = 512
num_heads = 8
d_ff = 2048
batch_size = 2
seq_len = 10

# 初始化
layer_norm = LayerNorm(d_model)
ffn = FFN(d_model, d_ff)
encoder_layer = TransformerEncoderLayer(d_model, num_heads, d_ff)

# 创建输入数据
x = np.random.randn(batch_size, seq_len, d_model)

# 前向传播
print("Input.shape: ", x.shape)

# 测试LayerNorm
norm_output = layer_norm.forward(x)
print("LayerNorm.shape: ", norm_output.shape)

# 测试FFN
ffn_output = ffn.forward(x, training=True)
print("FFN.shape: ", ffn_output.shape)

# 测试完整Encoder Layer
encoder_output = encoder_layer.forward(x, training=True)
print("EncoderLayer.shape: ", encoder_output.shape)

# 验证输出维度与输入一致
assert encoder_output.shape == x.shape, f"encoder_output.shape={encoder_output.shape}, x.shape={x.shape}"
