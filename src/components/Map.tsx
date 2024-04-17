import { MapContainer, ImageOverlay, GeoJSON } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
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

type StateCoords = {
  [key: string]: [number, number][][]
}

type Country = {
  name: string,
  color: string,
  coordinates: [number, number][][],
  states: State[]
}

type State = {
  name: string,
  color: string
  provinces: string[]
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
  const [stateData, setStateData] = useState<FeatureCollection | null>(null)
  const [stateCoords, setStateCoords] = useState<StateCoords>({})
  const [selectedCountry, setSelectedCountry] = useState('')

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
    const unlistenToStateData = listen<StateCoords>('load-state-coords', (data) => {
      console.log(data.payload)
      setStateCoords(data.payload)
    })
    return () => {
      unlistenToStateData.then((unlisten) => unlisten())
    }
  }, [])

  useEffect(() => {
    const unlistenToCountryData = listen<Country[]>('load-country-data', (data) => {
      console.log(data.payload)
      const geojsonData: FeatureCollection<Geometry, Country> = {
        type: "FeatureCollection",
        features: data.payload.map((country) => {
          return {
            type: "Feature",
            properties: country,
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

  const countryStyle = (feature?: Feature<Geometry, { name: string, color: string }>) => {
    return {
      fillColor: feature ? feature.properties.color.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 1
    }
  }

  const stateStyle = (feature?: Feature<Geometry, State>) => {
    return {
      fillColor: feature ? feature.properties.provinces[0].replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'purple',
      weight: 2
    }
  }

  const handleGeoJSONEvent = (event: LeafletMouseEvent) => {
    const country = event.sourceTarget.feature.properties as Country
    const geojsonData: FeatureCollection<Geometry, State> = {
      type: "FeatureCollection",
      features: country.states.map((state) => {
        return {
          type: "Feature",
          properties: state,
          geometry: {
            type: "Polygon",
            coordinates: stateCoords[`${country.name}:${state.name}`]
          }
        }
      })
    }

    setStateData(geojsonData)
    setSelectedCountry(country.name)
  }

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      { flatmap && <ImageOverlay url={flatmap} bounds={bounds} /> }
      { landMask && <ImageOverlay url={landMask} bounds={bounds} /> }
      { flatmapOverlay && <ImageOverlay url={flatmapOverlay} bounds={bounds} /> }
      { countryData && <GeoJSON data={countryData} style={countryStyle} eventHandlers={{ click: handleGeoJSONEvent }} /> }
      { stateData && <GeoJSON data={stateData} key={selectedCountry} style={stateStyle} /> }
    </MapContainer>
  ) 
}
