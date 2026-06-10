// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/LearnToken.sol";
import "../contracts/LearnTokenPresigner.sol";

contract LearnTokenPresignerTest is Test {
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

    function test_constructor_sets_token() public view {
        assertEq(address(presigner.token()), address(token));
    }

    function test_deposit_transfers_tokens_and_records_balance() public {
        token.mint(alice, 100 ether);
        vm.prank(alice);
        token.approve(address(presigner), 100 ether);

        assertEq(presigner.balanceOf(alice), 0);
        vm.prank(alice);
        presigner.deposit(100 ether);

        assertEq(presigner.balanceOf(alice), 100 ether);
        assertEq(token.balanceOf(address(presigner)), 100 ether);
        assertEq(token.balanceOf(alice), 0);
    }

    function test_deposit_emits_event() public {
        token.mint(alice, 50 ether);
        vm.prank(alice);
        token.approve(address(presigner), 50 ether);

        vm.prank(alice);
        vm.expectEmit(true, true, false, true);
        emit LearnTokenPresigner.Deposited(alice, 50 ether);
        presigner.deposit(50 ether);
    }

    function test_deposit_reverts_with_zero_amount() public {
        vm.prank(alice);
        vm.expectRevert("amount>0");
        presigner.deposit(0);
    }

    function test_withdraw_returns_tokens() public {
        _deposit(alice, 100 ether);

        vm.prank(alice);
        presigner.withdraw(30 ether);

        assertEq(presigner.balanceOf(alice), 70 ether);
        assertEq(token.balanceOf(alice), 30 ether);
    }

    function test_withdraw_emits_event() public {
        _deposit(alice, 100 ether);

        vm.prank(alice);
        vm.expectEmit(true, true, false, true);
        emit LearnTokenPresigner.Withdrawn(alice, 50 ether);
        presigner.withdraw(50 ether);
    }

    function test_withdraw_reverts_with_zero_amount() public {
        vm.prank(alice);
        vm.expectRevert("amount>0");
        presigner.withdraw(0);
    }

    function test_withdraw_reverts_if_insufficient_balance() public {
        _deposit(alice, 50 ether);
        vm.prank(alice);
        vm.expectRevert("insufficient balance");
        presigner.withdraw(51 ether);
    }

    function test_executePresigned_succeeds_with_valid_signature() public {
        _deposit(aliceWithKey, 100 ether);

        uint256 nonce = presigner.nonces(aliceWithKey);
        uint256 deadline = block.timestamp + 1 days;
        uint256 amount = 30 ether;

        bytes32 digest = presigner.hashTransferRequest(bob, amount, nonce, deadline);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.expectEmit(true, true, false, true);
        emit LearnTokenPresigner.PresignedExecuted(aliceWithKey, bob, amount, nonce);

        presigner.executePresigned(aliceWithKey, bob, amount, nonce, deadline, signature);

        assertEq(presigner.balanceOf(aliceWithKey), 70 ether);
        assertEq(token.balanceOf(bob), 30 ether);
        assertEq(presigner.nonces(aliceWithKey), nonce + 1);
    }

    function test_executePresigned_increments_nonce() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);

        bytes32 digest = presigner.hashTransferRequest(bob, 10 ether, nonce, block.timestamp + 1 days);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );

        assertEq(presigner.nonces(aliceWithKey), nonce + 1);
    }

    function test_executePresigned_reverts_with_expired_deadline() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);
        uint256 deadline = block.timestamp;
        bytes32 digest = presigner.hashTransferRequest(bob, 10 ether, nonce, deadline);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.warp(block.timestamp + 1);
        vm.expectRevert("signature expired");
        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce, deadline, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_reverts_with_wrong_nonce() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);

        bytes32 digest = presigner.hashTransferRequest(bob, 10 ether, nonce, block.timestamp + 1 days);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.expectRevert("invalid nonce");
        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce + 1, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_reverts_with_zero_amount() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);
        bytes32 digest = presigner.hashTransferRequest(bob, 0, nonce, block.timestamp + 1 days);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.expectRevert("amount>0");
        presigner.executePresigned(
            aliceWithKey, bob, 0, nonce, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_reverts_with_invalid_recipient() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);
        bytes32 digest = presigner.hashTransferRequest(address(0), 10 ether, nonce, block.timestamp + 1 days);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.expectRevert("invalid recipient");
        presigner.executePresigned(
            aliceWithKey, address(0), 10 ether, nonce, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_reverts_with_invalid_signature() public {
        _deposit(aliceWithKey, 100 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);

        bytes32 digest = presigner.hashTransferRequest(bob, 10 ether, nonce, block.timestamp + 1 days);

        // sign with a different key (bob's key instead of alice's)
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(0xB0B, digest);

        vm.expectRevert("invalid signature");
        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_reverts_if_insufficient_balance() public {
        _deposit(aliceWithKey, 30 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);
        bytes32 digest = presigner.hashTransferRequest(bob, 100 ether, nonce, block.timestamp + 1 days);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.expectRevert("insufficient balance");
        presigner.executePresigned(
            aliceWithKey, bob, 100 ether, nonce, block.timestamp + 1 days, abi.encodePacked(r, s, v)
        );
    }

    function test_executePresigned_replay_attack_fails() public {
        _deposit(aliceWithKey, 30 ether);
        uint256 nonce = presigner.nonces(aliceWithKey);
        uint256 deadline = block.timestamp + 1 days;
        bytes32 digest = presigner.hashTransferRequest(bob, 10 ether, nonce, deadline);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce, deadline, abi.encodePacked(r, s, v)
        );

        // replay fails because nonce was consumed
        vm.expectRevert("invalid nonce");
        presigner.executePresigned(
            aliceWithKey, bob, 10 ether, nonce, deadline, abi.encodePacked(r, s, v)
        );
    }

    function test_permit_and_deposit_combined_flow() public {
        uint256 depositAmount = 50 ether;

        // mint tokens to aliceWithKey
        token.mint(aliceWithKey, depositAmount);

        // alice approves presigner
        vm.prank(aliceWithKey);
        token.approve(address(presigner), depositAmount);

        // alice deposits
        vm.prank(aliceWithKey);
        presigner.deposit(depositAmount);

        // verify state
        assertEq(presigner.balanceOf(aliceWithKey), depositAmount);
        assertEq(token.balanceOf(address(presigner)), depositAmount);

        // alice creates a presigned transfer
        uint256 nonce = presigner.nonces(aliceWithKey);
        uint256 transferAmount = 20 ether;
        uint256 deadline = block.timestamp + 1 days;
        bytes32 digest = presigner.hashTransferRequest(bob, transferAmount, nonce, deadline);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        // bob executes on behalf of alice
        presigner.executePresigned(
            aliceWithKey, bob, transferAmount, nonce, deadline, abi.encodePacked(r, s, v)
        );

        assertEq(presigner.balanceOf(aliceWithKey), depositAmount - transferAmount);
        assertEq(token.balanceOf(bob), transferAmount);
    }
}
