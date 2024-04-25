import { PlusIcon } from "@heroicons/react/24/solid";
import { FocusEvent, useEffect, useRef, useState } from "react";

export default function AddStateBuilding({}) {
  const items = ['Item 1', 'Item 2', 'Jeff']
  const [showBuildings, setShowBuildings] = useState(false)
  const inputRef = useRef<HTMLInputElement>(null);
  const itemRefs = useRef<(HTMLButtonElement | null)[]>([])
  const [, setFocusedIndex] = useState(0)
  const [search, setSearch] = useState('')
  const filteredItems = items.filter(item => item.toLowerCase().includes(search.toLowerCase()))

  useEffect(() => {
    itemRefs.current = itemRefs.current.slice(0, filteredItems.length);
  }, [items, search]);

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
          itemRefs.current[index]?.focus()
          return index
        }
      })
    } else if (event.key === 'ArrowDown' || event.key === 'Tab') {
      setFocusedIndex((focusedIndex) => {
        if (focusedIndex === filteredItems.length - 1) {
          if(event.key !== 'Tab') {
            event.preventDefault()
            inputRef.current?.focus()
            return 0
          }
          return 0
        } else {
          event.preventDefault()
          const index = focusedIndex + 1
          itemRefs.current[index]?.focus()
          return index
        }
      })
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
      setFocusedIndex(0)
      itemRefs.current[0]?.focus()
    }
    if (event.key === 'ArrowUp') {
      event.preventDefault()
      setFocusedIndex(filteredItems.length - 1)
      itemRefs.current[filteredItems.length - 1]?.focus()
    }
  }

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => { 
      if (event.key === 'n' && !showBuildings) {
        event.preventDefault()
        inputRef.current?.focus() 
      }
    }
    window.addEventListener('keydown', handleKeyDown)

    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [showBuildings])

  return(
    <tr className="">
      <td>
        <button className="btn btn-square btn-xs btn-success w-4 min-h-4 h-4 ml-1" onClick={handleClick}>
          <PlusIcon className="w-3 h-3"/>
        </button>
      </td>
      <td className="max-w-16" onBlur={handleFormBlur} onFocus={handleFormFocus}>
        <div className="dropdown">
          <input ref={inputRef} value={search} onChange={event => setSearch(event.target.value)} className="input input-xs -ml-2" tabIndex={0} role="button" placeholder={(showBuildings || '') && "select building"} onKeyDown={handleInputKeyDown} />
          <ul className="menu-xs dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52" onKeyDown={handleOptionsKeyDown}>
            {filteredItems.map((item, index) => <li key={item}><button className="button" onClick={() => console.log(item)} tabIndex={0} ref={el => itemRefs.current[index] = el}>{item}</button></li>)}
          </ul>
        </div>
      </td>
      <td />
      <td />
    </tr>
  )
}
