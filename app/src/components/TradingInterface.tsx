import { useState, useEffect } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { Search, Filter, SortAsc, SortDesc, RefreshCw } from 'lucide-react';
import toast from 'react-hot-toast';

interface Listing {
  id: string;
  project: string;
  projectType: string;
  seller: string;
  amount: number;
  pricePerCredit: number;
  totalValue: number;
  expiryTime: Date;
  location: string;
  vintageYear: number;
  verificationStandard: string;
}

interface TradingInterfaceProps {
  listings: Listing[];
  onPurchase: (listingId: string, amount: number) => Promise<void>;
  onRefresh: () => void;
}

export default function TradingInterface({ listings, onPurchase, onRefresh }: TradingInterfaceProps) {
  const { connected } = useWallet();
  const [filteredListings, setFilteredListings] = useState<Listing[]>(listings);
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'price' | 'amount' | 'expiry'>('price');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const [filterType, setFilterType] = useState<string>('all');
  const [filterStandard, setFilterStandard] = useState<string>('all');
  const [priceRange, setPriceRange] = useState<[number, number]>([0, 100]);
  const [selectedListing, setSelectedListing] = useState<string | null>(null);
  const [purchaseAmount, setPurchaseAmount] = useState<number>(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    let filtered = listings.filter(listing => {
      const matchesSearch = listing.project.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           listing.seller.toLowerCase().includes(searchTerm.toLowerCase()) ||
                           listing.location.toLowerCase().includes(searchTerm.toLowerCase());
      
      const matchesType = filterType === 'all' || listing.projectType === filterType;
      const matchesStandard = filterStandard === 'all' || listing.verificationStandard === filterStandard;
      const matchesPrice = listing.pricePerCredit >= priceRange[0] && listing.pricePerCredit <= priceRange[1];
      
      return matchesSearch && matchesType && matchesStandard && matchesPrice;
    });

    filtered.sort((a, b) => {
      let aValue, bValue;
      switch (sortBy) {
        case 'price':
          aValue = a.pricePerCredit;
          bValue = b.pricePerCredit;
          break;
        case 'amount':
          aValue = a.amount;
          bValue = b.amount;
          break;
        case 'expiry':
          aValue = a.expiryTime.getTime();
          bValue = b.expiryTime.getTime();
          break;
        default:
          return 0;
      }
      
      return sortOrder === 'asc' ? aValue - bValue : bValue - aValue;
    });

    setFilteredListings(filtered);
  }, [listings, searchTerm, sortBy, sortOrder, filterType, filterStandard, priceRange]);

  const handlePurchase = async (listingId: string) => {
    if (!connected) {
      toast.error('Please connect your wallet first');
      return;
    }

    if (purchaseAmount <= 0) {
      toast.error('Please enter a valid amount');
      return;
    }

    setLoading(true);
    try {
      await onPurchase(listingId, purchaseAmount);
      setSelectedListing(null);
      setPurchaseAmount(0);
      toast.success('Purchase completed successfully!');
    } catch (error) {
      toast.error('Purchase failed. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const projectTypes = [...new Set(listings.map(l => l.projectType))];
  const verificationStandards = [...new Set(listings.map(l => l.verificationStandard))];

  return (
    <div className="space-y-6">
      {/* Search and Filters */}
      <div className="bg-white rounded-lg shadow-md p-6 border border-gray-200">
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-4 mb-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 h-4 w-4" />
            <input
              type="text"
              placeholder="Search projects, sellers, locations..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
            />
          </div>

          <select
            value={filterType}
            onChange={(e) => setFilterType(e.target.value)}
            className="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
          >
            <option value="all">All Project Types</option>
            {projectTypes.map(type => (
              <option key={type} value={type}>{type}</option>
            ))}
          </select>

          <select
            value={filterStandard}
            onChange={(e) => setFilterStandard(e.target.value)}
            className="px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
          >
            <option value="all">All Standards</option>
            {verificationStandards.map(standard => (
              <option key={standard} value={standard}>{standard}</option>
            ))}
          </select>

          <div className="flex space-x-2">
            <select
              value={sortBy}
              onChange={(e) => setSortBy(e.target.value as any)}
              className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
            >
              <option value="price">Sort by Price</option>
              <option value="amount">Sort by Amount</option>
              <option value="expiry">Sort by Expiry</option>
            </select>
            
            <button
              onClick={() => setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc')}
              className="px-3 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              {sortOrder === 'asc' ? <SortAsc className="h-4 w-4" /> : <SortDesc className="h-4 w-4" />}
            </button>
            
            <button
              onClick={onRefresh}
              className="px-3 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
            >
              <RefreshCw className="h-4 w-4" />
            </button>
          </div>
        </div>

        <div className="flex items-center space-x-4">
          <span className="text-sm font-medium text-gray-700">Price Range:</span>
          <input
            type="range"
            min="0"
            max="100"
            value={priceRange[0]}
            onChange={(e) => setPriceRange([parseInt(e.target.value), priceRange[1]])}
            className="flex-1"
          />
          <span className="text-sm text-gray-600">${priceRange[0]} - ${priceRange[1]}</span>
        </div>
      </div>

      {/* Listings */}
      <div className="space-y-4">
        <div className="flex justify-between items-center">
          <h3 className="text-xl font-semibold text-gray-900">
            Active Listings ({filteredListings.length})
          </h3>
        </div>

        {filteredListings.length === 0 ? (
          <div className="bg-white rounded-lg shadow-md p-12 text-center border border-gray-200">
            <p className="text-gray-500 text-lg">No listings match your criteria</p>
            <button
              onClick={() => {
                setSearchTerm('');
                setFilterType('all');
                setFilterStandard('all');
                setPriceRange([0, 100]);
              }}
              className="mt-4 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
            >
              Clear Filters
            </button>
          </div>
        ) : (
          <div className="grid gap-4">
            {filteredListings.map((listing) => (
              <div key={listing.id} className="bg-white rounded-lg shadow-md p-6 border border-gray-200 hover:shadow-lg transition-shadow">
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3 mb-2">
                      <h4 className="text-lg font-semibold text-gray-900">{listing.project}</h4>
                      <span className="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded-full">
                        {listing.projectType}
                      </span>
                      <span className="bg-green-100 text-green-800 text-xs font-medium px-2.5 py-0.5 rounded-full">
                        {listing.verificationStandard}
                      </span>
                    </div>
                    
                    <p className="text-gray-600 mb-3">
                      Seller: {listing.seller} • {listing.location} • Vintage: {listing.vintageYear}
                    </p>
                    
                    <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                      <div>
                        <p className="text-sm text-gray-500">Available</p>
                        <p className="font-semibold">{listing.amount.toLocaleString()} credits</p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-500">Price per Credit</p>
                        <p className="font-semibold text-green-600">${listing.pricePerCredit}</p>
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
                  
                  <div className="ml-6">
                    <button
                      onClick={() => setSelectedListing(listing.id)}
                      disabled={!connected}
                      className="px-6 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      Purchase
                    </button>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Purchase Modal */}
      {selectedListing && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-lg p-6 max-w-md w-full mx-4">
            <h3 className="text-lg font-semibold mb-4">Purchase Carbon Credits</h3>
            
            {(() => {
              const listing = listings.find(l => l.id === selectedListing);
              if (!listing) return null;
              
              return (
                <div className="space-y-4">
                  <div>
                    <p className="text-sm text-gray-600">Project: {listing.project}</p>
                    <p className="text-sm text-gray-600">Available: {listing.amount.toLocaleString()} credits</p>
                    <p className="text-sm text-gray-600">Price: ${listing.pricePerCredit} per credit</p>
                  </div>
                  
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-2">
                      Amount to Purchase
                    </label>
                    <input
                      type="number"
                      min="1"
                      max={listing.amount}
                      value={purchaseAmount}
                      onChange={(e) => setPurchaseAmount(parseInt(e.target.value) || 0)}
                      className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-transparent"
                      placeholder="Enter amount"
                    />
                  </div>
                  
                  {purchaseAmount > 0 && (
                    <div className="bg-gray-50 p-3 rounded-lg">
                      <p className="text-sm text-gray-600">Total Cost: ${(purchaseAmount * listing.pricePerCredit).toLocaleString()}</p>
                    </div>
                  )}
                  
                  <div className="flex space-x-3">
                    <button
                      onClick={() => {
                        setSelectedListing(null);
                        setPurchaseAmount(0);
                      }}
                      className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors"
                    >
                      Cancel
                    </button>
                    <button
                      onClick={() => handlePurchase(selectedListing)}
                      disabled={loading || purchaseAmount <= 0 || purchaseAmount > listing.amount}
                      className="flex-1 px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                      {loading ? 'Processing...' : 'Confirm Purchase'}
                    </button>
                  </div>
                </div>
              );
            })()}
          </div>
        </div>
      )}
    </div>
  );
}