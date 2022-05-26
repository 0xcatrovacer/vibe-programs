import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VibePrograms } from "../target/types/vibe_programs";
import * as assert from "assert";

describe("vibe-programs", async () => {
    anchor.setProvider(anchor.Provider.env());

    const program = anchor.workspace.VibePrograms as Program<VibePrograms>;

    const author = program.provider.wallet;
    const vibe = anchor.web3.Keypair.generate();

    const [userPDA, _bump] = await anchor.web3.PublicKey.findProgramAddress(
        [
            anchor.utils.bytes.utf8.encode("vibe_user"),
            author.publicKey.toBuffer(),
        ],
        program.programId
    );
});
