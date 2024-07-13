// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.4.1/contracts/utils/Context.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.4.1/contracts/access/Ownable.sol";

/**
 * @dev Interface of the AssetsBridge
 */
interface IAssetsBridge {
    /*
     * @dev mint the token to account for assets bridge admin.
     * @param account The receiver of token.
     * @param amount The amount of token.
     */
    function mint_into(address account, uint256 amount) external returns (bool);

    /*
     * @dev burn the token from account for assets bridge admin.
     * @param account The owner of token.
     * @param amount The amount of token.
     */
    function burn_from(address account, uint256 amount) external returns (bool);
}

interface ITransferToMagnet {
    function transferToMagnet(address tokenAddress, uint256 amount, string calldata ss58Address) external;
}

abstract contract AssetsBridgeAdmin is Context {
    address public constant admin = 0x8c2D71ecFBc9E7D4ee5E09dEc448671a2DEbea7f;

    modifier AssetsBridgeRequire() {
        require(_msgSender() == admin, "AssetsBridge: require called by the assets bridge admin address");

        _;
    }
}

abstract contract AssetsBridgeOwner is Context, Ownable {
    modifier AssetsBridgeRequire() {
        require(_msgSender() == owner(), "AssetsBridge: require called by owner");

        _;
    }
}

abstract contract AssetsBridgeAdminOrOwner is Context, Ownable {
    address public constant admin = 0x8c2D71ecFBc9E7D4ee5E09dEc448671a2DEbea7f;

    modifier AssetsBridgeRequire() {
        require(_msgSender() == owner() || _msgSender() == admin, "AssetsBridge: require called by owner or admin");

        _;
    }
}
