"use client"

import * as React from "react"
import { CaretLeft, CaretRight } from "@phosphor-icons/react"

import { Badge } from "@/components/ui/badge"
import { Button } from "@/components/ui/button"
import { Separator } from "@/components/ui/separator"
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group"
import { cn } from "@/lib/utils"
import {
  type EngineSimulationSpeed,
  type EngineScenarioId,
  resetEngineBook,
  setEngineSimulationSpeed,
  seedEngineScenario,
  startEngineSimulation,
  stopEngineSimulation,
  simulateCrossOrder,
} from "@/lib/engine-client"

import type { OrderbookEvent, OrderbookSnapshot } from "./orderbook-types"

type ScenarioControlsProps = {
  onSnapshotChange: (snapshot: OrderbookSnapshot) => void
  onScenarioEvent?: (event: OrderbookEvent) => void
  compact?: boolean
  marketSimulationActive?: boolean
  simulationSpeed?: EngineSimulationSpeed
  simulationScenario?: EngineScenarioId
  showTransport?: boolean
  showMeta?: boolean
  showStatus?: boolean
  onSimulationChange?: (
    snapshot: OrderbookSnapshot,
    event: OrderbookEvent
  ) => void
  mode?: "footer" | "panel"
}

const scenarios: Array<{
  id: EngineScenarioId
  label: string
}> = [
  { id: "balanced", label: "Balanced" },
  { id: "trend-up", label: "Trend Up" },
  { id: "trend-down", label: "Trend Down" },
  { id: "range", label: "Range" },
  { id: "bid-wall", label: "Bid Wall" },
  { id: "ask-wall", label: "Ask Wall" },
  { id: "thin-liquidity", label: "Thin" },
]

const scenarioRows: Array<Array<{ id: EngineScenarioId; label: string }>> = [
  scenarios.slice(0, 4),
  scenarios.slice(4),
]

const flowActions = [
  { id: "cross-buy", label: "Cross Buy", side: "buy" as const },
  { id: "cross-sell", label: "Cross Sell", side: "sell" as const },
]

const simulationSpeeds: EngineSimulationSpeed[] = [
  "slow",
  "normal",
  "fast",
  "burst",
]

const compactButtonClassName =
  "justify-center font-mono text-[9px] tracking-[0.05em] uppercase px-1.5"

const selectedButtonClassName =
  "border-border/80 bg-muted text-foreground hover:bg-muted/90"

