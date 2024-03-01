movie = {
    'title' : 'Life of Brian',
    'year' : 1979,
    'cast' : ['John','Eric','Michael','George','Terry']
}

movie['title'] = 'The Holy Grail'
print(movie.get('title'))  #fetching title exclusively.

movie.update({'title' : 'The Holy Grail','year':1975,'cast':['John','Eric','Michael','George','Terry']}) #updating/adding titles.
movie['budget'] = 250000
del movie['year']  # deleting values.
print(movie)

print(len(movie)) # to fetch the length of values of movies.
print(movie.keys()) # to fetch specific keys from movies.
print(movie.items()) # to fetch specific items.

for key, value in movie.items(): 
    print(key, value) # prints both values and keys from movie items.



python = {'John':35,'Eric':36,'Michael':35,'Terry':38,'Graham':37,'TerryG':34}
holy_grail = {'Arthur':40,'Galahad':35,'Lancelot':39,'Knight of NI':40, 'Zoot':17}
life_of_brian = {'Brian':33,'Reg':35,'Stan/Loretta':32,'Biccus Diccus':45}
#membership test
print('Arthur' in holy_grail)
if 'Arthur' not in python: print('bitchass aint here')

people = {}
people1 = {}
people2 = {}
#method 1 update (for one dictionary)
people.update(python)
people.update(holy_grail) 
print(sorted(people))

#method 2 comprehension (for more than 1/2 dictionaries)
for groups in (python,holy_grail) : people1.update(groups)
print(sorted(people1))

#method 3 (for multiple dictionaries)
people2 = {**python,**holy_grail,**life_of_brian}
print(sorted(people2))
print('sum of ages is' , sum(people.values))

# getting groceries
freelancers = {'name':'freelancing Shop','brian': 70, 'black knight':20, 'biccus diccus':100, 'grim reaper':500, 'minstrel':-15}
antiques = {'name':'Antique Shop','french castle':400, 'wooden grail':3, 'scythe':150, 'catapult':75, 'german joke':5}
pet_shop = {'name':'Pet Shop','blue parrot':10, 'white rabbit':5, 'newt': 2}

#create an empty shopping cart
cart = {}
#loop through stores/dicts
for shop in (freelancers,antiques,pet_shop) :
    #inputbox  to show what you can buy...capture textstring of what was bought...make lowercase
    buy_item = input(f'Welcome to {shop["name"]}! what do you want to buy: {shop}').lower()
    #update the cart
    cart.update({buy_item:shop.pop(buy_item)}) # use pop...
    buy_items = ", ".join(list(cart.keys()))
print(f'You Purchased {buy_items}. Today it is all free. Have a nice day of mayhem!')