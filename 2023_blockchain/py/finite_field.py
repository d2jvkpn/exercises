class FinitField:
    def __init__(self, num: int, prime: int) :
        self.num, self.prime = num, prime

    def __repr__(self):
        return f"FinitField(num={self.num}, prime={self.prime})"


ff = FinitField(6, 7)
print(ff)
