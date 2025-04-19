.data
myQWordVar sqword 0

.code
AddSubTestFunction proc
    xor rcx, rcx
    mov rax, 2147483648
    ADD RCX, rax
    mov myQWordVar, RCX
    ret
AddSubTestFunction endp
end

# inc is increment
AddSubTestFunction2 proc
    mov al, 255
    inc al
    ret
AddSubTestFunction endp
end

# dec is decrement
DecTestFunction proc
    mov al, 255
    dec al
    ret
DecTestFunction endp
end
