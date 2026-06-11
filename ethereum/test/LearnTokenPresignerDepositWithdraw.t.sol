// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./LearnTokenPresignerBase.t.sol";

contract LearnTokenPresignerDepositWithdrawTest is LearnTokenPresignerTestBase {
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
}
