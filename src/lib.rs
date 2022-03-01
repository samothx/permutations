pub struct Permutator<T: Clone> {
    length: usize,
    value: Option<T>,
    rec_permutator: Option<Box<Permutator<T>>>,
    curr_perm: Option<Vec<T>>,
    curr_pos: usize,
    active: bool
}

impl<T: Clone> Permutator<T> {
    pub fn new(values: &[T]) -> Permutator<T> {
        if values.is_empty() {
            Permutator {
                length: 0,
                active: false,
                value: None,
                rec_permutator: None,
                curr_perm: None,
                curr_pos: 0,
            }
        } else {
            let length = values.len();
            let value = values[0].clone();
            Permutator {
                length,
                active: true,
                value: Some(value),
                curr_perm: None,
                curr_pos: 0,
                rec_permutator: if length == 1 { None } else { Some(Box::new(Permutator::new(&values[1..]))) }
            }
        }
    }
}

impl<T: Clone> Iterator for Permutator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.active {
            if self.length == 1 {
                self.active = false;
                return Some(vec![self.value.as_ref().unwrap().clone()]);
            } else {
                let perm = match self.curr_perm.as_ref() {
                    Some(perm) => {
                        perm
                    },
                    None => {
                        self.curr_perm = self.rec_permutator.as_mut().expect("Unexpected missing recursive permutation").next();
                        if let Some(perm) = self.curr_perm.as_ref() {
                            self.curr_pos = 0;
                            perm
                        } else {
                            self.active = false;
                            return None;
                        }
                    }
                };
                assert!(self.curr_pos < self.length, "Invalid current position {}>={}", self.curr_pos, self.length);
                let mut res = Vec::with_capacity(self.length);
                if self.curr_pos == self.length - 1 {
                    for read_pos in 0..perm.len() {
                        res.push(perm[read_pos].clone());
                    }
                    res.push(self.value.as_ref().unwrap().clone());
                    self.curr_perm = None;
                } else {
                    for read_pos in 0..perm.len() {
                        if read_pos == self.curr_pos {
                            res.push(self.value.as_ref().unwrap().clone());
                        }
                        res.push(perm[read_pos].clone());
                    }
                    self.curr_pos += 1;
                }
                Some(res)
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let mut permutation = Permutator::new(&["test"]);
        assert_eq!(permutation.next(),Some(vec!["test"]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&[1,2]);
        assert_eq!(permutation.next(),Some(vec![1,2]));
        assert_eq!(permutation.next(),Some(vec![2,1]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&[1,2,3]);
        assert_eq!(permutation.next(),Some(vec![1,2,3]));
        assert_eq!(permutation.next(),Some(vec![2,1,3]));
        assert_eq!(permutation.next(),Some(vec![2,3,1]));
        assert_eq!(permutation.next(),Some(vec![1,3,2]));
        assert_eq!(permutation.next(),Some(vec![3,1,2]));
        assert_eq!(permutation.next(),Some(vec![3,2,1]));
        assert_eq!(permutation.next(),None);

        let mut permutation = Permutator::new(&["1","2","3"]);
        assert_eq!(permutation.next(),Some(vec!["1","2","3"]));
        assert_eq!(permutation.next(),Some(vec!["2","1","3"]));
        assert_eq!(permutation.next(),Some(vec!["2","3","1"]));
        assert_eq!(permutation.next(),Some(vec!["1","3","2"]));
        assert_eq!(permutation.next(),Some(vec!["3","1","2"]));
        assert_eq!(permutation.next(),Some(vec!["3","2","1"]));
        assert_eq!(permutation.next(),None);

    }
}
