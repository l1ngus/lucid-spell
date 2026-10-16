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

use crate::dto::KeyStoreError;
use keyring::Entry;

const SERVICE_NAME: &str = "io.github.l1ngus.lucid-spell.api_keys";

// Сохранить ключ привязав его к ID профиля
pub fn set_key(profile_id: &str, api_key: &str) -> Result<(), KeyStoreError> {
    let entry = Entry::new(SERVICE_NAME, profile_id)?;
    entry.set_password(api_key)?;
    Ok(())
}

// Получить ключ из системы (эта функция будет вызываться ТОЛЬКО на бэкенде для async-openai)
pub fn get_key(profile_id: &str) -> Result<String, String> {
    let entry = Entry::new(SERVICE_NAME, profile_id).map_err(|e| e.to_string())?;
    entry.get_password().map_err(|e| e.to_string())
}

// Проверить наличие ключа (для UI)
pub fn has_key(profile_id: &str) -> bool {
    if let Ok(entry) = Entry::new(SERVICE_NAME, profile_id) {
        entry.get_password().is_ok()
    } else {
        false
    }
}

// Удалить ключ (при удалении профиля или сбросе ключа)
pub fn delete_key(profile_id: &str) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, profile_id).map_err(|e| e.to_string())?;
    // Игнорируем ошибку, если ключа и так нет в хранилище
    let _ = entry.delete_credential();
    Ok(())
}
