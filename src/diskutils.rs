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

pub fn resize_disk(letter: &str, part_size: usize) -> io::Result<()>
{
    let output_size = Command::new("powershell")
                            .args(&["-command", "(", "Get-Partition", "-DriveLetter", letter, ").size"])
                            .output()?;

    let mut size_str: String = String::from_utf8(output_size.stdout).unwrap();
    size_str.retain(|c| !c.is_whitespace());

    let size: f64;

    match size_str.parse::<f64>()
    {
        Ok(raw_size) => size = raw_size / 1048576.0,
        Err(_) => panic!()
    }

    let shrink_size: usize = size.floor() as usize - part_size;

    Command::new("powershell")
            .args(&["-command", "Resize-Partition", "-DriveLetter", letter, "-Size", format!("({}MB)", shrink_size).as_str()])
            .spawn()?;

    Ok(())
}

pub fn create_part(letter: &str, part_size: usize) -> io::Result<()>
{
    let drive_letter = Command::new("powershell")
                                .args(&["-command", "(", "Get-Partition", "-Drive", "C", ").DiskNumber"])
                                .output()?;

    let mut drive_str = String::from_utf8(drive_letter.stdout).unwrap();
    drive_str.retain(|c| !c.is_whitespace());

    Command::new("powershell")
            .args(&["-command", "New-Partition", "-DiskNumber", &drive_str, "-Size", format!("{}MB", part_size).as_str(), "-DriveLetter", letter])
            .spawn()?;

    Ok(())
}

pub fn format_part(letter: &str, file_system: &str) -> io::Result<()>
{
    Command::new("powershell")
            .args(&["-command", "Format-Volume", "-DriveLetter", letter, "-FileSystem", file_system])
            .spawn()?;

    Ok(())
}