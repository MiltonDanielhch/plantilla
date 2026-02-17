import * as React from "react"
import { cn } from "../../lib/utils"
import { FileX2 } from "lucide-react"

interface EmptyStateProps extends React.HTMLAttributes<HTMLDivElement> {
  title: string
  description?: string
  icon?: React.ElementType
  action?: React.ReactNode
  className?: string
}

export function EmptyState({
  title,
  description,
  icon: Icon = FileX2,
  action,
  className,
  children,
  ...props
}: EmptyStateProps) {
  return (
    <div
      className={cn(
        "flex flex-col items-center justify-center py-12 text-center animate-in fade-in-50",
        className
      )}
      {...props}
    >
      <div className="flex h-20 w-20 items-center justify-center rounded-full bg-muted/50 mb-4">
        <Icon className="h-10 w-10 text-muted-foreground" />
      </div>
      <h3 className="text-lg font-semibold tracking-tight">{title}</h3>
      {description && (
        <p className="mt-2 text-sm text-muted-foreground max-w-sm text-balance">
          {description}
        </p>
      )}
      {children}
      {action && <div className="mt-6">{action}</div>}
    </div>
  )
}