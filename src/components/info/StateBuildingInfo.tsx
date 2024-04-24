import { MinusIcon, PlusIcon } from "@heroicons/react/24/solid"
import { StateBuilding } from "../States"
import { invoke } from "@tauri-apps/api"
import { ArrowLeftIcon } from "@heroicons/react/24/outline"
import ChooseProductionMethod, { ProductionMethodGroup } from "./ChooseProductionMethod"
import { useRef, useState } from "react"

type StateBuildingInfoProps = { 
  stateBuilding: StateBuilding
  onBuildingChange: (building: StateBuilding) => void
}
type Building = {
  name: string,
  production_method_groups: ProductionMethodGroup[]
}

export default function ({ stateBuilding, onBuildingChange }: StateBuildingInfoProps) {
  const adjustStateBuildingLevel = (stateBuilding: StateBuilding, amount: number) => {
    if (!stateBuilding.level) return
    const newLevel = stateBuilding.level + amount
    if (newLevel < 1) return

    const newBuilding = {...stateBuilding, level: stateBuilding.level + amount}
    onBuildingChange(newBuilding)
  }
  const handlePmChange = (newPms: string[]) => {
    const newBuilding = {...stateBuilding, activate_production_methods: newPms}
    onBuildingChange(newBuilding)
  }

  const renderBuildingsRef = useRef<HTMLDivElement>(null)

  const [pmgs, setPmgs] = useState<ProductionMethodGroup[]>([])
  const getBuilding = async (name: string) => { setPmgs((await invoke<Building>("get_building", { name })).production_method_groups) }

  const productionMethods = stateBuilding.activate_production_methods?.join(', ')
  return (
    <tr key={stateBuilding.name}>
      <td className="max-w-2 pl-3 dropdown dropdown-left">
        <button tabIndex={0} className="btn btn-square btn-xs btn-accent w-4 min-h-4 h-4" onClick={() => getBuilding(stateBuilding.name)}><ArrowLeftIcon className="w-3 h-3"/></button>
        <div tabIndex={0} className="dropdown-content" ref={renderBuildingsRef} >
          <ChooseProductionMethod pmgs={pmgs} stateBuildingPms={stateBuilding.activate_production_methods} onPmChange={handlePmChange}/>
        </div>
      </td>
      <td className="max-w-32">{stateBuilding.name}</td>
      <td className="max-w-24">
        <button className="btn btn-xs btn-accent min-h-4 h-4 tooltip tooltip-bottom max-w-full" data-tip={productionMethods || ''}>
          <div className="overflow-hidden truncate">{productionMethods}</div>
        </button>
      </td>
      <td className="flex justify-center items-center max-w-16">
        <button className="btn btn-square btn-xs btn-error w-4 min-h-4 h-4" onClick={() => adjustStateBuildingLevel(stateBuilding, -1)}>
          <MinusIcon className="w-3 h-3"/>
        </button>
        <p className="px-2 text-center">{stateBuilding.level}</p>
        <button className="btn btn-square btn-xs btn-success w-4 min-h-4 h-4" onClick={() => adjustStateBuildingLevel(stateBuilding, 1)}>
          <PlusIcon className="w-3 h-3"/>
        </button>
      </td>
    </tr>
  )
}
