"use client"

import * as React from "react"

import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import type { DocsHeading } from "@/lib/docs"

type DocsTocProps = {
  headings: DocsHeading[]
}

export function DocsToc({ headings }: DocsTocProps) {
  const handleHeadingClick = React.useCallback(
    (event: React.MouseEvent<HTMLAnchorElement>, id: string) => {
      event.preventDefault()

      const target = document.getElementById(id)
      if (!target) return

      const reduceMotion = window.matchMedia(
        "(prefers-reduced-motion: reduce)"
      ).matches

      history.replaceState(null, "", `#${id}`)
      target.scrollIntoView({
        behavior: reduceMotion ? "auto" : "smooth",
        block: "start",
      })

      target.classList.remove("docs-heading-flash")
      window.requestAnimationFrame(() => {
        target.classList.add("docs-heading-flash")
      })

      window.setTimeout(() => {
        target.classList.remove("docs-heading-flash")
      }, 700)
    },
    []
  )

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
              <a
                key={heading.id}
                href={`#${heading.id}`}
                onClick={(event) => handleHeadingClick(event, heading.id)}
                className="block px-2 py-1.5 text-sm text-muted-foreground transition-colors hover:text-foreground"
              >
                {heading.text}
              </a>
            ))}
        </div>
      </ScrollArea>
    </div>
  )
}
