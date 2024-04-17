import { MapContainer, ImageOverlay, GeoJSON } from 'react-leaflet'
import { CRS, LatLngBoundsExpression } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { appCacheDir, join } from '@tauri-apps/api/path'
import { exists } from '@tauri-apps/api/fs'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { FeatureCollection, Feature, Geometry } from 'geojson'

type Provinces = {
  [key: string]: [number, number][][]
}

type SubState = {
  coordinates: [number, number][][]
  provinces: string[]
  owner: string
}

type State = {
  name: string,
  sub_states: SubState[]
}

type Country = {
  name: string,
  color: string,
  coordinates: [number, number][][]
}

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
  const [, setProvinceData] = useState<Provinces | null>(null)
  const [countryData, setCountryData] = useState<FeatureCollection | null>(null)

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

  useEffect(() => {
    const unlistenToProvinceData = listen<Provinces>('load-province-data', (data) => {
      console.log(data.payload)
      setProvinceData(data.payload)
    })
    return () => {
      unlistenToProvinceData.then((unlisten) => unlisten())
    }
  }, [])

  useEffect(() => {
    const unlistenToStateData = listen<State[]>('load-state-data', (data) => {
      console.log(data.payload)
    })
    return () => {
      unlistenToStateData.then((unlisten) => unlisten())
    }
  }, [])

  useEffect(() => {
    const unlistenToCountryData = listen<Country[]>('load-country-data', (data) => {
      console.log(data.payload)
      const geojsonData: FeatureCollection<Geometry, { name: string, color: string }> = {
        type: "FeatureCollection",
        features: data.payload.map((country) => {
          return {
            type: "Feature",
            properties: {
              name: country.name,
              color: country.color
            },
            geometry: {
              type: "Polygon",
              coordinates: country.coordinates
            }
          }
        })
      }
      setCountryData(geojsonData)
    })
    return () => {
      unlistenToCountryData.then((unlisten) => unlisten())
    }
  }, [])

  const polygonStyle = (feature?: Feature<Geometry, { name: string, color: string }>) => {
    return {
      fillColor: feature ? feature.properties.color.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 1
    }
  }

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      { flatmap ? <ImageOverlay url={flatmap} bounds={bounds} /> : null }
      { landMask ? <ImageOverlay url={landMask} bounds={bounds} /> : null }
      { flatmapOverlay ? <ImageOverlay url={flatmapOverlay} bounds={bounds} /> : null }
      { countryData ? <GeoJSON data={countryData} style={polygonStyle} /> : null }
    </MapContainer>
  ) 
}
