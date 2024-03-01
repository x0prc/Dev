# enumerate goes into the list, modifies the functions into tuples and executes them.
friends = ['Brian', 'Judith', 'Reg', 'Loretta', 'Colin']

i = 50
for num, friend in enumerate(friends, 50):
    print(num, friend)

for friend in enumerate(enumerate(friends,51),-100): # goes in reverse order
    print(friend)   

for num, letter in enumerate('python',start = 5):
    print(num, letter)
    
print(list(enumerate(friends))) # working of the enumerate function as in line 1
    