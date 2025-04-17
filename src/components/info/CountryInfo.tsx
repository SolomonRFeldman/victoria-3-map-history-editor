import { MinusIcon } from "@heroicons/react/24/solid"
import { useEffect, useState } from "react";
import { Country, CountryWithoutBorders, emptyCountry } from "../Countries";
import { invoke } from "@tauri-apps/api/core";
import SearchBox from "../form/SearchBox";

type Technology = {
  name: string,
  era: string,
  category: string,
}

type CountryInfoProps = {
  countryId: number
}

export default function CountryInfo({ countryId }: CountryInfoProps) {
  const [country, setCountry] = useState<CountryWithoutBorders>(emptyCountry)
  useEffect(() => {
    invoke<Country>("get_country", { id: countryId }).then(country => {
      setCountry(country)
    })
  }, [countryId])
  const onChangeCountry = (updatedCountry: CountryWithoutBorders) => {
    invoke("update_country", { country: updatedCountry }).then(() => {
      setCountry(updatedCountry)
    })
  }

  const handleChangeBaseTech = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const setup = { ...country.setup, base_tech: event.target.value }
    const updatedCountry = { ...country, setup }

    onChangeCountry(updatedCountry)
  }

  const handleAddTech = (tech: string) => {
    const setup = { ...country.setup, technologies_researched: [...country.setup.technologies_researched, tech] }
    onChangeCountry({ ...country, setup })
  }
  const handleRemoveTech = (tech: string) => {
    const setup = { ...country.setup, technologies_researched: country.setup.technologies_researched.filter(t => t !== tech) }
    onChangeCountry({ ...country, setup })
  }

  const handleGetTechnologies = async () => { setTechnologies((await invoke<Technology[]>("get_technologies", {}))) }
  const [technologies, setTechnologies] = useState<Technology[]>([])
  useEffect(() => { handleGetTechnologies() }, [])
  const filteredTechnologies = technologies.filter(tech => !country.setup.technologies_researched.includes(tech.name))

  return (
    <div>
      <h2>Base Tech: 
        <select className="select select-bordered select-xs w-full max-w-xs" value={country.setup.base_tech || ''} onChange={handleChangeBaseTech}>
          <option value="tier_1">tier_1</option>
          <option value="tier_2">tier_2</option>
          <option value="tier_3">tier_3</option>
          <option value="tier_4">tier_4</option>
          <option value="tier_5">tier_5</option>
          <option value="tier_6">tier_6</option>
          <option value="tier_7">tier_7</option>
        </select>
      </h2>
      <h2>Technologies Researched:</h2>
      <ul>
        {country.setup.technologies_researched.map(tech => (
          <li>{tech}
            <button className="btn float-right btn-square btn-xs btn-error w-4 min-h-4 h-4" onClick={() => handleRemoveTech(tech)}>
              <MinusIcon className="w-3 h-3"/>
            </button>
          </li>
        ))}
      </ul>
      <SearchBox options={filteredTechnologies.map(tech => ({ value: tech.name, label: tech.name }))} onSelect={handleAddTech} placeholder="Add Tech" />
    </div>
  )
}
