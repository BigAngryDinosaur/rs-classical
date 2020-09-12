use std::fmt;

#[derive(Clone)]
struct NucleotideCompressorError { nucleotide: char }

impl fmt::Display for NucleotideCompressorError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid nucleotide: {}", self.nucleotide)
    }
}

pub struct NucleotideCompressor {
    bit_string: u32
}

impl NucleotideCompressor {

    pub fn new(gene: String) -> Option<Self> {
        let bit_string = match NucleotideCompressor::compress(gene) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{}", e);
                return None
            }
        };
        Some(NucleotideCompressor { bit_string })
    }

    fn compress(gene: String) -> Result<u32, NucleotideCompressorError> {
        let mut bit_string = 1;

        for nucleotide in gene.chars() {
            bit_string <<= 2;
            bit_string |= match nucleotide {
                'A' => 0,
                'C' => 1,
                'G' => 2,
                'T' => 3,
                _ => return Err(NucleotideCompressorError { nucleotide })
            }
        }

        Ok(bit_string)
    }

    fn bit_length(&self) -> u32 {
        ((self.bit_string as f64).log2() as u32) + 1
    }

    fn decompress(&self) -> String {
        let n = self.bit_length();
        let mut gene = vec![];
        for i in (0..n-1).step_by(2) {
            let bits = self.bit_string >> i & 3;
            let c = match bits {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => unreachable!()
            };
            gene.push(c);
        }
        gene.iter().rev().collect()
    }
}

impl fmt::Display for NucleotideCompressor {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.decompress())
    }
}

impl fmt::Debug for NucleotideCompressor {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bit string: {:b}", self.bit_string)
    }
}
