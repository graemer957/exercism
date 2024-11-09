use std::collections::HashMap;

const NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !NUCLEOTIDES.contains(&nucleotide) {
        return Err(nucleotide);
    }

    dna.chars().try_fold(0, |count, char| {
        if !NUCLEOTIDES.contains(&char) {
            Err(char)
        } else if nucleotide == char {
            Ok(count + 1)
        } else {
            Ok(count)
        }
    })
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    NUCLEOTIDES
        .into_iter()
        .map(|char| Ok((char, count(char, dna)?)))
        .collect()
}
