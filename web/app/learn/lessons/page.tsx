import Link from "next/link"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { LEARNING_LESSONS } from "@/lib/learning-content"

export default function LearnLessonsPage() {
  return (
    <main className="min-h-svh bg-[radial-gradient(circle_at_top_left,rgba(251,113,133,0.08),transparent_24%),linear-gradient(180deg,rgba(10,12,20,1),rgba(14,18,30,1))] px-4 py-5">
      <div className="mx-auto flex max-w-[1380px] flex-col gap-4">
        <Card
          size="sm"
          className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
        >
          <CardHeader className="gap-2 border-b border-border/60 pb-3">
            <div className="flex items-center gap-2">
              <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                Lessons
              </span>
              <Badge
                variant="secondary"
                className="px-1.5 font-mono text-[10px]"
              >
                Guided
              </Badge>
            </div>
            <CardTitle className="text-xl font-semibold">
              Structured study path for orderbook intuition
            </CardTitle>
            <p className="max-w-3xl text-sm text-muted-foreground">
              These lessons are the future guided track. For now they define the
              sequence the terminal should eventually teach.
            </p>
          </CardHeader>
          <CardContent className="flex flex-wrap items-center gap-2 pt-3">
            <Button asChild variant="outline" size="sm">
              <Link href="/learn">Back to Learn</Link>
            </Button>
            <Button asChild variant="outline" size="sm">
              <Link href="/">Open Terminal</Link>
            </Button>
          </CardContent>
        </Card>

        <div className="grid gap-4">
          {LEARNING_LESSONS.map((lesson) => (
            <Card
              key={lesson.title}
              size="sm"
              className="border-border/60 bg-card/80 shadow-2xl shadow-black/15"
            >
              <CardHeader className="gap-1.5 border-b border-border/60 pb-3">
                <div className="flex items-center gap-2">
                  <Badge
                    variant="secondary"
                    className="px-1.5 font-mono text-[10px]"
                  >
                    {lesson.stage}
                  </Badge>
                  <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                    Learning Path
                  </span>
                </div>
                <CardTitle>{lesson.title}</CardTitle>
              </CardHeader>
              <CardContent className="space-y-3 pt-3">
                <p className="text-sm text-muted-foreground">
                  {lesson.summary}
                </p>
                <div className="grid gap-2 xl:grid-cols-3">
                  {lesson.bullets.map((bullet) => (
                    <div
                      key={bullet}
                      className="border border-border/60 bg-background/40 p-3 text-sm text-muted-foreground"
                    >
                      {bullet}
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </main>
  )
}
