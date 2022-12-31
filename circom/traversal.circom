pragma circom 2.0.3;

template Traversal(){
    signal input a;
    signal input b;
    signal input c;

    log("hello world");
    a + b === c;
    a * 2 === 6;
    b * 2 === 10;
    c * 2 === 16;
}

component main = Traversal();
