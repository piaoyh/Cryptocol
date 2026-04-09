
// use cryptocol::number::SmallUInt;
// use cryptocol::symmetric::DES;
// use cryptocol::random::Slapdash_PRNG_Creator;


fn main()
{
    complement();
    avalanche();
    normal_keys();
    weak_keys();
    semi_weak_keys();

    small_rsa();
}

fn complement()
{
    println!("Complement case");
    use cryptocol::symmetric::DES;

    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#018X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = 0x_1234567890ABCDEF_u64;
    println!("M_u64 =\t{:#018X}", message);

    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 (16 rounds) =\t{:#018X}", cipher);
    assert_eq!(cipher, 0x_1BC4896735BBE206_u64);


    let c_key = !0x_1234567890ABCDEF_u64;
    println!("cK =\t{:#018X}", c_key);
    let mut a_des = DES::new_with_key_u64(c_key);

    let c_message = !0x_1234567890ABCDEF_u64;
    println!("cM_u64 =\t{:#018X}", c_message);

    let c_cipher = a_des.encrypt_u64(c_message);
    println!("cC_u64 (16 rounds) =\t{:#018X}", c_cipher);
    assert_eq!(c_cipher, !cipher);

    println!("K  =\t\t{:#066b}", key);
    println!("cK =\t\t{:#066b}", c_key);
    println!("M_u64  =\t{:#066b}", message);
    println!("cM_u64 =\t{:#066b}", c_message);
    println!("C_u64  =\t{:#066b}", cipher);
    println!("cC_u64 =\t{:#066b}", c_cipher);
    println!("--------------------");
}

fn avalanche()
{
    println!("Avalanche case");
    use cryptocol::symmetric::DES;

    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t{:#018X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = 0x_0000_0000_0000_0000_u64;
    println!("M_u64 =\t{:#018X}", message);

    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 (16 rounds) =\t{:#018X}", cipher);

    let message2 = 0x_0000_0000_0000_0001_u64;
    println!("M2_u64 =\t{:#018X}", message2);

    let cipher2 = a_des.encrypt_u64(message2);
    println!("C2_u64 (16 rounds) =\t{:#018X}", cipher2);

    println!("K  =\t\t{:#066b}", key);
    println!("M_u64  =\t{:#066b}", message);
    println!("M2_u64 =\t{:#066b}", message2);
    println!("C_u64  =\t{:#066b}", cipher);
    println!("C2_u64 =\t{:#066b}", cipher2);
    println!("--------------------");
}

fn normal_keys()
{
    println!("Normal key case");
    use cryptocol::symmetric::DES;

    let key = 0x_1234567890ABCDEF_u64;
    println!("K =\t\t{:#018X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = 0x_FEDCBA0987654321_u64;
    println!("M_u64 =\t\t{:#018X}", message);

    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 =\t\t{:#018X}", cipher);

    let cipher2 = a_des.encrypt_u64(cipher);
    println!("C2_u64 =\t{:#018X}", cipher2);
    println!("--------------------");
}

fn weak_keys()
{
    println!("Weak key case");
    use cryptocol::symmetric::DES;
    
    let key = 0x_0101010101010101_u64.to_be();
    println!("K =\t\t{:#018X}", key);
    let mut a_des = DES::new_with_key_u64(key);

    let message = 0x_FEDCBA0987654321_u64;
    println!("M_u64 =\t\t{:#018X}", message);

    let cipher = a_des.encrypt_u64(message);
    println!("C_u64 =\t\t{:#018X}", cipher);

    let cipher2 = a_des.encrypt_u64(cipher);
    println!("C2_u64 =\t{:#018X}", cipher2);
    println!("--------------------");
}

fn semi_weak_keys()
{
    println!("Semi-Weak key case");
    use cryptocol::symmetric::DES;

    let key1 = 0x_011F011F010E010E_u64.to_be();
    let key2 = 0x_1F011F010E010E01_u64.to_be();
    println!("K1 =\t\t{:#018X}", key1);
    println!("K2 =\t\t{:#018X}", key2);
    let mut a_des = DES::new_with_key_u64(key1);
    let mut b_des = DES::new_with_key_u64(key2);

    let message = 0x_FEDCBA0987654321_u64;
    println!("M_u64 =\t\t{:#018X}", message);

    let cipher = a_des.encrypt_u64(message);
    println!("C1_u64 =\t{:#018X}", cipher);

    let cipher2 = b_des.encrypt_u64(cipher);
    println!("C2_u64 =\t{:#018X}", cipher2);
    println!("--------------------");
}

fn small_rsa()
{
    println!("small_rsa");
    use cryptocol::number::SmallUInt;
    use cryptocol::random::Slapdash_PRNG_Creator;
    
    let mut slapdash = Slapdash_PRNG_Creator::create();
    let mut prime1 = slapdash.random_u32();
    let mut prime2 = slapdash.random_u32();

    prime1.set_msb();
    while !prime1.is_prime()
    {
        prime1 = slapdash.random_u32();
        prime1.set_msb();
    }

    prime2.set_msb();
    while !prime2.is_prime()
    {
        prime2 = slapdash.random_u32();
        prime2.set_msb();
    }

    let modulo = prime1 as u64 * prime2 as u64;
    println!("Prime 1 = {}", prime1);
    println!("Prime 2 = {}", prime2);
    println!("Modulo = {}", modulo);
    let phi = (prime1 - 1) as u64 * (prime2 - 1) as u64;

    let mut key1 = 2_u64;
    let (mut one, mut key2, _) = key1.extended_gcd(phi);

    while !one.is_one()
    {
        key1 += 1;
        (one, key2, _) = key1.extended_gcd(phi);
    }
    if key2 > phi
        { key2 = phi.wrapping_sub(0_u64.wrapping_sub(key2)); }
    else
        { key2 %= phi; }

    println!("Public Key = {}", key1);
    println!("Private Key = {}", key2);

    let message = 3_u64;
    let cipher = message.modular_pow(key1, modulo);
    let recover = cipher.modular_pow(key2, modulo);
    println!("Message = {}", message);
    println!("Cipher = {}", cipher);
    println!("Recover = {}", recover);

    let product = key1.modular_mul(key2, phi);
    println!("product = {} X {} (mod {}) = {}", key1, key2, phi, product);
    println!("--------------------");
}