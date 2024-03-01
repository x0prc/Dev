#iterating over a string 
s = "Gamers" 
for i in s: print(i)

#loop with step size
for i in range(0, 10, 2): print(i)

#loop in loop
for i in range(1, 4): 
    for j in range(1, 4): print(i, j)


#                     LOOP CONTROL
# for with continue
for letter in 'gamerwords':
    if letter == 'e' or letter == 's':
        continue
    print('Current Letter :', letter)
    
# for with break
for letter in 'gamerwords':
    if letter == 'e' or letter == 's':
        break
    print('Current Letter :', letter)

# for with pass
for letter in 'gamerwords':
    pass
print('Last Letter :', letter)

# nesting with for loops

for letter in 'blue': print(letter) 
print('for loops')

#using furgle
for furgle in range (1,15,3):
    print(furgle)

friends = ['John','Terry','Eric','Michael','George']
for index in range(len(friends)):
    print(friends[index])

# exercise
names = ['john ClEEse','Eric IDLE','michael']
names1 = ['graHam chapman', 'TERRY', 'terry jones']
msg = 'You are invited to the party on saturday.'
names.extend(names1)

for index in range(2):
    names.append(input('enter a new name:'))
    
for name in names: 
    msg1 = name.title() + '! ' + msg    
    print(msg1)
    
