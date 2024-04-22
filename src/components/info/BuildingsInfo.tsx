import { MinusIcon, PlusIcon } from "@heroicons/react/24/solid"
import { Building } from "../States"

type BuildingsInfoProps = {
  buildings: Building[]
  onBuildingsChange: (building: Building[]) => void
}

export default function BuildingsInfo({ buildings, onBuildingsChange }: BuildingsInfoProps) {
  const adjustBuildingLevel = (building: Building, amount: number) => {
    if (!building.level) return
    const newLevel = building.level + amount
    if (newLevel < 1) return

    const newBuilding = {...building, level: building.level + amount}
    const newBuildings = buildings.map((b) => b === building ? newBuilding : b)
    onBuildingsChange(newBuildings)
  }
  
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
              <td className="flex justify-center items-center max-w-16">
                <button className="btn btn-square btn-xs btn-error w-4 min-h-4 h-4" onClick={() => adjustBuildingLevel(building, -1)}><MinusIcon/></button>
                <p className="px-2 text-center">{building.level}</p>
                <button className="btn btn-square btn-xs btn-success w-4 min-h-4 h-4" onClick={() => adjustBuildingLevel(building, 1)}><PlusIcon/></button>
              </td>
            </tr>
          )
        })}
      </tbody>
    </table>
  )
}
