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

mod uac;

use std::io;
use std::process::exit;

use is_elevated::is_elevated;

fn main() -> io::Result<()>
{
    if !is_elevated()
    {
        uac::uac()?;
        exit(0);
    }

    Ok(())
}