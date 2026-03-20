function LoadingLine({ className }: { className: string }) {
  return <div className={`animate-pulse rounded-sm bg-muted/50 ${className}`} />
}

export default function Loading() {
  return (
    <main className="h-svh overflow-hidden bg-[radial-gradient(circle_at_top_left,rgba(94,234,212,0.07),transparent_22%),linear-gradient(180deg,rgba(10,12,20,1),rgba(14,18,30,1))] px-4 py-5">
      <div className="mx-auto grid h-full min-h-0 max-w-[1560px] gap-4 xl:grid-cols-[280px_minmax(0,1fr)_240px]">
        <div className="sticky top-4 flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
          <div className="border-b border-border/60 px-3 py-3">
            <LoadingLine className="h-4 w-24" />
          </div>
          <div className="space-y-5 px-3 py-3">
            <LoadingLine className="h-3 w-20" />
            <LoadingLine className="h-9 w-full" />
            <LoadingLine className="h-9 w-full" />
            <LoadingLine className="h-9 w-5/6" />
            <LoadingLine className="h-3 w-24" />
            <LoadingLine className="h-9 w-full" />
            <LoadingLine className="h-9 w-4/5" />
          </div>
        </div>

        <div className="min-h-0 min-w-0">
          <div className="flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
            <div className="border-b border-border/60 px-6 py-5">
              <LoadingLine className="h-4 w-24" />
              <LoadingLine className="mt-3 h-10 w-2/5" />
              <LoadingLine className="mt-3 h-4 w-4/5" />
              <LoadingLine className="mt-2 h-4 w-3/5" />
            </div>

            <div className="min-h-0 flex-1">
              <div className="mx-auto max-w-4xl space-y-4 px-6 py-8">
                <LoadingLine className="h-8 w-1/3" />
                <LoadingLine className="h-4 w-full" />
                <LoadingLine className="h-4 w-11/12" />
                <LoadingLine className="h-4 w-4/5" />
                <LoadingLine className="mt-6 h-7 w-1/4" />
                <LoadingLine className="h-4 w-full" />
                <LoadingLine className="h-4 w-10/12" />
                <LoadingLine className="h-4 w-9/12" />
                <LoadingLine className="mt-6 h-28 w-full" />
              </div>
            </div>

            <div className="border-t border-border/60 px-6 py-4">
              <div className="flex items-center justify-between gap-3">
                <LoadingLine className="h-9 w-40" />
                <LoadingLine className="h-9 w-36" />
              </div>
            </div>
          </div>
        </div>

        <div className="hidden xl:block">
          <div className="sticky top-4 flex h-[calc(100svh-2rem)] min-h-0 flex-col border border-border/60 bg-card/80">
            <div className="border-b border-border/60 px-3 py-3">
              <LoadingLine className="h-4 w-28" />
            </div>
            <div className="space-y-2 px-3 py-3">
              <LoadingLine className="h-8 w-full" />
              <LoadingLine className="h-8 w-5/6" />
              <LoadingLine className="h-8 w-4/5" />
              <LoadingLine className="h-8 w-3/4" />
            </div>
          </div>
        </div>
      </div>
    </main>
  )
}
