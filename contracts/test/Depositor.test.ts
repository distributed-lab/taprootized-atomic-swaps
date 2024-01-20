import { expect } from "chai";
import { ethers } from "hardhat";

import { SignerWithAddress } from "@nomicfoundation/hardhat-ethers/signers";
import { increase } from "@nomicfoundation/hardhat-network-helpers/dist/src/helpers/time";

import { Depositor } from "@ethers-v6";

import { Reverter, poseidonHash, getPoseidon } from "@utils";
import { impersonateAccount, setBalance, time } from "@nomicfoundation/hardhat-network-helpers";

describe("Taprootized Atomic Swaps", () => {
  const reverter = new Reverter();

  let DEPLOYER: SignerWithAddress;
  let USER1: SignerWithAddress;
  let USER2: SignerWithAddress;

  let depositor: Depositor;

  const LOCK_TIME = 3600; // One hour in seconds
  const DEPOSIT_AMOUNT = ethers.parseEther("1");

  before("setup", async () => {
    [DEPLOYER, USER1, USER2] = await ethers.getSigners();

    const DepositorFactory = await ethers.getContractFactory("Depositor", {
      libraries: {
        PoseidonUnit1L: await (await getPoseidon(1)).getAddress(),
      },
    });
    depositor = await DepositorFactory.connect(DEPLOYER).deploy();

    await reverter.snapshot();
  });

  afterEach(reverter.revert);

  it("should deposit ETH with correct details", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    const nextBlockTimestamp = (await time.latest()) + 1;
    await time.setNextBlockTimestamp(nextBlockTimestamp);

    await expect(depositor.connect(USER1).deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT }))
      .to.emit(depositor, "Deposited")
      .withArgs(USER1.address, USER2.address, DEPOSIT_AMOUNT, LOCK_TIME, secretHash);

    expect(await depositor.deposits(secretHash)).to.deep.equal([
      USER1.address,
      USER2.address,
      DEPOSIT_AMOUNT,
      nextBlockTimestamp + LOCK_TIME,
      false,
    ]);
  });

  it("should enforce minimum lock time", async () => {
    const shortLockTime = 1000;

    await expect(
      depositor.deposit(USER2.address, ethers.hexlify(ethers.randomBytes(32)), shortLockTime, { value: DEPOSIT_AMOUNT })
    )
      .to.be.revertedWithCustomError(depositor, "LockTimeTooShort")
      .withArgs(shortLockTime, LOCK_TIME);
  });

  it("should revert if trying to deposit with same secret hash", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await expect(depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT }))
      .to.be.revertedWithCustomError(depositor, "DepositAlreadyExists")
      .withArgs(secretHash);
  });

  it("should reject deposit to zero address", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await expect(
      depositor.deposit(ethers.ZeroAddress, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT })
    ).to.be.revertedWithCustomError(depositor, "ZeroAddressNotAllowed");
  });

  it("should reject deposit with insufficient amount", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await expect(depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: 0 })).to.be.revertedWithCustomError(
      depositor,
      "ZeroDepositAmount"
    );
  });

  it("should reject withdrawal with incorrect secret", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const incorrectSecret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await expect(depositor.withdraw(incorrectSecret))
      .to.be.revertedWithCustomError(depositor, "DepositDoesNotExist")
      .withArgs(poseidonHash(incorrectSecret));
  });

  it("should allow withdrawal with correct secret", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await expect(depositor.withdraw(secret))
      .to.emit(depositor, "Withdrawn")
      .withArgs(USER2.address, DEPOSIT_AMOUNT, secret, secretHash);
  });

  it("should prevent double withdrawal with same secret", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await depositor.withdraw(secret);

    await expect(depositor.withdraw(secret))
      .to.be.revertedWithCustomError(depositor, "DepositAlreadyWithdrawn")
      .withArgs(secretHash);
  });

  it("should reject withdrawal if the ETH transfer fails", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(await depositor.getAddress(), secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await expect(depositor.withdraw(secret)).to.be.revertedWithCustomError(depositor, "FailedInnerCall");
  });

  it("should reject restoring before lock time expires", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    const nextBlockTimestamp = (await time.latest()) + 1;
    await time.setNextBlockTimestamp(nextBlockTimestamp);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await expect(depositor.restore(secretHash))
      .to.be.revertedWithCustomError(depositor, "TimeLockNotExpired")
      .withArgs(nextBlockTimestamp + 1, nextBlockTimestamp + LOCK_TIME);
  });

  it("should reject restoring with incorrect secret", async () => {
    const secretHash = ethers.hexlify(ethers.randomBytes(32));

    await expect(depositor.restore(secretHash))
      .to.be.revertedWithCustomError(depositor, "DepositDoesNotExist")
      .withArgs(secretHash);
  });

  it("should reject restoring if the ETH transfer fails", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await impersonateAccount(await depositor.getAddress());
    const depositorAsSigner = await ethers.getSigner(await depositor.getAddress());
    await setBalance(await depositorAsSigner.getAddress(), "0xffffffffffffffffffffffffffffffffff");

    await depositor.connect(depositorAsSigner).deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await increase(LOCK_TIME);

    await expect(depositor.restore(secretHash)).to.be.revertedWithCustomError(depositor, "FailedInnerCall");
  });

  it("should reject restoring if the deposit is already withdrawn", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await depositor.withdraw(secret);

    await increase(LOCK_TIME);

    await expect(depositor.restore(secretHash))
      .to.be.revertedWithCustomError(depositor, "DepositAlreadyWithdrawn")
      .withArgs(secretHash);
  });

  it("should allow restoring after lock time", async () => {
    const secret = ethers.hexlify(ethers.randomBytes(32));
    const secretHash = poseidonHash(secret);

    await depositor.connect(USER1).deposit(USER2.address, secretHash, LOCK_TIME, { value: DEPOSIT_AMOUNT });

    await increase(LOCK_TIME);

    const user1BalanceBefore = await ethers.provider.getBalance(USER1.address);

    await expect(depositor.restore(secretHash))
      .to.emit(depositor, "Restored")
      .withArgs(USER1.address, DEPOSIT_AMOUNT, secretHash);

    const user1BalanceAfter = await ethers.provider.getBalance(USER1.address);

    expect(user1BalanceAfter - user1BalanceBefore).to.equal(DEPOSIT_AMOUNT);
  });
});
