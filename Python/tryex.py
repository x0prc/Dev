try:
    num=int(input('Enter a number between 1 and 30: '))
    num1 = 30/num
    if num > 30:
        raise ValueError(num)
except ValueError as err:
    print(err, 'bad value')
except:
    print("invalid")
else:
    print('baller')
finally:
    print('noiiiii')
