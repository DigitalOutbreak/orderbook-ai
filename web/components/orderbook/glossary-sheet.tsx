"use client"

import * as React from "react"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@/components/ui/sheet"
import { ScrollArea } from "@/components/ui/scroll-area"
import { GLOSSARY_ENTRIES } from "@/lib/glossary"

type GlossarySheetProps = {
  open: boolean
  onOpenChange: (open: boolean) => void
  animated?: boolean
}

export function GlossarySheet({
  open,
  onOpenChange,
  animated = true,
}: GlossarySheetProps) {
  const [query, setQuery] = React.useState("")

  const filteredEntries = React.useMemo(() => {
    const normalized = query.trim().toLowerCase()
    if (!normalized) return GLOSSARY_ENTRIES

    return GLOSSARY_ENTRIES.filter((entry) => {
      const haystack =
        `${entry.term} ${entry.category} ${entry.body.join(" ")}`.toLowerCase()
      return haystack.includes(normalized)
    })
  }, [query])

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent
        side="right"
        animated={animated}
        className="w-full p-0 sm:max-w-[680px]"
      >
        <SheetHeader className="gap-2">
          <div className="flex items-center justify-between gap-2">
            <div className="flex items-center gap-2">
              <SheetTitle>Glossary</SheetTitle>
              <Badge
                variant="secondary"
                className="px-1.5 font-mono text-[10px]"
              >
                Quant Learn
              </Badge>
            </div>
            <SheetClose asChild>
              <Button type="button" variant="ghost" size="xs">
                Close
              </Button>
            </SheetClose>
          </div>
          <SheetDescription>
            Search exchange terms while you use the terminal. This stays focused
            on orderbook and market microstructure concepts.
          </SheetDescription>
          <Input
            value={query}
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search spread, maker, top of book, slippage..."
            aria-label="Search glossary"
          />
        </SheetHeader>

        <div className="flex items-center justify-between border-b border-border/60 px-4 py-2 text-[11px] text-muted-foreground">
          <span>{filteredEntries.length} terms</span>
          <span className="font-mono tracking-[0.16em] uppercase">
            Docs View
          </span>
        </div>

        <ScrollArea className="h-[calc(100svh-142px)]">
          <div className="space-y-4 px-4 pt-4 pb-12">
            {filteredEntries.map((entry) => (
              <section
                key={entry.term}
                className="border border-border/60 bg-background/40 p-3"
              >
                <div className="flex items-center gap-2">
                  <h3 className="font-medium">{entry.term}</h3>
                  <Badge
                    variant="outline"
                    className="px-1.5 font-mono text-[10px]"
                  >
                    {entry.category}
                  </Badge>
                </div>
                <div className="mt-2 space-y-2 text-sm/6 text-muted-foreground">
                  {entry.body.map((paragraph) => (
                    <p key={paragraph}>{paragraph}</p>
                  ))}
                </div>
              </section>
            ))}
            {filteredEntries.length === 0 ? (
              <div className="border border-dashed border-border/60 bg-background/40 p-4 text-sm text-muted-foreground">
                No glossary terms matched that search.
              </div>
            ) : null}
          </div>
        </ScrollArea>
      </SheetContent>
    </Sheet>
  )
}
