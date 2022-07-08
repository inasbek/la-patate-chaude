use md5;

use crate::message:: {
    MD5HashCashInput,
    MD5HashCashOutput,
};

pub trait Challenge {
    /// Données en entrée du challenge
    type Input;
    /// Données en sortie du challenge
    type Output;
    /// Nom du challenge
    fn name() -> String;
    /// Create a challenge from the specific input
    fn new(input: Self::Input) -> Self;
    /// Résout le challenge
    fn solve(&self) -> Self::Output;
    /// Vérifie qu'une sortie est valide pour le challenge
    fn verify(&self, answer: &Self::Output) -> bool;
}

pub struct MD5HashCash {
    pub input: MD5HashCashInput
}

impl Challenge for MD5HashCash {
    type Input = MD5HashCashInput;
    type Output = MD5HashCashOutput;

    fn name() -> String {
        "MD5HashCash".to_string()
    }

    fn new(input: Self::Input) -> Self {
        MD5HashCash {
            input
        }
    }

    fn solve(&self) -> Self::Output {
        let mut output = MD5HashCashOutput {
            seed: 0,
            hashcode: "".to_string()
        };

        for seed in 0..u64::MAX {

            let input_to_solve = format!("{:016X}{}", seed, self.input.message);

            let hashcode_in_hexa = format!("{:016X}", md5::compute(&input_to_solve));

            let nb_leading_zeros = leading_zeros_of_hexa_string(&hashcode_in_hexa);

            if nb_leading_zeros >= self.input.complexity {
                output = MD5HashCashOutput {
                    seed,
                    hashcode: hashcode_in_hexa
                };
                break;
            }
        }
        output
    }

    fn verify(&self, answer: &Self::Output) -> bool {
        let input_to_solve = format!("{:0>16X}{}", answer.seed, self.input.message);

        let hashcode_in_hexa = format!("{:X}", md5::compute(&input_to_solve));

        let nb_leading_zeros = leading_zeros_of_hexa_string(&hashcode_in_hexa);

        if nb_leading_zeros >= self.input.complexity {
            return true;
        }
        return false;
    }
}

fn leading_zeros_of_hexa_string(hexadecimal_string: &str) -> u32 {
    u128::from_str_radix(hexadecimal_string, 16).unwrap()
        .leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new() -> MD5HashCash {
        MD5HashCash {
            input: MD5HashCashInput {
                complexity: 9,
                message: "hello".to_string()
            }
        }
    }

    #[test]
    pub fn test_verify() {
        let challenge = new();
        let output = MD5HashCashOutput {
            seed: 844,
            hashcode: "00441745D9BDF8E5D3C7872AC9DBB2C3".to_string()
        };
        assert_eq!(true, challenge.verify(&output));
    }

    #[test]
    pub fn test_solver() {
        let challenge = new();
        let output = challenge.solve();
        assert_eq!(true, challenge.verify(&output));
    }
}
