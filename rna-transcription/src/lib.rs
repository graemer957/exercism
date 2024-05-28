#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    sequence: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    sequence: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        for (i, nucleotide) in dna.char_indices() {
            if !['A', 'C', 'G', 'T'].contains(&nucleotide) {
                return Err(i);
            }
        }

        Ok(Self {
            sequence: dna.into(),
        })
    }

    pub fn into_rna(self) -> Rna {
        let sequence = self
            .sequence
            .chars()
            .map(|nucleotide| match nucleotide {
                'A' => 'U',
                'T' => 'A',
                'C' => 'G',
                'G' => 'C',
                _ => unreachable!("This should not be possible given `new` validates nucleotides"),
            })
            .collect();

        Rna { sequence }
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        for (i, nucleotide) in rna.char_indices() {
            if !['A', 'C', 'G', 'U'].contains(&nucleotide) {
                return Err(i);
            }
        }

        Ok(Self {
            sequence: rna.into(),
        })
    }
}
