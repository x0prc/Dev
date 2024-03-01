my_list = [1,5,3,7,2]
my_dict = {'car':4,'dog':2,'add':3,'bee':1}
my_tuple = ('d','c','e','a','b')
my_string = 'python'

print(my_dict,'original')
print(sorted(my_dict.values(), reverse=True)) #reverses the values of the index.
print(my_dict,'new') #display 


my_llist=[['car',4,65],['dog',2,30],['add',3,10],['bee',1,24]]
print(sorted(my_llist, key = lambda item :item[1])) # lambda as a throw away function. sorts according to the array index mentioned.
print(sorted(my_list, key = abs)) # sorts in absolute key order 

