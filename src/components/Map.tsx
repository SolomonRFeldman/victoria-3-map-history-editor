import { MapContainer, ImageOverlay } from 'react-leaflet'
import { CRS, LatLngBoundsExpression } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

export default function Map() {
  const [flatmap, setFlatmap] = useState<null | string>(null)
  const [flatmapOverlay, setFlatmapOverlay] = useState<null | string>('')

  useEffect(() => {
    const unlistenToFlatmap = listen<String>('load-flatmap', (event) => {
      setFlatmap(`data:image/png;base64,${event.payload}`)
    })

    const unlistenToFlatmapOverlay = listen<String>('load-flatmap-overlay', (event) => {
      setFlatmapOverlay(`data:image/png;base64,${event.payload}`)
    })

    return () => {
      unlistenToFlatmap.then((unlisten) => unlisten())
      unlistenToFlatmapOverlay.then((unlisten) => unlisten())
    }
  }, [])

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      { flatmap ? <ImageOverlay url={flatmap} bounds={bounds} /> : null }
      { flatmapOverlay ? <ImageOverlay url={flatmapOverlay} bounds={bounds} /> : null }
    </MapContainer>
  ) 
}
