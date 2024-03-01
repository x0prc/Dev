nums = [1,2,3,4] 
letters = ['a','b','c','d']
names =['John','Eric','Michael','Graham','Joe']

combo = list(zip(nums, letters, names)) # combining different lists, dicts or tuples
print(combo)

num,let,namme = zip(*combo) # unzipping

keys = 'this parrot is deceased'
values = 'denna papegojan är avliden'

keys = keys.split()
values = values.split()
print(keys,values)

en_sv_dict = dict(zip(keys,values))
print(en_sv_dict)

