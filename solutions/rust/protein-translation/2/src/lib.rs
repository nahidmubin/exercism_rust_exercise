use std::collections::HashMap;

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let codon_map: HashMap<&str, &str> = HashMap::from([
        ("AUG", "Methionine"),
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UGG", "Tryptophan"),
        ("UAA", "STOP"),
        ("UAG", "STOP"),
        ("UGA", "STOP")
    ]);

    let mut proteins: Vec<&str> = Vec::new();

    for codon in rna.chars().collect::<Vec<char>>().chunks(3) {
        let codon: String = codon.iter().collect();
        let protein = codon_map.get(codon.as_str())?;

        if *protein != "STOP" {
            proteins.push(protein);
        } else {
            break;
        }
    }

    Some(proteins)
}
