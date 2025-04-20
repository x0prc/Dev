TestFunction proc
    mov eax, 100111001010b
    mov ecx, 100111001110b

    ; &
    and eax, ecx
    not eax

    ; NAND
    and eax, eax
    not eax

    xor ecx, eax


    ret

TestFunction endp
end
