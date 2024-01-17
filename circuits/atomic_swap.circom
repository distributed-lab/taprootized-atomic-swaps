pragma circom 2.1.1;

include "./ecdsa/ecdsa.circom";
include "../node_modules/circomlib/circuits/poseidon.circom";

template AtomicSwap() {
    signal input secret[4];

    signal output pubkey[2][4];
    signal output secretHash;

    pubkey <== ECDSAPrivToPub(64, 4)(secret);
    secretHash <== Poseidon(4)(secret);
}

component main = AtomicSwap();