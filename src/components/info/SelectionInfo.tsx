import { Country } from "../Countries"
import { State } from "../States"
import { useEffect, useRef } from "react"
import { DomEvent } from "leaflet"
import StateInfo from "./StateInfo"
import CountryInfo from "./CountryInfo"

type SelectionInfoProps = {
  selectedCountry: Country
  selectedState: State | null
  selectedProvince: string | null
  onCountryChange: (country: Country) => void
}

export default function SelectionInfo({ selectedCountry, selectedState, selectedProvince, onCountryChange }: SelectionInfoProps) {
  const infoRef = useRef(null);
  useEffect(() => {
    if (infoRef.current) {
      DomEvent.disableClickPropagation(infoRef.current);
    }
  }, []);

  const handleStateChange = (state: State) => {
    onCountryChange({...selectedCountry, states: selectedCountry.states.map((s) => s.name === state.name ? state : s)})
  }

  return (
    <div ref={infoRef} className='fixed top-4 right-4 card card-compact bg-base-100 z-[400]'>
      <div className="card-body">
        <h1 className="card-title justify-end">Country: {selectedCountry.name}</h1>
        { 
          selectedProvince ? 
            <h3 className="card-title text-sm">Province: {selectedProvince}</h3> :
            selectedState ? 
              <StateInfo selectedState={selectedState} onStateChange={handleStateChange} /> :
              <CountryInfo country={selectedCountry} onChangeCountry={onCountryChange} />
        }
      </div>
    </div>
  )
}
