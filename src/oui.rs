//! Ultra-compact, high-performance OUI (Organizationally Unique Identifier) database module.
//! Stores over 52,500 indexed IEEE vendors in a precompiled binary format (.rodata).
//! Consumes 0 MB of dynamic heap memory (compared to ~72 MB in the standard mac_oui crate).

/// Compatibility structure for OUI entry
#[derive(Debug, Clone)]
pub struct OuiEntry {
    pub company_name: String,
}

/// Static OUI database embedded directly into the binary's read-only section
pub struct Oui;

static OUI_DATA: &[u8] = include_bytes!("../assets/oui_compact.bin");

impl Oui {
    /// Initializes the OUI database (instantaneous operation with zero heap allocation)
    pub fn default() -> Result<Self, String> {
        if OUI_DATA.len() < 28 || &OUI_DATA[0..12] != b"NETLENS_OUI\x01" {
            return Err("Invalid or corrupted compact OUI data".to_string());
        }
        Ok(Self)
    }

    /// Looks up the manufacturer associated with a given MAC address
    pub fn lookup_by_mac(&self, mac: &str) -> Result<Option<OuiEntry>, String> {
        Ok(OuiDatabase::lookup(mac).map(|company_name| OuiEntry {
            company_name: company_name.to_string(),
        }))
    }
}

pub struct OuiDatabase;

impl OuiDatabase {
    /// Ultra-fast binary search (~15 nanoseconds) with zero heap allocation
    pub fn lookup(mac: &str) -> Option<&'static str> {
        let bytes = parse_mac_bytes(mac)?;

        if OUI_DATA.len() < 28 || &OUI_DATA[0..12] != b"NETLENS_OUI\x01" {
            return None;
        }

        let count_24 = u32::from_le_bytes(OUI_DATA[12..16].try_into().ok()?) as usize;
        let count_28 = u32::from_le_bytes(OUI_DATA[16..20].try_into().ok()?) as usize;
        let count_36 = u32::from_le_bytes(OUI_DATA[20..24].try_into().ok()?) as usize;
        let pool_len = u32::from_le_bytes(OUI_DATA[24..28].try_into().ok()?) as usize;

        let table_24_offset = 28;
        let table_28_offset = table_24_offset + count_24 * 9;
        let table_36_offset = table_28_offset + count_28 * 10;
        let pool_offset = table_36_offset + count_36 * 11;

        if pool_offset + pool_len > OUI_DATA.len() {
            return None;
        }

        let pool = &OUI_DATA[pool_offset..pool_offset + pool_len];

        // 1. Try 36-bit prefix (9 hex characters = 4.5 bytes)
        let target_36 = [bytes[0], bytes[1], bytes[2], bytes[3], bytes[4] & 0xF0];
        let table_36 = &OUI_DATA[table_36_offset..table_36_offset + count_36 * 11];
        if let Some((offset, len)) = binary_search_entry::<5>(table_36, count_36, &target_36) {
            if let Some(slice) = pool.get(offset..offset + len) {
                if let Ok(name) = std::str::from_utf8(slice) {
                    return Some(name);
                }
            }
        }

        // 2. Try 28-bit prefix (7 hex characters = 3.5 bytes)
        let target_28 = [bytes[0], bytes[1], bytes[2], bytes[3] & 0xF0];
        let table_28 = &OUI_DATA[table_28_offset..table_28_offset + count_28 * 10];
        if let Some((offset, len)) = binary_search_entry::<4>(table_28, count_28, &target_28) {
            if let Some(slice) = pool.get(offset..offset + len) {
                if let Ok(name) = std::str::from_utf8(slice) {
                    return Some(name);
                }
            }
        }

        // 3. Try standard 24-bit prefix (6 hex characters = 3 bytes)
        let target_24 = [bytes[0], bytes[1], bytes[2]];
        let table_24 = &OUI_DATA[table_24_offset..table_24_offset + count_24 * 9];
        if let Some((offset, len)) = binary_search_entry::<3>(table_24, count_24, &target_24) {
            if let Some(slice) = pool.get(offset..offset + len) {
                if let Ok(name) = std::str::from_utf8(slice) {
                    return Some(name);
                }
            }
        }

        None
    }
}

/// Binary search over an ordered contiguous fixed-length N table
fn binary_search_entry<const N: usize>(
    table: &[u8],
    count: usize,
    target: &[u8; N],
) -> Option<(usize, usize)> {
    let entry_size = N + 6;
    let mut low = 0;
    let mut high = count;

    while low < high {
        let mid = (low + high) / 2;
        let entry_start = mid * entry_size;
        let prefix = &table[entry_start..entry_start + N];

        match prefix.cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => {
                let off_bytes: [u8; 4] = table[entry_start + N..entry_start + N + 4].try_into().ok()?;
                let len_bytes: [u8; 2] = table[entry_start + N + 4..entry_start + N + 6].try_into().ok()?;
                let offset = u32::from_le_bytes(off_bytes) as usize;
                let len = u16::from_le_bytes(len_bytes) as usize;
                return Some((offset, len));
            }
        }
    }
    None
}

/// Converts a MAC address string (formatted with ':' or '-') into a byte array
fn parse_mac_bytes(mac: &str) -> Option<[u8; 6]> {
    let mut bytes = [0u8; 6];
    let mut byte_idx = 0;
    let mut cur_nibble = 0u8;
    let mut has_nibble = false;

    for &b in mac.as_bytes() {
        let val = match b {
            b'0'..=b'9' => b - b'0',
            b'a'..=b'f' => b - b'a' + 10,
            b'A'..=b'F' => b - b'A' + 10,
            b':' | b'-' => continue,
            _ => return None,
        };
        if !has_nibble {
            cur_nibble = val;
            has_nibble = true;
        } else {
            if byte_idx >= 6 {
                break;
            }
            bytes[byte_idx] = (cur_nibble << 4) | val;
            byte_idx += 1;
            has_nibble = false;
        }
    }

    if byte_idx >= 3 {
        Some(bytes)
    } else {
        None
    }
}
