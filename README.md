<!--
Copyright 2023, 2024, 2025, 2026 PARK Youngho.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
<LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
This file may not be copied, modified, or distributed
except according to those terms.
 > int_union.rs를 참고하여 src/number/macros_for_integer_unions.rs의 3770행부터 
   3829행까지의 독스트링의 영어를 다듬어 줘. 예제와 그 제목은 건드리지 마. 양이 
   많으니까 한꺼번에 하지말고 차근차근 한 함수씩 독스트링을 교정해 줘. 누락된   
   것 없이 정교하게 교정해 줘.  
-->
# cryptocol: A Comprehensive Cryptography Library

## Endianness Support

`cryptocol` is optimized for **Little-endian** architectures, as they are the most widely used in modern computing. For more detailed information regarding Endianness (Little-endian vs. Big-endian), please read more [here](https://en.wikipedia.org/wiki/Endianness).

### Big-endian Compatibility Notice

Support for __Big-endian__ architectures is currently experimental. We do not recommend using this crate on Big-endian systems for production or critical applications. Users choosing to do so assume full responsibility for any potential issues.

## Roadmap for Version 1.0

The following features are planned for the `cryptocol` ecosystem.

- [X] **Completed:** Implementation and documentation are at least **95%** complete.
- [ ] __In Progress:__ Implementation or documentation is below __95%__, or work has not yet begun.

### 1. Small Numbers
_Foundational types for Big Numbers and other cryptographic modules._

- [X] **`SmallUInt` Trait:** Core implementation for primitive unsigned data types. --
      [SmallUInt](https://docs.rs/cryptocol/latest/cryptocol/number/small_uint/trait.SmallUInt.html#trait.SmallUInt)
- [X] **Primitive Unions:** Implementation of unions for primitive unsigned types. --
      [ShortUnion](https://docs.rs/cryptocol/latest/cryptocol/number/short_union/union.ShortUnion.html#union.ShortUnion),
      [IntUnion](https://docs.rs/cryptocol/latest/cryptocol/number/int_union/union.IntUnion.html#union.IntUnion),
      [LongUnion](https://docs.rs/cryptocol/latest/cryptocol/number/long_union/union.LongUnion.html#union.LongUnion),
      [LongerUnion](https://docs.rs/cryptocol/latest/cryptocol/number/longer_union/union.LongerUnion.html#union.LongerUnion),
      [SizeUnion](https://docs.rs/cryptocol/latest/cryptocol/number/size_union/union.SizeUnion.html#union.SizeUnion),
      [SharedValues](https://docs.rs/cryptocol/latest/cryptocol/number/shared_values/union.SharedValues.html#union.SharedValues), and
      [SharedArrays](https://docs.rs/cryptocol/latest/cryptocol/number/shared_arrays/union.SharedArrays.html#union.SharedArrays)      
<!--
- [ ] __`SmallSInt` Trait:__ (Planned) Core implementation for primitive signed data type. --
      `SmallSInt`
      ===> Moved to Roadmap for ver. 2.0
-->

### 2. Big Numbers
_Essential for Asymmetric-Key Algorithms and high-precision calculations._

- [X] **Fixed-Size Big Unsigned Integers:** Standard operations for large unsigned integers. --
      [BigUInt](https://docs.rs/cryptocol/latest/cryptocol/number/big_uint/struct.BigUInt.html#struct.BigUInt).
- [X] **Auxiliary Big Unsigned Integer Operations:** Specialized operations including modular arithmetic and primality testing. --
      [BigUInt_More](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_more/trait.BigUInt_More.html#trait.BigUInt_More),
      [BigUInt_Modular](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_modular/trait.BigUInt_Modular.html#trait.BigUInt_Modular),
      [BigUInt_Prime](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_prime/trait.BigUInt_Prime.html#trait.BigUInt_Prime), and
      [BigUInt_Panic_Free](https://docs.rs/cryptocol/latest/cryptocol/number/trait_big_uint_panic_free/trait.BigUInt_Panic_Free.html#trait.BigUInt_Panic_Free).
<!--
- [ ] __Fixed-Size Big Signed Integers:__ (Planned) Standard operations for large signed integers. --
      `BigSInt`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __Variable-Size Big Signed Integers:__ (Planned) Standard operations for large signed integers. --
      `LargeInt`
      ===> Moved to Roadmap for ver. 2.0 or higher
-->

### 3. Hash Algorithms

#### 3-1. Official SHA-series
- [X] **SHA-3 & Keccak:** Supports 8/16/32/64-bit variants, including SHA3-224/256/384/512 and SHAKE 128/256. --
      [Keccak_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/keccak/struct.Keccak_Generic.html#struct.Keccak_Generic)
- [X] **SHA-2 (512/t):** Including 512/256 and 512/224 variants. --
      [SHA2_512_t_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512_t/struct.SHA2_512_t_Generic.html#struct.SHA2_512_t_Generic)
- [X] **SHA-2 (512-bit):** SHA-512, SHA-384, and SHA-512/256. --
      [SHA2_512_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_512/struct.SHA2_512_Generic.html#struct.SHA2_512_Generic)
- [X] **SHA-2 (256-bit):** SHA-256 and SHA-224. --
      [SHA2_256_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/sha2_256/struct.SHA2_256_Generic.html#struct.SHA2_256_Generic)

#### 3-2. Educational Hash Algorithms (Insecure)
**Warning:** These are intended for educational purposes only and should not be used for securing sensitive data.

- [X] **SHA-1 & SHA-0:** 160-bit hash algorithms. --
      [SHA1_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/sha1/struct.SHA1_Generic.html#struct.SHA1_Generic)
<!--
- [ ] __RIPEMD:__ (Planned) 256-bit hash algorithms. --
      ===> Moved to Roadmap for ver. 2.0
- [ ] __BLAKE3:__ (Planned) 256-bit hash algorithms. --
      ===> Moved to Roadmap for ver. 2.0
- [ ] __BLAKE2:__ (Planned) 256-bit hash algorithms. --
      ===> Moved to Roadmap for ver. 2.0
- [ ] __MD6:__ (Planned) 256-bit hash algorithms. --
      ===> Moved to Roadmap for ver. 2.0
-->
- [X] **MD5:** 128-bit hash algorithm. --
      [MD5_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/md5/struct.MD5_Generic.html#struct.MD5_Generic)
- [X] **MD4:** 128-bit hash algorithm. --
      [MD4_Generic](https://docs.rs/cryptocol/latest/cryptocol/hash/md4/struct.MD4_Generic.html#struct.MD4_Generic)
<!--
- [ ] __MD2:__ (Planned) 128-bit hash algorithms. --
      ===> Moved to Roadmap for ver. 2.0
-->

### 4. Symmetric-Key Algorithms (Block Ciphers)
#### 4-1. AES Finalists
- [X] **AES (Advanced Encryption Standard) & Rijndael:** Full support for AES/Rijndael with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      [Rijndael_Generic](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.Rijndael_Generic.html#struct.Rijndael_Generic),
      [ECB_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [ECB_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [CBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [CBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [PCBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [PCBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [CFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [OFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [CTR](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
- [ ] __MARS:__ (Planned) Full support for MARS with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). -- 
      `MARS_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __Serpent:__ (Planned) Full support for Serpent with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `Serpent_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __Twofish:__ (Planned) Full support for Twofish with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `Twofish_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __RC6:__ (Planned) Full support for RC6 with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `RC6_Generic`
      ===> Moved to Roadmap for ver. 2.0

#### 4-2. South Korean Encryption/Decryption algorithms
- [ ] __SEED:__ (Planned) Full support for SEED with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `SEED_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] _HIGHT:__ (Planned) Full support for HIGHT with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `HIGHT_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __ARIA:__ (Planned) Full support for ARIA with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `ARIA_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __LEA:__ (Planned) Full support for LEA with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `LEA_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __IDEA:__ (Planned) Full support for IDEA with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `IDEA_Generic`
      ===> Moved to Roadmap for ver. 2.0

#### 4-3. Miscellaneous Encryption/Decryption algorithms
- [ ] __Bluefish:__ (Planned) Full support for Bluefish with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `Bluefish_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __RC5:__ (Planned) Full support for RC5 with various operation modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding schemes (PKCS#7, ISO 7816-4). --
      `RC5_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->

#### 4-2. Educational Block Ciphers (Insecure)
**Warning:** These are intended for educational purposes only and should not be used for securing sensitive data.
<!--
- [ ] __RC2:__ (Planned) Standard DES implementation with multiple modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding options (PKCS#7, ISO 7816-4). --
      `RC2_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->
- [X] **DES (Data Encryption Standard):** Standard DES implementation with multiple modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding options (PKCS#7, ISO 7816-4). --
      [DES_Generic](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.DES_Generic.html#struct.DES_Generic),
      [ECB_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [ECB_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [CBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [CBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [PCBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [PCBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [CFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [OFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [CTR](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
- [ ] __Lucifer:__ (Planned) Standard Lucifer implementation with multiple modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding options (PKCS#7, ISO 7816-4). --
      `Lucifer_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->

#### 4-3. Combined Cipher Containers
- [X] **BigCryptor128:** A tool for combining multiple symmetric algorithms (e.g., 2AES, 3AES) with multiple modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding options (PKCS#7, ISO 7816-4). --
      [BigCryptor128](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.BigCryptor128.html#struct.BigCryptor128),
      [ECB_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [ECB_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [CBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [CBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [PCBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [PCBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [CFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [OFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [CTR](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).

_Note: While supported, using native AES-256 or specific Rijndael variants is generally recommended over 2AES/3AES for better security and performance._

- [X] **BigCryptor64:** A tool for combining multiple symmetric algorithms (e.g., 2DES, 3DES) with multiple modes (ECB, CBC, PCBC, CFB, OFB, CTR) and padding options (PKCS#7, ISO 7816-4). --
      [BigCryptor64](https://docs.rs/cryptocol/latest/cryptocol/symmetric/struct.BigCryptor64.html#struct.BigCryptor64),
      [ECB_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_PKCS7.html#trait.ECB_PKCS7),
      [ECB_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.ECB_ISO.html#trait.ECB_ISO),
      [CBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_PKCS7.html#trait.CBC_PKCS7),
      [CBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CBC_ISO.html#trait.CBC_ISO),
      [PCBC_PKCS7](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_PKCS7.html#trait.PCBC_PKCS7),
      [PCBC_ISO](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.PCBC_ISO.html#trait.PCBC_ISO),
      [CFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CFB.html#trait.CFB),
      [OFB](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.OFB.html#trait.OFB), and
      [CTR](https://docs.rs/cryptocol/latest/cryptocol/symmetric/trait.CTR.html#trait.CTR).
<!--
### 5. Symmetric-Key Algorithms (Stream Ciphers)
- [ ] __Chacha20:__ (Planned) Full support for Chacha20. --
      `Chacha20_Generic`
      ===> Moved to Roadmap for ver. 2.0
- [ ] __Salsa20:__ (Planned) Full support for Chacha20. --
      `Salsa20_Generic`
      ===> Moved to Roadmap for ver. 2.0

### 5-1. Educational Stream Ciphers (Insecure)
- [ ] __RC4:__ (Planned) Full support for RC4. --
      ``RC4_Generic`
      ===> Moved to Roadmap for ver. 2.0
-->

### 5. Pseudo-Random Number Generators (PRNG)
- [X] **Generic PRNG Framework:** --
      struct [Random_Generic](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_Generic.html#struct.Random_Generic) and
      trait [Random_Engine](https://docs.rs/cryptocol/latest/cryptocol/random/trait_random_engine/trait.Random_Engine.html#trait.Random_Engine)
- [X] **Hash-based PRNGs:** Engines leveraging Keccak, SHA-3, SHA-2, and others. --
      [Random_BIG_KECCAK_1024](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_BIG_KECCAK_1024.html#struct.Random_BIG_KECCAK_1024),
      [Random_SHA3_512](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_SHA3_512.html#struct.Random_SHA3_512),
      [Random_SHA2_512](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_SHA2_512.html#struct.Random_SHA2_512),
      [Any_SHAKE_256](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHAKE_256.html#struct.Any_SHAKE_256),
      [Any_SHAKE_128](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHAKE_128.html#struct.Any_SHAKE_128),
      [Any_SHA3_512](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA3_512.html#struct.Any_SHA3_512),
      [Any_SHA3_256](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA3_256.html#struct.Any_SHA3_256),
      [Any_SHA2_512](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA2_512.html#struct.Any_SHA2_512),
      [Any_SHA2_256](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_SHA2_256.html#struct.Any_SHA2_256),
      [Slapdash_SHA1](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_SHA1.html#struct.Slapdash_SHA1),
      [Slapdash_SHA0](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_SHA0.html#struct.Slapdash_SHA0),
      [Slapdash_MD5](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_MD5.html#struct.Slapdash_MD5), and
      [Slapdash_MD4](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_MD4.html#struct.Slapdash_MD4).
- [X] **Cipher-based PRNGs:** Engines leveraging Rijndael and DES. --
      [Random_Rijndael](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Random_Rijndael.html#struct.Random_Rijndael),
      [Any_Rijndael](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Any_Rijndael.html#struct.Any_Rijndael), and
      [Slapdash_DES](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_DES.html#struct.Slapdash_DES).
- [X] **Simple PRNGs:** Basic randomization algorithms like
      [Slapdash_Num_C](https://docs.rs/cryptocol/latest/cryptocol/random/struct.Slapdash_Num_C.html#struct.Slapdash_Num_C).

### 6. Asymmetric-Key Algorithms

- [ ] __ECC (Elliptic Curve Cryptography):__ (Planned) Generic ECC implementation with padding support. --
      `ECC_Generic`.
ECC (Elliptic Curve Cryptosystem) asymmetric-key encryption/decryption algorithm, and the traits and its implementations of Padding bits for ECC_Generic
      --- Includes ECC and its expanded versions.
- [ ] __RSA (Ron Rivest, Adi Shamir, Leonard Adleman):__ (In Progress) Implementation of RSA with PKCS #1 v1.5 and OAEP (Optimal Asymmetric Encryption Padding) (PKCS #1 v2.0/RFC 2437) padding. --
      [RSA_Generic](https://docs.rs/cryptocol/latest/cryptocol/asymmetric/struct.RSA_Generic.html#struct.RSA_Generic),
      [PKCS1V15](https://docs.rs/cryptocol/latest/cryptocol/asymmetric/trait.PKCS1V15.html#trait.PKCS1V15), and
      OAEP.
<!--
- [ ] __Rabin:__ (Planned)
    ===> Moved to Roadmap for ver. 2.0
- [ ] __ElGamal:__ (Planned)
    ===> Moved to Roadmap for ver. 2.0
- [ ] __Diffie-Hellman:__ (Planned)
    ===> Moved to Roadmap for ver. 2.0
-->

## Versioning Policy
The project will reach Version 1.0.0.0 once all twenty-one functional areas listed above are fully implemented.

- __Pre-v1.0:__ Versions will range up to 0.21.x.x based on the progress of the listed functionalities.
- **Post-v1.0:** New features will increment the version beyond 1.0.0.0.

_Note: Version numbers like 0.5.0.0 indicate progress through the functionality list, not necessarily a 50% completion of the entire codebase._

## Breaking Changes
Please refer to `BreakingChanges.md` for a detailed history of API changes.

