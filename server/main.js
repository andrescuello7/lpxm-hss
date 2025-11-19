const { createConnection } = require("net");

/**
 * Cliente básico para conectarse al servidor SSH-MPLX
 * 
 * Uso: node server/main.js
 */

const HOST = process.argv[2] || "localhost";
const PORT = process.argv[3] || "22";

console.log(`Conectando a ${HOST}:${PORT}...`);

const socket = createConnection({ host: HOST, port: PORT }, () => {
  console.log("Conectado al servidor SSH-MPLX");
  console.log("Escribe comandos y presiona Enter. Escribe 'exit' para salir.\n");
});

socket.on('data', (buffer) => {
  process.stdout.write(buffer.toString());
});

socket.on('error', (err) => {
  console.error('Error de conexión:', err.message);
  if (err.code === 'ECONNREFUSED') {
    console.error('Asegúrate de que el servidor esté ejecutándose.');
  }
  process.exit(1);
});

socket.on('close', () => {
  console.log('\nConexión cerrada');
  process.exit(0);
});

process.stdin.on("data", (buffer) => {
  let data = buffer.toString().trim();
  
  if (data === 'exit' || data === 'quit') {
    socket.end();
    return;
  }
  
  socket.write(data + "\n");
});

// Manejar Ctrl+C
process.on('SIGINT', () => {
  console.log('\nCerrando conexión...');
  socket.end();
});
