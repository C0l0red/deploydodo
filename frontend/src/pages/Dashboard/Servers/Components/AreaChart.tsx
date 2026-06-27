export function AreaChart({ color = '#ff713e' }: { color?: string }) {
  // Approximate data to match the screenshot shape
  // The curve starts around 36, goes down to ~18 at mid, down to ~4, up to ~32.
  // We'll use 30 points to make a smooth curve.
  const data = [
    36, 33, 30, 28, 25, 23, 21, 19, 18, 18, 
    17, 16, 14, 10,  6,  4,  4,  5,  7, 10, 
    13, 17, 21, 25, 28, 32
  ]
  
  const w = 800
  const h = 260
  const pl = 50 // padding left
  const pb = 40 // padding bottom
  const pt = 10 // padding top
  const pr = 10 // padding right
  
  const chartW = w - pl - pr
  const chartH = h - pt - pb
  
  const maxY = 60
  
  const points = data.map((v, i) => {
    const x = pl + (i / (data.length - 1)) * chartW
    const y = pt + chartH - (v / maxY) * chartH
    return [x, y]
  })
  
  const linePath = points.map(([x, y], i) => `${i === 0 ? 'M' : 'L'}${x},${y}`).join(' ')
  
  const areaPath = `${linePath} L${pl + chartW},${pt + chartH} L${pl},${pt + chartH} Z`
  
  const yTicks = [0, 15, 30, 45]
  const xLabels = ['16.11', '17.11', '18.11', '19.11', '20.11', '21.11', '22.11', '23.11', '24.11', '25.11']
  return (
    <svg viewBox={`0 0 ${w} ${h}`} className="w-full">
      {/* Grid lines and Y-labels */}
      {yTicks.map((tick) => {
        // mapping tick to Y
        const y = pt + chartH - (tick / maxY) * chartH
        return (
          <g key={`y-${tick}`}>
            <text x={pl - 15} y={y + 3} fill="#8a8a8a" fontSize="8" fontFamily="sans-serif" textAnchor="end">
              {tick}%
            </text>
            <line x1={pl} y1={y} x2={w - pr} y2={y} stroke="#f0f0f0" strokeWidth="1" />
          </g>
        )
      })}
      
      {/* Top grid line (60%) */}
      <line x1={pl} y1={pt} x2={w - pr} y2={pt} stroke="#f0f0f0" strokeWidth="1" />
      
      {/* X-labels and vertical grid lines */}
      {xLabels.map((label, i) => {
        const x = pl + (i / (xLabels.length - 1)) * chartW
        return (
          <g key={`x-${label}`}>
            <text x={x} y={h - 10} fill="#8a8a8a" fontSize="8" fontFamily="sans-serif" textAnchor="middle">
              {label}
            </text>
            <line x1={x} y1={pt} x2={x} y2={pt + chartH} stroke="#f0f0f0" strokeWidth="1" />
          </g>
        )
      })}
      
      {/* Area and Line */}
      {/* Wait, the screenshot Area chart is fully filled, not a gradient. It's solid orange. */}
      <path d={areaPath} fill={color} />
      <path d={linePath} fill="none" stroke={color} strokeWidth="2" />
    </svg>
  )
}
