# Generatvie Pre-trained Transformer

#### 1. Tokenizer
- token_ids = tokenizer.encode(input_text)
- shape = (B, num_tokens, d_model)


#### 2. Emedding layer
- x = tok_emb(token_ids) + pos_emb(len(token_ids))
- nn.Embedding(vocab_size, d_model) +  nn.Embedding(context_length, d_model)
- x = dropout(x)
- shape = (B, T, d_out), d_out = d_model


#### 3. Layers(blocks)
##### 3.1 multi-head attention
- queries = (x @ Q).view(B, T, num_heads, head_dim).transpose(1, 2)
- keys = (x @ K).view(B, T, num_heads, head_dim).transpose(1, 2)
- values = (x @ V).view(B, T, num_heads, head_dim).transpose(1, 2)
- (B, T, d_out) @ (d_out, d_out) => (B, T, d_out)
- (B, T, d_out) => (B, T, n_heads, d_head), d_out = n_heads * d_head
- (B, T, n_heads, d_head) => (B, num_heads, T, head_dim)
- shape = (B, num_heads, T, head_dim)

- attn_scores = queries @ keys.transpose(2, 3)
- (B, num_heads, T, head_dim) @ (B, num_heads, head_dim, T) => (B, num_heads, T, T)
- attn_scores = masked_fill(attn_scores)
- attn_weights = softmax(attn_scores / keys.shape[-1]**0.5, dim=-1)
- attn_weights = dropout(attn_weights)
- shape = (B, num_heads, T, num_tokens)

- context_vec = (attn_weights @ values).transpose(1, 2)
- (B, num_heads, T, T) @ (B, num_heads, T, head_dim) => (B, num_heads, T, head_dim)
- (B, num_heads, T, head_dim) => (B, T, num_heads, head_dim)
- shape = (B, num_tokens, num_heads, d_head)

- context_vec = context_vec.contiguous().view(B, T, d_out)
- (B, T, num_heads, head_dim) => (B, T, d_out)
- context_vec = projection(context_vec)
- (B, T, d_out) @ (d_out, d_out) => (B, T, d_out)
- shape = (B, T, d_out)

##### 3.2 block
- shortcut = x
- x = norm1(x), mean=0.0, std=1.0
- x = att(x)
- x = dropout(x)
- x = shortcut + x

- shortcut = x
- x = norm2(x), mean=0.0, std=1.0
- x = ff(x)
- linear(d_model, 4 * d_model), GELU, linear(4 * d_model, d_model)
- x = dropout(x)
- x = shortcut + x


#### 4. Output layer
- x = final_norm(x), mean=0.0, std=1.0
- logits = x @ out_head
- (B, T, d_out) @ (d_model, vocab_size) => (B, T, vocab_size)

- logits = logits[:, -1, :]
- (B, T, vocab_size) => (B, vocab_size)

- idx_next = argmax(logits, dim=-1, keepdim=True)
- (B, vocab_size) => (B, 1)
- tokenizer.decode(idx_next.squeeze(0).tolist())


#### 5. 
torch.set_printoptions(sci_mode=False)
total_params = sum(p.numel() for p in model.parameters())

total_params_gpt2 = total_params - sum(p.numel() for p in model.out_head.parameters())

total_size_mb = total_params * 4 / (1024 * 1024) # 4 bytes


// GPT-2 medium
embed_dim = 1024
num_blocks = 24
num_heads = 16

// GPT-2 large
embed_dim = 1280
num_blocks = 36
num_heads = 20

// GPT-2 XL
embed_dim = 1600
num_blocks = 48
num_heads = 25
