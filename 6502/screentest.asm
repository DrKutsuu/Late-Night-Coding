; Darren Joseph 6502 screentest
corners:
	LDA #$05
	STA $0200
	LDA #$05
	STA $021f
	LDA #$05
	STA $05ff
	LDA #$05
	STA $05e0
ok2:
 jmp corners
