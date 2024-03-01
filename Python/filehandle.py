# reading files
big_chungus = open('greeting.txt', 'r')
print(big_chungus.read(10))
print(big_chungus.readline())

line_by_line = big_chungus.readlines() # OR
line_by_line1 = big_chungus.read().splitlines() 
big_chungus.close()

with open ('friends.csv', 'r') as f:
    print(f.read())
    friends = f.read().splitlines()
    print(friends)