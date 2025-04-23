FlagsFunc proc
    mov eax, 3
    and eax, eax

    mov eax, 7
    and eax, eax

    ret

FlagsFunc endp
end

; Auxiliary Carry Flag (AC)
ACFunc proc
    mov eax, 115
    add eax, 1

    ret
ACFunc endp
end

; Zero Flag (ZF)
ZFFunc proc
    cmp eax, ecx ;compare
    je SomeLabel ;jump if equal
ZFFunc endp
end

; Sign Flag (SF)
SFFunc proc
    mov eax, 7
    mov ecx, 12

    sub eax, ecx
    add eax, ecx

    ret
SFFunc endp
end

; Direction Flag
DFunc proc
    std ; set Direction
    cld ; clear Direction
    std

    ret
DFunc endp
end

; Overflow Flag (OF)
OFunc proc
    mov al, 255
    shr al, 1
    add al, 1

    ret
OFunc endp
end

; Push and Pop
ppFunc proc
    pushfq
    pop rax

    push rax
    popfq

    ret
ppFunc endp
end
