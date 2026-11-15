use crate::types::HorspoolTable;

impl HorspoolTable {
    pub fn find_all(&self, haystack: &[u8]) -> Vec<usize> {
        let m = self.needle.len();
        let mut out = Vec::new();
        if m > haystack.len() {
            return out;
        }
        let mut pos = 0usize;
        let last = m - 1;
        while pos <= haystack.len() - m {
            let mut j = last;
            while haystack[pos + j] == self.needle[j] {
                if j == 0 {
                    out.push(pos);
                    break;
                }
                j -= 1;
            }
            pos += self.shift[haystack[pos + last] as usize];
        }
        out
    }

    pub fn find_first(&self, haystack: &[u8]) -> Option<usize> {
        let m = self.needle.len();
        if m > haystack.len() {
            return None;
        }
        let mut pos = 0usize;
        let last = m - 1;
        while pos <= haystack.len() - m {
            let mut j = last;
            while haystack[pos + j] == self.needle[j] {
                if j == 0 {
                    return Some(pos);
                }
                j -= 1;
            }
            pos += self.shift[haystack[pos + last] as usize];
        }
        None
    }
}
