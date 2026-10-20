use keyring::Entry;

pub const SERVICE_NAME: &str = "com.jasonvibe.envswitch";

pub fn make_account_key(profile_id: &str, var_key: &str) -> String {
    format!("{}::{}", profile_id, var_key)
}

pub fn save_secret(profile_id: &str, var_key: &str, secret: &str) -> Result<(), String> {
    let account = make_account_key(profile_id, var_key);
    let entry = Entry::new(SERVICE_NAME, &account).map_err(|e| e.to_string())?;
    entry.set_password(secret).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_secret(profile_id: &str, var_key: &str) -> Result<String, String> {
    let account = make_account_key(profile_id, var_key);
    let entry = Entry::new(SERVICE_NAME, &account).map_err(|e| e.to_string())?;
    match entry.get_password() {
        Ok(pwd) => Ok(pwd),
        Err(keyring::Error::NoEntry) => Ok("".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn delete_secret(profile_id: &str, var_key: &str) -> Result<(), String> {
    let account = make_account_key(profile_id, var_key);
    let entry = Entry::new(SERVICE_NAME, &account).map_err(|e| e.to_string())?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain_save_get_delete() {
        let p_id = "test_profile";
        let k = "TEST_KEY";
        let secret = "my_super_secret_12345";

        let save_res = save_secret(p_id, k, secret);
        println!("save_secret result: {:?}", save_res);
        assert!(save_res.is_ok(), "save_secret failed: {:?}", save_res);

        let retrieved = get_secret(p_id, k);
        println!("get_secret result: {:?}", retrieved);
        assert_eq!(retrieved.unwrap(), secret);

        let del_res = delete_secret(p_id, k);
        println!("delete_secret result: {:?}", del_res);
        assert!(del_res.is_ok());
    }
}
