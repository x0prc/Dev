[10,20,30] #list
{10,20,30} #set
(10,20,30) #tuple
{100:'word', 200:'drow'} #dict

#list (duplicates are allowed, order is important)
l = [10,20,10,40]
print(type(l))
l.append(10)
l.append(20)
print(l) # adds the elements in the same order as written.

#tuple (duplicates are allowed, order is important, no changes are allowed)
t=(10,20,30,40)
print(type(t))
print(t[0])

#set (duplicates are not allowed, order is not important)
s={10,20,'word',30}
print(s) #index concept not applicable
s.add(50) #works instead of append.
s.remove(10) 

#frozenset
s=set(10,20,30) #python prioritizes dictionaries by default
fs=frozenset(s) #immutable

#dictionaries
d={100:'word', 200:'another', 300:'yet'}
print(d)

#range
r = range(10,20,-5) # (begin, end, increment/decrement)

#bytes  (works only from 0 to 255 values, immutable)
b = [10,20,30,40]
by = bytes(l)
print(type(by))

#bytearray
ba = bytearray(b)
print(b[0]) 
print(b[-1])

#none
a = None
print(id(a))
