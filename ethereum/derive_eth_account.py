from eth_account import Account
import os
import sys

def main():
    mnemonic = os.environ.get('ETH_MNEMONIC')
    if not mnemonic:
        print('ETH_MNEMONIC not set.', file=sys.stderr)
        sys.exit(1)
    Account.enable_unaudited_hdwallet_features()
    acct = Account.from_mnemonic(mnemonic)
    print(acct.address)
    print(acct.key.hex())

if __name__ == "__main__":
    main()
