#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    strand: String
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    strand: String
}

impl Dna {
    const DNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

    pub fn new(dna: &str) -> Result<Dna, usize> {
        for n in dna.chars() {
            if !Self::DNA_NUCLEOTIDES.contains(&n) {
                return Err(dna.chars().position(|ch| ch == n).unwrap());
            }
        }
        Ok(Self { strand: dna.to_string() })
    }

    pub fn into_rna(self) -> Rna {
        let mut rna_strand = String::new();

        for ch in self.strand.chars() {
            if ch == 'G' {
                rna_strand.push('C');
            } else if ch == 'C' {
                rna_strand.push('G');
            } else if ch == 'T' {
                rna_strand.push('A');
            } else if ch == 'A' {
                rna_strand.push('U');
            }
        }
        Rna { strand: rna_strand }
    }
}

impl Rna {
    const RNA_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'U'];

    pub fn new(rna: &str) -> Result<Rna, usize> {
        for n in rna.chars() {
            if !Self::RNA_NUCLEOTIDES.contains(&n) {
                return Err(rna.chars().position(|ch| ch == n).unwrap());
            }
        }
        Ok(Self { strand: rna.to_string() })
    }
}