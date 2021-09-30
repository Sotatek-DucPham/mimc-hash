const Web3Utils = require("web3-utils");
const Scalar = require("ffjavascript").Scalar;
const ZqField = require("ffjavascript").ZqField;
const F = new ZqField(
  Scalar.fromString(
    "21888242871839275222246405745257275088548364400416034343698204186575808495617"
  )
);
const SEED = "mimcsponge";
const NROUNDS = 220;

const getConstants = (seed, nRounds) => {
  const cts = new Array(nRounds);
  let c = Web3Utils.keccak256(SEED);
  let pivot = 74;
  for (let i = 1; i < nRounds; i++) {
    if (i == pivot) {
      console.log(Buffer.from(c.substring(2), "hex"));
      console.log(Web3Utils.toBN(c).toString());
    }
    c = Web3Utils.keccak256(c);
    if (i == pivot) {
      console.log(Buffer.from(c.substring(2), "hex"));
      console.log(Web3Utils.toBN(c).toString());
    }

    const n1 = Web3Utils.toBN(c).mod(Web3Utils.toBN(F.p.toString()));
    cts[i] = F.e(Web3Utils.toBN(n1).toString());
  }
  cts[0] = F.e(0);
  cts[cts.length - 1] = F.e(0);
  return cts;
};

console.log("---cts---");
const cts = getConstants(SEED, NROUNDS);
console.log(cts[100]);
