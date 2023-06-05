const main = (arg) => {
  const input = arg.trim().split("\n");

  const N = parseInt(input[0]);

  const nameAndAges = [];
  for (let i = 0; i < N; i++) {
    const [name, age] = input[i + 1].split(" ");
    nameAndAges.push([name, parseInt(age)]);
  }
  // find min of ages
  const ages = nameAndAges.map(([, age]) => age);
  const min = Math.min(...ages);
  // find index of min
  const index = ages.indexOf(min);
  // iterate over nameAndAges and print name of each person, starting from index
  // we also need to print all of the names
  for (let i = index; i < nameAndAges.length; i++) {
    console.log(nameAndAges[i][0]);
  }
  for (let i = 0; i < index; i++) {
    console.log(nameAndAges[i][0]);
  }
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

const input = `5
alice 31
bob 41
carol 5
dave 92
ellen 65
`;
main(input);
