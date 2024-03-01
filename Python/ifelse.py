#short hand if else statement
i = 10
print(True) if i < 15 else print(False)

#nested statements
i = 10
if (i == 10):
    if (i < 15):
        print("i is smaller than 15")
    if (i < 12):
        print("i is smaller than 12 too")
    else:
        print("i is greater than 15")
        
#if-elif-else statement        
i = 20
if (i == 10):
    print("i is 10")
elif (i == 15):
    print("i is 15")
elif (i == 20):
    print("i is 20")
else:
    print("i is not present")

# chaining comparison operators
x = 5
print(1 < x < 10)
print(10 < x < 20 )
print(x < 10 < x*10 < 100)
print(10 > x <= 9)
print(5 == x > 4)

x,y,z = 5,10,15
if x < y or y < z and z < x: print("we be gaming")

# using and, and not
is_raining = True
is_cold = True
print("Good Morning")
if is_raining and is_cold:
    print("Bring Umbrella and jacket")
elif is_raining and not(is_cold):
    print("Bring Umbrella")
elif not(is_raining) and is_cold:
    print("Bring Jacket")
else:
    print("nice shirt bro")