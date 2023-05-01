declare const fs: any;
declare const solc: any;
declare const source: any;
declare const input: {
    language: string;
    sources: {
        'ERC20.sol': {
            content: any;
        };
    };
    settings: {
        outputSelection: {
            '*': {
                '*': string[];
            };
        };
    };
};
declare const tempFile: any;
declare const contractFile: any;
