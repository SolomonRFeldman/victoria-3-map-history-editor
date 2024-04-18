import { MapContainer, ImageOverlay, GeoJSON } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { appCacheDir, join } from '@tauri-apps/api/path'
import { exists } from '@tauri-apps/api/fs'
import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
import { FeatureCollection, Feature, Geometry } from 'geojson'

type Coords = [number, number][][]

type ProvincesCoords = {
  [key: string]: Coords
}

type StateCoords = {
  [key: string]: Coords
}

type Country = {
  name: string,
  color: string,
  coordinates: Coords,
  states: State[]
}

type State = {
  name: string,
  color: string
  provinces: string[]
}

type TransferStateResponse = {
  to_country: Country,
  from_country: Country,
  state_coords: Coords
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
  const [countries, setCountries] = useState<Country[]>([])
  const [stateData, setStateData] = useState<FeatureCollection | null>(null)
  const [provinceData, setProvinceData] = useState<FeatureCollection | null>(null)
  const [stateCoords, setStateCoords] = useState<StateCoords>({})
  const [provinceCoords, setProvinceCoords] = useState<ProvincesCoords>({})
  const [selectedCountry, setSelectedCountry] = useState<Country | null>(null)
  const [selectedState, setSelectedState] = useState('')
  const [stateToTransfer, setStateToTransfer] = useState<State | null>(null)
  const [renderBreaker, setRenderBreaker] = useState(Date.now())
  const forceRerender = () => setRenderBreaker(Date.now())

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
    const unlistenToProvinceCoords = listen<ProvincesCoords>('load-province-coords', (data) => {
      console.log(data.payload)
      setProvinceCoords(data.payload)
    })
    return () => {
      unlistenToProvinceCoords.then((unlisten) => unlisten())
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
      setCountries(data.payload)
      forceRerender()
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

  const provinceStyle = (feature?: Feature<Geometry, { name: string }>) => {
    return {
      fillColor: feature ? feature.properties.name.replace('x', '#') : 'transparent',
      fillOpacity: 0.5,
      color: 'black',
      weight: 2
    }
  }

  const handleControlClickCountry = async (event: LeafletMouseEvent) => {
    if (selectedCountry && stateToTransfer) {
      setStateToTransfer(null)
      const toCountry = event.sourceTarget.feature.properties as Country
      const transferStateResponse = await invoke<TransferStateResponse>("transfer_state", { 
        state: stateToTransfer.name,
        fromCountry: selectedCountry,
        toCountry,
        fromCoords: stateCoords[`${selectedCountry.name}:${stateToTransfer.name}`],
        toCoords: stateCoords[`${toCountry.name}:${stateToTransfer.name}`] || [] 
      })
      console.log(transferStateResponse)
      const stateCoordsCopy = { ...stateCoords, [`${toCountry.name}:${stateToTransfer.name}`]: transferStateResponse.state_coords }
      setStateCoords(stateCoordsCopy)

      const fromCountryIndex = countries.findIndex((country) => country.name === selectedCountry.name)
      const toCountryIndex = countries.findIndex((country) => country.name === toCountry.name)

      countries[toCountryIndex] = transferStateResponse.to_country
      countries[fromCountryIndex] = transferStateResponse.from_country

      
      const geojsonData: FeatureCollection<Geometry, State> = {
        type: "FeatureCollection",
        features: transferStateResponse.from_country.states.map((state) => {
          return {
            type: "Feature",
            properties: state,
            geometry: {
              type: "Polygon",
              coordinates: stateCoords[`${transferStateResponse.from_country.name}:${state.name}`]
            }
          }
        })
      }
      setStateData(geojsonData)
      setSelectedCountry(transferStateResponse.from_country)

      setCountries([...countries])
      forceRerender()
    }
  }

  const handleClickCountry = (event: LeafletMouseEvent) => {
    if (event.originalEvent.ctrlKey) { return handleControlClickCountry(event) }

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

    setProvinceData(null)
    setSelectedState('')
    setStateData(geojsonData)
    setSelectedCountry(country)
  }

  const handleControlClickState = (event: LeafletMouseEvent) => {
    console.log(event.sourceTarget.feature.properties)
    setStateToTransfer(event.sourceTarget.feature.properties as State)
  }

  const handleClickState = (event: LeafletMouseEvent) => {
    if (event.originalEvent.ctrlKey) { return handleControlClickState(event) }

    const state = event.sourceTarget.feature.properties as State
    if (!state.provinces.every((province) => provinceCoords[province] !== undefined)) {
      console.log(`Provinces data for ${state.name} not found`)
      console.log(state.provinces)
      return
    }
    const geojsonData: FeatureCollection<Geometry, { name: string }> = {
      type: "FeatureCollection",
      features: state.provinces.map((province) => {
        return {
          type: "Feature",
          properties: { name: province },
          geometry: {
            type: "Polygon",
            coordinates: provinceCoords[province]
          }
        }
      })
    }
    setProvinceData(geojsonData)
    setSelectedState(state.name)
  }
  
  const countryData: FeatureCollection<Geometry, Country> = {
    type: "FeatureCollection",
    features: countries.map((country) => {
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

  useEffect(() => {
    console.log(countries)
  }, [countries])

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      { flatmap && <ImageOverlay url={flatmap} bounds={bounds} /> }
      { landMask && <ImageOverlay url={landMask} bounds={bounds} /> }
      { flatmapOverlay && <ImageOverlay url={flatmapOverlay} bounds={bounds} /> }
      { countryData && <GeoJSON data={countryData} key={renderBreaker} style={countryStyle} eventHandlers={{ click: handleClickCountry }} /> }
      { stateData && <GeoJSON data={stateData} key={selectedCountry ? selectedCountry.name + renderBreaker : undefined} style={stateStyle} eventHandlers={{ click: handleClickState }} /> }
      { provinceData && <GeoJSON data={provinceData} key={selectedState} style={provinceStyle} /> }
    </MapContainer>
  ) 
}
