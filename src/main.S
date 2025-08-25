
.equ PINB,  0x03
.equ DDRB,  0x04
.equ PORTB, 0x05
.equ PIND,  0x09 
.equ DDRD,  0x0A 
.equ PORTD, 0x0B 


.equ PB0, 0x01 ; 00000001
.equ PB1, 0x02 ; 00000010
.equ PB2, 0x04 ; 00000100
.equ PB3, 0x08 ; 00001000
.equ PB4, 0x10 ; 00010000
.equ PB5, 0x20 ; 00100000
.equ PB6, 0x40 ; 01000000
.equ PB7, 0x80 ; 10000000

.equ PD0, 0x01 ; 00000001
.equ PD1, 0x02 ; 00000010
.equ PD2, 0x04 ; 00000100
.equ PD3, 0x08 ; 00001000
.equ PD4, 0x10 ; 00010000
.equ PD5, 0x20 ; 00100000
.equ PD6, 0x40 ; 01000000
.equ PD7, 0x80 ; 10000000

.extern delay

.global setupasm
.global loopasm

setupasm:
    sbi DDRD, 7
    cbi PORTD, 7
    ret

loopasm:
    ret