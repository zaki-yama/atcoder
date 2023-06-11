const main = (arg) => {
  const input = arg.trim().split("\n");
  const N = parseInt(input[0]);
  const divided = parseInt(N / 5);
  const remainder = N % 5;

  if (remainder > 2) {
    console.log((divided + 1) * 5);
  } else {
    console.log(divided * 5);
  }
};
// main(require("fs").readFileSync("/dev/stdin", "utf8"));

let input = `53
`;
main(input);

input = `21
`;
main(input);

input = `100
`;
main(input);
