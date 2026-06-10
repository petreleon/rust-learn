// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "../contracts/LearnToken.sol";
import "../contracts/PlatformImporter.sol";

contract PlatformImporterTest is Test {
    LearnToken public token;
    PlatformImporter public importer;
    address public treasury = makeAddr("treasury");
    address public alice;
    address public bob = makeAddr("bob");
    uint256 internal alicePrivateKey = 0xA11CE;

    function setUp() public {
        alice = vm.addr(alicePrivateKey);
        token = new LearnToken("LearnToken", "LEARN", 18);
        importer = new PlatformImporter(treasury);
        token.mint(alice, 100 ether);
    }

    function test_constructor_sets_treasury() public view {
        assertEq(importer.treasury(), treasury);
    }

    function test_constructor_reverts_with_zero_address() public {
        vm.expectRevert("invalid treasury");
        new PlatformImporter(address(0));
    }

    function test_importWithPermit_transfers_tokens_to_treasury() public {
        uint256 amount = 30 ether;
        uint256 deadline = block.timestamp + 1 days;

        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 permitTypehash = keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
        );

        bytes32 structHash = keccak256(
            abi.encode(
                permitTypehash,
                alice,
                address(importer),
                amount,
                token.nonces(alice),
                deadline
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        importer.importWithPermit(address(token), alice, amount, deadline, v, r, s);

        assertEq(token.balanceOf(treasury), amount);
        assertEq(token.balanceOf(alice), 100 ether - amount);
    }

    function test_importWithPermit_emits_event() public {
        uint256 amount = 20 ether;
        uint256 deadline = block.timestamp + 1 days;

        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 permitTypehash = keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
        );
        bytes32 structHash = keccak256(
            abi.encode(permitTypehash, alice, address(importer), amount, token.nonces(alice), deadline)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.expectEmit(true, false, true, true);
        emit PlatformImporter.Imported(alice, amount, address(token));
        importer.importWithPermit(address(token), alice, amount, deadline, v, r, s);
    }

    function test_importWithPermit_reverts_with_zero_amount() public {
        vm.expectRevert("amount>0");
        importer.importWithPermit(address(token), alice, 0, block.timestamp + 1 days, 0, bytes32(0), bytes32(0));
    }

    function test_importToRecipientWithPermit_sends_to_custom_recipient() public {
        uint256 amount = 15 ether;
        uint256 deadline = block.timestamp + 1 days;

        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 permitTypehash = keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
        );
        bytes32 structHash = keccak256(
            abi.encode(permitTypehash, alice, address(importer), amount, token.nonces(alice), deadline)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        importer.importToRecipientWithPermit(address(token), alice, bob, amount, deadline, v, r, s);

        assertEq(token.balanceOf(bob), amount);
        assertEq(token.balanceOf(treasury), 0);
        assertEq(token.balanceOf(alice), 100 ether - amount);
    }

    function test_importToRecipientWithPermit_reverts_with_zero_recipient() public {
        vm.expectRevert("invalid recipient");
        importer.importToRecipientWithPermit(address(token), alice, address(0), 1, 1, 0, bytes32(0), bytes32(0));
    }

    function test_importToRecipientWithPermit_reverts_with_zero_amount() public {
        vm.expectRevert("amount>0");
        importer.importToRecipientWithPermit(address(token), alice, bob, 0, 1 days, 0, bytes32(0), bytes32(0));
    }

    function test_importWithPermit_multiple_imports_from_same_user() public {
        uint256 deadline = block.timestamp + 1 days;
        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 permitTypehash = keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
        );

        for (uint256 i = 0; i < 3; i++) {
            uint256 amount = 10 ether;
            uint256 nonce = token.nonces(alice);
            bytes32 structHash = keccak256(
                abi.encode(permitTypehash, alice, address(importer), amount, nonce, deadline)
            );
            bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
            (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);
            importer.importWithPermit(address(token), alice, amount, deadline, v, r, s);
        }

        assertEq(token.balanceOf(treasury), 30 ether);
        assertEq(token.balanceOf(alice), 70 ether);
    }

    function test_importWithPermit_expired_deadline() public {
        uint256 deadline = block.timestamp;
        bytes32 domainSeparator = token.DOMAIN_SEPARATOR();
        bytes32 permitTypehash = keccak256(
            "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
        );
        bytes32 structHash = keccak256(
            abi.encode(permitTypehash, alice, address(importer), 10 ether, token.nonces(alice), deadline)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(alicePrivateKey, digest);

        vm.warp(block.timestamp + 1);
        vm.expectRevert();
        importer.importWithPermit(address(token), alice, 10 ether, deadline, v, r, s);
    }
}
