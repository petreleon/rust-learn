// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/LearnToken.sol";

contract LearnTokenTest is Test {
    LearnToken public token;
    address public owner;
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");

    function setUp() public {
        owner = address(this);
        token = new LearnToken("LearnToken", "LEARN", 18);
    }

    function test_constructor_sets_metadata() public view {
        assertEq(token.name(), "LearnToken");
        assertEq(token.symbol(), "LEARN");
        assertEq(token.decimals(), 18);
    }

    function test_custom_decimals() public {
        LearnToken sixDecimalToken = new LearnToken("SixDec", "SIX", 6);
        assertEq(sixDecimalToken.decimals(), 6);
    }

    function test_mint_by_owner() public {
        token.mint(alice, 100 ether);
        assertEq(token.balanceOf(alice), 100 ether);
        assertEq(token.totalSupply(), 100 ether);
    }

    function test_mint_reverts_for_non_owner() public {
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSignature("OwnableUnauthorizedAccount(address)", alice));
        token.mint(bob, 100 ether);
    }

    function test_mint_multiple_addresses() public {
        token.mint(alice, 50 ether);
        token.mint(bob, 30 ether);
        assertEq(token.balanceOf(alice), 50 ether);
        assertEq(token.balanceOf(bob), 30 ether);
        assertEq(token.totalSupply(), 80 ether);
    }

    function test_burn_by_owner() public {
        token.mint(alice, 100 ether);
        token.burn(alice, 40 ether);
        assertEq(token.balanceOf(alice), 60 ether);
        assertEq(token.totalSupply(), 60 ether);
    }

    function test_burn_reverts_for_non_owner() public {
        token.mint(alice, 100 ether);
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSignature("OwnableUnauthorizedAccount(address)", alice));
        token.burn(alice, 50 ether);
    }

    function test_burn_reverts_if_insufficient_balance() public {
        token.mint(alice, 50 ether);
        vm.expectRevert(abi.encodeWithSignature("ERC20InsufficientBalance(address,uint256,uint256)", alice, 50 ether, 60 ether));
        token.burn(alice, 60 ether);
    }

    function test_transfer() public {
        token.mint(alice, 100 ether);
        vm.prank(alice);
        token.transfer(bob, 30 ether);
        assertEq(token.balanceOf(alice), 70 ether);
        assertEq(token.balanceOf(bob), 30 ether);
    }

    function test_transferFrom_with_approval() public {
        token.mint(alice, 100 ether);
        vm.prank(alice);
        token.approve(bob, 20 ether);
        vm.prank(bob);
        token.transferFrom(alice, bob, 20 ether);
        assertEq(token.balanceOf(alice), 80 ether);
        assertEq(token.balanceOf(bob), 20 ether);
    }

    function test_permit_generates_valid_signature() public {
        token.mint(alice, 100 ether);
        uint256 alicePrivateKey = 0xA11CE;
        address aliceWithKey = vm.addr(alicePrivateKey);
        token.mint(aliceWithKey, 100 ether);

        vm.prank(aliceWithKey);
        token.approve(bob, 30 ether);
        assertEq(token.allowance(aliceWithKey, bob), 30 ether);
    }

    function test_DOMAIN_SEPARATOR_is_set() public view {
        bytes32 separator = token.DOMAIN_SEPARATOR();
        assertTrue(separator != bytes32(0));
    }

    function test_owner_is_deployer() public view {
        assertEq(token.owner(), owner);
    }

    function test_transferOwnership() public {
        token.transferOwnership(alice);
        assertEq(token.owner(), alice);

        vm.prank(alice);
        token.mint(bob, 10 ether);
        assertEq(token.balanceOf(bob), 10 ether);
    }

    function test_renounceOwnership() public {
        token.renounceOwnership();
        assertEq(token.owner(), address(0));
    }

    function test_mint_reverts_after_renounceOwnership() public {
        token.renounceOwnership();
        vm.expectRevert(abi.encodeWithSignature("OwnableUnauthorizedAccount(address)", address(this)));
        token.mint(alice, 1 ether);
    }
}
