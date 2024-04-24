import { Building } from "../States"
import BuildingInfo from "./BuildingInfo"

type BuildingsInfoProps = {
  buildings: Building[]
  onBuildingsChange: (building: Building[]) => void
}

export default function BuildingsInfo({ buildings, onBuildingsChange }: BuildingsInfoProps) {
  const onBuildingChange = (building: Building) => {
    const newBuildings = buildings.map((b) => b.name === building.name ? building : b)
    onBuildingsChange(newBuildings)
  }
  
  return(
    <table className="table table-xs">
      <thead>
        <tr>
          <th className="max-w-5">PMs</th>
          <th className="max-w-32">Building</th>
          <th className="max-w-24">Production Methods</th>
          <th>Level</th>
        </tr>
      </thead>
      <tbody>
        {buildings.sort((building1, building2) => (building2.level || 0) - (building1.level || 0)).map((building) => <BuildingInfo key={building.name} stateBuilding={building} onBuildingChange={onBuildingChange} />)}
      </tbody>
    </table>
  )
}
