import { MapContainer } from 'react-leaflet'
import { CRS, LatLngBoundsExpression, LeafletMouseEvent } from 'leaflet'
import './Map.css'
import { useEffect, useState } from 'react'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import Countries, { Country } from './Countries'
import States, { State } from './States'
import Provinces from './Provinces'
import Background from './Background'

export type Coords = [number, number][][]

type ProvincesCoords = {
  [key: string]: Coords
}

type StateCoords = {
  [key: string]: Coords
}

type TransferStateResponse = {
  to_country: Country,
  from_country: Country,
  state_coords: Coords
}

const bounds: LatLngBoundsExpression = [[0, 0], [3616, 8192]]

export default function Map() {
  const [countries, setCountries] = useState<Country[]>([])
  const [stateCoords, setStateCoords] = useState<StateCoords>({})
  const [provinceCoords, setProvinceCoords] = useState<ProvincesCoords>({})
  const [selectedCountry, setSelectedCountry] = useState<Country | null>(null)
  const [selectedState, setSelectedState] = useState<State | null>(null)
  const [renderBreaker, setRenderBreaker] = useState(Date.now())
  const forceRerender = () => setRenderBreaker(Date.now())

  useEffect(() => {
    const unlistenToProvinceCoords = listen<ProvincesCoords>('load-province-coords', (data) => {
      console.log(data.payload)
      setProvinceCoords(data.payload)
    })

    const unlistenToStateData = listen<StateCoords>('load-state-coords', (data) => {
      console.log(data.payload)
      setStateCoords(data.payload)
    })

    const unlistenToCountryData = listen<Country[]>('load-country-data', (data) => {
      console.log(data.payload)
      setCountries(data.payload)
      forceRerender()
    })

    return () => {
      unlistenToProvinceCoords.then((unlisten) => unlisten())
      unlistenToStateData.then((unlisten) => unlisten())
      unlistenToCountryData.then((unlisten) => unlisten())
    }
  }, [])

  const handleControlClickCountry = async (event: LeafletMouseEvent) => {
    if (selectedCountry && selectedState) {
      const toCountry = event.sourceTarget.feature.properties as Country
      const transferStateResponse = await invoke<TransferStateResponse>("transfer_state", { 
        state: selectedState.name,
        fromCountry: selectedCountry,
        toCountry,
        fromCoords: stateCoords[`${selectedCountry.name}:${selectedState.name}`],
        toCoords: stateCoords[`${toCountry.name}:${selectedState.name}`] || [] 
      })
      console.log(transferStateResponse)
      const stateCoordsCopy = { ...stateCoords, [`${toCountry.name}:${selectedState.name}`]: transferStateResponse.state_coords }
      setStateCoords(stateCoordsCopy)

      const fromCountryIndex = countries.findIndex((country) => country.name === selectedCountry.name)
      const toCountryIndex = countries.findIndex((country) => country.name === toCountry.name)

      countries[toCountryIndex] = transferStateResponse.to_country
      countries[fromCountryIndex] = transferStateResponse.from_country

      setSelectedState(null)
      setSelectedCountry(transferStateResponse.from_country)

      setCountries([...countries])
      forceRerender()
    }
  }

  const handleClickCountry = (event: LeafletMouseEvent) => {
    if (event.originalEvent.ctrlKey) { return handleControlClickCountry(event) }

    const country = event.sourceTarget.feature.properties as Country
    setSelectedState(null)
    setSelectedState(null)
    setSelectedCountry(country)
  }

  const handleClickState = (event: LeafletMouseEvent) => {
    const state = event.sourceTarget.feature.properties as State
    if (!state.provinces.every((province) => provinceCoords[province] !== undefined)) {
      console.log(`Provinces data for ${state.name} not found`)
      console.log(state.provinces)
      return
    }
    setSelectedState(state)
  }

  useEffect(() => {
    const handleEscapePress = (event: KeyboardEvent) => {
      if (event.key === 'Escape') {
        if (selectedState) { setSelectedState(null) } else if (selectedCountry) { setSelectedCountry(null) }
      }
    }
    window.addEventListener('keydown', handleEscapePress)
    return () => window.removeEventListener('keydown', handleEscapePress)
  }, [selectedCountry, selectedState])

  return (
    <MapContainer center={[0, 0]} minZoom={-2} maxZoom={2} doubleClickZoom={false} crs={CRS.Simple} bounds={bounds}>
      <Background bounds={bounds} />
      <Countries countries={countries} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickCountry }} />
      { selectedCountry && <States country={selectedCountry} stateCoords={stateCoords} renderBreaker={renderBreaker} eventHandlers={{ click: handleClickState }} /> }
      { selectedState && <Provinces state={selectedState} provinceCoords={provinceCoords} renderBreaker={renderBreaker} /> }
    </MapContainer>
  ) 
}
