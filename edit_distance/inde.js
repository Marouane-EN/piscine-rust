// JavaScript program to find minimum number
// of operations to convert s1 to s2

// Function to find the minimum number
// of operations to convert s1 to s2
function editDistance(s1, s2) {
  let m = s1.length;
  let n = s2.length;

  // Create a table to store results of subproblems
  let dp = Array.from({ length: m + 1 }, () => Array(n + 1).fill(0));

  // Fill the known entries in dp[][]
  // If one string is empty, then answer
  // is length of the other string
  for (let i = 0; i <= m; i++) dp[i][0] = i;
  for (let j = 0; j <= n; j++) dp[0][j] = j;

  console.log(dp);
  // Fill the rest of dp[][]
  for (let i = 1; i <= m; i++) {
    for (let j = 1; j <= n; j++) {
      if (s1[i - 1] === s2[j - 1]) {
        dp[i][j] = dp[i - 1][j - 1];
        console.log("+++++++++++++++++++++++");

        console.log(dp, dp[i - 1][j - 1], i, j);
        console.log("-----------------------");

      } else {
        console.log("=====================");

        console.log(
          "i=",
          i,
          "j=",
          j,
          "db",
          dp[i][j - 1],
          dp[i - 1][j],
          dp[i - 1][j - 1]
        );
        console.log("*******************");

        dp[i][j] = 1 + Math.min(dp[i][j - 1], dp[i - 1][j], dp[i - 1][j - 1]);

        console.log(dp);

        console.log("*******************");


      }
    }
  }

  return dp[m][n];
}

// Driver Code
let s1 = "abcd";

let s2 = "bcfe";

console.log(editDistance(s1, s2));
