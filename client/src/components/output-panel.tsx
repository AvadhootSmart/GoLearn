"use client";

import { cn } from "@/lib/utils";
import { ScrollArea } from "@/components/ui/scroll-area";

interface OutputPanelProps {
  stdout: string;
  stderr: string;
  passed?: boolean;
  expectedOutput?: string;
  isLoading?: boolean;
}

export function OutputPanel({
  stdout,
  stderr,
  passed,
  expectedOutput,
  isLoading,
}: OutputPanelProps) {
  return (
    <div className="h-full flex flex-col bg-[#0A0A0A] font-mono text-[13px] leading-relaxed">
      {/* Header */}
      <div className="flex-none flex items-center justify-between px-6 py-2.5 border-b border-white/[0.05] bg-[#0C0C0C]">
        <div className="flex items-center gap-3">
          <span className="text-zinc-500 font-semibold tracking-wider uppercase text-[10px]">Output</span>
          {passed !== undefined && !isLoading && (
            <span
              className={cn(
                "px-1.5 py-0.5 rounded text-[10px] font-bold tracking-widest uppercase border",
                passed
                  ? "border-primary text-primary bg-primary/5"
                  : "border-destructive text-destructive bg-destructive/5"
              )}
            >
              {passed ? "Passed" : "Failed"}
            </span>
          )}
          {isLoading && (
            <span className="px-1.5 py-0.5 rounded text-[10px] font-bold tracking-widest uppercase border border-blue-500/30 text-blue-500 bg-blue-500/5 animate-pulse">
              Running
            </span>
          )}
        </div>
      </div>

      {/* Output Content */}
      <ScrollArea className="flex-1 min-h-0 p-4">
        {isLoading ? (
          <div className="text-zinc-500 animate-pulse">Executing code...</div>
        ) : (
          <>
            {stdout && (
              <div className="mb-6">
                <div className="text-zinc-500 text-[10px] font-bold tracking-widest uppercase mb-2">stdout</div>
                <pre className="text-zinc-300 whitespace-pre-wrap">{stdout}</pre>
              </div>
            )}
            
            {stderr && (
              <div className="mb-6">
                <div className="text-destructive text-[10px] font-bold tracking-widest uppercase mb-2">stderr</div>
                <pre className="text-destructive/80 whitespace-pre-wrap">{stderr}</pre>
              </div>
            )}

            {expectedOutput && passed === false && (
              <div className="mt-6 pt-6 border-t border-border">
                <div className="text-zinc-500 text-[10px] font-bold tracking-widest uppercase mb-2">Expected output</div>
                <pre className="text-zinc-300 whitespace-pre-wrap">{expectedOutput}</pre>
              </div>
            )}

            {!stdout && !stderr && !isLoading && (
              <div className="text-zinc-500 italic">
                Click &quot;Run&quot; to execute your code.
              </div>
            )}
          </>
        )}
      </ScrollArea>
    </div>
  );
}
