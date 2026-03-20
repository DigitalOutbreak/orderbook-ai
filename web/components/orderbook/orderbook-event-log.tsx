"use client"

import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"

import type { OrderbookEvent } from "./orderbook-types"

type OrderbookEventLogProps = {
  events: OrderbookEvent[]
  onOpenGlossary: () => void
}

export function OrderbookEventLog({
  events,
  onOpenGlossary,
}: OrderbookEventLogProps) {
  return (
    <div className="flex h-full min-h-0 flex-col border border-border/60 bg-card/80 shadow-2xl shadow-black/15">
      <div className="flex items-center justify-between gap-2 border-b border-border/60 px-3 py-2.5">
        <div className="flex min-w-0 items-center gap-2">
          <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
            Learn
          </span>
          <Badge variant="secondary" className="px-1.5 font-mono text-[10px]">
            Event Log
          </Badge>
          <Badge variant="outline" className="font-mono text-[10px]">
            {events.length}
          </Badge>
        </div>
        <div className="flex flex-wrap items-center justify-end gap-2">
          <Button
            type="button"
            variant="outline"
            size="xs"
            onClick={onOpenGlossary}
          >
            Glossary
          </Button>
          <Button asChild type="button" variant="outline" size="xs">
            <Link href="/docs">Docs</Link>
          </Button>
          <Button asChild type="button" variant="outline" size="xs">
            <Link href="/learn/lessons">Lessons</Link>
          </Button>
          <Button type="button" variant="outline" size="xs" disabled>
            Tutorial
          </Button>
        </div>
      </div>
      <div className="min-h-0 flex-1 p-3">
        <ScrollArea className="h-full">
          <div className="space-y-2">
            {events.length === 0 ? (
              <div className="border border-dashed border-border/60 bg-background/40 px-2.5 py-2 text-[10px] text-muted-foreground">
                Submit an order to record how the book changed.
              </div>
            ) : (
              events.map((event, index) => (
                <div
                  key={event.id}
                  className="border border-border/60 bg-background/40 px-2.5 py-2"
                >
                  <div className="flex items-center justify-between gap-2">
                    <span
                      className={cn(
                        "text-[10px] font-semibold tracking-[0.14em] uppercase",
                        event.tone === "buy" && "text-emerald-300",
                        event.tone === "sell" && "text-rose-300",
                        event.tone === "neutral" && "text-foreground/80"
                      )}
                    >
                      {event.title}
                    </span>
                    <div className="flex flex-col items-end text-right">
                      <span className="font-mono text-[9px] text-muted-foreground">
                        {event.time}
                      </span>
                      <span className="font-mono text-[8px] text-muted-foreground/70">
                        #{events.length - index}
                      </span>
                    </div>
                  </div>
                  <p className="mt-1 text-[10px] leading-4.5 text-muted-foreground">
                    {event.detail}
                  </p>
                </div>
              ))
            )}
          </div>
        </ScrollArea>
      </div>
    </div>
  )
}
