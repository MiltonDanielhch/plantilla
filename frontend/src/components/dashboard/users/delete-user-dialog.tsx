import { useState } from "react"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, DialogTrigger } from "../../ui/dialog"
import { Button } from "../../ui/button" // Asegúrate de tener button.tsx o usa html button
import { Trash2 } from "lucide-react"
import { api } from "../../../lib/api"
import { addToast } from "../../../stores/toast"

interface DeleteUserDialogProps {
  userId: number
  username: string
}

export function DeleteUserDialog({ userId, username }: DeleteUserDialogProps) {
  const [open, setOpen] = useState(false)
  const [loading, setLoading] = useState(false)

  const handleDelete = async () => {
    setLoading(true)
    try {
      await api.deleteUser(userId)
      // Recargar para reflejar cambios (o podrías usar lógica más compleja de estado)
      window.location.reload()
    } catch (error) {
      console.error("Error deleting user:", error)
      addToast({
        type: 'error',
        title: 'Error',
        message: 'No se pudo eliminar el usuario. Inténtalo de nuevo.'
      })
    } finally {
      setLoading(false)
      setOpen(false)
    }
  }

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <button className="text-destructive hover:text-destructive/80 text-sm font-medium transition-colors flex items-center gap-1">
           Eliminar
        </button>
      </DialogTrigger>
      <DialogContent className="sm:max-w-106.25">
        <DialogHeader>
          <DialogTitle>¿Eliminar usuario?</DialogTitle>
          <DialogDescription>
            Estás a punto de eliminar a <strong>{username}</strong>. Esta acción no se puede deshacer.
          </DialogDescription>
        </DialogHeader>
        <DialogFooter className="mt-4 gap-2 sm:gap-0">
          <Button variant="outline" onClick={() => setOpen(false)} disabled={loading}>
            Cancelar
          </Button>
          <Button variant="destructive" onClick={handleDelete} disabled={loading}>
            {loading ? "Eliminando..." : "Sí, eliminar"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  )
}
