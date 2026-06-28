import * as readline from "readline";

const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

const ask = (q: string): Promise<string> => new Promise(res => rl.question(q, res));

async function main() {
  console.log("Welcome to the Python Developer Job Portal!");
  console.log("-".repeat(45));

  const python = (await ask("Do you know Python? (yes/no): ")).trim().toLowerCase();
  const experience = parseInt((await ask("How many years of experience or projects do you have? ")).trim());
  const degree = (await ask("Do you have a CS degree or completed a Bootcamp? (yes/no): ")).trim().toLowerCase();

  if (python === "yes" && (experience >= 2 || degree === "yes")) {
    console.log("Congratulations! You have been accepted to the next interview stage.");
  } else {
    console.log("Sorry, your current qualifications do not match the job requirements.");
  }

  rl.close();
}

main();
