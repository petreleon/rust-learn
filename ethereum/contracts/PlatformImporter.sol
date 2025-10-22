// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";

interface IERC20Permit {
    function permit(
        address owner,
        address spender,
        uint256 value,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external;
}

/// @title Platform Importer
/// @notice Allows the server (or anyone) to import tokens into the platform treasury
/// by submitting an EIP-2612 permit signed by the user and then calling transferFrom.
contract PlatformImporter {
    using SafeERC20 for IERC20;

    address public immutable treasury;

    event Imported(address indexed user, uint256 amount, address indexed token);

    constructor(address _treasury) {
        require(_treasury != address(0), "invalid treasury");
        treasury = _treasury;
    }

    /// @notice Import tokens into the treasury using an EIP-2612 permit from the user.
    /// @param tokenAddr address of the ERC20 token supporting EIP-2612
    /// @param user user address who signed the permit
    /// @param amount amount to transfer
    /// @param deadline permit deadline
    /// @param v,r,s permit signature components
    function importWithPermit(
        address tokenAddr,
        address user,
        uint256 amount,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        require(amount > 0, "amount>0");
        IERC20Permit(tokenAddr).permit(user, address(this), amount, deadline, v, r, s);

        IERC20(tokenAddr).safeTransferFrom(user, treasury, amount);

        emit Imported(user, amount, tokenAddr);
    }

    /// @notice Import tokens directly to a recipient (cheaper than routing via treasury)
    /// @param tokenAddr address of the ERC20 token supporting EIP-2612
    /// @param user user address who signed the permit
    /// @param recipient destination address to receive tokens
    /// @param amount amount to transfer
    /// @param deadline permit deadline
    /// @param v,r,s permit signature components
    function importToRecipientWithPermit(
        address tokenAddr,
        address user,
        address recipient,
        uint256 amount,
        uint256 deadline,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        require(recipient != address(0), "invalid recipient");
        require(amount > 0, "amount>0");
        IERC20Permit(tokenAddr).permit(user, address(this), amount, deadline, v, r, s);

        IERC20(tokenAddr).safeTransferFrom(user, recipient, amount);

        emit Imported(user, amount, tokenAddr);
    }
}
