import { Deployer, Reporter } from "@solarity/hardhat-migrate";

import { Depositor__factory } from "@ethers-v6";

import { poseidonContract } from "circomlibjs";

export = async (deployer: Deployer) => {
  await deployer.deploy({
    contractName: "contracts/Depositor.sol:PoseidonUnit4L",
    bytecode: poseidonContract.createCode(4),
    abi: poseidonContract.generateABI(4),
  });

  const depositor = await deployer.deploy(Depositor__factory);

  Reporter.reportContracts(["Depositor", await depositor.getAddress()]);
};
