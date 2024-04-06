import { MapContainer, ImageOverlay } from 'react-leaflet'
import { CRS, LatLngBoundsExpression } from 'leaflet'
import flatmap from '../assets/flatmap_votp.png'
import land_mask from '../assets/land_mask.png'
import flatmap_overlay from '../assets/flatmap_overlay_votp.png'
import './Map.css'

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

export default function Map() {
  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      <ImageOverlay url={flatmap} bounds={bounds} />
      <ImageOverlay url={land_mask} bounds={bounds} />
      <ImageOverlay url={flatmap_overlay} bounds={bounds} />
    </MapContainer>
  ) 
}
