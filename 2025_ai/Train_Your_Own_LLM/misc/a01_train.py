#!/usr/bin/env python3
optimizer = torch.optim.AdamW(model.parameters(), lr=5e-4)

scheduler = torch.optim.lr_scheduler.CosineAnnealingLR(
    optimizer, 
    T_max=num_training_steps,
)

for step, (x_batch, y_batch) in enumerate(dataloader):
    logits, loss = model(x_batch, y_batch)
    optimizer.zero_grad(set_to_none=True)
    loss.backward()
    optimizer.step()
    scheduler.step()

    loss_val = loss.item()

for epoch in range(num_epochs):
    for x_batch, y_batch in dataloader:
        logits, loss = model(x_batch, y_batch)
        optimizer.zero_grad(set_to_none=True)
        loss.backward()
        optimizer.step()

    scheduler.step()
    # lr = scheduler.get_last_lr()[0]

# gradient clipping
torch.nn.utils.clip_grad_norm_(model.parameters(), max_norm=1.0)
