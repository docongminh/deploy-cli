import * as anchor from "@coral-xyz/anchor";

import { Program } from "@coral-xyz/anchor";
import { DeployCli } from "../target/types/deploy_cli";

describe("deploy-cli", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DeployCli as Program<DeployCli>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
