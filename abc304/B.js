const main = (arg) => {
  const input = arg.trim();

  if (input.length < 4) {
    console.log(input);
  } else {
    console.log(input.slice(0, 3) + "0".repeat(input.length - 3));
  }
};
main(require("fs").readFileSync("/dev/stdin", "utf8"));

// test
main("0");
main("20230603");
