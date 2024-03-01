def greeting(name, age=28, color = 'red'):
    print(f'Hello {name.capitalize()}, you will be {age+1} next birthday!')
    print(f'We hear you like the color {color.lower()}!')

name = input('Enter your name: ') # taking input from user
age = input('Enter your age: ')
color = input('Enter favorite color: ')
greeting(name, int(age), color)
greeting(age=27, name="brian",color="Blue") # changing the arguements for the function

# return statements
def value_added_tax(amount):
    tax = amount * 0.25
    total_amount = amount * 1.25
    return f"{amount}, {tax}, {total_amount}" # returns as string (can return as set, tuple, list)

price = value_added_tax(100)
print(price, type(price))
