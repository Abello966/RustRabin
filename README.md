# RustRabin

Implementation of the [Rabin Cryptosystem](https://en.wikipedia.org/wiki/Rabin_cryptosystem) in Rust

Works only for 64bit numbers and on the special case the private keys are both 3 mod 4

## Structure

The code is in the src directory

* euclidean.rs - implements extended euclidean algorithm and utilities

* crt.rs - implements Chinese Remainder Theorem

* expmod.rs - implements a safe version for modular exponentiation

* rabin.rs - implements Rabin encryption and decryption 

* main.rs - has an example of using the rabin cryptosystem plus a quick implementation of the ElGamal Signature using the above mentioned libs.


## Dependencies

* Rust

## Usage

cargo run will run an example with verbose mode on detailing every step of encryption and decryption
