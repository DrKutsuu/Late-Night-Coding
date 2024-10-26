    processor 6502
    org $1000 # load into address 4096
;https://stackoverflow.com/questions/43498617/ascii-to-c64-screen-codes-in-dasm-assembler
;Original by nexus_2006
    ldx #$00    ;x=0
printloop:
    lda hexmsg,x;load bit x from hexmsg byte
    sta $0400,x ;store this bit in row 0 col 0 address
    inx         ;x++
    cpx #$0B    ;Compare X to Hex 11
    bne printloop   ;if (not equal) loop again
    rts         ;else return from program
;define a series of Hex bytes for our message
hexmsg .byte $08,$05,$0c,$0c,$0f,$20,$17,$0F,$12,$0C,$04
