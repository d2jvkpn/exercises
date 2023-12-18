import math

def is_prime(n: int) -> bool:
    if n <= 1: return False

    for i in range(2, int(math.sqrt(n)) + 1):
        if n % i == 0: return False

    return True

def find_primes(max_num: int, limit: int) -> list[int]:
    num, primes = max_num, []

    while True:
        num -= 1
        if is_prime(num): primes.append(num)
        if len(primes) == limit or num < 2: break

    primes.reverse()
    return primes


primes = find_primes(int((2**32)**0.5), 1000)
print(primes)
