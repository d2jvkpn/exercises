a = 2
b = 2
p = 17  # 有限域上的素数

# 基点
G = (5, 1)

# 点加法函数
def point_addition(P, Q):
    if P == "O":  # P 是无穷远点
        return Q
    if Q == "O":  # Q 是无穷远点
        return P

    x1, y1 = P
    x2, y2 = Q

    if P != Q:  # P 和 Q 不相等
        # 计算斜率
        s = ((y2 - y1) * pow(x2 - x1, p - 2, p)) % p
    else:  # P 和 Q 相等
        # 计算斜率
        s = ((3 * x1 * x1 + a) * pow(2 * y1, p - 2, p)) % p

    # 计算 R 的 x 坐标
    x3 = (s * s - x1 - x2) % p

    # 计算 R 的 y 坐标
    y3 = (s * (x1 - x3) - y1) % p

    return x3, y3

# 测试点加法
P = (5, 1)
Q = (6, 3)
R = point_addition(P, Q)
print("P + Q =", R)
