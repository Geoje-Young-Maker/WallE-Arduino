
extern void delayms(unsigned int ms);

extern void setupasm();
extern void loopasm();

void delay(unsigned int ms) {
    delayms(ms);
}

void setupcpp() {
    setupasm();
}

void loopcpp() {
    loopasm();
}
