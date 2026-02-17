import * as React from "react"
import { 
  Settings, 
  User,
  LayoutDashboard,
  LogOut,
  Moon,
  Sun,
  Users,
  Shield,
} from "lucide-react"

import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
} from "../../components/ui/command"
import { api } from "../../lib/api"

export function CommandMenu() {
  const [open, setOpen] = React.useState(false)
  const [query, setQuery] = React.useState("")
  const [users, setUsers] = React.useState<any[]>([])
  const [roles, setRoles] = React.useState<any[]>([])
  const [loading, setLoading] = React.useState(false)

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

  React.useEffect(() => {
    const openMenu = () => setOpen(true)
    document.addEventListener("open-command-menu", openMenu)
    return () => document.removeEventListener("open-command-menu", openMenu)
  }, [])

  // Búsqueda dinámica
  React.useEffect(() => {
    if (query.length === 0) {
        setUsers([])
        setRoles([])
        return
    }
    
    const delayDebounceFn = setTimeout(async () => {
      setLoading(true)
      try {
        // Buscar usuarios
        const usersRes = await api.getUsers({ search: query, limit: 5 })
        setUsers(usersRes.data || [])

        // Buscar roles (filtrado en cliente)
        const rolesRes = await api.getRoles()
        const filteredRoles = (rolesRes || []).filter((r: any) => 
            r.name.toLowerCase().includes(query.toLowerCase()) || 
            (r.description && r.description.toLowerCase().includes(query.toLowerCase()))
        )
        setRoles(filteredRoles)
      } catch (error) {
        console.error(error)
      } finally {
        setLoading(false)
      }
    }, 300)

    return () => clearTimeout(delayDebounceFn)
  }, [query])

  const runCommand = React.useCallback((command: () => unknown) => {
    setOpen(false)
    command()
  }, [])

  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Escribe un comando o busca..." value={query} onValueChange={setQuery} />
      <CommandList>
        <CommandEmpty>No se encontraron resultados.</CommandEmpty>
        
        {/* Resultados de Roles */}
        {roles.length > 0 && (
            <CommandGroup heading="Roles">
                {roles.map((role) => (
                    <CommandItem
                        key={role.id}
                        onSelect={() => {
                            runCommand(() => window.location.href = `/dashboard/roles`)
                        }}
                    >
                        <Shield className="mr-2 h-4 w-4" />
                        <span>{role.name}</span>
                        <span className="ml-2 text-xs text-muted-foreground truncate">{role.description}</span>
                    </CommandItem>
                ))}
            </CommandGroup>
        )}

        {/* Resultados de Usuarios */}
        {users.length > 0 && (
            <CommandGroup heading="Usuarios">
                {users.map((user) => (
                    <CommandItem
                        key={user.id}
                        onSelect={() => {
                            runCommand(() => window.location.href = `/dashboard/users/${user.id}`)
                        }}
                    >
                        <User className="mr-2 h-4 w-4" />
                        <span>{user.username}</span>
                        <span className="ml-2 text-xs text-muted-foreground">{user.email}</span>
                    </CommandItem>
                ))}
            </CommandGroup>
        )}
        
        {(users.length > 0 || roles.length > 0) && <CommandSeparator />}

        <CommandGroup heading="Navegación">
          <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard')}>
            <LayoutDashboard className="mr-2 h-4 w-4" />
            <span>Dashboard</span>
          </CommandItem>
          <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/users')}>
            <Users className="mr-2 h-4 w-4" />
            <span>Usuarios</span>
          </CommandItem>
          <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/audit')}>
            <Shield className="mr-2 h-4 w-4" />
            <span>Auditoría</span>
          </CommandItem>
          <CommandItem onSelect={() => runCommand(() => window.location.href = '/dashboard/settings')}>
            <Settings className="mr-2 h-4 w-4" />
            <span>Configuración</span>
          </CommandItem>
        </CommandGroup>
        
        <CommandSeparator />
        
        <CommandGroup heading="Tema">
          <CommandItem onSelect={() => runCommand(() => {
              document.documentElement.classList.remove('dark')
              localStorage.setItem('theme', 'light')
          })}>
            <Sun className="mr-2 h-4 w-4" />
            <span>Claro</span>
          </CommandItem>
          <CommandItem onSelect={() => runCommand(() => {
              document.documentElement.classList.add('dark')
              localStorage.setItem('theme', 'dark')
          })}>
            <Moon className="mr-2 h-4 w-4" />
            <span>Oscuro</span>
          </CommandItem>
        </CommandGroup>

        <CommandSeparator />

        <CommandGroup heading="Cuenta">
            <CommandItem onSelect={() => runCommand(async () => {
                await api.logout()
            })}>
                <LogOut className="mr-2 h-4 w-4" />
                <span>Cerrar Sesión</span>
            </CommandItem>
        </CommandGroup>
      </CommandList>
    </CommandDialog>
  )
}