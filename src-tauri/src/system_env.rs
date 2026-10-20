use std::process::Command;
use crate::models::StoreData;
use crate::keychain;

pub fn execute_launchctl_set(key: &str, value: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let status = Command::new("launchctl")
            .arg("setenv")
            .arg(key)
            .arg(value)
            .status()
            .map_err(|e| format!("Failed to run launchctl setenv: {}", e))?;

        if !status.success() {
            return Err(format!("launchctl setenv {} failed with code {:?}", key, status.code()));
        }
    }
    Ok(())
}

pub fn execute_launchctl_unset(key: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        let status = Command::new("launchctl")
            .arg("unsetenv")
            .arg(key)
            .status()
            .map_err(|e| format!("Failed to run launchctl unsetenv: {}", e))?;

        if !status.success() {
            return Err(format!("launchctl unsetenv {} failed with code {:?}", key, status.code()));
        }
    }
    Ok(())
}

pub fn apply_profile_to_system(profile_id: &str, store: &mut StoreData) -> Result<Vec<String>, String> {
    let profile = store.profiles.iter().find(|p| p.id == profile_id)
        .ok_or_else(|| format!("Profile not found: {}", profile_id))?
        .clone();

    let enabled_vars: Vec<_> = store.variables.iter()
        .filter(|v| v.profile_id == profile_id && v.enabled)
        .cloned()
        .collect();

    let mut new_keys = Vec::new();

    // 1. Unset keys that were previously injected but are not in the new profile
    for old_key in &store.injected_keys {
        if !enabled_vars.iter().any(|v| &v.key == old_key) {
            let _ = execute_launchctl_unset(old_key);
        }
    }

    // 2. Set new keys
    for var in enabled_vars {
        let val = if var.is_secret {
            keychain::get_secret(profile_id, &var.key).unwrap_or_default()
        } else {
            var.value.clone()
        };

        execute_launchctl_set(&var.key, &val)?;
        new_keys.push(var.key);
    }

    // 3. Update active states
    store.active_profile_id = Some(profile_id.to_string());
    store.injected_keys = new_keys.clone();

    for p in &mut store.profiles {
        if p.project_id == profile.project_id {
            p.is_active = p.id == profile_id;
        }
    }

    Ok(new_keys)
}

pub fn clear_system_env(store: &mut StoreData) -> Result<(), String> {
    for key in &store.injected_keys {
        let _ = execute_launchctl_unset(key);
    }
    store.injected_keys.clear();
    Ok(())
}
