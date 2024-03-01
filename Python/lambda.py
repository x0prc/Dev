square1 = lambda x: x*x # use once instead of def and throw away functions
print(square1(13123123131313131314434355666677722))

double_mult = lambda x,y: 2*x*y
print(double_mult(2,3))

#def name_and_alias(name,alias): return name.strip().title() + ':' + alias.strip().title() 
name_and_alias1 = lambda name, alias:name.strip().title() + ':' + alias.strip().title()
print(name_and_alias1('joe who ', ' baller'))

def price_calc(start,hourly_rate):
    return lambda hours: start + hourly_rate * hours
    
walkin_cost = price_calc(10,30)
premium_cost = price_calc(1,25)
print(walkin_cost(2))
print(premium_cost(2))
print(price_calc(1,25)(2))

print((lambda a,b,c: a+b+c)(2,3,4)) # positional arguements 
print((lambda a,b,c=2: a+b+c)(2,3)) # use default values putting them on the last.
print((lambda a,b,c=2: a+b+c)(2,c=3,b=4)) # specify your arguements
print((lambda *args: sum(args))(2,3,4,50)) # unpacking (sum of arguements in the order mentioned)