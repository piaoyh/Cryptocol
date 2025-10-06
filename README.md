# cryptocol crate provides libraries for cryptography

## Endianness

This crate is optimized for Little-endian CPUs because Little-Endian CPUs
are far more popular than Big-endian CPUs. For the information about
Endianness (including Little-endian and Big-endian)
[Read more](https://en.wikipedia.org/wiki/Endianness).

### Big-endian issue

This crate is just experimental for Big-endian CPUs. So, you are not
encouraged to use this crate for Big-endian CPUs for serious purpose.
Only use this crate for Big-endian CPUs with your own full responsibility.

## Road Map for Version 1.0

This crate Cryptocol is planned to provide the following functionalities.
The checked items have already been implemented including documentation __at least 95%__. The unchecked items have been implemented including documentation __less than 95%__ or have __not__ yet even been started to implement.

### 1. Small Numbers: meaningful as itself, and also the foundations mainly for Big Numbers as well as for other modules

- [X] Unions for primitive data types and their implementation, and the implementation
      of trait SmallUInt for the Unions --- 
      [`ShortUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/short_union/union.ShortUnion.html#union.ShortUnion),
      [`IntUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/int_union/union.IntUnion.html#union.IntUnion),
      [`LongUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/long_union/union.LongUnion.html#union.LongUnion),
      [`LongerUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/longer_union/union.LongerUnion.html#union.LongerUnion),
      [`SizeUnion`](https://docs.rs/cryptocol/latest/cryptocol/number/size_union/union.SizeUnion.html#union.SizeUnion),
      [`SharedValues`](https://docs.rs/cryptocol/latest/cryptocol/number/shared_values/union.SharedValues.html#union.SharedValues), and
      [`SharedArrays`](https://docs.rs/cryptocol/latest/cryptocol/number/shared_arrays/union.SharedArrays.html#union.SharedArrays)
- [X] Trait SmallUInt, its implementation for primitive data types, and the implementation
      of it for the Unions ---
      [`SmallUInt`](https://docs.rs/cryptocol/latest/cryptocol/number/small_uint/trait.SmallUInt.html#trait.SmallUInt)
<!--
- [ ] Trait SmallSInt and its implementation for primitive data types --- SmallSInt
      ===> Moved to Roadmap for ver. 2.0
-->

### 2. Big Numbers: meaningful as itself and also the foundation for Asymmetric-Key Algorithms

- [X] Fixed Sized Big Unsigned Integer Operation --- 
      You can find most of the methods you need for big integer calculation at
      [`BigUInt`](https://docs.rs/cryptocol/latest/cryptocol/number/big_uint/struct.BigUInt.html#struct.BigUInt).
- [X] Auxiliary Fixed Sized Big Unsigned Integer Operation --- If you don't find what you need at
      [`BigUInt`](https://docs.rs/cryptocol/latest/cryptocol/number/big_uint/struct.BigUInt.html#struct.BigUInt), you may find it at
      [`BigUInt_More`](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_more/trait.BigUInt_More.html#trait.BigUInt_More),
      [`BigUInt_Modular`](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_modular/trait.BigUInt_Modular.html#trait.BigUInt_Modular),
      [`BigUInt_Prime`](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_prime/trait.BigUInt_Prime.html#trait.BigUInt_Prime), and
      [`BigUInt_Panic_Free`](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_panic_free/trait.BigUInt_Panic_Free.html#trait.BigUInt_Panic_Free).
<!--
- [ ] Fixed Sized Big Signed Integer Operation --- BigSInt
      ===> Moved to Roadmap for ver. 2.0
- [ ] Variable Sized Big Signed Integer Operation --- LargeInt
      ===> Moved to Roadmap for ver. 2.0 or higher
-->

### 3. Hash Algorithms

#### 3-1. Official SHA-series

- [X] SHA-3 and Keccak hash algorithms based on 8/16/32/64 bits --- 
      Includes SHA3-224, SHA3-256, SHA3-384, SHA3-512, SHAKE 128, SHAKE 256, and their expanded versions.
      [`Keccak_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/keccak/struct.Keccak_Generic.html#struct.Keccak_Generic)
- [X] SHA-2 hash algorithms based on 512/t bits --- 
      Includes 512/256, SHA-512/224, and their expanded versions.
      [`SHA2_512_t_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512_t/struct.SHA2_512_t_Generic.html#struct.SHA2_512_t_Generic)
- [X] SHA-2 hash algorithms based on 512 bits --- 
      Includes SHA-512, SHA-384, SHA-512/256, and their expanded versions.
      [`SHA2_512_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic)
- [X] SHA-2 hash algorithms based on 256 bits --- 
      Includes SHA-256, SHA-224, and their expanded versions.
      [`SHA2_256_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic)

#### 3-2. Insecure Hash algorithms ONLY for educational purposes

- [X] SHA-1 hash algorithms based on 160 bits --- 
      Includes SHA-1, SHA-0, and their expanded versions.
      [`SHA1_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/sha1/struct.SHA1_Generic.html#struct.SHA1_Generic)
<!--
- [ ] RIPEMD hash algorithms based on 256 bits
      --- Includes RIPEMD and its expanded versions.
      ===> Moved to Roadmap for ver. 2.0
- [ ] BLAKE3 hash algorithms based on 256 bits
      --- Includes BLAKE3 and its expanded versions.
      ===> Moved to Roadmap for ver. 2.0
- [ ] BLAKE2 hash algorithms based on 256 bits
      --- Includes BLAKE2 and its expanded versions.
      ===> Moved to Roadmap for ver. 2.0
- [ ] MD6 hash algorithms based on 256 bits
      --- Includes MD4 and its expanded versions.
      ===> Moved to Roadmap for ver. 2.0
-->
- [X] MD5 hash algorithms based on 128 bits --- Includes MD5 and its expanded versions.
      [`MD5_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/md5/struct.MD5_Generic.html#struct.MD5_Generic)
- [X] MD4 hash algorithms based on 128 bits --- Includes MD4 and its expanded versions.
      [`MD4_Generic`](https://docs.rs/cryptocol/latest/cryptocol/hash/md4/struct.MD4_Generic.html#struct.MD4_Generic)
<!--
- [ ] MD2 hash algorithms based on 128 bits
      --- Includes MD4 and its expanded versions.
      ===> Moved to Roadmap for ver. 2.0
-->

### 4. Symmetric-key Algorithms for Block Encryption/Decryption of digital data
#### 4-1. AES Finalists
- [X] AES (Advanced Encryption Standard) and Rijndael symmetric-key encryption/decryption algorithm, and the trait implementations of Operation modes and Padding bits for Rijndael_Generic
      --- Includes AES, Rijndael and its expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
      [`Rijndael_Generic`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.Rijndael_Generic.html#struct.Rijndael_Generic),
      [`ECB_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [`ECB_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [`CBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [`CBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [`PCBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [`PCBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [`CFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [`OFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [`CTR`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
- [ ] MARS symmetric-key encryption/decryption algorithm
      --- Includes MARS and its expanded versions. `MARS_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] Serpent symmetric-key encryption/decryption algorithm
      --- Includes RC6 and its expanded versions. `RC6_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] Twofish symmetric-key encryption/decryption algorithm
      --- Includes Twofish and its expanded versions. `Twofish_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] RC6 symmetric-key encryption/decryption algorithm
      --- Includes RC6 and its expanded versions. `RC6_Generic`
      ===> Moved to Roadmap for ver. 2.0

#### South Korean Encryption/Decryption algorithms
- [ ] SEED symmetric-key encryption/decryption algorithm
      --- Includes SEED and its expanded versions. `SEED_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] HIGHT symmetric-key encryption/decryption algorithm
      --- Includes HIGHT and its expanded versions. `HIGHT_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] ARIA symmetric-key encryption/decryption algorithm
      --- Includes ARIA and its expanded versions. `ARIA_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] LEA symmetric-key encryption/decryption algorithm
      --- Includes LEA and its expanded versions. `LEA_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] IDEA symmetric-key encryption/decryption algorithm
      --- Includes IDEA and its expanded versions. `IDEA_Generic`
      ===> Moved to Roadmap for ver. 2.0

#### Miscellaneous Encryption/Decryption algorithms
- [ ] Bluefish symmetric-key encryption/decryption algorithm
      --- Includes Bluefish and its expanded versions. `Bluefish_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] RC5 symmetric-key encryption/decryption algorithm
      --- Includes RC5 and its expanded versions. `RC5_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->
#### 4-2. Insecure Block Encryption/Decryption algorithms ONLY for educational purposes
<!--
- [ ] RC2 symmetric-key encryption/decryption algorithm
      --- Includes RC2 and its expanded versions. `RC2_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->
- [X] DES (Data Encryption Standard) symmetric-key encryption/decryption algorithm, and the traits and its implementations of Operation modes and Padding bits for DES_Generic
      --- Includes DES and its expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
      [`DES_Generic`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.DES_Generic.html#struct.DES_Generic),
      [`ECB_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [`ECB_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [`CBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [`CBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [`PCBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [`PCBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [`CFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [`OFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [`CTR`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
- [ ] Lucifer symmetric-key encryption/decryption algorithm
      --- Includes Lucifer and its expanded versions. `Lucifer_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->
#### 4-3. Containers for combining encryption/decryption algorithms
- [X] BigCryptor128 combinations of symmetric-key encryption/decryption algorithms, and the trait implementations of Operation modes and Padding bits for BigCryptor128
      --- Includes 2AES, 3AES, 4AES, etc., and their expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
      [BigCryptor128](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.BigCryptor128.html#struct.BigCryptor128),
      [`ECB_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [`ECB_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [`CBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [`CBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [`PCBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [`PCBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [`CFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [`OFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [`CTR`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
      However, it is considered that 2AES, 3AES, 4AES, etc. are not very meaningful because AES-256, Rijndael_128_384, Rijndael_128_512, etc. are considered to be better than 2AES, 3AES, 4AES, etc.
- [X] BigCryptor64 combinations of symmetric-key encryption/decryption algorithms, and the trait implementations of Operation modes and Padding bits for BigCryptor64
      --- Includes 2DES, 3DES, 4DES, etc., and their expanded versions, and ECB, CBC, PCBC, CFB, OFB, and CTR modes, and padding bits according to PKCS#7 and ISO 7816-4.
      [BigCryptor64](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.BigCryptor64.html#struct.BigCryptor64),
      [`ECB_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [`ECB_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [`CBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [`CBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [`PCBC_PKCS7`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [`PCBC_ISO`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [`CFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [`OFB`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [`CTR`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
### Symmetric-key Algorithms for Stream Encryption/Decryption of digital data

- [ ] Chacha20 symmetric-key encryption/decryption algorithm
      --- Includes Chacha20 and its expanded versions. `Chacha20_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] Salsa20 symmetric-key encryption/decryption algorithm
      --- Includes Salsa20 and its expanded versions. `Salsa20_Generic`
      ===> Moved to Roadmap for ver. 2.0

#### Insecure Stream Encryption/Decryption algorithms
- [ ] RC4 symmetric-key encryption/decryption algorithm
      --- Includes RC4 and its expanded versions. `RC4_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->
### 5. Pseudo-Random Number Generator Algorithms

- [ ] Pseudo-random number generator ---
      struct [`Random_Generic`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_Generic.html#struct.Random_Generic) and
      trait [`Random_Engine`](https://docs.rs/cryptocol/latest/cryptocol/random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
- [ ] Pseudo-random number generator engines using hash algorithms ---
      [`Random_BIG_KECCAK_1024`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_BIG_KECCAK_1024.html#struct.Random_BIG_KECCAK_1024),
      [`Random_SHA3_512`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_SHA3_512.html#struct.Random_SHA3_512),
      [`Random_SHA2_512`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_SHA2_512.html#struct.Random_SHA2_512),
      [`Any_SHAKE_256`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHAKE_256.html#struct.Any_SHAKE_256),
      [`Any_SHAKE_128`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHAKE_128.html#struct.Any_SHAKE_128),
      [`Any_SHA3_512`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA3_512.html#struct.Any_SHA3_512),
      [`Any_SHA3_256`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA3_256.html#struct.Any_SHA3_256),
      [`Any_SHA2_512`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA2_512.html#struct.Any_SHA2_512),
      [`Any_SHA2_256`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA2_256.html#struct.Any_SHA2_256),
      [`Any_SHA1`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA1.html#struct.Any_SHA1),
      [`Any_SHA0`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA0.html#struct.Any_SHA0),
      [`Any_MD5`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_MD5.html#struct.Any_MD5), and
      [`Any_MD4`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_MD4.html#struct.Any_MD4).
- [X] Pseudo-random number generator engines using symmetric-key block encryption algorithms ---
      [`Random_Rijndael`](https://docs.rs/cryptocol/latest/cryptocol/struct.Random_Rijndael.html#struct.Random_Rijndael),
      [`Any_Rijndael`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_Rijndael.html#struct.Any_Rijndael), and
      [`Any_DES`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_DES.html#struct.Any_DES).
- [X] Pseudo-random number generator engines using simple randomization algorithm ---
      [`Any_Num_C`](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_Num_C.html#struct.Any_Num_C).

### 6. Asymmetric-key Algorithms for Encryption/Decryption of digital data

- [ ] ECC (Elliptic Curve Cryptosystem) asymmetric-key encryption/decryption algorithm, and the traits and its implementations of Padding bits for ECC_Generic
      --- Includes ECC and its expanded versions.
- [ ] RSA (Ron Rivest, Adi Shamir, Leonard Adleman) asymmetric-key encryption/decryption algorithm, and the traits and its implementations of Padding bits for RSA_Generic
      --- Includes RSA and its expanded versions, and Padding bits according to PKCS #1 ver. 1.5 and OAEP (Optimal Asymmetric Encryption Padding) according to PKCS #1 ver. 2.0 and RFC 2437.
      [`RSA_Generic`](https://docs.rs/cryptocol/latest/cryptocol/asymmetric/struct.RSA_Generic.html#struct.RSA_Generic),
      [`PKCS1V15`](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PKCS1V15.html#trait.PKCS1V15), and
      OAEP.
<!--
- [ ] Rabin
    ===> Moved to Roadmap for ver. 2.0
- [ ] ElGamal
    ===> Moved to Roadmap for ver. 2.0
- [ ] Diffie-Hellman
    ===> Moved to Roadmap for ver. 2.0
-->

When the implementation of all the above functionalitis are completed,
the version number 1.0.0.0 will be given. After that whenever another
functionality is added to this crate, the version number will get higher
beyond 1.0.0.0. Before the version number 1.0.0.0, the maximum version
number will be 0.21.x.x since there are all twenty-five functionalities
listed above. So, for example, even if the version number is 0.5.0.0,
it does not mean that 50% of all functionalities are implemented.

## Breaking Changes

Refer to the file `BreakingChanges.md` in this crate.
