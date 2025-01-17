#![cfg(feature = "crypto")]

use rand::RngCore;

/// Generate a random 16-byte salt.
pub fn gen_salt() -> [u8; 16] {
    let mut result = [0u8; 16];

    rand::thread_rng().fill_bytes(&mut result);

    result
}

/// Use bcrypt to hash a password whose length is not bigger than 72 bytes to 24 bytes data. If the salt is not 16 bytes, it will be MD5 hashed first.
pub fn bcrypt<T: ?Sized + AsRef<[u8]>, K: ?Sized + AsRef<[u8]>>(
    cost: u8,
    salt: &K,
    password: &T,
) -> Result<[u8; 24], &'static str> {
    let mut result = [0u8; 24];

    if cost >= 32 {
        return Err("Cost needs to be smaller than 32.");
    }

    let password = password.as_ref();

    let password_len = password.len();

    if password_len == 0 {
        return Err("The password is empty.");
    }

    if password_len > 72 {
        return Err("The length of the password should not be bigger than 72.");
    }

    let salt = salt.as_ref();

    if salt.len() != 16 {
        let new_salt = md5::compute(salt);
        bcrypt::bcrypt(cost as u32, &*new_salt, password, &mut result);
    } else {
        bcrypt::bcrypt(cost as u32, salt, password, &mut result);
    }

    Ok(result)
}

/// Identify a plain text password by using the bcrypt-hashed data we've stored before.
pub fn identify_bcrypt<T: ?Sized + AsRef<[u8]>, K: ?Sized + AsRef<[u8]>>(
    cost: u8,
    salt: &K,
    password: &T,
    hashed: &[u8; 24],
) -> Result<bool, &'static str> {
    let p = bcrypt(cost, salt, password)?;

    Ok(hashed.eq(&p))
}
