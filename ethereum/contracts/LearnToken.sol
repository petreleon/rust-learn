// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/// @notice LearnToken with optional custom decimals and ERC-2612 permit support
contract LearnToken is ERC20, ERC20Permit, Ownable {
    uint8 private _customDecimals;

    constructor(string memory name, string memory symbol, uint8 decimals_) ERC20(name, symbol) ERC20Permit(name) Ownable(msg.sender) {
        _customDecimals = decimals_;
    }

    function decimals() public view virtual override returns (uint8) {
        return _customDecimals;
    }

    function mint(address to, uint256 amount) public onlyOwner {
        _mint(to, amount);
    }

    function burn(address from, uint256 amount) public onlyOwner {
        _burn(from, amount);
    }
}
