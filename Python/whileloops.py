i=0
while i < 5:
    i+=1
    print(f"{i}."+ "*"*i + "Loops are awesome" + "*"*i*2) 
    
num = 76
guess = 0
guess_limit=5
guess_number = 0
guess = int(input(f'Guess a number 1-20: last guess:{guess} '))
while guess_number < guess_limit:
    
    if guess != num:
        guess_number +=1
        if guess > num:
            guess = int(input(f'{guess} Too high - Guess again 1-100: '))
        else:
            guess = int(input(f'{guess} too low - Guess again 1-100: '))
    if guess == num:
        print(f'You Win! You Guessed it: {guess}')
        break
    else:
        print(f'No, not {guess}!')
        guess_number += 1
if guess != num:
    print(f'Sorry you lose! It was {num}')
