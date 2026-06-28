import * as readline from "readline";

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

const ask = (q: string): Promise<string> => new Promise(res => rl.question(q, res));

async function main() {
  console.log("Welcome, dear customer!");

  const input = await ask("Please enter the amount: ");
  const amount = parseFloat(input);

  if (amount <= 0) {
    console.log("The amount cannot be zero or negative!");
  } else {
    let discount = 0;
    if (amount >= 500) discount = amount * 0.2;
    else if (amount >= 100) discount = amount * 0.1;

    console.log(`Your available discount is: ${discount.toFixed(2)}`);
    console.log(`Your total amount after discount is: ${(amount - discount).toFixed(2)}`);
  }

  rl.close();
}

main();
