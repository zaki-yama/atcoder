const main = (arg) => {
  const input = arg.trim().split("\n");
  const N = parseInt(input[0]);
  const A = input[1].split(" ").map((v) => parseInt(v));

  const count = new Array(N + 1).fill(0);
  const ans = [];
  for (let i = 0; i < A.length; i++) {
    count[A[i]] += 1;
    if (count[A[i]] === 2) {
      ans.push(A[i]);
    }
  }
  console.log(ans.join(" "));
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

input = `3
1 1 3 2 3 2 2 3 1
`;
main(input);

input = `1
1 1 1
`;
main(input);

/*
const main = (arg) => {
  const input = arg.trim().split("\n");
  const N = parseInt(input[0]);
  const A = input[1].split(" ").map((v) => parseInt(v));

  const fiMap = new Map();
  for (let i = 0; i < N; i++) {
    // console.log("---search f[" + (i + 1) + "]----");
    let foundNum = 0;
    for (let j = 0; j < A.length; j++) {
      if (A[j] === i + 1) {
        // console.log(`A[${j}] is ${A[j]}. matches to ${i + 1}`);
        foundNum++;
        if (foundNum === 2) {
          fiMap.set(i + 1, j);
          break;
        }
      }
    }
    // console.log(fiMap);
  }
  const mapSort1 = new Map([...fiMap.entries()].sort((a, b) => a[1] - b[1]));
  console.log(Array.from(mapSort1.keys()).join(" "));
};
*/
