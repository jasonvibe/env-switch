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
