const main = (arg) => {
  const input = arg.trim().split("\n");
  const A = input[0].split(" ");

  const reversed = A.reverse();
  console.log(BigInt(`0b${reversed.join("")}`).toString(10));
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

input = `1 0 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
`;
main(input);

input = `1 0 1 0 1 0 0 0 0 1 0 0 1 1 0 1 1 1 1 0 0 0 1 0 0 1 1 1 1 1 1 0 0 0 0 1 0 1 0 1 0 1 1 1 1 0 0 1 1 0 0 0 0 1 0 1 0 1 0 1 0 0 0 0
`;
main(input);
