pragma circom 2.0.3;

// include "https://github.com/0xPARC/circom-secp256k1/blob/master/circuits/bigint.circom";

template Example () {
    signal input a[2];
    signal input b;
    signal input c[2][2];

    signal output a_out[2];
    signal output b_out;

    a_out[0] <== a[0] + b + c[0][0] + c[1][0];
    a_out[1] <== a[1] + b + c[0][1] + c[1][1];
    b_out <== b * 2;
}

component main { public [a, b] } = Example();
