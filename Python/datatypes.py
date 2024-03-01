class Test:
    def f1():
        print('testing')

t=Test    #creating an object for the class test.
t.f1()    #declaring the object and running the function.

# data types:
#   int, float, bool, str, complex
#   list, tuple, set, frozenset, dict
#   bytes, bytearray, range, array


# decimal form : (base-10)
# binary form : (base-2)
# octal form : (base-8)
# hexadecimal form : (base-16)

# base conversion functions :
bin(15)
bin(0o123)
bin(0xFace)

oct(100)
oct(0xFace)

hex(1000)
hex(0o123456)

# float data type
f = 1.2e3
print(f) # 1.2e3 --> 1.2 times 10^3

# complex data type
x=10+20j # j is complex
x.real # to display real number
x.imag # to display imaginary number

# boolean data type
n = True
h = False

# str data type
e= 'word'
print(e[0]) # to print specific index number
print(e[-1])

# slice operator
s = 'acdefghijklmnopqrstuvwxyz'
print(s[3:9]) # returns from 3 to 8 index. (n-1)
print(s[0].upper)+s[1:] # returns capitalised value/s and the remaining values normally.

#operators
a = 4
b = 2
print('Addition : ', a + b)
print('Subtraction : ', a - b)
print('Multiplication : ', a * b)
print('Division (float) : ', a / b)
print('Division (floor) : ', a // b)
print('Modulus : ', a % b)
print('Exponent : ', a ** b)