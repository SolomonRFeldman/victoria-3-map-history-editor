import { PlusIcon } from "@heroicons/react/24/solid";
import { FocusEvent, useEffect, useRef, useState } from "react";
import { Building } from "./StateBuildingInfo";
import { invoke } from "@tauri-apps/api/core";
import { StateBuilding } from "../States";

const buildingsFilter = (buildings: Building[], stateBuildings: StateBuilding[], search: string) => buildings.filter(building => {
  return !building.unique &&
    building.buildable &&
    !stateBuildings.some(stateBuilding => stateBuilding.name === building.name) &&
    building.name.toLowerCase().includes(search.toLowerCase())
})

type AddStateBuildingProps = {
  stateBuildings: StateBuilding[]
  onAddStateBuilding: (building: StateBuilding) => void
}

export default function AddStateBuilding({ stateBuildings, onAddStateBuilding }: AddStateBuildingProps) {
  const [showBuildings, setShowBuildings] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null);
  const buildingRefs = useRef<(HTMLButtonElement | null)[]>([])
  const [focusedIndex, setFocusedIndex] = useState(0)
  const [search, setSearch] = useState('')
  const [buildings, setBuildings] = useState<Building[]>([])

  const handleGetBuildings = async () => { setBuildings((await invoke<Building[]>("get_buildings", {}))) }
  const filteredBuildings = buildingsFilter(buildings, stateBuildings, search)
  useEffect(() => { handleGetBuildings() }, [])

  useEffect(() => {
    buildingRefs.current = buildingRefs.current.slice(0, filteredBuildings.length);
    setFocusedIndex(0)
  }, [buildings, search]);

  const handleOptionsKeyDown = (event: React.KeyboardEvent) => {
    const shiftTab = event.shiftKey && event.key === 'Tab'
    if (event.key === 'ArrowUp' || shiftTab) {
      event.preventDefault()
      setFocusedIndex((focusedIndex) => {
        if (focusedIndex < 1) {
          inputRef.current?.focus()
          return 0
        } else {
          const index = focusedIndex - 1
          buildingRefs.current[index]?.focus()
          return index
        }
      })
    } else if (event.key === 'ArrowDown' || event.key === 'Tab') {
      setFocusedIndex((focusedIndex) => {
        if (focusedIndex === filteredBuildings.length - 1) {
          if(event.key !== 'Tab') {
            event.preventDefault()
            inputRef.current?.focus()
            return 0
          }
          return 0
        } else {
          event.preventDefault()
          const index = focusedIndex + 1
          buildingRefs.current[index]?.focus()
          return index
        }
      })
    } else if (event.key === 'a') { 
      event.preventDefault()
      inputRef.current?.focus()
    }
  }

  const handleClick = () => {
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }

  const handleFormBlur = (event: FocusEvent) => {
    if (!event.currentTarget?.contains(event.relatedTarget)) {
      setShowBuildings(false)
    }
  }
  const handleFormFocus = (event: FocusEvent) => {
    if (event.currentTarget === inputRef.current || event.currentTarget === event.relatedTarget) {
      setFocusedIndex(0)
    }
    setShowBuildings(true)
  }

  const handleInputKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'ArrowDown') {
      event.preventDefault()
      buildingRefs.current[focusedIndex]?.focus()
    }
    if (event.key === 'ArrowUp') {
      event.preventDefault()
      if (focusedIndex === 0) {
        setFocusedIndex(filteredBuildings.length - 1)
        buildingRefs.current[filteredBuildings.length - 1]?.focus()
      } else {
        buildingRefs.current[focusedIndex]?.focus()
      }
    }
  }

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => { 
      if (event.key === 'a' && !showBuildings) {
        event.preventDefault()
        inputRef.current?.focus() 
      }
    }
    window.addEventListener('keydown', handleKeyDown)

    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [showBuildings])

  const handleClickBuilding = (building: Building) => {
    const activate_production_methods = building.production_method_groups.map(pmGroup => pmGroup.production_methods[0].name)

    onAddStateBuilding({ name: building.name, level: 1, activate_production_methods, reserves: 1, condition: null })
  }


  return(
    <tr>
      <td>
        <button className="btn btn-square btn-xs btn-success w-4 min-h-4 h-4 ml-1 tooltip tooltip-bottom" data-tip='a' onClick={handleClick}>
          <PlusIcon className="w-3 h-3"/>
        </button>
      </td>
      <td className="max-w-16" onBlur={handleFormBlur} onFocus={handleFormFocus}>
        <div className="dropdown">
          <input ref={inputRef} value={search} onChange={event => setSearch(event.target.value)} className="input input-xs -ml-2" tabIndex={0} role="button" placeholder={(showBuildings || '') && "select building"} onKeyDown={handleInputKeyDown} />
          <ul className="menu-xs dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-60 max-h-60 overflow-y-scroll block" onKeyDown={handleOptionsKeyDown}>
            {filteredBuildings.map((building, index) => <li key={building.name}><button className="button" onClick={() => handleClickBuilding(building)} tabIndex={0} ref={el => buildingRefs.current[index] = el}>{building.name}</button></li>)}
          </ul>
        </div>
      </td>
      <td />
      <td />
    </tr>
  )
}
