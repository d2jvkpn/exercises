def sieve_of_eratosthenes(limit):
    # 创建一个布尔数组，表示从2到limit的所有数字，默认全部标记为True
    primes = [True] * (limit + 1)
    
    # 0和1不是质数，将其标记为False
    primes[0] = primes[1] = False
    
    # 从2开始进行筛选
    p = 2
    while p * p <= limit:
        if primes[p]:
            # 如果p是质数，则将其倍数标记为False
            for i in range(p * p, limit + 1, p):
                primes[i] = False
        p += 1
    
    # 收集所有标记为True的数字，即质数
    prime_numbers = []
    for num in range(2, limit + 1):
        if primes[num]:
            prime_numbers.append(num)
    
    return prime_numbers

# 示例用法
limit = 1000
primes = sieve_of_eratosthenes(limit)
print(primes)
