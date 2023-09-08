import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MetaplexAnchorBoilerplate } from "../target/types/metaplex_anchor_boilerplate";

describe("metaplex-anchor-boilerplate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MetaplexAnchorBoilerplate as Program<MetaplexAnchorBoilerplate>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
