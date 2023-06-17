const main = (arg) => {
  const input = arg.trim().split("\n");
  const N = parseInt(input[0]);
  const S = input[1];

  let result = "";
  for (let i = 0; i < S.length; i++) {
    result += S[i] + S[i];
  }
  console.log(result);
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

input = `8
beginer
`;
main(input);

input = `3
aaa
`;
main(input);
