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
}

export default function SelectionInfo({ selectedCountry, selectedState, selectedProvince }: SelectionInfoProps) {
  const infoRef = useRef(null);
  useEffect(() => {
    if (infoRef.current) {
      DomEvent.disableClickPropagation(infoRef.current);
    }
  }, []);

  return (
    <div ref={infoRef} className='fixed top-4 right-4 card card-compact bg-base-100 z-[400]'>
      <div className="card-body">
        <h1 className="card-title justify-end">Country: {selectedCountry.tag}</h1>
        { 
          selectedProvince ? 
            <h3 className="card-title text-sm">Province: {selectedProvince}</h3> :
            selectedState ? 
              <StateInfo selectedState={selectedState} /> :
              <CountryInfo countryId={selectedCountry.id} />
        }
      </div>
    </div>
  )
}
