import { Deployer, Reporter } from "@solarity/hardhat-migrate";

import { Depositor__factory } from "@ethers-v6";

import { poseidonContract } from "circomlibjs";

export = async (deployer: Deployer) => {
  await deployer.deploy({
    contractName: "contracts/Depositor.sol:PoseidonUnit1L",
    bytecode: poseidonContract.createCode(1),
    abi: poseidonContract.generateABI(1),
  });

  const depositor = await deployer.deploy(Depositor__factory);

  Reporter.reportContracts(["Depositor", await depositor.getAddress()]);
};
