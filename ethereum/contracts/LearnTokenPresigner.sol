// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/// @title LearnToken Presigner
/// @notice Allows users to deposit LearnToken and create off-chain presigned transfer instructions
/// which can be executed on-chain by anyone presenting a valid signature.
contract LearnTokenPresigner is EIP712 {
    using SafeERC20 for IERC20;

    IERC20 public immutable token;

    // deposited balances per user
    mapping(address => uint256) public balanceOf;

    // nonces for replay protection
    mapping(address => uint256) public nonces;

    bytes32 private constant TRANSFER_TYPEHASH = keccak256("TransferRequest(address to,uint256 amount,uint256 nonce,uint256 deadline)");

    event Deposited(address indexed user, uint256 amount);
    event Withdrawn(address indexed user, uint256 amount);
    event PresignedExecuted(address indexed signer, address indexed to, uint256 amount, uint256 nonce);

    /// @param _token address of the LearnToken (ERC20)
    constructor(IERC20 _token) EIP712("LearnTokenPresigner", "1") {
        token = _token;
    }

    /// @notice Deposit tokens into the presigner contract. Caller must approve this contract first.
    function deposit(uint256 amount) external {
        require(amount > 0, "amount>0");
        balanceOf[msg.sender] += amount;
        token.safeTransferFrom(msg.sender, address(this), amount);
        emit Deposited(msg.sender, amount);
    }

    /// @notice Withdraw deposited tokens
    function withdraw(uint256 amount) external {
        require(amount > 0, "amount>0");
        uint256 bal = balanceOf[msg.sender];
        require(bal >= amount, "insufficient balance");
        balanceOf[msg.sender] = bal - amount;
        token.safeTransfer(msg.sender, amount);
        emit Withdrawn(msg.sender, amount);
    }

    /// @notice Execute a presigned transfer. The signer must have deposited funds previously.
    /// @param signer the address who signed the transfer
    /// @param to recipient
    /// @param amount transfer amount
    /// @param nonce expected nonce (replay protection)
    /// @param deadline signature expiry timestamp
    /// @param signature EIP-712 signature
    function executePresigned(
        address signer,
        address to,
        uint256 amount,
        uint256 nonce,
        uint256 deadline,
        bytes calldata signature
    ) external {
        require(block.timestamp <= deadline, "signature expired");
        require(amount > 0, "amount>0");
        require(to != address(0), "invalid recipient");

        // verify nonce
        require(nonces[signer] == nonce, "invalid nonce");

        // build the digest
        bytes32 structHash = keccak256(abi.encode(TRANSFER_TYPEHASH, to, amount, nonce, deadline));
        bytes32 digest = _hashTypedDataV4(structHash);
        address recovered = ECDSA.recover(digest, signature);
        require(recovered == signer, "invalid signature");

        // mark nonce used
        nonces[signer] = nonce + 1;

        // ensure signer has deposited balance
        uint256 bal = balanceOf[signer];
        require(bal >= amount, "insufficient balance");
        balanceOf[signer] = bal - amount;

        // transfer token to recipient
        token.safeTransfer(to, amount);

        emit PresignedExecuted(signer, to, amount, nonce);
    }

    /// @notice Helper to compute the EIP-712 digest for a transfer request
    function hashTransferRequest(address to, uint256 amount, uint256 nonce, uint256 deadline) public view returns (bytes32) {
        bytes32 structHash = keccak256(abi.encode(TRANSFER_TYPEHASH, to, amount, nonce, deadline));
        return _hashTypedDataV4(structHash);
    }
}
