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
use crate::dto::{KeyStatus, KeyStoreError};
use crate::infra::keychain;

#[tauri::command]
#[specta::specta]
pub fn save_profile_api_key(profile_id: String, api_key: String) -> Result<(), KeyStoreError> {
    println!("Key save");
    keychain::set_key(&profile_id, &api_key)
}

#[tauri::command]
#[specta::specta]
pub fn check_profile_api_key(profile_id: String) -> KeyStatus {
    KeyStatus {
        is_saved: keychain::has_key(&profile_id),
        profile_id,
    }
}

#[tauri::command]
#[specta::specta]
pub fn remove_profile_api_key(profile_id: String) -> Result<(), String> {
    keychain::delete_key(&profile_id)
}
