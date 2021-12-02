const anchor = require('@project-serum/anchor');

const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log(' Starting test...');

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CatchAFeel;
  
  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.initialize({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log(" Your transaction signature", tx);

  // Fetch data from 'account'
  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(':eyes: Feeler Count', account.totalFeelers.toString())

  function Something(src, transform) {
    this.src = src;
    this.transform = transform;
  }

  function Transform(pos, rot, scale) {
    this.position = pos;
    this.rotation = rot;
    this.scale = scale;
  }

  let aTransform = new Transform([0,0,0], [0,0,0], [1,1,1]);

  let aSomething = new Something('', aTransform);

  let somethings = [aSomething];

  await program.rpc.addFeel(somethings, {
    accounts: {
      user: provider.wallet.publicKey,
      baseAccount: baseAccount.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log(':eyes: Feeler Count', account.totalFeelers.toString());

  console.log(':eyes: Feeler List', account.feels.toString());
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (err) {
    console.error(err);
    process.exit(1);
  }
}

runMain();
