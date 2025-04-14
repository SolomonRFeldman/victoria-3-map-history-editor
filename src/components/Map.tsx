import { MapContainer } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/core'
import Countries, { Country } from './Countries'
import States, { State } from './States'
import Provinces from './Provinces'
import Background from './Background'
import { exists, readTextFile } from '@tauri-apps/plugin-fs';
import { appCacheDir } from '@tauri-apps/api/path'
import SelectionInfo from './info/SelectionInfo'
import CreateCountry from './CreateCountry'

export type Coords = [number, number][][]

type ProvincesCoords = {
  [key: string]: Coords
}

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

const getProvinceCoords = async () => {
  const cacheDir = await appCacheDir()
  const path = `${cacheDir}/provinces.json`
  const fileExists = await exists(path)
  
  if (!fileExists) { return {} }

  const fileContents = await readTextFile(`${cacheDir}/provinces.json`);
  return JSON.parse(fileContents) as ProvincesCoords
}

const getCountries = async () => invoke<Country[]>("get_countries", {})

export default function Map() {
  const [countries, setCountries] = useState<Country[]>([])
  const [provinceCoords, setProvinceCoords] = useState<ProvincesCoords>({})
  const [selectedCountry, setSelectedCountry] = useState<Country | null>(null)
  const [selectedState, setSelectedState] = useState<State | null>(null)
  const [selectedProvince, setSelectedProvince] = useState<string | null>(null)
  const [renderBreaker, setRenderBreaker] = useState(Date.now())
  const forceRerender = () => setRenderBreaker(Date.now())

  useEffect(() => {
    const unlistenToProvinceCoords = listen<ProvincesCoords>('load-province-coords', () => {
      getProvinceCoords().then((provinceCoords) => {
        console.log(provinceCoords)
        setProvinceCoords(provinceCoords)
      })
    })

    const unlistenToCountryData = listen<Country[]>('load-country-data', () => {
      getCountries().then((countries) => {
        console.log(countries)
        setCountries(countries)
        setSelectedProvince(null)
        setSelectedState(null)
        setSelectedCountry(null)
        forceRerender()
      })
    })

    getProvinceCoords().then((provinceCoords) => setProvinceCoords(provinceCoords))
    getCountries().then((countries) => {
      setCountries(countries)
      forceRerender()
    })

    return () => {
      unlistenToProvinceCoords.then((unlisten) => unlisten())
      unlistenToCountryData.then((unlisten) => unlisten())
    }
  }, [])

  const handleControlClickCountry = async () => {
  }

  const handleClickCountry = (event: LeafletMouseEvent) => {
    const country = event.sourceTarget.feature.properties as Country
    const selectedCountry = countries.find((c) => c.tag === country.tag) || null

    if (event.originalEvent.ctrlKey || event.originalEvent.metaKey) { return handleControlClickCountry() }

    setSelectedProvince(null)
    setSelectedState(null)
    setSelectedCountry(selectedCountry)
    setStates([])
    if (selectedCountry) {
      invoke<State[]>('get_states', { countryId: selectedCountry.id }).then((states) => {
        setStates(states)
      })
    }
  }

  const handleClickState = (event: LeafletMouseEvent) => {
    const state = event.sourceTarget.feature.properties as State
    setSelectedProvince(null)
    setSelectedState(state)
  }

  const handleClickProvince = (event: LeafletMouseEvent) => {
    const province = event.sourceTarget.feature.properties.name
    setSelectedProvince(province)
  }

  useEffect(() => {
    const handleEscapePress = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (selectedProvince) { setSelectedProvince(null) } else if (selectedState) { setSelectedState(null) } else if (selectedCountry) { setSelectedCountry(null) }
      }
    }
    window.addEventListener('keydown', handleEscapePress)
    return () => window.removeEventListener('keydown', handleEscapePress)
  }, [selectedCountry, selectedState, selectedProvince])

  const handleCountryChange = () => {
  }

  const handleCreateCountry = async () => {
  }

  const [states, setStates] = useState<State[]>([])

  useEffect(() => {
 }, [selectedCountry])

  return (
    <div>
      <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
        <Background bounds={bounds} />
        <Countries countries={countries} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickCountry }} />
        {
          selectedCountry && states.length && 
            <States 
              country={selectedCountry} 
              states={states} 
              renderBreaker={renderBreaker} 
              eventHandlers={{ click: handleClickState  }} 
              selectedState={selectedState} 
            /> 
        }
        {
          selectedState &&
            <Provinces
              state={selectedState}
              provinceCoords={provinceCoords}
              renderBreaker={renderBreaker}
              eventHandlers={{ click: handleClickProvince }}
              selectedProvince={selectedProvince}
            />
        }
      </MapContainer>
      {
        selectedCountry &&
          <SelectionInfo
            selectedCountry={selectedCountry}
            selectedState={selectedState}
            selectedProvince={selectedProvince}
            onCountryChange={handleCountryChange}
          />
      }
      {
        selectedState &&
          <CreateCountry
            createdCountries={countries}
            onCreateCountry={handleCreateCountry}
          />
      }
    </div>
  ) 
}
