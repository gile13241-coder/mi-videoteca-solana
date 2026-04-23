// No se necesitan imports: web3, anchor, pg y más están disponibles globalmente en Solana Playground

describe("Mi Videoteca", () => {

  //////////////////////////// Test 1: Crear Videoteca /////////////////////////////////////
  it("crear_videoteca", async () => {
    // Calculamos la dirección PDA de la videoteca usando las mismas semillas del programa
    const [videotecaPDA] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Llamamos la instrucción crear_videoteca
    const txHash = await pg.program.methods
      .crearVideoteca("Mi Videoteca")
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Videoteca creada. TX: ${txHash}`);
    await pg.connection.confirmTransaction(txHash);

    // Verificamos que la cuenta fue creada correctamente
    const cuenta = await pg.program.account.videoteca.fetch(videotecaPDA);
    console.log("Nombre de videoteca:", cuenta.nombre);
    console.log("Owner:", cuenta.owner.toString());
    console.log("Juegos:", cuenta.juegos);

    assert(cuenta.nombre === "Mi Videoteca");
    assert(cuenta.juegos.length === 0);
  });

  //////////////////////////// Test 2: Agregar Juegos /////////////////////////////////////
  it("agregar_juego", async () => {
    const [videotecaPDA] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Agregamos el primer juego
    const tx1 = await pg.program.methods
      .agregarJuego("The Legend of Zelda: BOTW", "Aventura", 2017)
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
      })
      .rpc();
    await pg.connection.confirmTransaction(tx1);
    console.log("Juego 1 agregado. TX:", tx1);

    // Agregamos el segundo juego
    const tx2 = await pg.program.methods
      .agregarJuego("Elden Ring", "RPG", 2022)
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
      })
      .rpc();
    await pg.connection.confirmTransaction(tx2);
    console.log("Juego 2 agregado. TX:", tx2);

    // Verificamos que los juegos fueron agregados
    const cuenta = await pg.program.account.videoteca.fetch(videotecaPDA);
    console.log("Juegos en videoteca:", cuenta.juegos);

    assert(cuenta.juegos.length === 2);
    assert(cuenta.juegos[0].nombre === "The Legend of Zelda: BOTW");
    assert(cuenta.juegos[1].nombre === "Elden Ring");
    assert(cuenta.juegos[0].completado === false); // Por defecto no completado
  });

  //////////////////////////// Test 3: Alternar Completado /////////////////////////////////////
  it("alternar_completado", async () => {
    const [videotecaPDA] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Marcamos Zelda como completado
    const tx = await pg.program.methods
      .alternarCompletado("The Legend of Zelda: BOTW")
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
      })
      .rpc();
    await pg.connection.confirmTransaction(tx);
    console.log("Estado alternado. TX:", tx);

    // Verificamos el cambio
    const cuenta = await pg.program.account.videoteca.fetch(videotecaPDA);
    console.log("Zelda completado:", cuenta.juegos[0].completado);

    assert(cuenta.juegos[0].completado === true); // Ahora sí está completado
  });

  //////////////////////////// Test 4: Ver Juegos /////////////////////////////////////
  it("ver_juegos", async () => {
    const [videotecaPDA] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Ver juegos imprime en el log de la transacción
    const tx = await pg.program.methods
      .verJuegos()
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
      })
      .rpc();
    await pg.connection.confirmTransaction(tx);
    console.log(`Ver logs con: solana confirm -v ${tx}`);
  });

  //////////////////////////// Test 5: Eliminar Juego /////////////////////////////////////
  it("eliminar_juego", async () => {
    const [videotecaPDA] = await web3.PublicKey.findProgramAddressSync(
      [Buffer.from("videoteca"), pg.wallet.publicKey.toBuffer()],
      pg.program.programId
    );

    // Eliminamos Elden Ring
    const tx = await pg.program.methods
      .eliminarJuego("Elden Ring")
      .accounts({
        owner: pg.wallet.publicKey,
        videoteca: videotecaPDA,
      })
      .rpc();
    await pg.connection.confirmTransaction(tx);
    console.log("Juego eliminado. TX:", tx);

    // Verificamos que solo queda 1 juego
    const cuenta = await pg.program.account.videoteca.fetch(videotecaPDA);
    console.log("Juegos restantes:", cuenta.juegos);

    assert(cuenta.juegos.length === 1);
    assert(cuenta.juegos[0].nombre === "The Legend of Zelda: BOTW");
  });

});
