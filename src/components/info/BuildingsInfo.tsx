import { Building } from "../States"

type BuildingsInfoProps = {
  buildings: Building[]
}

export default function BuildingsInfo({ buildings }: BuildingsInfoProps) {  
  return(
    <table className="table table-xs">
      <thead>
        <tr>
          <th className="max-w-32">Building</th>
          <th className="max-w-24">Production Methods</th>
          <th>Level</th>
        </tr>
      </thead>
      <tbody>
        {buildings.sort((building1, building2) => (building2.level || 0) - (building1.level || 0)).map((building) => {
          const productionMethods = building.activate_production_methods?.join(', ')
          return (
            <tr key={building.name}>
              <td className="max-w-32">{building.name}</td>
              <td className="max-w-24">
                <div className="tooltip tooltip-bottom max-w-full" data-tip={productionMethods || ''}>
                  <div className="overflow-hidden truncate">{productionMethods}</div>
                </div>
              </td>
              <td>{building.level}</td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}
