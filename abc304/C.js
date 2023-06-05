const main = (arg) => {
  const input = arg.trim().split("\n");

  const N = parseInt(input[0].split(" ")[0]);
  const D = parseInt(input[0].split(" ")[1]);

  const infected = [];
  const findInfected = (myIndex) => {
    const [myX, myY] = input[myIndex + 1].split(" ").map((x) => parseInt(x));
    for (let i = 0; i < N; i++) {
      const [xi, yi] = input[i + 1].split(" ").map((x) => parseInt(x));
      const distance = Math.sqrt((myX - xi) ** 2 + (myY - yi) ** 2);
      if (distance > 0 && distance <= D && !infected.includes(i)) {
        infected.push(i);
        findInfected(i);
      }
    }
  };
  findInfected(0);

  for (let i = 0; i < N; i++) {
    console.log(infected.includes(i) ? "Yes" : "No");
  }
};
// main(require("fs").readFileSync("/dev/stdin", "utf8"));

let input = `4 5
2 -1
3 1
8 8
0 5
`;
input = `9 4
3 2
6 -1
1 6
6 5
-2 -3
5 3
2 -3
2 1
2 6
`;
main(input);
