# Finity Fields

####
p = 19
k = 2

ef1 = [ i for i in range(0, p) ]
ans1 = [ i%p for i in ef1 ]
ans1.sort()
print(ans1)

ef2 = [ i*k for i in ef1  ]
ans2 = [ i%p for i in ef2 ]
ans2.sort()
print(ans2)

print(ans1 == ef1, ans2 == ef1)

####
p = 6
ef3 = [ i for i in range(0, p) ]
ans3 = [ i%p for i in ef3 ]
ans3.sort()
print(ans3)

ef4 = [ i*k for i in ef3  ]
ans4 = [ i%p for i in ef4 ]
ans4.sort()
ans4 = list(set(ans4))
print(ans4)

print(ef3 == ans4)
