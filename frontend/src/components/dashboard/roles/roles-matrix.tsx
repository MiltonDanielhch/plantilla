import { useState } from "react"
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "../../ui/card"
import { Button } from "../../ui/button"
import { Check, X, Shield, User, Plus, Trash2, Edit2 } from "lucide-react"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter, DialogDescription } from "../../ui/dialog"
import { api } from "../../../lib/api"
import { addToast } from "../../../stores/toast"

interface Role {
  id: number
  name: string
  description?: string
}

interface Permission {
  id: number
  name: string
  description?: string
}

interface RolePermission {
  role_id: number
  permission_id: number
}

interface RolesMatrixProps {
  initialRoles: Role[]
  initialPermissions: Permission[]
  initialRolePermissions: RolePermission[]
}

export function RolesMatrix({ initialRoles, initialPermissions, initialRolePermissions }: RolesMatrixProps) {
  const [roles, setRoles] = useState(initialRoles)
  const [rolePermissions, setRolePermissions] = useState(initialRolePermissions)
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isPermDialogOpen, setIsPermDialogOpen] = useState(false)
  const [editingRole, setEditingRole] = useState<Role | null>(null)
  const [editingPermission, setEditingPermission] = useState<Permission | null>(null)
  const [formData, setFormData] = useState({ name: "", description: "", permissions: [] as number[] })
  const [permFormData, setPermFormData] = useState({ description: "" })
  const [loading, setLoading] = useState(false)

  const handleOpenDialog = (role?: Role) => {
    if (role) {
      setEditingRole(role)
      const currentPerms = rolePermissions
        .filter(rp => rp.role_id === role.id)
        .map(rp => rp.permission_id)
      setFormData({ name: role.name, description: role.description || "", permissions: currentPerms })
    } else {
      setEditingRole(null)
      setFormData({ name: "", description: "", permissions: [] })
    }
    setIsDialogOpen(true)
  }

  const handleOpenPermDialog = (perm: Permission) => {
    setEditingPermission(perm)
    setPermFormData({ description: perm.description || "" })
    setIsPermDialogOpen(true)
  }

  const handleSave = async () => {
    setLoading(true)
    try {
      if (editingRole) {
        await api.updateRole(editingRole.id, formData)
        addToast({ type: 'success', title: 'Rol actualizado', message: 'Los cambios se han guardado.' })
      } else {
        await api.createRole(formData)
        addToast({ type: 'success', title: 'Rol creado', message: 'El nuevo rol ha sido creado.' })
      }
      // Reload to refresh data (simpler than updating local state accurately with all relations)
      window.location.reload()
    } catch (error: any) {
      addToast({ type: 'error', title: 'Error', message: error.message || 'Ocurrió un error.' })
    } finally {
      setLoading(false)
      setIsDialogOpen(false)
    }
  }

  const handleSavePermission = async () => {
    if (!editingPermission) return
    setLoading(true)
    try {
      await api.updatePermission(editingPermission.id, permFormData.description)
      addToast({ type: 'success', title: 'Permiso actualizado', message: 'La descripción ha sido actualizada.' })
      window.location.reload()
    } catch (error: any) {
      addToast({ type: 'error', title: 'Error', message: error.message || 'Ocurrió un error.' })
    } finally {
      setLoading(false)
      setIsPermDialogOpen(false)
    }
  }

  const handleDelete = async (id: number) => {
    if (!confirm("¿Estás seguro de eliminar este rol?")) return
    try {
      await api.deleteRole(id)
      setRoles(roles.filter(r => r.id !== id))
      addToast({ type: 'success', title: 'Rol eliminado', message: 'El rol ha sido eliminado.' })
    } catch (error: any) {
      addToast({ type: 'error', title: 'Error', message: error.message || 'No se pudo eliminar.' })
    }
  }

  const togglePermission = (permId: number) => {
    setFormData(prev => {
      const exists = prev.permissions.includes(permId)
      return {
        ...prev,
        permissions: exists 
          ? prev.permissions.filter(p => p !== permId)
          : [...prev.permissions, permId]
      }
    })
  }

  const hasPermission = (roleId: number, permId: number) => {
    return rolePermissions.some(rp => rp.role_id === roleId && rp.permission_id === permId)
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-lg font-semibold">Roles del Sistema</h2>
        <Button onClick={() => handleOpenDialog()}>
          <Plus className="mr-2 h-4 w-4" /> Nuevo Rol
        </Button>
      </div>

      <div className="grid gap-4 md:grid-cols-3">
        {roles.map(role => (
          <Card key={role.id}>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">{role.name}</CardTitle>
              {role.name === 'Admin' ? <Shield className="h-4 w-4 text-primary" /> : <User className="h-4 w-4 text-muted-foreground" />}
            </CardHeader>
            <CardContent>
              <div className="text-sm text-muted-foreground mb-4">{role.description || "Sin descripción"}</div>
              <div className="flex justify-end gap-2">
                <Button variant="ghost" size="sm" onClick={() => handleOpenDialog(role)}>
                  <Edit2 className="h-4 w-4" />
                </Button>
                {role.name !== 'Admin' && role.name !== 'User' && (
                  <Button variant="ghost" size="sm" className="text-destructive" onClick={() => handleDelete(role.id)}>
                    <Trash2 className="h-4 w-4" />
                  </Button>
                )}
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Matriz de Permisos</CardTitle>
          <CardDescription>Vista detallada de capacidades por rol.</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="relative w-full overflow-auto">
            <table className="w-full caption-bottom text-sm">
              <thead>
                <tr className="border-b">
                  <th className="h-12 px-4 text-left font-medium text-muted-foreground">Permiso</th>
                  {roles.map(role => (
                    <th key={role.id} className="h-12 px-4 text-center font-medium text-muted-foreground">{role.name}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {initialPermissions.map(perm => (
                  <tr key={perm.id} className="border-b hover:bg-muted/50">
                    <td className="p-4 font-medium">
                      <div className="flex items-center gap-2">
                        <span>{perm.name}</span>
                        <Button variant="ghost" size="icon" className="h-6 w-6 text-muted-foreground hover:text-primary" onClick={() => handleOpenPermDialog(perm)}>
                          <Edit2 className="h-3 w-3" />
                        </Button>
                      </div>
                      <div className="text-xs text-muted-foreground font-normal mt-1">{perm.description}</div>
                    </td>
                    {roles.map(role => (
                      <td key={role.id} className="p-4 text-center">
                        {hasPermission(role.id, perm.id) ? (
                          <Check className="mx-auto h-4 w-4 text-primary" />
                        ) : (
                          <X className="mx-auto h-4 w-4 text-muted-foreground/30" />
                        )}
                      </td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </CardContent>
      </Card>

      <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
        <DialogContent className="sm:max-w-[500px]">
          <DialogHeader>
            <DialogTitle>{editingRole ? 'Editar Rol' : 'Crear Nuevo Rol'}</DialogTitle>
            <DialogDescription>Configura el nombre y los permisos del rol.</DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <label htmlFor="name" className="text-sm font-medium">Nombre</label>
              <input id="name" className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value={formData.name} onChange={e => setFormData({...formData, name: e.target.value})} />
            </div>
            <div className="grid gap-2">
              <label htmlFor="desc" className="text-sm font-medium">Descripción</label>
              <input id="desc" className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value={formData.description} onChange={e => setFormData({...formData, description: e.target.value})} />
            </div>
            <div className="grid gap-2">
              <label className="text-sm font-medium mb-2">Permisos</label>
              <div className="grid grid-cols-2 gap-2 max-h-[200px] overflow-y-auto border p-2 rounded-md">
                {initialPermissions.map(perm => (
                  <div key={perm.id} className="flex items-center space-x-2">
                    <input type="checkbox" id={`perm-${perm.id}`} checked={formData.permissions.includes(perm.id)} onChange={() => togglePermission(perm.id)} className="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary" />
                    <label htmlFor={`perm-${perm.id}`} className="text-sm leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">{perm.name}</label>
                  </div>
                ))}
              </div>
            </div>
          </div>
          <DialogFooter>
            <Button onClick={handleSave} disabled={loading}>{loading ? 'Guardando...' : 'Guardar'}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Dialog open={isPermDialogOpen} onOpenChange={setIsPermDialogOpen}>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle>Editar Permiso: {editingPermission?.name}</DialogTitle>
            <DialogDescription>Actualiza la descripción para que sea más clara para los usuarios.</DialogDescription>
          </DialogHeader>
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <label htmlFor="perm-desc" className="text-sm font-medium">Descripción</label>
              <input id="perm-desc" className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm" value={permFormData.description} onChange={e => setPermFormData({...permFormData, description: e.target.value})} />
            </div>
          </div>
          <DialogFooter>
            <Button onClick={handleSavePermission} disabled={loading}>{loading ? 'Guardando...' : 'Guardar'}</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  )
}