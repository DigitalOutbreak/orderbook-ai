import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { LEARNING_LESSONS, LEARNING_LINKS } from "@/lib/learning-content"

export default function LearnPage() {
  return (
    <main className="min-h-svh bg-[radial-gradient(circle_at_top_left,rgba(52,211,153,0.08),transparent_24%),linear-gradient(180deg,rgba(10,12,20,1),rgba(14,18,30,1))] px-4 py-5">
      <div className="mx-auto flex max-w-[1380px] flex-col gap-4">
        <Card
          size="sm"
          className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
        >
          <CardHeader className="gap-2 border-b border-border/60 pb-3">
            <div className="flex items-center gap-2">
              <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                Learn
              </span>
              <Badge
                variant="secondary"
                className="px-1.5 font-mono text-[10px]"
              >
                Quant Dev
              </Badge>
            </div>
            <CardTitle className="text-xl font-semibold">
              Study the terminal like a matching-engine workstation
            </CardTitle>
            <p className="max-w-3xl text-sm text-muted-foreground">
              Use the terminal for interaction, then come here for the
              longer-form explanations. Keep Glossary for quick lookup and use
              Docs and Lessons for deeper study.
            </p>
          </CardHeader>
          <CardContent className="flex flex-wrap items-center gap-2 pt-3">
            <Button asChild variant="outline" size="sm">
              <Link href="/">Back to Terminal</Link>
            </Button>
            {LEARNING_LINKS.map((link) => (
              <Button key={link.href} asChild variant="outline" size="sm">
                <Link href={link.href}>{link.title}</Link>
              </Button>
            ))}
          </CardContent>
        </Card>

        <div className="grid gap-4 xl:grid-cols-[minmax(0,1fr)_minmax(360px,0.85fr)]">
          <div className="grid gap-4">
            {LEARNING_LINKS.map((link) => (
              <Card
                key={link.href}
                size="sm"
                className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
              >
                <CardHeader className="gap-1.5 border-b border-border/60 pb-3">
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                      Learn
                    </span>
                    <Badge
                      variant="outline"
                      className="px-1.5 font-mono text-[10px]"
                    >
                      {link.title}
                    </Badge>
                  </div>
                  <CardTitle>{link.title}</CardTitle>
                </CardHeader>
                <CardContent className="space-y-3 pt-3">
                  <p className="text-sm text-muted-foreground">
                    {link.description}
                  </p>
                  <Button asChild variant="outline" size="sm">
                    <Link href={link.href}>Open {link.title}</Link>
                  </Button>
                </CardContent>
              </Card>
            ))}
          </div>

          <Card
            size="sm"
            className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
          >
            <CardHeader className="gap-1.5 border-b border-border/60 pb-3">
              <div className="flex items-center gap-2">
                <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                  Preview
                </span>
                <Badge
                  variant="outline"
                  className="px-1.5 font-mono text-[10px]"
                >
                  Lessons
                </Badge>
              </div>
              <CardTitle>What the learning track will teach</CardTitle>
            </CardHeader>
            <CardContent className="space-y-3 pt-3">
              {LEARNING_LESSONS.slice(0, 3).map((lesson) => (
                <div
                  key={lesson.title}
                  className="border border-border/60 bg-background/40 p-3"
                >
                  <div className="flex items-center gap-2">
                    <Badge
                      variant="secondary"
                      className="px-1.5 font-mono text-[10px]"
                    >
                      {lesson.stage}
                    </Badge>
                    <span className="font-medium">{lesson.title}</span>
                  </div>
                  <p className="mt-2 text-sm text-muted-foreground">
                    {lesson.summary}
                  </p>
                </div>
              ))}
            </CardContent>
          </Card>
        </div>
      </div>
    </main>
  )
}
