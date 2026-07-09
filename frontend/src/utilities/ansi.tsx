const ANSI_RE = /\x1b\[([\d;]*)m/g

type StyleState = {
  bold: boolean
  dim: boolean
  italic: boolean
  underline: boolean
  fg: string | null
  bg: string | null
}

const EMPTY_STATE: StyleState = {
  bold: false,
  dim: false,
  italic: false,
  underline: false,
  fg: null,
  bg: null,
}

const ANSI_COLORS: Record<string, string> = {
  '30': '#1c1c1c',
  '31': '#d75f5f',
  '32': '#5faf5f',
  '33': '#d7af5f',
  '34': '#5f87d7',
  '35': '#af5fd7',
  '36': '#5fafaf',
  '37': '#c6c6c6',
  '90': '#707070',
  '91': '#d78787',
  '92': '#87d787',
  '93': '#d7d787',
  '94': '#87afd7',
  '95': '#d787d7',
  '96': '#87d7d7',
  '97': '#ffffff',
}

const ANSI_BG: Record<string, string> = {
  '40': '#1c1c1c',
  '41': '#d75f5f',
  '42': '#5faf5f',
  '43': '#d7af5f',
  '44': '#5f87d7',
  '45': '#af5fd7',
  '46': '#5fafaf',
  '47': '#c6c6c6',
  '100': '#707070',
  '101': '#d78787',
  '102': '#87d787',
  '103': '#d7d787',
  '104': '#87afd7',
  '105': '#d787d7',
  '106': '#87d7d7',
  '107': '#ffffff',
}

type Segment = { text: string; style: StyleState }

function parseAnsi(text: string): Segment[] {
  const segments: Segment[] = []
  let state: StyleState = { ...EMPTY_STATE }
  let lastIndex = 0

  ANSI_RE.lastIndex = 0
  let match: RegExpExecArray | null
  while ((match = ANSI_RE.exec(text)) !== null) {
    if (match.index > lastIndex) {
      segments.push({
        text: text.slice(lastIndex, match.index),
        style: { ...state },
      })
    }

    const codes = match[1] || '0'
    if (codes === '0' || codes === '') {
      state = { ...EMPTY_STATE }
    } else {
      for (const code of codes.split(';')) {
        if (code === '1') state.bold = true
        else if (code === '2') state.dim = true
        else if (code === '3') state.italic = true
        else if (code === '4') state.underline = true
        else if (code === '22') state.bold = false
        else if (code === '23') state.italic = false
        else if (code === '24') state.underline = false
        else if (ANSI_COLORS[code]) state.fg = ANSI_COLORS[code]
        else if (ANSI_BG[code]) state.bg = ANSI_BG[code]
        else if (code === '39') state.fg = null
        else if (code === '49') state.bg = null
      }
    }

    lastIndex = ANSI_RE.lastIndex
  }

  if (lastIndex < text.length) {
    segments.push({ text: text.slice(lastIndex), style: { ...state } })
  }

  return segments
}

export function colorizeAnsi(text: string): React.ReactElement[] {
  const segments = parseAnsi(text)
  return segments.map((seg, i) => {
    const style: React.CSSProperties = {}
    if (seg.style.fg) style.color = seg.style.fg
    if (seg.style.bg) style.backgroundColor = seg.style.bg
    if (seg.style.bold) style.fontWeight = 'bold'
    if (seg.style.italic) style.fontStyle = 'italic'
    if (seg.style.underline) style.textDecoration = 'underline'
    return (
      <span key={i} style={style}>
        {seg.text}
      </span>
    )
  })
}
