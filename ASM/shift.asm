; shift left
shl rdx, 1

; shift right
shr rdx, 64

; for obtaining a 0 flag
and eax, eax

; rotate left
rol rdx, 1

; rotate right
ror rdx, 1

; left rotate through carry flag
rcl rdx, 1

; right rotate through carry flag
rcr rdx, 1

; double precision
shld al, ah, 12
