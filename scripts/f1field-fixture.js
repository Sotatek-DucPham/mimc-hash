const Web3Utils = require("web3-utils");
const Scalar = require("ffjavascript").Scalar;
const ZqField = require("ffjavascript").ZqField;
const F = new ZqField(
  Scalar.fromString(
    "21888242871839275222246405745257275088548364400416034343698204186575808495617"
  )
);

console.log(F.e(0).toString());

const modRes = Web3Utils.toBN(
  "7382897923368232911732724532248874244391730858766999739770637613918792750368"
).mod(Web3Utils.toBN(F.p.toString()));
console.log({ modRes: modRes.toString() });
