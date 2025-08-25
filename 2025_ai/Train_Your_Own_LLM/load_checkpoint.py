#!/usr/bin/env python3
from pathlib import Path

import torch


checkpoint_dir = Path("data") / "ch03" / "checkpoints"

last_pt = sorted(
    checkpoint_dir.glob("checkpoint_*.pt"),
    key=lambda v: int(v.name.replace("checkpoint_", "").replace(".pt", "")),
    reverse=True,
)[0]

last_ckpt = torch.load(last_pt)
