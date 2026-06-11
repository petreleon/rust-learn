// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./LearnTokenPresignerBase.t.sol";

contract LearnTokenPresignerFlowTest is LearnTokenPresignerTestBase {
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
