import random as raand, string as stri
for i in range(100): print(raand.raand())
print(raand.uniform(1,6))
print(raand.randint(1,4))
print(raand.randrange(1,4,6))

friends_list = ['i', 'dont', 'have', 'any']
print(raand.choice(friends_list))
print(raand.sample(friends_list, 3))
print(raand.shuffle(friends_list)) 

smallcaps = 'abcdefghijklmnopqrstuvwxyz'
largecaps = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'
digits = '0123456789'
letters_numbers = stri.ascii_letters + stri.digits

word = ''
for i in range(7): word += raand.choice(letters_numbers)
print(word)
