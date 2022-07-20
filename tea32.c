#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>

const uint32_t delta=0x9e3779b9;                       // a key schedule constant
uint32_t KEY[4]; // Key space for bit shifts

void set_key(const uint32_t *k) {
    KEY[0] = k[0];
    KEY[1] = k[1];
    KEY[2] = k[2];
    KEY[3] = k[3];
}

void encrypt(uint32_t* v) {
    uint32_t v0=v[0], v1=v[1], sum=0, i;             // set up
    for (i=0; i < 32; i++) {                         // basic cycle start
        sum += delta;
        v0 += ((v1<<4) + KEY[0]) ^ (v1 + sum) ^ ((v1>>5) + KEY[1]);
        v1 += ((v0<<4) + KEY[2]) ^ (v0 + sum) ^ ((v0>>5) + KEY[3]);
    }                                                // end cycle
    v[0] = v0;
    v[1] = v1;
}

void decrypt (uint32_t* v) {
    uint32_t v0=v[0], v1=v[1], sum=0xC6EF3720, i;  // set up
    for (i=0; i<32; i++) {                         // basic cycle start
        v1 -= ((v0<<4) + KEY[2]) ^ (v0 + sum) ^ ((v0>>5) + KEY[3]);
        v0 -= ((v1<<4) + KEY[0]) ^ (v1 + sum) ^ ((v1>>5) + KEY[1]);
        sum -= delta;
    }                                              // end cycle
    v[0]=v0; v[1]=v1;
}
