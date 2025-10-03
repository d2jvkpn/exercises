#!/usr/bin/env python3
import numpy as np

from layer_norm import LayerNorm
from multi_head_attention import MultiHeadAttention
from feed_forward_network import FFN

class TransformerEncoderLayer:
    def __init__(self, d_model, num_heads, d_ff, dropout=0.1):
        self.d_model = d_model
        self.num_heads = num_heads
        self.d_ff = d_ff

        # 初始化各组件 (这里简化了MultiHeadAttention的实现)
        self.norm1 = LayerNorm(d_model)

        # 假设我们已经有一个MultiHeadAttention的实现
        self.self_attn = MultiHeadAttention(d_model, num_heads, dropout)

        self.ffn = FFN(d_model, d_ff, dropout)

        self.norm2 = LayerNorm(d_model)


    def forward(self, x, training=True):
        """
        x: [batch_size, seq_len, d_model]
        """
        # 保存残差连接的输入
        residual1 = x.copy()

        # Self-Attention子层 (这里简化，实际应该有完整的注意力计算)
        attn_output = self.self_attn.forward(x, training=training)
        #attn_output = x  # 这里用identity代替，实际应该是注意力输出

        # 残差连接 + LayerNorm
        x = self.norm1.forward(residual1 + attn_output)

        # 保存第二个残差连接的输入
        residual2 = x.copy()

        # FFN子层
        ffn_output = self.ffn.forward(x, training=training)

        # 残差连接 + LayerNorm
        output = self.norm2.forward(residual2 + ffn_output)

        return output
