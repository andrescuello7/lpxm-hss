const { createConnection } = require("net");

const socket = createConnection({ host: "0.0.0.0", port: "22" }, () => {
  socket.on('data', (buffer) => {
    console.log(buffer.toString());
  });
  process.stdin.on("data", (buffer) => {
    let data = buffer.toString().trim();
    socket.write(data + "\n");
  });
});
