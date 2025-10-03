#!/usr/bin/env python3

weight = 0.1
lr = 0.01 # learning rate

def neural_network(data, weight):
    pred = data * weight
    return pred


number_of_toes = [8.5]
win_or_loss_binary = [1] # (won!!!)

d = number_of_toes[0]
t = win_or_loss_binary[0]

#### 1.
p = neural_network(d, weight)
error = (p - t) ** 2

print("--> 2. raw_error={:.6f}".format(error))

#### 2.
p = neural_network(d, weight + lr)
err_up = (p - t) ** 2

print("--> 3. err_up={:.6f}".format(err_up))

#### 3.
lr = 0.01
p = neural_network(d, weight - lr)
err_dn = (p - t) ** 2

print("--> 4. err_dn={:.6f}".format(err_dn))

#### 4.
print("~~~ raw weight: {}".format(weight))

if (error > err_dn or error > err_up):
    if (err_dn < err_up):
        weight -= lr
    elif (err_dn > err_up):
        weight += lr

print("~~~ updated weight: {}".format(weight))
p = neural_network(d, weight)
error = (p - t) ** 2

print("--> 5. error_updated={:.6f}".format(error))
