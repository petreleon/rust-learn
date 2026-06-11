// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./LearnTokenPresignerBase.t.sol";

contract LearnTokenPresignerExecutionTest is LearnTokenPresignerTestBase {
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
}
