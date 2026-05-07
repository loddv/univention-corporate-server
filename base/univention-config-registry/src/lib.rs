#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    dead_code
)]

// SPDX-FileCopyrightText: 2026 Univention GmbH
// SPDX-License-Identifier: AGPL-3.0-only

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ffi::{CStr, CString};

pub struct ConfigRegistry;

impl ConfigRegistry {
    pub fn new() -> Self {
        ConfigRegistry
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let ckey = CString::new(key).ok()?;
        unsafe {
            let ptr = univention_config_get_string(ckey.as_ptr());
            if ptr.is_null() {
                return None;
            }
            let val = CStr::from_ptr(ptr).to_string_lossy().into_owned();
            libc::free(ptr as *mut libc::c_void);
            Some(val)
        }
    }

    pub fn get_int(&self, key: &str, default: i32) -> i32 {
        let ckey = match CString::new(key) {
            Ok(k) => k,
            Err(_) => return default,
        };
        let val = unsafe { univention_config_get_int(ckey.as_ptr()) };
        if val == -1 {
            default
        } else {
            val
        }
    }

    pub fn is_true(&self, key: &str, default_value: bool) -> bool {
        let ckey = match CString::new(key) {
            Ok(k) => k,
            Err(_) => return default_value,
        };
        unsafe { univention_config_is_true(ckey.as_ptr(), default_value) }
    }

    pub fn iter_prefix(&self, prefix: &str) -> Vec<(String, String)> {
        // UCR C lib has no iteration API; fall back to reading /etc/univention/base.conf
        let mut results = Vec::new();
        let content = match std::fs::read_to_string("/etc/univention/base.conf") {
            Ok(c) => c,
            Err(_) => return results,
        };
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((k, v)) = line.split_once(':') {
                let k = k.trim();
                let v = v.trim();
                if k.starts_with(prefix) {
                    results.push((k.to_string(), v.to_string()));
                }
            }
        }
        results
    }
}

impl Default for ConfigRegistry {
    fn default() -> Self {
        ConfigRegistry
    }
}
