import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { VibePrograms } from '../target/types/vibe_programs';

describe('vibe-programs', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.VibePrograms as Program<VibePrograms>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
