import { useEffect, useRef, useState } from "react";

type SearchBoxProps = {
  options: { value: string, label: string }[]
  onSelect: (value: string) => void
  placeholder?: string
}

export default function SearchBox({ options, onSelect, placeholder }: SearchBoxProps) {
  const inputRef = useRef<HTMLInputElement>(null);
  const optionRef = useRef<(HTMLButtonElement | null)[]>([])
  const [focusedIndex, setFocusedIndex] = useState(0)
  const [search, setSearch] = useState('')
  const filteredOptions = options.filter(({ label }) => label.toLowerCase().includes(search.toLowerCase()))

  useEffect(() => {
    optionRef.current = optionRef.current.slice(0, filteredOptions.length);
    setFocusedIndex(0)
  }, [options, search]);

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
          optionRef.current[index]?.focus()
          return index
        }
      })
    } else if (event.key === 'ArrowDown' || event.key === 'Tab') {
      setFocusedIndex((focusedIndex) => {
        if (focusedIndex === filteredOptions.length - 1) {
          if(event.key !== 'Tab') {
            event.preventDefault()
            inputRef.current?.focus()
            return 0
          }
          return 0
        } else {
          event.preventDefault()
          const index = focusedIndex + 1
          optionRef.current[index]?.focus()
          return index
        }
      })
    } else if (event.key === 'Escape') {
      event.stopPropagation()
      inputRef.current?.focus()
      inputRef.current?.blur()
    }
  }

  const handleInputKeyDown = (event: React.KeyboardEvent) => {
    if (event.key === 'ArrowDown') {
      event.preventDefault()
      optionRef.current[focusedIndex]?.focus()
    }
    if (event.key === 'ArrowUp') {
      event.preventDefault()
      if (focusedIndex === 0) {
        setFocusedIndex(filteredOptions.length - 1)
        optionRef.current[filteredOptions.length - 1]?.focus()
      } else {
        optionRef.current[focusedIndex]?.focus()
      }
    }
    if (event.key === 'Escape') {
      event.stopPropagation()
      inputRef.current?.blur()
    }
  }

  return(
    <div className="dropdown">
      <input ref={inputRef} value={search} onChange={event => setSearch(event.target.value)} className="input input-xs -ml-2" tabIndex={0} role="button" placeholder={placeholder} onKeyDown={handleInputKeyDown} />
      <ul className="menu-xs dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-60 max-h-60 overflow-y-scroll block" onKeyDown={handleOptionsKeyDown}>
        {filteredOptions.map(({ value, label }, index) => <li key={value}><button className="button" onClick={() => onSelect(value)} tabIndex={0} ref={el => optionRef.current[index] = el}>{label}</button></li>)}
      </ul>
    </div>
  )
}
