import * as readline from "readline";

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

const ask = (q: string): Promise<string> => new Promise(res => rl.question(q, res));

async function main() {
  const pin = 1234;
  let balance = 5000;

  const pinInput = parseInt((await ask("Please enter your 4-digit PIN: ")).trim());

  if (pinInput === pin) {
    const choice = (await ask("would you like to withdraw or access your balance? ")).trim().toLowerCase();

    if (choice === "access balance") {
      console.log(`Your current balance is: $${balance}`);
    } else if (choice === "withdraw") {
      const amount = parseFloat((await ask("Enter the amount you want to withdraw: ")).trim());

      if (amount > 0) {
        if (amount <= balance) {
          balance -= amount;
          console.log(`Withdrawal successful $${amount}! Your new balance is: $${balance}`);
        } else {
          console.log("Sorry, your balance is insufficient.");
        }
      } else {
        console.log("Invalid amount.");
      }
    } else {
      console.log("Invalid option. Please choose 'withdraw' or 'access balance'.");
    }
  } else {
    console.log("Incorrect PIN. Please try again.");
  }

  rl.close();
}

main();
