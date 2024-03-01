# float to int
int(10.989) 

#bool to int
int(True)
int(False)

#str to int (works only for base-10)
int(0B1111) 

a = 10
b = 23
print(a is b) # will return false

a = True
b = True
print(a is b ) # will return true

#operators and identities
a=7
b=3
print('a == b is', a == b)
print('a != b is', a != b)
print('a > b is', a > b)
print('a < b is', a < b)
print('a >= b is', a >= b)
print('a <= b is', a <= b)
print('o in John is ','o' in 'John') #membership
print('o in John is ','o' not in 'John') #non membership
print('John is John ','John' is 'John') #identity
print('John is not John is ','John' is not 'John') # negative identity