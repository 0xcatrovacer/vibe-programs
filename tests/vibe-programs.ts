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

    it("can create user account", async () => {
        await program.rpc.initUser("Nickname", {
            accounts: {
                user: userPDA,
                author: author.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        });

        const createdUser = await program.account.user.fetch(userPDA);

        assert.equal(
            createdUser.userKey.toBase58(),
            author.publicKey.toBase58()
        );
        assert.equal(createdUser.nick, "Nickname");
        assert.equal(createdUser.vibes, 0);
        assert.equal(createdUser.comments, 0);
        assert.equal(createdUser.followers, 0);
        assert.equal(createdUser.followings, 0);
    });

    it("can update user nickname", async () => {
        await program.rpc.updateNickname("New Nickname", {
            accounts: {
                user: userPDA,
                author: author.publicKey,
            },
        });

        const updatedUser = await program.account.user.fetch(userPDA);

        assert.equal(updatedUser.nick, "New Nickname");
    });
});
