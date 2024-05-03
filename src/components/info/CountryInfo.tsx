import { Country } from "../Countries";

type CountryInfoProps = {
  country: Country
  onChangeCountry: (country: Country) => void
}

export default function CountryInfo({ country, onChangeCountry }: CountryInfoProps) {
  const handleChangeBaseTech = (event: React.ChangeEvent<HTMLSelectElement>) => {
    const setup = { ...country.setup, base_tech: event.target.value }
    const updatedCountry = { ...country, setup }

    onChangeCountry(updatedCountry)
  }

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
          <li>{tech}</li>
        ))}
      </ul>
    </div>
  )
}
