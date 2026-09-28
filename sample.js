function greet(name) {
  const cleaned = name.trim();
  if (cleaned === "") {
    return [null, "name must not be empty"];
  }
  return [`Hello, ${cleaned}!`, null];
}

const name = process.argv[2] ?? "world";
const [message, err] = greet(name);
if (!message) {
  console.error(err);
  process.exit(1);
}

console.log(message);
