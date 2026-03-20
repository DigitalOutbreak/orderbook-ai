"use client"

import Link from "next/link"
import { useMemo } from "react"

import { Badge } from "@/components/ui/badge"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"
import type { DocsPage } from "@/lib/docs-config"

type DocsSidebarProps = {
  pages: DocsPage[]
  activeSlug: string
}

export function DocsSidebar({ pages, activeSlug }: DocsSidebarProps) {
  const grouped = useMemo(() => {
    const sections = new Map<string, DocsPage[]>()
    for (const page of pages) {
      const current = sections.get(page.section) ?? []
      current.push(page)
      sections.set(page.section, current)
    }
    return Array.from(sections.entries())
  }, [pages])

  return (
    <div className="sticky top-4 flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
      <div className="border-b border-border/60 px-3 py-3">
        <div className="flex items-center gap-2">
          <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
            Docs
          </span>
          <Badge variant="secondary" className="px-1.5 font-mono text-[10px]">
            Orderbook
          </Badge>
        </div>
      </div>

      <ScrollArea className="min-h-0 flex-1">
        <div className="space-y-5 px-3 py-3">
          {grouped.map(([section, sectionPages]) => (
            <div key={section} className="space-y-2">
              <div className="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
                {section}
              </div>
              <div className="space-y-1">
                {sectionPages.map((page) => {
                  const active = page.slug === activeSlug
                  return (
                    <Link
                      key={page.slug}
                      href={`/docs/${page.slug}`}
                      className={cn(
                        "block border px-2.5 py-2 text-sm transition-colors",
                        active
                          ? "border-border bg-muted text-foreground"
                          : "border-transparent text-muted-foreground hover:border-border/60 hover:bg-background/50 hover:text-foreground"
                      )}
                    >
                      {page.title}
                    </Link>
                  )
                })}
              </div>
            </div>
          ))}
        </div>
      </ScrollArea>
    </div>
  )
}
