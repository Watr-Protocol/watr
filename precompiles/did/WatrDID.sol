// SPDX-License-Identifier: GPL-3.0-or-later 
pragma solidity ^0.8.0;

interface WatrDID {
    function create_did(
        address controller,
        address authentication,
        bytes calldata service 
    ) external returns (bool);

    function create_did(
        address controller,
        address authentication,
        address verification,
        bytes calldata service 
    ) external returns (bool);
}