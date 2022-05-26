import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VibePrograms } from "../target/types/vibe_programs";
import * as assert from "assert";

describe("vibe-programs", async () => {
    anchor.setProvider(anchor.Provider.env());

    const program = anchor.workspace.VibePrograms as Program<VibePrograms>;

    const author = program.provider.wallet;
    const vibe = anchor.web3.Keypair.generate();

    const [userPDA, _bump1] = await anchor.web3.PublicKey.findProgramAddress(
        [
            anchor.utils.bytes.utf8.encode("vibe_user"),
            author.publicKey.toBuffer(),
        ],
        program.programId
    );

    const [likePDA, _bump2] = await anchor.web3.PublicKey.findProgramAddress(
        [author.publicKey.toBuffer(), vibe.publicKey.toBuffer()],
        program.programId
    );

    const comment = anchor.web3.Keypair.generate();

    const newUser = anchor.web3.Keypair.generate();

    const [newUserPDA, _bump3] = await anchor.web3.PublicKey.findProgramAddress(
        [
            anchor.utils.bytes.utf8.encode("vibe_user"),
            newUser.publicKey.toBuffer(),
        ],
        program.programId
    );

    const [followPDA1, _bump4] = await anchor.web3.PublicKey.findProgramAddress(
        [newUser.publicKey.toBuffer(), author.publicKey.toBuffer()],
        program.programId
    );

    const [followPDA2, _bump5] = await anchor.web3.PublicKey.findProgramAddress(
        [author.publicKey.toBuffer(), newUser.publicKey.toBuffer()],
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

    it("can create a vibe", async () => {
        await program.rpc.initVibe("Vibe Title", "Vibe Content", true, {
            accounts: {
                vibe: vibe.publicKey,
                user: userPDA,
                author: author.publicKey,
                clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [vibe],
        });

        const createdVibe = await program.account.vibe.fetch(vibe.publicKey);

        assert.equal(createdVibe.vibeTitle, "Vibe Title");
        assert.equal(createdVibe.vibeContent, "Vibe Content");
        assert.equal(
            createdVibe.author.toBase58(),
            program.provider.wallet.publicKey.toBase58()
        );
        assert.equal(createdVibe.version, 0);
        assert.equal(createdVibe.likes, 0);
        assert.equal(createdVibe.comments, 0);
        assert.equal(createdVibe.allowedComments, true);
        assert.ok(createdVibe.timestamp);
    });

    it("can like a vibe", async () => {
        await program.rpc.addLike({
            accounts: {
                like: likePDA,
                vibe: vibe.publicKey,
                liker: author.publicKey,
                user: userPDA,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        });

        const likeAccount = await program.account.like.fetch(likePDA);
        const likedVibe = await program.account.vibe.fetch(vibe.publicKey);

        assert.equal(likeAccount.vibe.toBase58(), vibe.publicKey.toBase58());
        assert.equal(likeAccount.liker.toBase58(), author.publicKey.toBase58());
        assert.equal(likedVibe.likes, 1);
    });

    it("can add a comment", async () => {
        await program.rpc.addComment("New Comment", {
            accounts: {
                comment: comment.publicKey,
                vibe: vibe.publicKey,
                commentor: author.publicKey,
                user: userPDA,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [comment],
        });

        const createdComment = await program.account.comment.fetch(
            comment.publicKey
        );
        const commentedVibe = await program.account.vibe.fetch(vibe.publicKey);
        const commentorAccount = await program.account.user.fetch(userPDA);

        assert.equal(createdComment.vibe.toBase58(), vibe.publicKey.toBase58());
        assert.equal(
            createdComment.commentor.toBase58(),
            author.publicKey.toBase58()
        );
        assert.equal(createdComment.content, "New Comment");
        assert.equal(commentedVibe.comments, 1);
        assert.equal(commentorAccount.comments, 1);
    });

    it("can remove a like", async () => {
        await program.rpc.removeLike({
            accounts: {
                like: likePDA,
                vibe: vibe.publicKey,
                liker: author.publicKey,
                user: userPDA,
            },
        });

        const unlikedVibe = await program.account.vibe.fetch(vibe.publicKey);

        assert.equal(unlikedVibe.likes, 0);
    });

    it("can remove a comment", async () => {
        await program.rpc.removeComment({
            accounts: {
                comment: comment.publicKey,
                vibe: vibe.publicKey,
                commentor: author.publicKey,
                user: userPDA,
            },
        });

        const uncommentedVibe = await program.account.vibe.fetch(
            vibe.publicKey
        );
        const uncommentor = await program.account.user.fetch(userPDA);

        assert.equal(uncommentedVibe.comments, 0);
        assert.equal(uncommentor.comments, 0);
    });

    it("can remove a vibe", async () => {
        await program.rpc.removeVibe({
            accounts: {
                vibe: vibe.publicKey,
                user: userPDA,
                author: author.publicKey,
            },
        });

        const removeUser = await program.account.user.fetch(userPDA);

        assert.equal(removeUser.vibes, 0);
    });

    it("can follow another user", async () => {
        const signature = await program.provider.connection.requestAirdrop(
            newUser.publicKey,
            1000000000
        );
        await program.provider.connection.confirmTransaction(signature);

        await program.rpc.initUser("New User Nick", {
            accounts: {
                user: newUserPDA,
                author: newUser.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [newUser],
        });

        await program.rpc.follow({
            accounts: {
                follow: followPDA1,
                followed: newUser.publicKey,
                follower: author.publicKey,
                followedAccount: newUserPDA,
                followerAccount: userPDA,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
        });

        const followAccount = await program.account.follow.fetch(followPDA1);
        const followedUser = await program.account.user.fetch(newUserPDA);
        const followerUser = await program.account.user.fetch(userPDA);

        assert.equal(
            followAccount.followed.toBase58(),
            newUser.publicKey.toBase58()
        );
        assert.equal(
            followAccount.follower.toBase58(),
            author.publicKey.toBase58()
        );
        assert.equal(followedUser.followers, 1);
        assert.equal(followedUser.followings, 0);
        assert.equal(followerUser.followers, 0);
        assert.equal(followerUser.followings, 1);
    });

    it("can follow back user", async () => {
        await program.rpc.follow({
            accounts: {
                follow: followPDA2,
                followed: author.publicKey,
                follower: newUser.publicKey,
                followedAccount: userPDA,
                followerAccount: newUserPDA,
                systemProgram: anchor.web3.SystemProgram.programId,
            },
            signers: [newUser],
        });

        const newFollowAccount = await program.account.follow.fetch(followPDA2);
        const newFollowedUser = await program.account.user.fetch(newUserPDA);
        const newFollowerUser = await program.account.user.fetch(userPDA);

        assert.equal(
            newFollowAccount.followed.toBase58(),
            author.publicKey.toBase58()
        );
        assert.equal(
            newFollowAccount.follower.toBase58(),
            newUser.publicKey.toBase58()
        );
        assert.equal(newFollowedUser.followers, 1);
        assert.equal(newFollowedUser.followings, 1);
        assert.equal(newFollowerUser.followers, 1);
        assert.equal(newFollowerUser.followings, 1);
    });
});
