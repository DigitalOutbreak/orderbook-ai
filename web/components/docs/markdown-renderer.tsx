import * as React from "react"

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table"
import { cn } from "@/lib/utils"

type Block =
  | { type: "heading"; level: number; text: string; id: string }
  | { type: "paragraph"; text: string }
  | { type: "blockquote"; text: string }
  | { type: "unordered-list"; items: string[] }
  | { type: "ordered-list"; items: string[] }
  | { type: "code"; language: string; code: string }
  | { type: "table"; headers: string[]; rows: string[][] }
  | { type: "hr" }

function slugify(value: string) {
  return value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9\s-]/g, "")
    .replace(/\s+/g, "-")
    .replace(/-+/g, "-")
}

function splitTableRow(line: string) {
  return line
    .split("|")
    .map((cell) => cell.trim())
    .filter((cell, index, cells) => {
      if (index === 0 && cell === "") return false
      if (index === cells.length - 1 && cell === "") return false
      return true
    })
}

function renderInline(text: string): React.ReactNode[] {
  const tokens: React.ReactNode[] = []
  const pattern = /(`[^`]+`)|(\[([^\]]+)\]\(([^)]+)\))/g
  let lastIndex = 0
  let match: RegExpExecArray | null

  while ((match = pattern.exec(text)) !== null) {
    if (match.index > lastIndex) {
      tokens.push(text.slice(lastIndex, match.index))
    }

    if (match[1]) {
      tokens.push(
        <code
          key={`${match.index}-code`}
          className="rounded-none bg-muted px-1.5 py-0.5 font-mono text-[0.92em] text-foreground"
        >
          {match[1].slice(1, -1)}
        </code>
      )
    } else if (match[2]) {
      tokens.push(
        <a
          key={`${match.index}-link`}
          href={match[4]}
          className="text-emerald-300 underline underline-offset-4 hover:text-emerald-200"
          target={match[4].startsWith("http") ? "_blank" : undefined}
          rel={match[4].startsWith("http") ? "noreferrer" : undefined}
        >
          {match[3]}
        </a>
      )
    }

    lastIndex = pattern.lastIndex
  }

  if (lastIndex < text.length) {
    tokens.push(text.slice(lastIndex))
  }

  return tokens
}

function parseMarkdown(markdown: string): Block[] {
  const lines = markdown.replace(/\r\n/g, "\n").split("\n")
  const blocks: Block[] = []
  let index = 0

  while (index < lines.length) {
    const line = lines[index]
    const trimmed = line.trim()

    if (!trimmed) {
      index += 1
      continue
    }

    if (trimmed === "---") {
      blocks.push({ type: "hr" })
      index += 1
      continue
    }

    const headingMatch = trimmed.match(/^(#{1,3})\s+(.*)$/)
    if (headingMatch) {
      blocks.push({
        type: "heading",
        level: headingMatch[1].length,
        text: headingMatch[2].trim(),
        id: slugify(headingMatch[2].trim()),
      })
      index += 1
      continue
    }

    if (trimmed.startsWith("```")) {
      const language = trimmed.slice(3).trim()
      const codeLines: string[] = []
      index += 1
      while (index < lines.length && !lines[index].trim().startsWith("```")) {
        codeLines.push(lines[index])
        index += 1
      }
      index += 1
      blocks.push({
        type: "code",
        language,
        code: codeLines.join("\n"),
      })
      continue
    }

    const next = lines[index + 1]?.trim() ?? ""
    if (trimmed.includes("|") && /^\|?[\s:-]+\|[\s|:-]*$/.test(next)) {
      const headers = splitTableRow(trimmed)
      const rows: string[][] = []
      index += 2
      while (index < lines.length && lines[index].includes("|")) {
        rows.push(splitTableRow(lines[index]))
        index += 1
      }
      blocks.push({ type: "table", headers, rows })
      continue
    }

    if (trimmed.startsWith(">")) {
      const quoteLines: string[] = []
      while (index < lines.length && lines[index].trim().startsWith(">")) {
        quoteLines.push(lines[index].trim().replace(/^>\s?/, ""))
        index += 1
      }
      blocks.push({ type: "blockquote", text: quoteLines.join(" ") })
      continue
    }

    if (/^\d+\.\s+/.test(trimmed)) {
      const items: string[] = []
      while (index < lines.length && /^\d+\.\s+/.test(lines[index].trim())) {
        items.push(lines[index].trim().replace(/^\d+\.\s+/, ""))
        index += 1
      }
      blocks.push({ type: "ordered-list", items })
      continue
    }

    if (/^[-*+]\s+/.test(trimmed)) {
      const items: string[] = []
      while (index < lines.length && /^[-*+]\s+/.test(lines[index].trim())) {
        items.push(lines[index].trim().replace(/^[-*+]\s+/, ""))
        index += 1
      }
      blocks.push({ type: "unordered-list", items })
      continue
    }

    const paragraphLines: string[] = []
    while (index < lines.length) {
      const current = lines[index].trim()
      const upcoming = lines[index + 1]?.trim() ?? ""
      if (
        !current ||
        /^#{1,3}\s+/.test(current) ||
        current.startsWith("```") ||
        current.startsWith(">") ||
        /^[-*+]\s+/.test(current) ||
        /^\d+\.\s+/.test(current) ||
        current === "---" ||
        (current.includes("|") && /^\|?[\s:-]+\|[\s|:-]*$/.test(upcoming))
      ) {
        break
      }
      paragraphLines.push(current)
      index += 1
    }
    blocks.push({ type: "paragraph", text: paragraphLines.join(" ") })
  }

  return blocks
}

