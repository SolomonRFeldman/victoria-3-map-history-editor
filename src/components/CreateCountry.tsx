import { useEffect, useState } from "react";
import { Country } from "./Countries";
import { invoke } from "@tauri-apps/api";

type CountryDefinition = {
  tag: string
  color: string
}

type CreateCountryProps = {
  createdCountries: Country[]
}

export default function CreateCountry({ createdCountries }: CreateCountryProps) {
  const [countryDefinitions, setCountryDefinitions] = useState<CountryDefinition[]>([]);
  const [filter, setFilter] = useState<string>("");

  useEffect(() => {
    const createdTagSet = createdCountries.map(country => country.name)
    handleGetUncreatedCountryDefinitions(createdTagSet)
  }, [createdCountries])
  
  const handleGetUncreatedCountryDefinitions = async (createdTagSet: string[]) => { setCountryDefinitions((await invoke<CountryDefinition[]>("get_uncreated_country_definitions", { createdTagSet: createdTagSet }))) }

  const filteredCountryDefinitions = countryDefinitions.filter(countryDefinition => countryDefinition.tag.toLowerCase().includes(filter.toLowerCase()))
  return (
    <div className="dropdown fixed top-3 left-16 z-[400]">
      <div tabIndex={0} role="button" className="btn">Create Country</div>
      <div tabIndex={0} className="dropdown-content z-[1] card bg-base-100 shadow-xl">
        <input type="text" value={filter} placeholder="Search Tags" className="input input-bordered input-sm" onChange={event => setFilter(event.target.value)} />
        <ul className="menu menu-vertical p-2 max-h-60 overflow-y-scroll block">
          {filteredCountryDefinitions.map(countryDefinition => (
            <li className="block w-full"><a>{countryDefinition.tag}</a></li>
          ))}
        </ul>
      </div>
    </div>
  )
}
