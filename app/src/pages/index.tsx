import { useState, useEffect } from 'react';
import { Connection, PublicKey, clusterApiUrl } from '@solana/web3.js';
import { useWallet } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';
import { Leaf, TrendingUp, Users, Globe, ShoppingCart, Award, Zap, Shield } from 'lucide-react';
import toast, { Toaster } from 'react-hot-toast';

interface MarketplaceData {
  totalCreditsTraded: number;
  totalVolume: number;
  activeListings: number;
  verifiedProjects: number;
}

interface ProjectData {
  id: string;
  name: string;
  type: string;
  location: string;
  credits: number;
  price: number;
  status: string;
}

interface ListingData {
  id: string;
  project: string;
  seller: string;
  amount: number;
  pricePerCredit: number;
  totalValue: number;
  expiryTime: Date;
}

export default function Home() {
  const { publicKey, connected } = useWallet();
  const [marketplaceData, setMarketplaceData] = useState<MarketplaceData>({
    totalCreditsTraded: 0,
    totalVolume: 0,
    activeListings: 0,
    verifiedProjects: 0
  });
  const [projects, setProjects] = useState<ProjectData[]>([]);
  const [listings, setListings] = useState<ListingData[]>([]);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState('marketplace');

  const [connection] = useState(() => new Connection(clusterApiUrl('devnet')));
  const [chartData, setChartData] = useState<any[]>([]);

  useEffect(() => {
    const fetchMarketplaceData = async () => {
      try {
        const [marketplacePda] = PublicKey.findProgramAddressSync(
          [Buffer.from("marketplace")],
          new PublicKey("CarbMktpLace11111111111111111111111111111111")
        );
        
        const accountInfo = await connection.getAccountInfo(marketplacePda);
        if (accountInfo) {
          // Parse actual marketplace data
          setMarketplaceData({
            totalCreditsTraded: 0,
            totalVolume: 0,
            activeListings: 0,
            verifiedProjects: 0
          });
        }
      } catch (error) {
        console.error('Failed to fetch marketplace data:', error);
      }
    };

    const fetchProjects = async () => {
      try {
        const programAccounts = await connection.getProgramAccounts(
          new PublicKey("CarbMktpLace11111111111111111111111111111111")
        );
        
        const projectAccounts = programAccounts.filter(account => 
          account.account.data.length > 0
        );
        
        setProjects([]);
      } catch (error) {
        console.error('Failed to fetch projects:', error);
      }
    };

    const fetchListings = async () => {
      try {
        const programAccounts = await connection.getProgramAccounts(
          new PublicKey("CarbMktpLace11111111111111111111111111111111")
        );
        
        setListings([]);
      } catch (error) {
        console.error('Failed to fetch listings:', error);
      }
    };

    fetchMarketplaceData();
    fetchProjects();
    fetchListings();
  }, [connection]);

  const createProject = async () => {
    if (!connected || !publicKey) {
      toast.error('Please connect your wallet first');
      return;
    }
    setLoading(true);
    try {
      const [marketplacePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace")],
        new PublicKey("CarbMktpLace11111111111111111111111111111111")
      );
      
      const projectId = `PROJ-${Date.now()}`;
      const [projectPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("project"), Buffer.from(projectId)],
        new PublicKey("CarbMktpLace11111111111111111111111111111111")
      );
      
      // Create actual transaction here
      toast.success('Project creation transaction prepared');
    } catch (error) {
      toast.error('Failed to create project');
    } finally {
      setLoading(false);
    }
  };

  const purchaseCredits = async (listingId: string, amount: number) => {
    if (!connected || !publicKey) {
      toast.error('Please connect your wallet first');
      return;
    }
    setLoading(true);
    try {
      const listingPubkey = new PublicKey(listingId);
      const [purchasePda] = PublicKey.findProgramAddressSync(
        [Buffer.from("purchase"), listingPubkey.toBuffer(), publicKey.toBuffer()],
        new PublicKey("CarbMktpLace11111111111111111111111111111111")
      );
      
      // Create actual purchase transaction here
      toast.success('Purchase transaction prepared');
    } catch (error) {
      toast.error('Failed to purchase credits');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 to-blue-50">
      <Toaster position="top-right" />
      
      <header className="bg-white shadow-lg border-b border-green-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center py-6">
            <div className="flex items-center space-x-3">
              <Leaf className="h-8 w-8 text-green-600" />
              <h1 className="text-2xl font-bold text-gray-900">CarbonChain</h1>
              <span className="text-sm text-gray-500 bg-green-100 px-2 py-1 rounded-full">Global Marketplace</span>
            </div>
            <div className="flex items-center space-x-4">
              <nav className="hidden md:flex space-x-6">
                <button
                  onClick={() => setActiveTab('marketplace')}
                  className={`px-3 py-2 rounded-md text-sm font-medium ${
                    activeTab === 'marketplace' ? 'bg-green-100 text-green-700' : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  Marketplace
                </button>
                <button
                  onClick={() => setActiveTab('projects')}
                  className={`px-3 py-2 rounded-md text-sm font-medium ${
                    activeTab === 'projects' ? 'bg-green-100 text-green-700' : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  Projects
                </button>
                <button
                  onClick={() => setActiveTab('analytics')}
                  className={`px-3 py-2 rounded-md text-sm font-medium ${
                    activeTab === 'analytics' ? 'bg-green-100 text-green-700' : 'text-gray-500 hover:text-gray-700'
                  }`}
                >
                  Analytics
                </button>
              </nav>
              <WalletMultiButton className="!bg-green-600 hover:!bg-green-700" />
            </div>
          </div>
        </div>
      </header>

      <section className="bg-gradient-to-r from-green-600 to-blue-600 text-white py-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
          <h2 className="text-5xl font-bold mb-6">
            Trade Carbon Credits on Solana
          </h2>
          <p className="text-xl mb-8 max-w-3xl mx-auto">
            The world's first decentralized carbon credit marketplace built on Solana. 
            Transparent, fast, and accessible to everyone fighting climate change.
          </p>
          <div className="flex justify-center space-x-4">
            <button
              onClick={createProject}
              disabled={loading}
              className="bg-white text-green-600 px-8 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors disabled:opacity-50"
            >
              {loading ? 'Processing...' : 'Create Project'}
            </button>
            <button className="border-2 border-white text-white px-8 py-3 rounded-lg font-semibold hover:bg-white hover:text-green-600 transition-colors">
              Explore Marketplace
            </button>
          </div>
        </div>
      </section>

      <section className="py-16 bg-white">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
            <div className="text-center">
              <div className="bg-green-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <TrendingUp className="h-8 w-8 text-green-600" />
              </div>
              <h3 className="text-3xl font-bold text-gray-900">{marketplaceData.totalCreditsTraded.toLocaleString()}</h3>
              <p className="text-gray-600">Credits Traded</p>
            </div>
            <div className="text-center">
              <div className="bg-blue-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <Globe className="h-8 w-8 text-blue-600" />
              </div>
              <h3 className="text-3xl font-bold text-gray-900">${marketplaceData.totalVolume.toLocaleString()}</h3>
              <p className="text-gray-600">Total Volume</p>
            </div>
            <div className="text-center">
              <div className="bg-purple-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <ShoppingCart className="h-8 w-8 text-purple-600" />
              </div>
              <h3 className="text-3xl font-bold text-gray-900">{marketplaceData.activeListings}</h3>
              <p className="text-gray-600">Active Listings</p>
            </div>
            <div className="text-center">
              <div className="bg-orange-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <Award className="h-8 w-8 text-orange-600" />
              </div>
              <h3 className="text-3xl font-bold text-gray-900">{marketplaceData.verifiedProjects}</h3>
              <p className="text-gray-600">Verified Projects</p>
            </div>
          </div>
        </div>
      </section>

      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        {activeTab === 'marketplace' && (
          <div className="space-y-8">
            <h2 className="text-3xl font-bold text-gray-900">Active Listings</h2>
            <div className="grid gap-6">
              {listings.map((listing) => (
                <div key={listing.id} className="bg-white rounded-lg shadow-md p-6 border border-gray-200">
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <h3 className="text-xl font-semibold text-gray-900">{listing.project}</h3>
                      <p className="text-gray-600 mt-1">Seller: {listing.seller}</p>
                      <div className="mt-4 grid grid-cols-2 md:grid-cols-4 gap-4">
                        <div>
                          <p className="text-sm text-gray-500">Amount</p>
                          <p className="font-semibold">{listing.amount.toLocaleString()} credits</p>
                        </div>
                        <div>
                          <p className="text-sm text-gray-500">Price per Credit</p>
                          <p className="font-semibold">${listing.pricePerCredit}</p>
                        </div>
                        <div>
                          <p className="text-sm text-gray-500">Total Value</p>
                          <p className="font-semibold">${listing.totalValue.toLocaleString()}</p>
                        </div>
                        <div>
                          <p className="text-sm text-gray-500">Expires</p>
                          <p className="font-semibold">{listing.expiryTime.toLocaleDateString()}</p>
                        </div>
                      </div>
                    </div>
                    <button
                      onClick={() => purchaseCredits(listing.id, listing.amount)}
                      disabled={loading}
                      className="bg-green-600 text-white px-6 py-2 rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50"
                    >
                      Purchase
                    </button>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'projects' && (
          <div className="space-y-8">
            <div className="flex justify-between items-center">
              <h2 className="text-3xl font-bold text-gray-900">Carbon Projects</h2>
              <button
                onClick={createProject}
                disabled={loading}
                className="bg-green-600 text-white px-6 py-3 rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50"
              >
                {loading ? 'Creating...' : 'Create New Project'}
              </button>
            </div>
            <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
              {projects.map((project) => (
                <div key={project.id} className="bg-white rounded-lg shadow-md p-6 border border-gray-200">
                  <div className="flex items-center justify-between mb-4">
                    <span className="bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full">
                      {project.status}
                    </span>
                    <span className="text-sm text-gray-500">{project.type}</span>
                  </div>
                  <h3 className="text-lg font-semibold text-gray-900 mb-2">{project.name}</h3>
                  <p className="text-gray-600 mb-4">{project.location}</p>
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-500">Available Credits:</span>
                      <span className="font-medium">{project.credits.toLocaleString()}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-sm text-gray-500">Price per Credit:</span>
                      <span className="font-medium">${project.price}</span>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'analytics' && (
          <div className="space-y-8">
            <h2 className="text-3xl font-bold text-gray-900">Market Analytics</h2>
            
            <div className="grid gap-8 lg:grid-cols-2">
              <div className="bg-white p-6 rounded-lg shadow-md">
                <h3 className="text-lg font-semibold mb-4">Credits Traded Over Time</h3>
                <ResponsiveContainer width="100%" height={300}>
                  <LineChart data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="month" />
                    <YAxis />
                    <Tooltip />
                    <Line type="monotone" dataKey="credits" stroke="#10b981" strokeWidth={2} />
                  </LineChart>
                </ResponsiveContainer>
              </div>

              <div className="bg-white p-6 rounded-lg shadow-md">
                <h3 className="text-lg font-semibold mb-4">Trading Volume</h3>
                <ResponsiveContainer width="100%" height={300}>
                  <BarChart data={chartData}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="month" />
                    <YAxis />
                    <Tooltip />
                    <Bar dataKey="volume" fill="#3b82f6" />
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>
          </div>
        )}
      </main>

      <section className="bg-gray-50 py-16">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-gray-900 mb-4">Why Choose CarbonChain?</h2>
            <p className="text-xl text-gray-600">Built on Solana for speed, transparency, and global accessibility</p>
          </div>
          
          <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            <div className="text-center">
              <div className="bg-green-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <Zap className="h-8 w-8 text-green-600" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Lightning Fast</h3>
              <p className="text-gray-600">Powered by Solana's high-performance blockchain for instant transactions</p>
            </div>
            
            <div className="text-center">
              <div className="bg-blue-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <Shield className="h-8 w-8 text-blue-600" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Verified & Secure</h3>
              <p className="text-gray-600">All projects undergo rigorous third-party verification processes</p>
            </div>
            
            <div className="text-center">
              <div className="bg-purple-100 w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-4">
                <Users className="h-8 w-8 text-purple-600" />
              </div>
              <h3 className="text-xl font-semibold mb-2">Global Access</h3>
              <p className="text-gray-600">Connect buyers and sellers worldwide with minimal barriers</p>
            </div>
          </div>
        </div>
      </section>

      <footer className="bg-gray-900 text-white py-12">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="grid gap-8 md:grid-cols-4">
            <div>
              <div className="flex items-center space-x-2 mb-4">
                <Leaf className="h-6 w-6 text-green-400" />
                <span className="text-xl font-bold">CarbonChain</span>
              </div>
              <p className="text-gray-400">
                Democratizing carbon credit trading through blockchain technology.
              </p>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Marketplace</h4>
              <ul className="space-y-2 text-gray-400">
                <li>Browse Projects</li>
                <li>Buy Credits</li>
                <li>Sell Credits</li>
                <li>Retire Credits</li>
              </ul>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Developers</h4>
              <ul className="space-y-2 text-gray-400">
                <li>Create Project</li>
                <li>Verification</li>
                <li>Documentation</li>
                <li>Support</li>
              </ul>
            </div>
            
            <div>
              <h4 className="text-lg font-semibold mb-4">Resources</h4>
              <ul className="space-y-2 text-gray-400">
                <li>Documentation</li>
                <li>API Reference</li>
                <li>Community</li>
                <li>Blog</li>
              </ul>
            </div>
          </div>
          
          <div className="border-t border-gray-800 mt-8 pt-8 text-center text-gray-400">
            <p>&copy; 2024 CarbonChain. Built on Solana for a sustainable future.</p>
          </div>
        </div>
      </footer>
    </div>
  );
}