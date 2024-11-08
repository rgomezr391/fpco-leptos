// Date FNS import
import { format, add, sub } from 'date-fns';

// Lodash Import
import _ from 'lodash';

// UUID Import
import { v4 as uuidv4 } from 'uuid';

// Reown Imports
import { createAppKit } from '@reown/appkit/react'
import { SolanaAdapter } from '@reown/appkit-adapter-solana/react'
import { solana, solanaTestnet, solanaDevnet } from '@reown/appkit/networks'
import { PhantomWalletAdapter, SolflareWalletAdapter } from '@solana/wallet-adapter-wallets'

// Exported functions

export function addDate(date, duration) {
    return add(date, duration);
}

export function subDate(date, duration) {
    return sub(date, duration);
}

export function dateformat(date, formatString) {
    return format(date, formatString);
}

export function generateUuidv4() {
    return uuidv4();
}

// 0. Set up Solana Adapter
const solanaWeb3JsAdapter = new SolanaAdapter({
    wallets: [new PhantomWalletAdapter()]
});

// 1. Get projectId from https://cloud.reown.com
const projectId = '798155558acabfff825742603c438ce5';

// 2. Create a metadata object - optional
const metadata = {
    name: 'FPCompleteTest',
    description: 'AppKit Example',
    url: 'https://reown.com/appkit', // origin must match your domain & subdomain
    icons: ['https://assets.reown.com/reown-profile-pic.png']
};

let modal = null;

export function setupSolanaAdapter() {
    if (!modal) {
        // 3. Create modal
        modal = createAppKit({
            adapters: [solanaWeb3JsAdapter],
            networks: [solana, solanaTestnet, solanaDevnet],
            metadata: metadata,
            projectId,
            features: {
            analytics: true // Optional - defaults to your Cloud configuration
            }
        });
    }
}

export function openModal() {
    if (modal) {
        modal.open()
    }
}

export function getWalletMetadata() {
    if (modal) {
        let info = modal.getWalletInfo();

        console.log("Wallet info\n");
        console.log(info);

        return "HELLO";
    }
}