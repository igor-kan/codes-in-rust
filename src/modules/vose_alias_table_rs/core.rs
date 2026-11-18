use crate::types::AliasTable;

pub fn build_alias(weights: &[f64]) -> AliasTable {
    let n = weights.len();
    let sum: f64 = weights.iter().sum();
    let mut scaled: Vec<f64> = weights.iter().map(|&w| w * (n as f64) / sum).collect();
    let mut prob = vec![1.0; n];
    let mut alias: Vec<usize> = (0..n).collect();
    let mut small = Vec::new();
    let mut large = Vec::new();
    for i in 0..n { if scaled[i] < 1.0 { small.push(i); } else { large.push(i); } }
    while let (Some(s), Some(l)) = (small.pop(), large.pop()) {
        prob[s] = scaled[s]; alias[s] = l;
        scaled[l] = (scaled[l] + scaled[s]) - 1.0;
        if scaled[l] < 1.0 { small.push(l); } else { large.push(l); }
    }
    AliasTable { prob, alias }
}

pub fn sample_alias(t: &AliasTable, bucket: usize, u01: f64) -> usize {
    if u01 < t.prob[bucket] { bucket } else { t.alias[bucket] }
}
