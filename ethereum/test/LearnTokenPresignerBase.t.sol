// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/LearnToken.sol";
import "../contracts/LearnTokenPresigner.sol";

abstract contract LearnTokenPresignerTestBase is Test {
    LearnToken public token;
    LearnTokenPresigner public presigner;
    address public owner;
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    uint256 internal alicePrivateKey = 0xA11CE;
    address internal aliceWithKey;

    function setUp() public {
        owner = address(this);
        token = new LearnToken("LearnToken", "LEARN", 18);
        presigner = new LearnTokenPresigner(IERC20(address(token)));
        aliceWithKey = vm.addr(alicePrivateKey);
    }

    function _deposit(address user, uint256 amount) internal {
        token.mint(user, amount);
        vm.prank(user);
        token.approve(address(presigner), amount);
        vm.prank(user);
        presigner.deposit(amount);
    }
}
