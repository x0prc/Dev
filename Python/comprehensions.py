numbers = [1,2,3,4,5,6,7,8,9]
new_list = []
for num in numbers: new_list.append(num*num)
print(new_list)

new_list = filter(lambda num : num % 2 == 0, numbers)
print(list(new_list))

new_list = [(letter,num) for letter in 'spam' for num in range(4)]
print(new_list)

#dictonary comprehensions
movies = ["And Now for Something Completely Different","Monty Python and the Holy Grail",
"Monty Python's Life of Brian","Monty Python Live at the Hollywood Bowl","Monty Python's The Meaning of Life","Monty Python Live (Mostly)"]
year =[1971,1975,1979,1982,1983,2014]
names = ['John','Eric','Michael','Graham','Terry','TerryG']

print(list(zip(movies, year)))
new_dict = dict()
# with comprehension method : 
new_dict = {movie:yr for  movie, yr in zip(movies, year) if yr < 1983 and movie.startswith('Monty')}
print(new_dict)
n1 =[(name,movie,yr) for name,movie,yr in zip(names,movies,year) if yr < 1981]
print(n1)