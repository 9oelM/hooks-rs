# Getting started

## The problem

Hooks is a smart contract API on XRPL, and hooks-rs is a tool to write the smart contract in Rust.

At the time of writing, the only known and established way of writing is to write it in C. However, there are many problems with C as everyone knows: undefined behaviors, weak language intrinsics, bad developer experience, etc.

This is where the robustness of Rust comes into play: we want to code with confidence. We are writing something that potentially affects the balances of many wallets. In that regard, C is not the most optimal choice, because it makes you fall into many undiscovered traps as discussed. Rust is a really good candidate, because it is not only low level but also provides great developer experience and language semantics that naturally guide you to write better and safer code.

This book will walk you through basics as well as how to make the most of using hooks-rs.
