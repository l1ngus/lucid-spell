/*
 * Copyright (C) 2026 l1ngus
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use rodio::mixer::Mixer;
use rodio::{Decoder, Sink};
use std::io::Cursor;
use std::sync::Arc;

pub fn play_audio(wav_buffer: Vec<u8>, mixer: Arc<Mixer>) -> Result<(), String> {
    let sink = Sink::connect_new(&mixer);

    let cursor = Cursor::new(wav_buffer);
    let source = Decoder::new(cursor).map_err(|e| format!("Failed to decode audio: {}", e))?;

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
