use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'G', 'C', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut nucleotide_count = 0;
    for n in dna.chars() {
        if !NUCLEOTIDES.contains(&n) {
            return Err(n);
        }
        if n == nucleotide {
            nucleotide_count += 1;
        }
    }
    Ok(nucleotide_count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {

    let mut nucleii_table: HashMap<char, usize> = HashMap::new();

    for nucleotide in NUCLEOTIDES {
        let val = count(nucleotide, dna)?;
        nucleii_table.insert(nucleotide, val);
    }

    Ok(nucleii_table)
}