type MarkdownRendererProps = {
  content: string
}

export function MarkdownRenderer({ content }: MarkdownRendererProps) {
  const blocks = React.useMemo(() => parseMarkdown(content), [content])

  return (
    <div className="space-y-6">
      {blocks.map((block, index) => {
        if (block.type === "heading") {
          if (block.level === 1) {
            return (
              <h1
                key={`${block.id}-${index}`}
                id={block.id}
                className="scroll-mt-20 text-3xl font-semibold tracking-tight"
              >
                {block.text}
              </h1>
            )
          }

          if (block.level === 2) {
            return (
              <h2
                key={`${block.id}-${index}`}
                id={block.id}
                className="scroll-mt-20 border-t border-border/60 pt-6 text-xl font-semibold"
              >
                {block.text}
              </h2>
            )
          }

          return (
            <h3
              key={`${block.id}-${index}`}
              id={block.id}
              className="scroll-mt-20 text-lg font-semibold"
            >
              {block.text}
            </h3>
          )
        }

        if (block.type === "paragraph") {
          return (
            <p key={`p-${index}`} className="text-[15px] leading-7 text-foreground/88">
              {renderInline(block.text)}
            </p>
          )
        }

        if (block.type === "blockquote") {
          return (
            <blockquote
              key={`q-${index}`}
              className="border-l-2 border-emerald-400/50 bg-muted/20 px-4 py-3 text-[15px] leading-7 text-foreground/82"
            >
              {renderInline(block.text)}
            </blockquote>
          )
        }

        if (block.type === "unordered-list") {
          return (
            <ul key={`ul-${index}`} className="space-y-2 pl-5 text-[15px] leading-7 text-foreground/88">
              {block.items.map((item) => (
                <li key={item} className="list-disc">
                  {renderInline(item)}
                </li>
              ))}
            </ul>
          )
        }

        if (block.type === "ordered-list") {
          return (
            <ol key={`ol-${index}`} className="space-y-2 pl-5 text-[15px] leading-7 text-foreground/88">
              {block.items.map((item, itemIndex) => (
                <li key={`${item}-${itemIndex}`} className="list-decimal">
                  {renderInline(item)}
                </li>
              ))}
            </ol>
          )
        }

        if (block.type === "code") {
          return (
            <div key={`code-${index}`} className="overflow-hidden border border-border/60 bg-background/70">
              <div className="flex items-center justify-between border-b border-border/60 px-3 py-2">
                <span className="font-mono text-[10px] tracking-[0.22em] text-muted-foreground uppercase">
                  {block.language || "code"}
                </span>
              </div>
              <pre className="overflow-x-auto p-4 text-[13px] leading-6 text-foreground/92">
                <code>{block.code}</code>
              </pre>
            </div>
          )
        }

        if (block.type === "table") {
          return (
            <div key={`table-${index}`} className="overflow-hidden border border-border/60 bg-background/40">
              <Table>
                <TableHeader>
                  <TableRow>
                    {block.headers.map((header) => (
                      <TableHead key={header}>{header}</TableHead>
                    ))}
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {block.rows.map((row, rowIndex) => (
                    <TableRow key={`${row.join("-")}-${rowIndex}`}>
                      {row.map((cell, cellIndex) => (
                        <TableCell key={`${cell}-${cellIndex}`}>
                          {renderInline(cell)}
                        </TableCell>
                      ))}
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          )
        }

        if (block.type === "hr") {
          return <div key={`hr-${index}`} className="border-t border-border/60" />
        }

        return null
      })}
    </div>
  )
}
