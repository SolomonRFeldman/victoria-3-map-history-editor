import { MinusIcon, PlusIcon } from "@heroicons/react/24/solid"
import { StateBuilding } from "../States"
import { invoke } from "@tauri-apps/api"
import { ArrowLeftIcon } from "@heroicons/react/24/outline"
import ChooseProductionMethods, { ProductionMethodGroup } from "./ChooseProductionMethods"
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
      <td className="pl-3 dropdown dropdown-left">
        <button tabIndex={0} className="btn btn-square btn-xs btn-accent w-4 min-h-4 h-4" onClick={() => getBuilding(stateBuilding.name)}><ArrowLeftIcon className="w-3 h-3"/></button>
        <div tabIndex={0} className="dropdown-content" ref={renderBuildingsRef} >
          <ChooseProductionMethods pmgs={pmgs} stateBuildingPms={stateBuilding.activate_production_methods} onPmChange={handlePmChange}/>
        </div>
      </td>
      <td>{stateBuilding.name}</td>
      <td className="max-w-36">
        <button className="btn btn-xs btn-accent min-h-4 h-4 tooltip tooltip-bottom max-w-full" data-tip={productionMethods || ''}>
          <div className="overflow-hidden truncate">{productionMethods}</div>
        </button>
      </td>
      <td className="flex justify-center items-center">
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
