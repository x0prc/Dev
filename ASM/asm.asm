.data
myByte db 78
myWord dw 1234

.code
SomeFunction proc
    mov eax, real4 ptr [myFloat]
    ret
SomeFunction endp
endp
