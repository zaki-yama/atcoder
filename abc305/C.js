const main = (arg) => {
  const input = arg.trim().split("\n");
  const p = input[0].split(" ")[0];
  const q = input[0].split(" ")[1];

  const distance = {
    A: 0,
    B: 3,
    C: 3 + 1,
    D: 3 + 1 + 4,
    E: 3 + 1 + 4 + 1,
    F: 3 + 1 + 4 + 1 + 5,
    G: 3 + 1 + 4 + 1 + 5 + 9,
  };

  const pDistance = distance[p];
  const qDistance = distance[q];

  console.log(Math.abs(pDistance - qDistance));
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

let input = `5 6
......
..#.#.
..###.
..###.
......
`;
main(input);

/*
input = `3 2
#.
##
##
`;
main(input);

input = `6 6
..####
..##.#
..####
..####
..####
......
`;
main(input);
*/
