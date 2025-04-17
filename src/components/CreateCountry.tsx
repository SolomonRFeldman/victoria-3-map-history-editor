import { useEffect, useState } from "react";
import { Country } from "./Countries";
import { invoke } from "@tauri-apps/api/core";

export type CountryDefinition = {
  tag: string
  color: string
}

type CreateCountryProps = {
  createdCountries: Country[]
  onCreateCountry: (countryDefinition: CountryDefinition) => void
}

export default function CreateCountry({ createdCountries, onCreateCountry }: CreateCountryProps) {
  const [countryDefinitions, setCountryDefinitions] = useState<CountryDefinition[]>([]);
  const [filter, setFilter] = useState<string>("");

  useEffect(() => {
    const createdTagSet = createdCountries.filter(country => country.border.length > 0).map(country => country.tag)
    handleGetUncreatedCountryDefinitions(createdTagSet)
  }, [createdCountries])
  
  const handleGetUncreatedCountryDefinitions = async (createdTagSet: string[]) => { 
    setCountryDefinitions((await invoke<CountryDefinition[]>("get_uncreated_country_definitions", { createdTagSet: createdTagSet }))) 
  }

  const handleKeyDown = (event: React.KeyboardEvent) => { event.stopPropagation() }
  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => { setFilter(event.target.value) }

  const filteredCountryDefinitions = countryDefinitions.filter(countryDefinition => countryDefinition.tag.toLowerCase().includes(filter.toLowerCase()))
  return (
    <div className="dropdown fixed top-3 left-16 z-[400]" onKeyDown={handleKeyDown}>
      <div tabIndex={0} role="button" className="btn">Create Country</div>
      <div tabIndex={0} className="dropdown-content z-[1] card bg-base-100 shadow-xl">
        <input type="text" value={filter} placeholder="Search Tags" className="input input-bordered input-sm" onChange={handleChange} />
        <ul className="menu menu-vertical p-2 max-h-60 overflow-y-scroll block">
          {filteredCountryDefinitions.map(countryDefinition => (
            <li className="block w-full" onClick={() => onCreateCountry(countryDefinition)}><a>{countryDefinition.tag}</a></li>
          ))}
        </ul>
      </div>
    </div>
  )
}
