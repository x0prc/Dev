.data
    bittyye db 7
    qwral db 56

TestFunction proc

    mov rax, 89
    mov cx, ax

    mov bittyye, cl
    mov al, bittyye

    ret

TestFunction endp


# LEA Load Effective Address
lea rax, myBte

mov byte ptr [rax], 7
