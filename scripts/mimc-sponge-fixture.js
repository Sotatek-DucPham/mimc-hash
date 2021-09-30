const mimcSponge = require("circomlib/src/mimcsponge.js");
const SEED = "mimcsponge";
const NROUNDS = 220;

let res1 = mimcSponge.hash(1, 2, 3);
console.log(res1.xL.toString(), res1.xR.toString());

console.log("---cts---");
const cts = mimcSponge.getConstants(SEED, NROUNDS);
console.log(cts[100]);