export function ScenarioControls({
  onSnapshotChange,
  onScenarioEvent,
  compact = false,
  marketSimulationActive = false,
  simulationSpeed = "normal",
  simulationScenario = "balanced",
  showTransport = true,
  showMeta = true,
  showStatus = true,
  onSimulationChange,
  mode = "panel",
}: ScenarioControlsProps) {
  const [pendingAction, setPendingAction] = React.useState<string | null>(null)
  const [statusMessage, setStatusMessage] = React.useState<string | null>(null)

  async function handleReset() {
    try {
      setPendingAction("reset")
      const result = await resetEngineBook()
      onSnapshotChange(result.snapshot)
      onScenarioEvent?.(result.event)
      setStatusMessage(result.event.detail)
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to reset book."
      )
    } finally {
      setPendingAction(null)
    }
  }

  async function handleScenarioLoad(scenarioId: EngineScenarioId) {
    try {
      setPendingAction(scenarioId)
      const result = await seedEngineScenario(scenarioId)
      onSnapshotChange(result.snapshot)
      onScenarioEvent?.(result.event)
      setStatusMessage(result.event.detail)
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to load scenario."
      )
    } finally {
      setPendingAction(null)
    }
  }

  async function handleCross(side: "buy" | "sell") {
    try {
      setPendingAction(`cross-${side}`)
      const result = await simulateCrossOrder(side)
      onSnapshotChange(result.snapshot)
      onScenarioEvent?.(result.event)
      setStatusMessage(result.event.detail)
    } catch (error) {
      setStatusMessage(
        error instanceof Error ? error.message : "Failed to run crossing flow."
      )
    } finally {
      setPendingAction(null)
    }
  }

  async function handleSimulationToggle() {
    try {
      setPendingAction("simulation-toggle")
      const result = marketSimulationActive
        ? await stopEngineSimulation()
        : await startEngineSimulation()
      onSnapshotChange(result.snapshot)
      onScenarioEvent?.(result.event)
      onSimulationChange?.(result.snapshot, result.event)
      setStatusMessage(result.event.detail)
    } catch (error) {
      setStatusMessage(
        error instanceof Error
          ? error.message
          : "Failed to update market simulation."
      )
    } finally {
      setPendingAction(null)
    }
  }

  async function handleSpeedChange(speed: EngineSimulationSpeed) {
    try {
      setPendingAction(`speed-${speed}`)
      const result = await setEngineSimulationSpeed(speed)
      onSnapshotChange(result.snapshot)
      onScenarioEvent?.(result.event)
      onSimulationChange?.(result.snapshot, result.event)
      setStatusMessage(result.event.detail)
    } catch (error) {
      setStatusMessage(
        error instanceof Error
          ? error.message
          : "Failed to change simulation speed."
      )
    } finally {
      setPendingAction(null)
    }
  }

  async function handleSpeedStep(direction: -1 | 1) {
    const currentIndex = simulationSpeeds.indexOf(simulationSpeed)
    const nextIndex = Math.min(
      simulationSpeeds.length - 1,
      Math.max(0, currentIndex + direction)
    )

    if (nextIndex === currentIndex) return

    await handleSpeedChange(simulationSpeeds[nextIndex])
  }

  const footerMode = mode === "footer"
  const currentSpeedIndex = simulationSpeeds.indexOf(simulationSpeed)
  const canDecreaseSpeed = currentSpeedIndex > 0
  const canIncreaseSpeed = currentSpeedIndex < simulationSpeeds.length - 1
  const gapClassName =
    !showMeta && !showStatus
      ? compact
        ? "gap-1.5"
        : "gap-2"
      : compact
        ? "gap-2"
        : "gap-3"

  return (
    <div className={cn("flex flex-col", gapClassName)}>
      {showMeta ? (
        <div
          className={cn(
            "flex items-start justify-between gap-3",
            compact && "items-center"
          )}
        >
          <div className="flex items-center justify-between gap-3">
            <div className="flex items-center gap-2">
              <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                Simulate
              </span>
              <Badge
                variant="secondary"
                className="px-1.5 font-mono text-[10px]"
              >
                Scenario Lab
              </Badge>
            </div>
          </div>
          {!footerMode ? (
            <Button
              type="button"
              variant="outline"
              size="xs"
              onClick={handleReset}
              disabled={pendingAction != null}
            >
              Reset
            </Button>
          ) : null}
        </div>
      ) : null}

      {!compact && !footerMode && showMeta ? (
        <p className="font-mono text-[11px] text-muted-foreground">
          Load repeatable market states to study spread, depth, and order flow.
        </p>
      ) : null}

      {showTransport ? (
        <div className="flex items-center gap-1.5">
          <Button
            type="button"
            variant="outline"
            size="icon-xs"
            className="shrink-0"
            onClick={() => void handleSpeedStep(-1)}
            disabled={pendingAction != null || !canDecreaseSpeed}
            aria-label="Decrease simulation speed"
            title={`Decrease speed from ${simulationSpeed}`}
          >
            <CaretLeft />
          </Button>
          <Button
            type="button"
            variant={marketSimulationActive ? "default" : "outline"}
            size="xs"
            className={cn(
              "min-w-0 flex-1 font-mono text-[9px] tracking-[0.05em] uppercase",
              marketSimulationActive &&
                "border-cyan-300/60 bg-cyan-300/95 text-black hover:bg-cyan-300"
            )}
            onClick={handleSimulationToggle}
            disabled={pendingAction != null}
          >
            {marketSimulationActive ? "Simulating" : "Simulate"} ·{" "}
            {simulationSpeed}
          </Button>
          <Button
            type="button"
            variant="outline"
            size="icon-xs"
            className="shrink-0"
            onClick={() => void handleSpeedStep(1)}
            disabled={pendingAction != null || !canIncreaseSpeed}
            aria-label="Increase simulation speed"
            title={`Increase speed from ${simulationSpeed}`}
          >
            <CaretRight />
          </Button>
        </div>
      ) : null}
      {!footerMode ? (
        <>
          <div className="flex flex-wrap items-center gap-1">
            {scenarioRows.map((row, rowIndex) => (
              <React.Fragment key={rowIndex}>
                {rowIndex > 0 ? (
                  <Separator
                    orientation="vertical"
                    className="h-5 shrink-0 bg-border/70"
                  />
                ) : null}
                <ToggleGroup
                  type="single"
                  value={simulationScenario}
                  onValueChange={(value) => {
                    if (value)
                      void handleScenarioLoad(value as EngineScenarioId)
                  }}
                  variant="outline"
                  size="sm"
                  className="w-fit shrink-0"
                  spacing={0}
                >
                  {row.map((scenario) => (
                    <ToggleGroupItem
                      key={scenario.id}
                      value={scenario.id}
                      aria-label={`Set ${scenario.label} scenario`}
                      className={cn(
                        compactButtonClassName,
                        "h-6 w-auto shrink-0 px-1.5 data-[state=on]:bg-muted data-[state=on]:text-foreground"
                      )}
                      disabled={pendingAction != null}
                    >
                      {scenario.label}
                    </ToggleGroupItem>
                  ))}
                </ToggleGroup>
              </React.Fragment>
            ))}
            <Separator
              orientation="vertical"
              className="h-5 shrink-0 bg-border/70"
            />
            <div className="flex min-w-[150px] flex-1 items-center gap-1">
              {flowActions.map((action) => (
                <Button
                  key={action.id}
                  type="button"
                  variant="secondary"
                  size="xs"
                  className={cn(
                    compactButtonClassName,
                    "min-w-0 flex-1",
                    action.side === "buy"
                      ? "bg-emerald-500/12 text-emerald-300 hover:bg-emerald-500/18"
                      : "bg-rose-500/12 text-rose-300 hover:bg-rose-500/18"
                  )}
                  disabled={pendingAction != null}
                  onClick={() => handleCross(action.side)}
                >
                  {action.label}
                </Button>
              ))}
            </div>
          </div>
        </>
      ) : null}
      {showStatus ? (
        <p
          className={cn(
            "min-h-[16px] font-mono text-muted-foreground",
            compact ? "text-[9px]" : "text-[10px]"
          )}
        >
          {statusMessage ?? "Balanced reset restores the default seeded book."}
        </p>
      ) : null}
    </div>
  )
}
