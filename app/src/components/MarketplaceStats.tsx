import { TrendingUp, Globe, ShoppingCart, Award, Users, Zap } from 'lucide-react';

interface MarketplaceStatsProps {
  data: {
    totalCreditsTraded: number;
    totalVolume: number;
    activeListings: number;
    verifiedProjects: number;
    totalUsers?: number;
    avgTransactionTime?: number;
  };
}

export default function MarketplaceStats({ data }: MarketplaceStatsProps) {
  const stats = [
    {
      name: 'Credits Traded',
      value: data.totalCreditsTraded.toLocaleString(),
      icon: TrendingUp,
      color: 'text-green-600',
      bgColor: 'bg-green-100',
      change: '+12.5%',
      changeType: 'positive'
    },
    {
      name: 'Total Volume',
      value: `$${data.totalVolume.toLocaleString()}`,
      icon: Globe,
      color: 'text-blue-600',
      bgColor: 'bg-blue-100',
      change: '+8.2%',
      changeType: 'positive'
    },
    {
      name: 'Active Listings',
      value: data.activeListings.toString(),
      icon: ShoppingCart,
      color: 'text-purple-600',
      bgColor: 'bg-purple-100',
      change: '+3.1%',
      changeType: 'positive'
    },
    {
      name: 'Verified Projects',
      value: data.verifiedProjects.toString(),
      icon: Award,
      color: 'text-orange-600',
      bgColor: 'bg-orange-100',
      change: '+15.7%',
      changeType: 'positive'
    }
  ];

  if (data.totalUsers) {
    stats.push({
      name: 'Total Users',
      value: data.totalUsers.toLocaleString(),
      icon: Users,
      color: 'text-indigo-600',
      bgColor: 'bg-indigo-100',
      change: '+22.3%',
      changeType: 'positive'
    });
  }

  if (data.avgTransactionTime) {
    stats.push({
      name: 'Avg Transaction Time',
      value: `${data.avgTransactionTime}ms`,
      icon: Zap,
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-100',
      change: '-5.2%',
      changeType: 'positive'
    });
  }

  return (
    <div className="bg-white py-16">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold text-gray-900 mb-4">Marketplace Statistics</h2>
          <p className="text-xl text-gray-600">Real-time data from the global carbon credit marketplace</p>
        </div>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {stats.map((stat, index) => (
            <div key={index} className="relative bg-white p-6 rounded-lg shadow-md border border-gray-200 hover:shadow-lg transition-shadow">
              <div className="flex items-center justify-between mb-4">
                <div className={`w-12 h-12 rounded-full ${stat.bgColor} flex items-center justify-center`}>
                  <stat.icon className={`h-6 w-6 ${stat.color}`} />
                </div>
                <div className={`text-sm font-medium ${
                  stat.changeType === 'positive' ? 'text-green-600' : 'text-red-600'
                }`}>
                  {stat.change}
                </div>
              </div>
              
              <div>
                <h3 className="text-2xl font-bold text-gray-900 mb-1">{stat.value}</h3>
                <p className="text-gray-600 text-sm">{stat.name}</p>
              </div>
              
              <div className="absolute bottom-0 left-0 w-full h-1 bg-gradient-to-r from-green-400 to-blue-500 rounded-b-lg"></div>
            </div>
          ))}
        </div>

        <div className="mt-12 grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="bg-gradient-to-r from-green-500 to-green-600 rounded-lg p-6 text-white">
            <h3 className="text-lg font-semibold mb-2">Environmental Impact</h3>
            <p className="text-3xl font-bold mb-1">{(data.totalCreditsTraded * 1.2).toLocaleString()}</p>
            <p className="text-green-100">Tons CO₂ Offset</p>
          </div>
          
          <div className="bg-gradient-to-r from-blue-500 to-blue-600 rounded-lg p-6 text-white">
            <h3 className="text-lg font-semibold mb-2">Global Reach</h3>
            <p className="text-3xl font-bold mb-1">45+</p>
            <p className="text-blue-100">Countries Participating</p>
          </div>
          
          <div className="bg-gradient-to-r from-purple-500 to-purple-600 rounded-lg p-6 text-white">
            <h3 className="text-lg font-semibold mb-2">Transaction Speed</h3>
            <p className="text-3xl font-bold mb-1">0.4s</p>
            <p className="text-purple-100">Average Settlement Time</p>
          </div>
        </div>
      </div>
    </div>
  );
}