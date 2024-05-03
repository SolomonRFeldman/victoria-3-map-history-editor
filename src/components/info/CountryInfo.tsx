import { Country } from "../Countries";

export default function CountryInfo({ country }: { country: Country }) {
  return (
    <div>
      <h2>Base Tech: {country.setup.base_tech}</h2>
      <h2>Technologies Researched:</h2>
      <ul>
        {country.setup.technologies_researched.map(tech => (
          <li>{tech}</li>
        ))}
      </ul>
    </div>
  )
}
