import { MapContainer, ImageOverlay } from 'react-leaflet'
import { CRS, LatLngBoundsExpression } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { appCacheDir, join } from '@tauri-apps/api/path'
import { exists } from '@tauri-apps/api/fs'
import { convertFileSrc } from '@tauri-apps/api/tauri'

const flatmapFileName = 'flatmap_votp.png'
const landMaskFileName = 'land_mask.png'
const flatmapOverlayFileName = 'flatmap_overlay_votp.png'

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

const getImagePath = async (filename: string, callback: (path: string | null) => void) => {
  const cacheDir = await appCacheDir()
  const path = await join(cacheDir, filename)
  const fileExists = await exists(path)
  console.log(`File ${filename} at ${path} exists: ${fileExists}`)

  return fileExists ? callback(convertFileSrc(path)) : null
}

export default function Map() {
  const [flatmap, setFlatmap] = useState<null | string>(null)
  const [flatmapOverlay, setFlatmapOverlay] = useState<null | string>('')
  const [landMask, setLandMask] = useState<null | string>('')

  useEffect(() => {
    const unlistenToFlatmap = listen<String>('load-flatmap', () => {
      getImagePath(flatmapFileName, setFlatmap)
    })
    getImagePath(flatmapFileName, setFlatmap)

    const unlistenToLandMask = listen<String>('load-land-mask', () => {
      getImagePath(landMaskFileName, setLandMask)
    })
    getImagePath(landMaskFileName, setLandMask)

    const unlistenToFlatmapOverlay = listen<String>('load-flatmap-overlay', () => {
      getImagePath(flatmapOverlayFileName, setFlatmapOverlay)
    })
    getImagePath(flatmapOverlayFileName, setFlatmapOverlay)

    return () => {
      unlistenToFlatmap.then((unlisten) => unlisten())
      unlistenToFlatmapOverlay.then((unlisten) => unlisten())
      unlistenToLandMask.then((unlisten) => unlisten())
    }
  }, [])

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      { flatmap ? <ImageOverlay url={flatmap} bounds={bounds} /> : null }
      { landMask ? <ImageOverlay url={landMask} bounds={bounds} /> : null }
      { flatmapOverlay ? <ImageOverlay url={flatmapOverlay} bounds={bounds} /> : null }
    </MapContainer>
  ) 
}
