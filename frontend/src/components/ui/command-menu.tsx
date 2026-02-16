import * as React from "react"
import { Command } from "cmdk"
import { 
  LayoutDashboard, 
  Users, 
  FileText, 
  Settings, 
  LogOut, 
  Search,
  UserPlus
} from "lucide-react"
import { cn } from "../../lib/utils"

export function CommandMenu() {
  const [open, setOpen] = React.useState(false)

  React.useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault()
        setOpen((open) => !open)
      }
    }

    document.addEventListener("keydown", down)
    return () => document.removeEventListener("keydown", down)
  }, [])

  const runCommand = React.useCallback((command: () => unknown) => {
    setOpen(false)
    command()
  }, [])

  return (
    <>
      <Command.Dialog
        open={open}
        onOpenChange={setOpen}
        label="Global Command Menu"
        className="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-full max-w-[640px] rounded-xl border bg-popover p-0 shadow-2xl text-popover-foreground z-[9999] overflow-hidden"
      >
        <div className="flex items-center border-b px-3" cmdk-input-wrapper="">
            <Search className="mr-2 h-4 w-4 shrink-0 opacity-50" />
            <Command.Input 
                placeholder="Escribe un comando o busca..." 
                className="flex h-11 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50 border-none focus:ring-0"
            />
        </div>
        <Command.List className="max-h-[300px] overflow-y-auto overflow-x-hidden p-2">
          <Command.Empty className="py-6 text-center text-sm">No se encontraron resultados.</Command.Empty>
          
          <Command.Group heading="Navegación">
            <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard')}>
              <LayoutDashboard className="mr-2 h-4 w-4" />
              <span>Dashboard</span>
            </CommandItem>
            <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/users')}>
              <Users className="mr-2 h-4 w-4" />
              <span>Usuarios</span>
            </CommandItem>
            <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/audit')}>
              <FileText className="mr-2 h-4 w-4" />
              <span>Auditoría</span>
            </CommandItem>
            <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/settings')}>
              <Settings className="mr-2 h-4 w-4" />
              <span>Configuración</span>
            </CommandItem>
          </Command.Group>

          <Command.Separator className="-mx-1 h-px bg-border my-1" />

          <Command.Group heading="Acciones Rápidas">
             <CommandItem onSelect={() => runCommand(() => window.location.href = '/register')}>
              <UserPlus className="mr-2 h-4 w-4" />
              <span>Nuevo Usuario</span>
            </CommandItem>
          </Command.Group>

          <Command.Separator className="-mx-1 h-px bg-border my-1" />

          <Command.Group heading="Cuenta">
            <CommandItem 
                onSelect={() => runCommand(() => {
                    document.cookie = "auth_token=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT";
                    window.location.href = '/login';
                })}
                className="text-destructive aria-selected:text-destructive"
            >
              <LogOut className="mr-2 h-4 w-4" />
              <span>Cerrar Sesión</span>
            </CommandItem>
          </Command.Group>
        </Command.List>
      </Command.Dialog>
    </>
  )
}

function CommandItem({ children, className, ...props }: any) {
    return (
        <Command.Item 
            className={cn(
                "relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50 cursor-pointer", 
                className
            )}
            {...props}
        >
            {children}
        </Command.Item>
    )
}