export function BarChart({ color = '#ff713e' }: { color?: string }) {
  const bars = [
    1, 1, 3,
    38, 35, 27, 22, 25, 30, 32, 38, 32, 27, 15,
    38, 35, 33, 28, 38, 32, 38,
    1, 1, 2,
    38, 1, 1, 2,
    38, 38, 38, 33, 20, 13, 2, 1, 1, 2, 8, 2
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
  
  const barW = (chartW / bars.length) * 0.8
  const gap = (chartW / bars.length) * 0.2
  
  const yTicks = [0, 15, 30, 45]
  const xLabels = ['16.11', '17.11', '18.11', '19.11', '20.11', '21.11', '22.11', '23.11', '24.11', '25.11']

  return (
    <svg viewBox={`0 0 ${w} ${h}`} className="w-full">
      {/* Grid lines and Y-labels */}
      {yTicks.map((tick) => {
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
      
      {/* Bars */}
      {bars.map((v, i) => {
        const x = pl + i * (chartW / bars.length) + gap / 2
        const barH = (v / maxY) * chartH
        const y = pt + chartH - barH
        return (
          <rect key={i} x={x} y={y} width={barW} height={barH} fill={color} />
        )
      })}
    </svg>
  )
}
