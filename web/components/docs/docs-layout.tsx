import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { DocsSidebar } from "@/components/docs/docs-sidebar"
import { DocsToc } from "@/components/docs/docs-toc"
import { MarkdownRenderer } from "@/components/docs/markdown-renderer"
import { ScrollArea } from "@/components/ui/scroll-area"
import type { DocsContent } from "@/lib/docs"
import type { DocsPage } from "@/lib/docs-config"

type DocsLayoutProps = {
  page: DocsContent
  pages: DocsPage[]
  previous: DocsPage | null
  next: DocsPage | null
}

export function DocsLayout({ page, pages, previous, next }: DocsLayoutProps) {
  return (
    <main className="h-svh overflow-hidden bg-[radial-gradient(circle_at_top_left,rgba(94,234,212,0.07),transparent_22%),linear-gradient(180deg,rgba(10,12,20,1),rgba(14,18,30,1))] px-4 py-5">
      <div className="mx-auto grid h-full min-h-0 max-w-[1560px] gap-4 xl:grid-cols-[280px_minmax(0,1fr)_240px]">
        <DocsSidebar pages={pages} activeSlug={page.slug} />

        <div className="min-h-0 min-w-0">
          <div className="flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
            <div className="border-b border-border/60 px-6 py-5">
              <div className="flex flex-wrap items-center gap-2">
                <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                  Docs
                </span>
                <Badge
                  variant="secondary"
                  className="px-1.5 font-mono text-[10px]"
                >
                  {page.section}
                </Badge>
              </div>
              <h1 className="mt-3 text-3xl font-semibold tracking-tight">
                {page.title}
              </h1>
              <p className="mt-3 max-w-3xl text-[15px] leading-7 text-muted-foreground">
                {page.description}
              </p>
            </div>

            <ScrollArea className="min-h-0 flex-1">
              <article className="mx-auto max-w-4xl px-6 py-8">
                <MarkdownRenderer content={page.content} />
              </article>
            </ScrollArea>

            <div className="flex flex-wrap items-center justify-between gap-3 border-t border-border/60 px-6 py-4">
              {previous ? (
                <Button asChild variant="outline" size="sm">
                  <Link href={`/docs/${previous.slug}`}>
                    Previous: {previous.title}
                  </Link>
                </Button>
              ) : (
                <div />
              )}
              {next ? (
                <Button asChild variant="outline" size="sm">
                  <Link href={`/docs/${next.slug}`}>Next: {next.title}</Link>
                </Button>
              ) : null}
            </div>
          </div>
        </div>

        <div className="hidden xl:block">
          <DocsToc
            headings={page.headings.filter((heading) => heading.level >= 2)}
          />
        </div>
      </div>
    </main>
  )
}
