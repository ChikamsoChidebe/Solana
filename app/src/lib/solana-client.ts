import { Connection, PublicKey, Transaction, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Program, AnchorProvider, BN, web3 } from '@coral-xyz/anchor';
import { WalletContextState } from '@solana/wallet-adapter-react';

export const CARBON_MARKETPLACE_PROGRAM_ID = new PublicKey("CarbMktpLace11111111111111111111111111111111");
export const CARBON_VERIFICATION_PROGRAM_ID = new PublicKey("CarbVerify11111111111111111111111111111111");
export const CARBON_REGISTRY_PROGRAM_ID = new PublicKey("CarbRegistry1111111111111111111111111111111");

export class SolanaClient {
  private connection: Connection;
  private provider: AnchorProvider | null = null;

  constructor(network: string = 'devnet') {
    this.connection = new Connection(
      network === 'mainnet' 
        ? 'https://api.mainnet-beta.solana.com'
        : 'https://api.devnet.solana.com',
      'confirmed'
    );
  }

  async initialize(wallet: WalletContextState) {
    if (!wallet.publicKey || !wallet.signTransaction) {
      throw new Error('Wallet not connected');
    }

    this.provider = new AnchorProvider(
      this.connection,
      wallet as any,
      { commitment: 'confirmed' }
    );
  }

  async getMarketplaceData() {
    try {
      const [marketplacePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        CARBON_MARKETPLACE_PROGRAM_ID
      );

      const accountInfo = await this.connection.getAccountInfo(marketplacePda);
      
      if (!accountInfo) {
        return {
          totalCreditsTraded: 0,
          totalVolume: 0,
          activeListings: 0,
          verifiedProjects: 0
        };
      }

      // Parse account data here when IDL is available
      return {
        totalCreditsTraded: 0,
        totalVolume: 0,
        activeListings: 0,
        verifiedProjects: 0
      };
    } catch (error) {
      console.error('Error fetching marketplace data:', error);
      throw error;
    }
  }

  async getProjects() {
    try {
      const programAccounts = await this.connection.getProgramAccounts(
        CARBON_MARKETPLACE_PROGRAM_ID,
        {
          filters: [
            {
              memcmp: {
                offset: 0,
                bytes: Buffer.from("project").toString('base64')
              }
            }
          ]
        }
      );

      return programAccounts.map(account => ({
        pubkey: account.pubkey.toString(),
        data: account.account.data
      }));
    } catch (error) {
      console.error('Error fetching projects:', error);
      return [];
    }
  }

  async getListings() {
    try {
      const programAccounts = await this.connection.getProgramAccounts(
        CARBON_MARKETPLACE_PROGRAM_ID,
        {
          filters: [
            {
              memcmp: {
                offset: 0,
                bytes: Buffer.from("listing").toString('base64')
              }
            }
          ]
        }
      );

      return programAccounts.map(account => ({
        pubkey: account.pubkey.toString(),
        data: account.account.data
      }));
    } catch (error) {
      console.error('Error fetching listings:', error);
      return [];
    }
  }

  async createProject(
    projectId: string,
    projectName: string,
    projectType: string,
    location: string,
    estimatedCredits: number,
    verificationStandard: string,
    metadataUri: string
  ) {
    if (!this.provider) {
      throw new Error('Provider not initialized');
    }

    const [marketplacePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace")],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const [projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectId)],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    // Create transaction instruction here when IDL is available
    const transaction = new Transaction();
    
    return {
      transaction,
      projectPda: projectPda.toString(),
      marketplacePda: marketplacePda.toString()
    };
  }

  async listCredits(
    projectId: string,
    amount: number,
    pricePerCredit: number,
    expiryTime: number
  ) {
    if (!this.provider) {
      throw new Error('Provider not initialized');
    }

    const [projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectId)],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const [listingPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("listing"),
        projectPda.toBuffer(),
        this.provider.wallet.publicKey.toBuffer()
      ],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const transaction = new Transaction();
    
    return {
      transaction,
      listingPda: listingPda.toString(),
      projectPda: projectPda.toString()
    };
  }

  async purchaseCredits(listingPubkey: string, amount: number) {
    if (!this.provider) {
      throw new Error('Provider not initialized');
    }

    const listingKey = new PublicKey(listingPubkey);
    
    const [purchasePda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("purchase"),
        listingKey.toBuffer(),
        this.provider.wallet.publicKey.toBuffer()
      ],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const transaction = new Transaction();
    
    return {
      transaction,
      purchasePda: purchasePda.toString()
    };
  }

  async retireCredits(
    projectId: string,
    amount: number,
    retirementReason: string
  ) {
    if (!this.provider) {
      throw new Error('Provider not initialized');
    }

    const [projectPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("project"), Buffer.from(projectId)],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const [retirementPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("retirement"),
        projectPda.toBuffer(),
        this.provider.wallet.publicKey.toBuffer()
      ],
      CARBON_MARKETPLACE_PROGRAM_ID
    );

    const transaction = new Transaction();
    
    return {
      transaction,
      retirementPda: retirementPda.toString(),
      projectPda: projectPda.toString()
    };
  }

  async getAccountBalance(publicKey: PublicKey): Promise<number> {
    const balance = await this.connection.getBalance(publicKey);
    return balance / LAMPORTS_PER_SOL;
  }

  async getRecentBlockhash() {
    const { blockhash } = await this.connection.getLatestBlockhash();
    return blockhash;
  }

  async sendTransaction(transaction: Transaction, signers: any[]) {
    if (!this.provider) {
      throw new Error('Provider not initialized');
    }

    const signature = await this.provider.sendAndConfirm(transaction, signers);
    return signature;
  }

  getConnection(): Connection {
    return this.connection;
  }
}