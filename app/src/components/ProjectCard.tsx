import { useState } from 'react';
import { MapPin, Calendar, Award, TrendingUp, Leaf, Zap, Globe, Users, Shield } from 'lucide-react';

interface ProjectCardProps {
  project: {
    id: string;
    name: string;
    type: string;
    location: string;
    credits: number;
    price: number;
    status: string;
    developer?: string;
    vintageYear?: number;
    methodology?: string;
    verificationStandard?: string;
  };
  onPurchase?: (projectId: string) => void;
  onViewDetails?: (projectId: string) => void;
}

export default function ProjectCard({ project, onPurchase, onViewDetails }: ProjectCardProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'verified':
        return 'bg-green-100 text-green-800';
      case 'pending':
        return 'bg-yellow-100 text-yellow-800';
      case 'suspended':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  const getProjectTypeIcon = (type: string) => {
    switch (type.toLowerCase()) {
      case 'forestry':
        return <Leaf className="h-6 w-6 text-green-600" />;
      case 'renewable energy':
        return <Zap className="h-6 w-6 text-yellow-600" />;
      case 'energy efficiency':
        return <TrendingUp className="h-6 w-6 text-blue-600" />;
      case 'methane':
        return <Award className="h-6 w-6 text-orange-600" />;
      case 'transportation':
        return <Globe className="h-6 w-6 text-purple-600" />;
      case 'agriculture':
        return <Users className="h-6 w-6 text-green-500" />;
      case 'waste management':
        return <Shield className="h-6 w-6 text-gray-600" />;
      case 'carbon capture':
        return <TrendingUp className="h-6 w-6 text-indigo-600" />;
      default:
        return <Globe className="h-6 w-6 text-gray-600" />;
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-md border border-gray-200 overflow-hidden hover:shadow-lg transition-shadow">
      <div className="p-6">
        <div className="flex items-start justify-between mb-4">
          <div className="flex items-center space-x-3">
            <div className="w-8 h-8 flex items-center justify-center">{getProjectTypeIcon(project.type)}</div>
            <div>
              <h3 className="text-lg font-semibold text-gray-900">{project.name}</h3>
              <p className="text-sm text-gray-600">{project.id}</p>
            </div>
          </div>
          <span className={`px-2.5 py-0.5 rounded-full text-xs font-medium ${getStatusColor(project.status)}`}>
            {project.status}
          </span>
        </div>

        <div className="space-y-3 mb-4">
          <div className="flex items-center text-sm text-gray-600">
            <MapPin className="h-4 w-4 mr-2" />
            {project.location}
          </div>
          
          <div className="flex items-center text-sm text-gray-600">
            <Award className="h-4 w-4 mr-2" />
            {project.type}
          </div>

          {project.vintageYear && (
            <div className="flex items-center text-sm text-gray-600">
              <Calendar className="h-4 w-4 mr-2" />
              Vintage: {project.vintageYear}
            </div>
          )}
        </div>

        <div className="grid grid-cols-2 gap-4 mb-4">
          <div className="bg-gray-50 p-3 rounded-lg">
            <p className="text-xs text-gray-500 mb-1">Available Credits</p>
            <p className="text-lg font-semibold text-gray-900">
              {project.credits.toLocaleString()}
            </p>
          </div>
          <div className="bg-gray-50 p-3 rounded-lg">
            <p className="text-xs text-gray-500 mb-1">Price per Credit</p>
            <p className="text-lg font-semibold text-green-600">
              ${project.price}
            </p>
          </div>
        </div>

        {isExpanded && (
          <div className="border-t pt-4 mb-4 space-y-2">
            {project.developer && (
              <div className="text-sm">
                <span className="font-medium text-gray-700">Developer:</span>
                <span className="ml-2 text-gray-600">{project.developer}</span>
              </div>
            )}
            {project.methodology && (
              <div className="text-sm">
                <span className="font-medium text-gray-700">Methodology:</span>
                <span className="ml-2 text-gray-600">{project.methodology}</span>
              </div>
            )}
            {project.verificationStandard && (
              <div className="text-sm">
                <span className="font-medium text-gray-700">Standard:</span>
                <span className="ml-2 text-gray-600">{project.verificationStandard}</span>
              </div>
            )}
          </div>
        )}

        <div className="flex space-x-3">
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="flex-1 px-4 py-2 text-sm font-medium text-gray-700 bg-gray-100 rounded-lg hover:bg-gray-200 transition-colors"
          >
            {isExpanded ? 'Show Less' : 'View Details'}
          </button>
          
          {onPurchase && project.status === 'Verified' && (
            <button
              onClick={() => onPurchase(project.id)}
              className="flex-1 px-4 py-2 text-sm font-medium text-white bg-green-600 rounded-lg hover:bg-green-700 transition-colors flex items-center justify-center"
            >
              <TrendingUp className="h-4 w-4 mr-1" />
              Purchase
            </button>
          )}
        </div>
      </div>
    </div>
  );
}