#[derive(Debug)]
pub enum GenPattError {
    InvalidN,
    InvalidLen,
}

#[derive(Debug)]
pub enum FindPattError {
    PatternNotFound,
    NegativeOffset,

}

pub fn gen_patt(len: usize, n: usize) -> Result<String, GenPattError> {

    papillon::print_banner();

    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let k = alphabet.len();
    let mut a = vec![0; n*k];
    let mut sequence: Vec<usize> = Vec::new();

    if len == 0 || len > k.pow(n as u32) {
        return Err(GenPattError::InvalidLen);
    }

    if n == 0 || n > 26 {
        return Err(GenPattError::InvalidN);
    }

    fn de_bruijn(t: usize, p: usize, k: usize, n: usize, sequence: &mut Vec<usize>, a: &mut Vec<usize>, len: usize) {
        if sequence.len() >= len {
            return;
        }

        if t > n {
            if n % p == 0 {
                sequence.extend_from_slice(&a[1..=p]);
            }
        }

        else {
            a[t] = a[t-p];
            de_bruijn(t+1, p, k, n, sequence, a, len);
            for j in (a[t-p]+1)..k {
                a[t] = j;
                de_bruijn(t+1, t, k, n, sequence, a, len);
            }
        }
    }

    de_bruijn(1, 1, k, n, &mut sequence, &mut a, len);

    let result: String = sequence
        .into_iter()
        .map(|i| alphabet.chars().nth(i).unwrap())
        .collect();

    Ok(result)
}

pub fn find_offset(sequence: &str, subsequence: &str) -> Result<usize, FindPattError> {
    match sequence.find(subsequence) {
        Some(offset) => {
            match offset.checked_sub(1) {
                Some(offset) => Ok(offset),
                None => {
                    Err(FindPattError::NegativeOffset)
                }
            }
        }
        None => {
            Err(FindPattError::PatternNotFound)
        }
    }
}
