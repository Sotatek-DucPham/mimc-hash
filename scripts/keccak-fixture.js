const Web3Utils = require("web3-utils");

const SEED = "mimcsponge";
let c = Web3Utils.keccak256(SEED);
console.log(c);

const n1 = Web3Utils.toBN(c);
console.log(n1.toString());

c = Web3Utils.keccak256(c);
const n2 = Web3Utils.toBN(c);
console.log(n2.toString());

c = Web3Utils.keccak256(c);
const n3 = Web3Utils.toBN(c);
console.log({ n3: n3.toString() });

const c3 = Web3Utils.padLeft(Web3Utils.toHex(n3), 64);
console.log({ c3: Web3Utils.toBN(c3).toString() });
