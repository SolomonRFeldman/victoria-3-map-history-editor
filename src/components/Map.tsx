import { MapContainer, ImageOverlay, Marker, Popup } from 'react-leaflet'
import { CRS, LatLngBoundsExpression } from 'leaflet';
import flatmap from '../assets/flatmap_votp.png'
import './Map.css'

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

export default function Map() {
  return (
    <MapContainer center={[0, 0]} minZoom={-3} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      <ImageOverlay url={flatmap} bounds={bounds} />
    </MapContainer>
  ) 
}
