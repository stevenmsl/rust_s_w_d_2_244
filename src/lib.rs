use std::cmp;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Solution {
  indexes: HashMap<String, Vec<usize>>,
}

/* key takeaways
   - word distance means difference between the
     word1's index and word2's index
   - same word can appear more than once in the
     input array
   - construct a hashmap to improve the speed
     as the shortest will be called multiple times
   - take advantage of how we build the hashmap
     - let say word1 appears in index 1,3, and 5,
       while word2 appears in index 2, 7, 10
     - the map will look like this
       "word1": [1,3,5]
       "word2": [2,7,10]
     - the indexes for each word will be in
       ascending order
     - so we calculate the distance as such:
       - abs(1-2) == 1 and since 1 < 2, there is
         no point to continue with 1, as the next
         index from word2 will be even bigger (7),
         which means the distance will be bigger
       - so for word1 we can move to the next index 3
         and ignore the comparisons between 1 and 7
         and 1 and 10

*/

impl Solution {
  /*
    - each word can appear multiple times
      as the value of the hashmap needs
      to be a Vec<usize> to track all
      indexes
    - for each word its index Vec is in
      ascending order
      - this is due to how we populate
        the index Vec
  */
  pub fn from(words: &Vec<String>) -> Solution {
    let mut indexes: HashMap<String, Vec<usize>> = HashMap::new();
    for (index, word) in words.iter().enumerate() {
      indexes.entry(String::from(word)).or_insert(vec![]);
      indexes.get_mut(word).unwrap().push(index);
    }
    Solution { indexes: indexes }
  }

  pub fn shortest(&self, word1: &String, word2: &String) -> usize {
    let word1_ind_vec = self.indexes.get(word1).unwrap();
    let word2_ind_vec = self.indexes.get(word2).unwrap();

    /*
      - help to iterate through the index vec
    */
    let mut word1_ind = 0;
    let mut word2_ind = 0;
    let mut min = usize::MAX;
    while word1_ind < word1_ind_vec.len() && word2_ind < word2_ind_vec.len() {
      let word1_index_val = word1_ind_vec[word1_ind];
      let word2_index_val = word2_ind_vec[word2_ind];

      /*
        - a bit clumsy here to calculate the min; not sure if there
          is a better way
      */
      let mut dist = word1_index_val as isize - word2_index_val as isize;
      dist = isize::abs(dist);
      min = cmp::min(min, dist as usize);

      if word1_index_val < word2_index_val {
        /*
          - no point to compare word1_index_val
            with the rest of word2_index_val(s)
          - so move on to the next index
        */
        word1_ind += 1;
      } else {
        /* vice versa */
        word2_ind += 1;
      }
    }

    min
  }

  pub fn test_fixrure() -> Vec<String> {
    vec![
      String::from("practice"),
      String::from("makes"),
      String::from("perfect"),
      String::from("coding"),
      String::from("makes"),
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let sn = Solution::from(&Solution::test_fixrure());
    let dist = sn.shortest(&String::from("practice"), &String::from("coding"));
    assert_eq!(dist, 3);
  }
  #[test]
  fn test_2() {
    let sn = Solution::from(&Solution::test_fixrure());
    let dist = sn.shortest(&String::from("coding"), &String::from("makes"));
    assert_eq!(dist, 1);
  }
}
