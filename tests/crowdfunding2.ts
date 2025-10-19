import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Crowdfunding2 } from "../target/types/crowdfunding2";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";

describe("crowdfunding2", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider;

  anchor.setProvider(provider.env());

  const program = anchor.workspace.crowdfunding2 as Program<Crowdfunding2>;

  const payer = Keypair.generate();

  it("create", async () => {
    // Add your test here.
    const tx = await program.methods
      .create("Hello", "World")
      .accounts({
        campaign: payer.publicKey,
      })
      .signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);
    const campaignAccount = await program.account.campaign.fetch(
      payer.publicKey
    );
    console.log("campaign name:", campaignAccount.name);
    console.log("campaign description:", campaignAccount.description);
    assert.ok(campaignAccount.name, "Hello");
    assert.ok(campaignAccount.description, "World");
  });
});
