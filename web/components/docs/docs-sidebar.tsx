"use client"

import Link from "next/link"
import { ArrowsOutLineHorizontal, CaretDown } from "@phosphor-icons/react"
import { useEffect, useMemo, useState } from "react"

import { Badge } from "@/components/ui/badge"
import { Button, buttonVariants } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import { cn } from "@/lib/utils"
import type { DocsPage } from "@/lib/docs-config"

type DocsSidebarProps = {
  pages: DocsPage[]
  activeSlug: string
}

const STORAGE_KEY = "docs-sidebar-open-sections"

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
  const activeSections = useMemo(
    () =>
      grouped
        .filter(([, sectionPages]) =>
          sectionPages.some((page) => page.slug === activeSlug)
        )
        .map(([section]) => section),
    [activeSlug, grouped]
  )
  const [openSections, setOpenSections] = useState<string[]>(activeSections)
  const [animateSections, setAnimateSections] = useState(false)
  const allSectionsOpen = openSections.length === grouped.length

  useEffect(() => {
    if (typeof window === "undefined" || activeSections.length === 0) return

    const currentActiveSection = activeSections[0]

    let restoredSections = activeSections
    const stored = window.sessionStorage.getItem(STORAGE_KEY)
    if (stored) {
      try {
        restoredSections = JSON.parse(stored) as string[]
      } catch {
        restoredSections = activeSections
      }
    }

    const next = new Set(restoredSections)
    next.add(currentActiveSection)
    setAnimateSections(false)
    setOpenSections(Array.from(next))
  }, [activeSections])

  useEffect(() => {
    if (typeof window === "undefined") return
    window.sessionStorage.setItem(STORAGE_KEY, JSON.stringify(openSections))
  }, [openSections])

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
        <div className="space-y-2 px-3 py-3">
          {grouped.map(([section, sectionPages]) => {
            const sectionOpen = openSections.includes(section)

            return (
              <div
                key={section}
                className="border border-border/60 bg-background/35"
              >
                <button
                  type="button"
                  className="flex w-full cursor-pointer items-center justify-between gap-2 px-3 py-2.5 text-left"
                  onClick={() => {
                    setAnimateSections(true)
                    setOpenSections((current) =>
                      sectionOpen
                        ? current.filter((value) => value !== section)
                        : [...new Set([...current, section])]
                    )
                  }}
                  aria-expanded={sectionOpen}
                >
                  <span className="font-mono text-[10px] tracking-[0.2em] text-muted-foreground uppercase">
                    {section}
                  </span>
                  <CaretDown
                    className={cn(
                      "size-3 text-muted-foreground transition-transform",
                      sectionOpen && "rotate-180"
                    )}
                  />
                </button>
                <div
                  className={cn(
                    "grid",
                    animateSections &&
                      "transition-[grid-template-rows] duration-200 ease-out",
                    sectionOpen ? "grid-rows-[1fr]" : "grid-rows-[0fr]"
                  )}
                >
                  <div className="overflow-hidden">
                    <div className="space-y-1 border-t border-border/60 px-2 py-2">
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
                </div>
              </div>
            )
          })}
        </div>
      </ScrollArea>

      <div className="border-t border-border/60 px-3 py-3">
        <div className="flex items-center justify-between gap-2">
          <div className="flex items-center gap-2">
            <Button asChild variant="outline" size="xs">
              <Link href="/learn">Learn</Link>
            </Button>
            <a
              href="/"
              className={cn(buttonVariants({ variant: "outline", size: "xs" }))}
            >
              Terminal
            </a>
          </div>
          <Button
            type="button"
            variant="outline"
            size="icon-xs"
            onClick={() => {
              setAnimateSections(true)
              setOpenSections(
                allSectionsOpen ? [] : grouped.map(([section]) => section)
              )
            }}
            aria-label={
              allSectionsOpen ? "Collapse all sections" : "Open all sections"
            }
            title={
              allSectionsOpen ? "Collapse all sections" : "Open all sections"
            }
          >
            <ArrowsOutLineHorizontal
              className={cn(
                "transition-transform",
                allSectionsOpen && "rotate-180"
              )}
            />
          </Button>
        </div>
      </div>
    </div>
  )
}
