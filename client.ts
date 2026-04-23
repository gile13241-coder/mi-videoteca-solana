// Client - Mi Videoteca en Solana

console.log("🎮 Mi Videoteca - Solana Program");
console.log("================================");
console.log("Mi dirección (wallet):", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Mi balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

// Calculamos la dirección de nuestra videoteca (PDA)
const [videotecaPDA, bump] = await web3.PublicKey.findProgramAddressSync(
  [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

console.log("Dirección de tu Videoteca (PDA):", videotecaPDA.toString());
console.log("Bump:", bump);

// Intentamos leer la videoteca si ya fue creada
try {
  const videoteca = await pg.program.account.videoteca.fetch(videotecaPDA);
  console.log("\n Tu videoteca existe:");
  console.log("  Nombre:", videoteca.nombre);
  console.log("  Juegos guardados:", videoteca.juegos.length);
  videoteca.juegos.forEach((juego, i) => {
    console.log(`  [${i + 1}] ${juego.nombre} (${juego.genero}, ${juego.anio}) - ${juego.completado ? " Completado" : " Pendiente"}`);
  });
} catch (e) {
  console.log("\n  Tu videoteca aún no ha sido creada. Ejecuta el test 'crear_videoteca' primero.");
}
