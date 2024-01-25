// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

import {Address} from "@openzeppelin/contracts/utils/Address.sol";

library PoseidonUnit4L {
    function poseidon(bytes32[4] calldata) public pure returns (bytes32) {}
}

/**
 * @title Taprootized Atomic Swaps Contract
 * @notice This contract facilitates atomic swaps using between Bitcoin and Ethereum, ensuring privacy and security for both parties.
 *
 * Functionality:
 * - Users can deposit ETH into the contract, specifying a recipient, a Poseidon hash of a secret, and a lock time.
 * - Deposits can only be withdrawn by the recipient if they provide the correct secret that matches the hash used at the time of deposit.
 * - Deposits are locked for a minimum duration, defined by the contract, ensuring that the depositor cannot reclaim them before this period.
 * - If the recipient does not withdraw the funds within the lock time, the depositor can reclaim them.
 *
 * Limitations:
 * - The contract only supports ETH deposits and does not handle ERC20 tokens or other assets.
 * - The lock time is fixed and cannot be modified after a deposit is made.
 * - The contract does not support incremental deposits to the same secret hash; each deposit must have a unique secret hash.
 */
contract Depositor {
    /**
     * @notice Represents the minimum time (in seconds) that a deposit must be locked in the contract. Set to one hour.
     */
    uint256 public constant MIN_LOCK_TIME = 1 hours;

    /**
     * @notice Struct to store details of each deposit.
     * @param sender The Ethereum address of the user who created the deposit.
     * @param recipient The Ethereum address of the user eligible to withdraw the deposit using the correct secret.
     * @param amount The amount of ETH deposited.
     * @param lockTime The UNIX timestamp until which the deposit is locked and cannot be withdrawn.
     * @param isWithdrawn Boolean flag indicating whether the deposit has been withdrawn. This helps prevent double spending.
     */
    struct Deposit {
        address sender;
        address recipient;
        uint256 amount;
        uint256 lockTime;
        bool isWithdrawn;
    }

    /**
     * @notice Mapping of Poseidon hash of the secret to Deposit structs.
     */
    mapping(bytes32 => Deposit) public deposits;

    /**
     * @notice Emitted when a new deposit is made.
     * @param sender The Ethereum address of the user who created the deposit.
     * @param recipient The Ethereum address of the user eligible to withdraw the deposit using the correct secret.
     * @param amount The amount of ETH deposited.
     * @param lockTime The UNIX timestamp until which the deposit is locked and cannot be withdrawn.
     * @param secretHash The Poseidon hash of the secret required to withdraw the deposit.
     */
    event Deposited(
        address indexed sender,
        address indexed recipient,
        uint256 amount,
        uint256 lockTime,
        bytes32 secretHash
    );

    /**
     * @notice Emitted when a deposit is successfully withdrawn.
     * @param recipient The Ethereum address of the user who withdrew the deposit.
     * @param amount The amount of ETH withdrawn.
     * @param secret The secret used to withdraw the deposit.
     * @param secretHash The Poseidon hash of the secret used to create the deposit.
     */
    event Withdrawn(address indexed recipient, uint256 amount, bytes32[4] secret, bytes32 secretHash);

    /**
     * @notice Emitted when deposited funds are restored to the sender after the lock time has expired.
     * @param sender The Ethereum address of the sender to whom the funds are restored.
     * @param amount The amount of ETH restored.
     * @param secretHash The Poseidon hash of the secret originally used for the deposit.
     */
    event Restored(address indexed sender, uint256 amount, bytes32 secretHash);

    /**
     * @notice Error thrown when a deposit is attempted with an amount of 0 ETH.
     */
    error ZeroDepositAmount();

    /**
     * @notice Error thrown when a deposit with the given secret hash already exists.
     * @param secretHash The Poseidon hash of the secret for which a deposit already exists.
     */
    error DepositAlreadyExists(bytes32 secretHash);

    /**
     * @notice Error thrown when the provided lock time for a deposit is too short.
     * @param providedLockTime The lock time provided for the deposit.
     * @param minimumLockTime The minimum required lock time, typically one hour.
     */
    error LockTimeTooShort(uint256 providedLockTime, uint256 minimumLockTime);

    /**
     * @notice Error thrown when an operation (like withdrawal or restoration) is attempted on a non-existent deposit.
     * @param secretHash The Poseidon hash of the secret for which the deposit does not exist.
     */
    error DepositDoesNotExist(bytes32 secretHash);

    /**
     * @notice Error thrown when an attempt is made to withdraw or restore an already withdrawn deposit.
     * @param secretHash The Poseidon hash of the secret for the deposit that has already been withdrawn.
     */
    error DepositAlreadyWithdrawn(bytes32 secretHash);

    /**
     * @notice Error thrown when an attempt is made to restore funds before the lock time has expired.
     * @param currentTime The current UNIX timestamp.
     * @param lockTime The UNIX timestamp until which the deposit is locked.
     */
    error TimeLockNotExpired(uint256 currentTime, uint256 lockTime);

    /**
     * @notice Error thrown when an attempt is made to make a deposit with the zero Ethereum address as the recipient.
     */
    error ZeroAddressNotAllowed();

    /**
     * @notice Allows a user to deposit ETH into the contract with a given recipient, secret hash, and lock time.
     * @dev Emits a `Deposited` event upon successful deposit. Checks for zero deposit amount, duplicate deposits,
     *      short lock times, and zero address recipient.
     * @param recipient_ The Ethereum address of the recipient eligible to withdraw the deposit using the correct secret.
     * @param secretHash_ The Poseidon hash of the secret required for the recipient to withdraw the deposit.
     * @param lockTime_ The duration (in seconds) for which the deposit is locked and cannot be withdrawn.
     */
    function deposit(address recipient_, bytes32 secretHash_, uint256 lockTime_) external payable {
        if (msg.value == 0) revert ZeroDepositAmount();
        if (deposits[secretHash_].amount != 0) revert DepositAlreadyExists(secretHash_);
        if (lockTime_ < MIN_LOCK_TIME) revert LockTimeTooShort(lockTime_, MIN_LOCK_TIME);
        if (recipient_ == address(0)) revert ZeroAddressNotAllowed();

        deposits[secretHash_] = Deposit({
            sender: msg.sender,
            recipient: recipient_,
            amount: msg.value,
            lockTime: block.timestamp + lockTime_,
            isWithdrawn: false
        });

        emit Deposited(msg.sender, recipient_, msg.value, lockTime_, secretHash_);
    }

    /**
     * @notice Allows the recipient to withdraw a deposit using the correct secret.
     * @dev Emits a `Withdrawn` event upon successful withdrawal. Checks for non-existent or already withdrawn deposits.
     *      Uses the PoseidonUnit1L library to hash the provided secret.
     * @param secret_ The prototype of the `secretHash` used in the deposit function.
     */
    function withdraw(bytes32[4] calldata secret_) external {
        bytes32 secretHash_ = PoseidonUnit4L.poseidon(secret_);

        Deposit storage userDeposit = deposits[secretHash_];

        uint256 depositAmount_ = userDeposit.amount;
        address depositRecipient_ = userDeposit.recipient;

        if (depositAmount_ == 0) revert DepositDoesNotExist(secretHash_);
        if (userDeposit.isWithdrawn) revert DepositAlreadyWithdrawn(secretHash_);

        userDeposit.isWithdrawn = true;

        (bool success_, bytes memory data_) = payable(depositRecipient_).call{
            value: depositAmount_
        }("");
        Address.verifyCallResult(success_, data_);

        emit Withdrawn(depositRecipient_, depositAmount_, secret_, secretHash_);
    }

    /**
     * @notice Allows the sender to restore a deposit back to themselves after the lock time has expired.
     * @dev Emits a `Restored` event upon successful restoration. Checks for non-existent, already withdrawn,
     *      or not yet expired deposits.
     * @param secretHash_ The Poseidon hash of the secret used when the deposit was created.
     */
    function restore(bytes32 secretHash_) external {
        Deposit storage userDeposit = deposits[secretHash_];

        uint256 depositAmount_ = userDeposit.amount;

        if (depositAmount_ == 0) revert DepositDoesNotExist(secretHash_);
        if (userDeposit.isWithdrawn) revert DepositAlreadyWithdrawn(secretHash_);
        if (userDeposit.lockTime > block.timestamp)
            revert TimeLockNotExpired(block.timestamp, userDeposit.lockTime);

        userDeposit.isWithdrawn = true;

        (bool success_, bytes memory data_) = payable(userDeposit.sender).call{
            value: depositAmount_
        }("");
        Address.verifyCallResult(success_, data_);

        emit Restored(userDeposit.sender, depositAmount_, secretHash_);
    }
}
