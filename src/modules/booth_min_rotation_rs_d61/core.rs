use crate::types::RotationInfo;

pub fn least_rotation(s: &str) -> usize {
    let doubled: Vec<char> = s.chars().chain(s.chars()).collect();
    let n = doubled.len() / 2;
    if n == 0 {
        return 0;
    }
    let mut i = 0usize;
    let mut j = 1usize;
    let mut k = 0usize;
    while i < n && j < n && k < n {
        let a = doubled[i + k];
        let b = doubled[j + k];
        if a == b {
            k += 1;
            continue;
        }
        if a > b {
            i += k + 1;
            if i <= j {
                i = j + 1;
            }
        } else {
            j += k + 1;
            if j <= i {
                j = i + 1;
            }
        }
        k = 0;
    }
    i.min(j)
}

pub fn minimal_rotation(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return String::new();
    }
    let start = least_rotation(s);
    let mut out = String::with_capacity(s.len());
    out.extend(chars[start..].iter().copied());
    out.extend(chars[..start].iter().copied());
    out
}

pub fn rotation_info(s: &str) -> RotationInfo {
    RotationInfo {
        start: least_rotation(s),
        text: minimal_rotation(s),
    }
}
