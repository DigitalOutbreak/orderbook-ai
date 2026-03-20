"use client"

import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import type { DocsHeading } from "@/lib/docs"

type DocsTocProps = {
  headings: DocsHeading[]
}

export function DocsToc({ headings }: DocsTocProps) {
  if (headings.length === 0) return null

  return (
    <div className="sticky top-4 flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
      <div className="border-b border-border/60 px-3 py-3">
        <div className="flex items-center gap-2">
          <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
            On This Page
          </span>
          <Badge variant="outline" className="px-1.5 font-mono text-[10px]">
            {headings.length}
          </Badge>
        </div>
      </div>
      <ScrollArea className="min-h-0 flex-1">
        <div className="space-y-1 px-3 py-3">
          {headings
            .filter((heading) => heading.level <= 3)
            .map((heading) => (
              <Link
                key={heading.id}
                href={`#${heading.id}`}
                className="block px-2 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
              >
                {heading.text}
              </Link>
            ))}
        </div>
      </ScrollArea>
      <div className="border-t border-border/60 px-3 py-3">
        <div className="flex items-center gap-2">
          <Button asChild variant="outline" size="xs">
            <Link href="/learn">Learn</Link>
          </Button>
          <Button asChild variant="outline" size="xs">
            <Link href="/">Terminal</Link>
          </Button>
        </div>
      </div>
    </div>
  )
}
