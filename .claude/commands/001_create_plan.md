# Plan

Please create a plan to implement threshold signature verification in SP1 of zkVM using ed25519-dalek for threshold signing.

Please generate the plan to:

<project-dir>/.claude/commands/002_plan.md

The pland should include creating a Rust program that:

* implements threshold signing using ed25519-dalek
* In SP1 validate the combined signature
* Compile to RISC-V using SP1 tooling.
* Generate a STARK proof of execution.
* Submit the proof to a Solidity verifier on-chain.

Ordinariy in a final productm we would create software for each Threshold Signing node.

What I want instead is to:

* show serialisation and deserialisation of what would be sent to threshold nodes in the code but not to send that over the network.

For example the code could look similar to this pseudo code:

```
function receivedSerializedargs( serializedArgs ) {
  const deserializedArgs = serializeHelper.deserializeArgs( serializedArgs );
  // do some threshold signing
  return getSignature( desserializedArgs);
}

function sendToSigner( someArgs ) {
   const serialisedArgs = serializeHelper.serialize( someArgs );
   receiveSerializedArgs( serializedArgs );
}

main() {
  const signature = sendToSigner( someArgs );
}
```

So you can see that we demonstrate how to serialize and deserialize but we are not actually starting 4 threshold signing nodes.

The idea is that this Proof of Concept can later be cloned and modified to make network calls.

I think you should start by making some directories like these.

<project-dir>/solidity_threshold_signing
<project-dir>/rust_threshold_siging

In your plan please include all the necessary steps.  If you find it useful you can show some code snippets in the plan.

We will later use claude to execute 002_plan.md.  Your job is to generate the plan.




