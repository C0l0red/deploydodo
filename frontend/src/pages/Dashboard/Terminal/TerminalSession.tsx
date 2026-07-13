import { useTerminalSocket } from '@/hooks/useTerminalSocket';
import { useRef, useState } from 'react';
import { type Status, statusColor, statusLabel } from './utils';

export function TerminalSession({ serverId }: { serverId: number; }) {
    const containerRef = useRef<HTMLDivElement>(null);
    const [status, setStatus] = useState<Status>('connecting');

    useTerminalSocket(containerRef, serverId, setStatus);

    return (
        <div className="flex flex-col rounded-lg overflow-hidden border border-neutral-100">
            <div className="flex items-center gap-2 px-4 py-2 bg-secondary">
                <span className={`size-2 rounded-full ${statusColor[status]}`} />
                <span className="font-manrope text-sm text-pure-white">{statusLabel[status]}</span>
            </div>
            <div ref={containerRef} className="h-130 w-full bg-high-contrast p-2" />
        </div>
    );
}
