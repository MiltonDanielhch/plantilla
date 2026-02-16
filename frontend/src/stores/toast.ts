import { atom } from 'nanostores';

export type ToastType = 'success' | 'error' | 'info' | 'warning';

export interface Toast {
  id: string;
  type: ToastType;
  title?: string;
  message: string;
  duration?: number;
}

export const $toasts = atom<Toast[]>([]);

export function addToast(toast: Omit<Toast, 'id'>) {
  const id = crypto.randomUUID();
  const newToast = { ...toast, id };
  $toasts.set([...$toasts.get(), newToast]);

  if (toast.duration !== 0) {
    setTimeout(() => {
      removeToast(id);
    }, toast.duration || 5000);
  }
}

export function removeToast(id: string) {
  $toasts.set($toasts.get().filter((t) => t.id !== id));
}