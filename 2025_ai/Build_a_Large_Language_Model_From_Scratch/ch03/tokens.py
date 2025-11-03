#!/usr/bin/env python3
import torch

t = torch.tensor(token_ids, dtype=torch.long)               # 1D: [L]
w = t.unfold(dimension=0, size=max_length + 1, step=stride) # [N, max_length+1]

input_ids  = w[..., :max_length]   # [N, max_length]  视图
target_ids = w[..., 1:]            # [N, max_length]  视图（右移一位）

# 如果后续需要真实连续内存（比如要送到某些 kernel），再按需 materialize：
# input_ids  = input_ids.contiguous()
# target_ids = target_ids.contiguous()

# 如果你确实需要原来那种“list 里装 1D Tensor”的结构：
# input_list  = list(torch.unbind(input_ids,  dim=0))
# target_list = list(torch.unbind(target_ids, dim=0))
