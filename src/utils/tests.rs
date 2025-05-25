/// Computes the Levenshtein distance between two string slices.
///
/// The Levenshtein distance is the minimum number of single-character
/// edits (insertions, deletions or substitutions) required to change
/// one string into the other.
///
/// # Arguments
///
/// * `s1` - The first input string slice.
/// * `s2` - The second input string slice.
///
/// # Returns
///
/// The Levenshtein distance between `s1` and `s2`, i.e. the count of
/// insertions, deletions, and substitutions needed to transform `s1` into `s2`.
///
/// # Examples
///
/// ```rust
/// use your_crate::levenshtein_distance;
///
/// let dist = levenshtein_distance("kitten", "sitting");
/// assert_eq!(dist, 3);
///
/// let dist2 = levenshtein_distance("flaw", "lawn");
/// assert_eq!(dist2, 2);
/// ```
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
  let len_s1 = s1.len();
  let len_s2 = s2.len();

  // Create a 2D matrix to store distances
  let mut dp = vec![vec![0; len_s2 + 1]; len_s1 + 1];

  // Initialize the first row/column
  for (i, row) in dp.iter_mut().enumerate() {
    row[0] = i;
  }
  for (j, cell) in dp[0].iter_mut().enumerate() {
    *cell = j;
  }

  // Compute the cost of deletions, insertions, and substitutions
  for i in 1..=len_s1 {
    for j in 1..=len_s2 {
      let cost = if s1.as_bytes()[i - 1] == s2.as_bytes()[j - 1] {
        0
      } else {
        1
      };
      dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1] + cost)
        + if cost == 0 {
          0
        } else {
          // Substitution cost is already accounted for, only add extra if it's insertion or deletion
          if dp[i - 1][j] == dp[i][j - 1] { 1 } else { 0 }
        };
    }
  }

  dp[len_s1][len_s2]
}
